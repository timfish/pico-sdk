mod aux_io;
mod block;
mod capabilities;
mod digital;
mod ets;
mod extras;
mod led;
mod rapid_block;
mod siggen;
mod streaming;
mod trigger;

use super::{
    bandwidth_code, buffer_pointer,
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result, rate_limits, EnumerationResult,
    OscilloscopeDriverInternal, PicoStreamMode, SampleBuffer,
};
use parking_lot::RwLock;
use pico_common::{
    Downsample, DownsampleKind, Driver, FromPicoStr, OscilloscopeChannelConfig,
    OscilloscopeSampleConfig, PicoChannel, PicoError, PicoInfo, PicoRange, PicoResolution,
    PicoResult, PicoStatus, PowerSource, RateLimit, SampleType, ToPicoStr,
};
use pico_sys_dynamic::psospa::{
    enPicoAction_PICO_ADD, enPicoAction_PICO_CLEAR_ALL, enPicoBandwidthLimiter_PICO_BW_FULL,
    enPicoDataType_PICO_INT16_T, enPicoDeviceResolution, enPicoDeviceResolution_PICO_DR_8BIT,
    enPicoRatioMode_PICO_RATIO_MODE_RAW, enPicoTextFormat_PICO_JSON_DATA, PSOSPALoader,
    PICO_POINTER, PICO_STREAMING_DATA_INFO, PICO_STREAMING_DATA_TRIGGER_INFO,
    PICO_USB_POWER_DETAILS,
};
use std::{collections::HashMap, mem::MaybeUninit, sync::Arc};
use tinyjson::JsonValue;

fn parse_device_json(parsed_json: &JsonValue) -> Vec<PicoRange> {
    let range_settings: &Vec<_> = parsed_json["RangeSettings"]
        .get()
        .expect("Failed to parse JSON from Pico driver");

    let mut ranges = range_settings
        .iter()
        .flat_map(|r| {
            let probe_settings: &Vec<_> = r["ProbeSettings"]
                .get()
                .expect("Failed to parse JSON from Pico driver");

            probe_settings
                .iter()
                .filter_map(|probe_settings| {
                    let ty = probe_settings["Type"]
                        .get::<f64>()
                        .expect("Failed to parse JSON from Pico driver");
                    let max = probe_settings["Max"]
                        .get::<f64>()
                        .expect("Failed to parse JSON from Pico driver");

                    PicoRange::from_probe_and_nano_volts(*ty as u32, *max as i64)
                })
                .collect::<Vec<PicoRange>>()
        })
        .collect::<Vec<PicoRange>>();

    ranges.sort();
    ranges
}

/// The resolution a variant opens at: its default, else the first it
/// lists. Models differ, and open fails on one the model lacks.
fn default_resolution(details: &JsonValue) -> Option<enPicoDeviceResolution> {
    type Object = HashMap<String, JsonValue>;
    let details: &Object = details.get()?;
    let from_defaults = || {
        let defaults: &Object = details.get("Defaults")?.get()?;
        defaults.get("VerticalResolution")?.get::<f64>()
    };
    let first_listed = || {
        let listed: &Vec<JsonValue> = details.get("VerticalResolutions")?.get()?;
        let first: &Object = listed.first()?.get()?;
        first.get("Resolution")?.get::<f64>()
    };
    from_defaults()
        .or_else(first_listed)
        .map(|r| *r as enPicoDeviceResolution)
}

/// The number of analog channels the variant has.
fn analogue_channels(details: &JsonValue) -> Option<u8> {
    type Object = HashMap<String, JsonValue>;
    let details: &Object = details.get()?;
    let count = details.get("NumberOfAnalogueChannels")?.get::<f64>()?;
    Some(*count as u8)
}

/// Where a unit takes its power from, by what open reports of its USB-C
/// ports (psospa guide psospa-3, 3.29.1 and 3.29.2): a USB-C supply on
/// the power port, else the data port.
fn power_from_details(details: &PICO_USB_POWER_DETAILS) -> Option<PowerSource> {
    let power_port = details.powerPort_;
    let data_port = details.dataPort_;
    if power_port.valid_ != 0 && power_port.partnerConnected_ != 0 {
        Some(PowerSource::External)
    } else if data_port.valid_ != 0 {
        Some(PowerSource::Usb)
    } else {
        None
    }
}

/// The fastest rate with one channel on at each resolution the variant
/// lists (`MaxSampleRateHz`). The JSON gives no rate for more channels.
fn json_rate_limits(details: &JsonValue) -> Vec<RateLimit> {
    type Object = HashMap<String, JsonValue>;
    let listed = || -> Option<Vec<RateLimit>> {
        let details: &Object = details.get()?;
        let listed: &Vec<JsonValue> = details.get("VerticalResolutions")?.get()?;
        Some(
            listed
                .iter()
                .filter_map(|r| {
                    let r: &Object = r.get()?;
                    let value = *r.get("Resolution")?.get::<f64>()? as u32;
                    let rate = *r.get("MaxSampleRateHz")?.get::<f64>()?;
                    Some(RateLimit {
                        resolution: Some(PicoResolution::from_driver_value(value)?),
                        channels: 1,
                        max_samples_per_second: rate,
                    })
                })
                .collect(),
        )
    };
    listed().unwrap_or_default()
}

/// Every resolution the variant lists, in the listed order.
fn listed_resolutions(details: &JsonValue) -> Vec<enPicoDeviceResolution> {
    type Object = HashMap<String, JsonValue>;
    let listed = || -> Option<Vec<enPicoDeviceResolution>> {
        let details: &Object = details.get()?;
        let listed: &Vec<JsonValue> = details.get("VerticalResolutions")?.get()?;
        Some(
            listed
                .iter()
                .filter_map(|r| {
                    let r: &Object = r.get()?;
                    r.get("Resolution")?
                        .get::<f64>()
                        .map(|v| *v as enPicoDeviceResolution)
                })
                .collect(),
        )
    };
    listed().unwrap_or_default()
}

pub struct PSOSPADriver {
    _dependencies: LoadedDependencies,
    bindings: PSOSPALoader,
    /// The resolution each open handle was opened at. One driver serves
    /// every unit of the family, and units differ in what they support.
    resolutions: RwLock<HashMap<i16, enPicoDeviceResolution>>,
    /// Parsed `psospaGetVariantDetails` output, by variant name.
    variant_details: RwLock<HashMap<String, Arc<JsonValue>>>,
    /// The ratio mode and data type of each handle's buffers.
    streams: RwLock<HashMap<i16, PicoStreamMode>>,
    /// The power each handle reported at open.
    power: RwLock<HashMap<i16, PowerSource>>,
}

impl std::fmt::Debug for PSOSPADriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PSOspaDriver").finish()
    }
}

impl PSOSPADriver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let dependencies = load_dependencies(Driver::PSOSPA, path.as_ref());
        let bindings = unsafe { PSOSPALoader::new(path)? };
        Ok(PSOSPADriver {
            bindings,
            _dependencies: dependencies,
            resolutions: RwLock::new(HashMap::new()),
            variant_details: RwLock::new(HashMap::new()),
            streams: RwLock::new(HashMap::new()),
            power: RwLock::new(HashMap::new()),
        })
    }

    fn resolution(&self, handle: i16) -> enPicoDeviceResolution {
        self.resolutions
            .read()
            .get(&handle)
            .copied()
            .unwrap_or(enPicoDeviceResolution_PICO_DR_8BIT)
    }

    fn variant_details(&self, variant: &str) -> PicoResult<Arc<JsonValue>> {
        if let Some(details) = self.variant_details.read().get(variant) {
            return Ok(details.clone());
        }

        let variant_buf = variant.into_pico_i8_string();
        let mut json_buf = vec![0i8; 64 * 1024];
        loop {
            let mut json_buf_len = json_buf.len() as i32;
            let status = PicoStatus::from(unsafe {
                self.bindings.psospaGetVariantDetails(
                    variant_buf.as_ptr(),
                    variant_buf.len() as i16,
                    json_buf.as_mut_ptr(),
                    &mut json_buf_len,
                    enPicoTextFormat_PICO_JSON_DATA,
                )
            });
            match status {
                PicoStatus::OK => {
                    let json = json_buf.from_pico_i8_string(json_buf.len());
                    let details: JsonValue = json.parse().map_err(|_| {
                        PicoError::from_status(PicoStatus::INVALID_PARAMETER, "get_variant_details")
                    })?;
                    let details = Arc::new(details);
                    self.variant_details
                        .write()
                        .insert(variant.to_string(), details.clone());
                    return Ok(details);
                }
                PicoStatus::STRING_BUFFER_TO_SMALL if json_buf.len() < 16 * 1024 * 1024 => {
                    let wanted = (json_buf_len as usize + 1).max(json_buf.len() * 2);
                    json_buf = vec![0i8; wanted];
                }
                x => return Err(PicoError::from_status(x, "get_variant_details")),
            }
        }
    }
}

impl OscilloscopeDriverInternal for PSOSPADriver {
    fn get_driver(&self) -> Driver {
        Driver::PS3000A
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_version(&self) -> PicoResult<String> {
        let raw_version = self.get_unit_info(0, PicoInfo::DRIVER_VERSION)?;

        // On non-Windows platforms, the drivers return extra text before the
        // version string
        Ok(get_version_string(&raw_version))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_path(&self) -> PicoResult<Option<String>> {
        Ok(Some(self.get_unit_info(0, PicoInfo::DRIVER_PATH)?))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enumerate_units(&self) -> PicoResult<Vec<EnumerationResult>> {
        let mut device_count = 0;
        let mut serial_buf = "-v".into_pico_i8_string();
        serial_buf.extend(vec![0i8; 1000]);
        let mut serial_buf_len = serial_buf.len() as i16;

        let status = PicoStatus::from(unsafe {
            self.bindings.psospaEnumerateUnits(
                &mut device_count,
                serial_buf.as_mut_ptr(),
                &mut serial_buf_len,
            )
        });

        match status {
            PicoStatus::NOT_FOUND => Ok(Vec::new()),
            PicoStatus::OK => Ok(parse_enum_result(&serial_buf, serial_buf_len as usize)),
            x => Err(PicoError::from_status(x, "enumerate_units")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        // Open needs a resolution the variant supports, and only the
        // variant details say which, so find the unit's variant first.
        let unit = self
            .enumerate_units()?
            .into_iter()
            .find(|u| serial.is_none_or(|s| u.serial == s))
            .ok_or_else(|| PicoError::from_status(PicoStatus::NOT_FOUND, "open_unit"))?;
        let details = self.variant_details(&unit.variant)?;
        let resolution =
            default_resolution(&details).unwrap_or(enPicoDeviceResolution_PICO_DR_8BIT);

        let mut serial = unit.serial.as_str().into_pico_i8_string();
        let mut handle = -1i16;
        let mut details = MaybeUninit::<PICO_USB_POWER_DETAILS>::zeroed();
        let status = PicoStatus::from(unsafe {
            self.bindings.psospaOpenUnit(
                &mut handle,
                serial.as_mut_ptr(),
                resolution,
                details.as_mut_ptr(),
            )
        });
        // Zeroed, and the driver fills it on open, so it is always valid.
        let details = unsafe { details.assume_init() };

        match status {
            PicoStatus::OK => {
                self.resolutions.write().insert(handle, resolution);
                if let Some(power) = power_from_details(&details) {
                    self.power.write().insert(handle, power);
                }
                tracing::debug!(
                    power = ?power_from_details(&details),
                    data_port_mv = { details.dataPort_.busVoltagemV_ },
                    power_port_mv = { details.powerPort_.busVoltagemV_ },
                    "unit opened"
                );
                Ok(handle)
            }
            x => {
                if details.powerErrorLikely_ != 0 {
                    tracing::warn!(status = ?x, "open failed, likely for lack of power");
                }
                Err(PicoError::from_status(x, "open_unit"))
            }
        }
    }

    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaPingUnit(handle) }).to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        let mut min_value = 0;
        let mut max_value = 0;

        PicoStatus::from(unsafe {
            self.bindings.psospaGetAdcLimits(
                handle,
                self.resolution(handle),
                &mut min_value,
                &mut max_value,
            )
        })
        .to_result(max_value, "maximum_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        self.streams.write().remove(&handle);
        self.resolutions.write().remove(&handle);
        self.power.write().remove(&handle);
        PicoStatus::from(unsafe { self.bindings.psospaCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = 0;

        let status = PicoStatus::from(unsafe {
            self.bindings.psospaGetUnitInfo(
                handle,
                string_buf.as_mut_ptr(),
                string_buf.len() as i16,
                &mut string_buf_out_len,
                info_type.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(string_buf.from_pico_i8_string(string_buf_out_len as usize)),
            x => Err(PicoError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        Ok(parse_device_json(&*self.variant_details(&variant)?))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_resolutions(&self, handle: i16) -> PicoResult<Vec<(PicoResolution, i16)>> {
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let details = self.variant_details(&variant)?;

        listed_resolutions(&details)
            .into_iter()
            .filter_map(|value| Some((value, PicoResolution::from_driver_value(value)?)))
            .map(|(value, resolution)| {
                let mut min_value = 0;
                let mut max_value = 0;
                PicoStatus::from(unsafe {
                    self.bindings
                        .psospaGetAdcLimits(handle, value, &mut min_value, &mut max_value)
                })
                .to_result((resolution, max_value), "get_resolutions")
            })
            .collect()
    }

    // Every resolution allows every channel; only the fastest rate
    // falls as channels go on (psospa-3, 2.7; 5000E and 3000E data
    // sheets, "Maximum sampling rate").
    #[tracing::instrument(level = "trace", skip(self))]
    fn get_rate_limits(&self, handle: i16) -> Vec<RateLimit> {
        let Ok(variant) = self.get_unit_info(handle, PicoInfo::VARIANT_INFO) else {
            return Vec::new();
        };
        let Ok(details) = self.variant_details(&variant) else {
            return Vec::new();
        };
        let resolutions: Vec<PicoResolution> = listed_resolutions(&details)
            .into_iter()
            .filter_map(PicoResolution::from_driver_value)
            .collect();
        if self.bindings.psospaGetMinimumTimebaseStateless.is_err() {
            return json_rate_limits(&details);
        }
        let channels = analogue_channels(&details).unwrap_or(0);
        rate_limits(&resolutions, channels, |flags, resolution| {
            let mut timebase = 0u32;
            let mut interval = 0f64;
            let status = PicoStatus::from(unsafe {
                self.bindings.psospaGetMinimumTimebaseStateless(
                    handle,
                    flags,
                    &mut timebase,
                    &mut interval,
                    resolution.to_driver_value(),
                )
            });
            (status == PicoStatus::OK).then_some(interval)
        })
    }

    fn get_power_source(&self, handle: i16) -> Option<PowerSource> {
        self.power.read().get(&handle).copied()
    }

    fn get_resolution(&self, handle: i16) -> Option<PicoResolution> {
        let mut value = 0;
        let status = PicoStatus::from(unsafe {
            self.bindings.psospaGetDeviceResolution(handle, &mut value)
        });
        if status == PicoStatus::OK {
            PicoResolution::from_driver_value(value)
        } else {
            PicoResolution::from_driver_value(self.resolution(handle))
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn set_resolution(&self, handle: i16, resolution: PicoResolution) -> PicoResult<()> {
        let value = resolution.to_driver_value();
        PicoStatus::from(unsafe { self.bindings.psospaSetDeviceResolution(handle, value) })
            .to_result((), "set_resolution")?;
        self.resolutions.write().insert(handle, value);
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn capabilities(&self, handle: i16) -> PicoResult<pico_common::ScopeCapabilities> {
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let mut capabilities = capabilities::of(&variant);
        capabilities.channel_leds = led::has_channel_leds(self, handle);
        Ok(capabilities)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &OscilloscopeChannelConfig,
    ) -> PicoResult<()> {
        let bandwidth = bandwidth_code(
            config.bandwidth,
            &self.capabilities(handle)?.bandwidth_limits,
            enPicoBandwidthLimiter_PICO_BW_FULL,
            &capabilities::LIMITER_CODES,
        )?;
        PicoStatus::from(unsafe {
            self.bindings.psospaSetChannelOn(
                handle,
                channel.into(),
                config.coupling.into(),
                -config.range.to_nano_volts(),
                config.range.to_nano_volts(),
                config.range.to_probe_range(),
                config.offset,
                bandwidth,
            )
        })
        .to_result((), "enable_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaSetChannelOff(handle, channel.into()) })
            .to_result((), "disable_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn clear_data_buffers(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.psospaSetDataBuffer(
                handle,
                PicoChannel::A.into(),
                std::ptr::null_mut() as PICO_POINTER,
                0,
                enPicoDataType_PICO_INT16_T,
                0,
                enPicoRatioMode_PICO_RATIO_MODE_RAW,
                enPicoAction_PICO_CLEAR_ALL,
            )
        })
        .to_result((), "clear_data_buffers")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaStop(handle) }).to_result((), "stop")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_sys_dynamic::psospa::{enPicoDeviceResolution_PICO_DR_16BIT, PICO_USB_POWER_DELIVERY};

    /// What psospaGetVariantDetails gave for the 5464E MSO (serial
    /// 13474/0004) on 2026-10-14.
    fn ps5464e_mso() -> JsonValue {
        include_str!("../fixtures/psospa-5464E+MSO.json")
            .parse()
            .unwrap()
    }

    #[test]
    fn the_5464e_lists_8_and_16_bit_on_four_channels() {
        let details = ps5464e_mso();
        assert_eq!(
            listed_resolutions(&details),
            vec![
                enPicoDeviceResolution_PICO_DR_8BIT,
                enPicoDeviceResolution_PICO_DR_16BIT
            ]
        );
        assert_eq!(
            default_resolution(&details),
            Some(enPicoDeviceResolution_PICO_DR_16BIT)
        );
        assert_eq!(analogue_channels(&details), Some(4));
        assert_eq!(parse_device_json(&details).len(), 24);
    }

    /// The JSON gives each resolution's fastest rate with one channel
    /// and no limit on channels.
    #[test]
    fn the_5464e_json_gives_one_channel_rates() {
        let limits = json_rate_limits(&ps5464e_mso());
        assert_eq!(
            limits,
            vec![
                RateLimit {
                    resolution: Some(PicoResolution::DR_8BIT),
                    channels: 1,
                    max_samples_per_second: 5e9,
                },
                RateLimit {
                    resolution: Some(PicoResolution::DR_16BIT),
                    channels: 1,
                    max_samples_per_second: 2.5e9,
                },
            ]
        );
    }

    /// The driver's shortest intervals on the 5464E MSO, 2026-10-14:
    /// halved with each doubling of channels, as the data sheet lists.
    #[test]
    fn rate_limits_halve_as_channels_go_on() {
        let intervals = |flags: u32, resolution: PicoResolution| {
            let base = match resolution {
                PicoResolution::DR_8BIT => 0.2e-9,
                _ => 0.4e-9,
            };
            Some(match flags.count_ones() {
                1 => base,
                2 => base * 2.0,
                _ => base * 4.0,
            })
        };
        let limits = rate_limits(
            &[PicoResolution::DR_8BIT, PicoResolution::DR_16BIT],
            4,
            intervals,
        );
        let rate = |r, n| {
            limits
                .iter()
                .find(|l| l.resolution == Some(r) && l.channels == n)
                .map(|l| l.max_samples_per_second.round())
        };
        assert_eq!(rate(PicoResolution::DR_8BIT, 1), Some(5e9));
        assert_eq!(rate(PicoResolution::DR_8BIT, 2), Some(2.5e9));
        assert_eq!(rate(PicoResolution::DR_8BIT, 4), Some(1.25e9));
        assert_eq!(rate(PicoResolution::DR_16BIT, 3), Some(625e6));
        assert_eq!(limits.len(), 8);
    }

    fn port(valid: u8, partner: u8) -> PICO_USB_POWER_DELIVERY {
        PICO_USB_POWER_DELIVERY {
            valid_: valid,
            busVoltagemV_: 5000,
            rpCurrentLimitmA_: 3000,
            partnerConnected_: partner,
            ccPolarity_: 0,
            attachedDevice_: 0,
            contractExists_: 0,
            currentPdo_: 0,
            currentRdo_: 0,
        }
    }

    #[test]
    fn power_follows_the_usb_c_ports() {
        let details = |data, power| PICO_USB_POWER_DETAILS {
            powerErrorLikely_: 0,
            dataPort_: data,
            powerPort_: power,
        };
        assert_eq!(
            power_from_details(&details(port(1, 1), port(1, 1))),
            Some(PowerSource::External)
        );
        assert_eq!(
            power_from_details(&details(port(1, 1), port(0, 0))),
            Some(PowerSource::Usb)
        );
        assert_eq!(power_from_details(&details(port(0, 0), port(0, 0))), None);
    }
}
