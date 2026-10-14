# Scope limits and power

What Pico's published guides and data sheets say about which resolution, channel and rate combinations each scope family runs, and which channels need the external supply, and what this crate does with it. Gathered 2026-10-14.

## Sources

- **G3** ps3000apg.en-15, PicoScope 3000 Series (A API): https://www.picotech.com/download/manuals/picoscope-3000-series-a-api-programmers-guide.pdf
- **G5** ps4000apg.en-9, PicoScope 4000 Series (A API): https://www.picotech.com/download/manuals/picoscope-4000-series-a-api-programmers-guide.pdf, and the online pages https://www.picotech.com/helpfiles/4000a-api/ps4000achangepowersource.html and https://www.picotech.com/helpfiles/4000a-api/ps4000acurrentpowersource.html
- **G6** ps5000apg.en r4, PicoScope 5000 Series (A API): https://www.picotech.com/download/manuals/picoscope-5000-series-a-api-programmers-guide.pdf
- **G8** ps6000apg-9, PicoScope 6000 Series (A API): https://www.picotech.com/download/manuals/picoscope-6000-series-a-api-programmers-guide.pdf
- **G9** psospa-3, PicoScope 3000E Series (psospa API): https://www.picotech.com/download/manuals/picoscope-3000e-series-psospa-programmers-guide.pdf
- **D1** PicoScope 3000D Series data sheet: https://www.picotech.com/download/datasheets/picoscope-3000d-series-data-sheet.pdf
- **D2** PicoScope 5000D Series data sheet: https://www.picotech.com/download/datasheets/picoscope-5000d-series-data-sheet.pdf
- **D3** PicoScope 3000E Series data sheet: https://www.picotech.com/download/datasheets/picoscope-3000e-series-data-sheet.pdf
- **D4** PicoScope 5000E Series data sheet: https://www.picotech.com/download/datasheets/picoscope-5000e-series-data-sheet.pdf
- **F1** Pico forum, 5442D: "Channels C and D will only operate if you have the power supply plugged in." https://www.picotech.com/support/viewtopic.php?p=145468 (and the same for a 5444D MSO, https://www.picotech.com/support/viewtopic.php?p=146881)
- **H1** `psospaGetVariantDetails` and `psospaGetMinimumTimebaseStateless` on a PicoScope 5464E MSO (serial 13474/0004), driver cache `drivers-2026.07.22`, 2026-10-14.

## Resolution, channels and rate

"Channel" in the rate limits means an analog channel or an 8-bit digital port (D2, D3, D4). Rates are the fastest real-time (block) rates. Streaming to the host is slower and shares the USB link between channels.

| Family | Resolutions | Channels per resolution | Fastest rate by channels | This crate |
|---|---|---|---|---|
| ps2000, ps2000a, ps3000a, ps4000, ps6000 | Fixed (8 bit; 4262 16 bit) | No limit | Falls with channels, per model data sheet (3000D: 1 GS/s on 1, 500 MS/s on 2, 250 MS/s on 3 or 4, D1) | No resolution or rate data. |
| ps4000a | 4000A, 4824: fixed 12 bit. 4444: 12 or 14 bit (G5 4.51) | No limit stated | 4444: timebases 0 to 2 (400, 200, 100 MS/s) in 12-bit mode only (G5 3.7) | No 4444 resolution support yet. |
| ps5000a | 8, 12, 14, 15, 16 bit (G6 4.49) | 15 bit: 2; 16 bit: 1 (G6 4.49) | 8 bit 1 GS/s, 12 bit 500 MS/s, 14 and 15 bit 125 MS/s, 16 bit 62.5 MS/s, each with one channel, halving with more channels at 8 and 12 bit (G6 3.6, D2) | `channel_limits` from the guide; `rate_limits` from `ps5000aGetMinimumTimebaseStateless` for the first 1 to n channels at each resolution. Not checked on hardware. |
| ps6000a | FlexRes models (6424E, 6824E, 6428E-D): 8, 10, 12 bit; others 8 bit | 12 bit: 2 (G8 3.47) | Per model data sheet | `channel_limits` from the guide; `rate_limits` from `ps6000aGetMinimumTimebaseStateless`. Not checked on hardware. |
| psospa (3000E, 5000E) | Per variant JSON: 3000E 8 and 10 bit; 5000E 16 bit; 5000E+ 8 and 16 bit | No limit: every resolution runs every channel (D3, D4, H1) | Halves from 1 to 2 channels and again to 3 or 4 (D3, D4, H1; table below) | No `channel_limits`. `rate_limits` from `psospaGetMinimumTimebaseStateless`, or the JSON's one-channel `MaxSampleRateHz` where the driver lacks that call. |

The 5464E MSO gave these shortest intervals from `psospaGetMinimumTimebaseStateless` (H1), which match D4:

| Channels on | 8 bit | 16 bit |
|---|---|---|
| A | 200 ps (5 GS/s) | 400 ps (2.5 GS/s) |
| A, B | 400 ps (2.5 GS/s) | 800 ps (1.25 GS/s) |
| A to C, or A to D | 800 ps (1.25 GS/s) | 1.6 ns (625 MS/s) |
| Port 0 or ports 0 and 1 alone | 800 ps | refused: `PORTS_WITHOUT_ANALOGUE_CHANNELS_ONLY_ALLOWED_IN_8BIT_RESOLUTION` |
| A to D and ports | 1.6 ns | 3.2 ns |

10 and 12 bit were refused with `RESOLUTION_NOT_SUPPORTED_BY_VARIANT`, as the variant lists only 8 and 16 bit. `psospaChannelCombinationsStateless` is in the header but not in this driver build (`dlsym` found no symbol).

What the variant JSON holds (H1): per resolution in `VerticalResolutions`, `Resolution` (driver code), `MaxSampleRateHz` (one channel), `MaxTimebaseps` (the slowest interval, 1.677 ms), `ValuesRange` (the ADC limits) and, per range, `HighestBandwidthLimitAvailable`; and `NumberOfAnalogueChannels`, `NumberOfDigitalPorts`, `BandwidthLimits`, `CouplingTypes`, `Defaults`, `MemoryDepth`, `RangeSettings` (ranges per probe with offset limits). It holds no limit on channels per resolution, and no rate for more than one channel. `GetVariantDetails` needs no open unit. The variant names the driver lists (`all-series`) are 3415E to 3418E and their MSOs, and 5462E to 5464E, their MSOs and their `+` models.

Streaming rates from the data sheets, for reference (host dependent): 5000E 16 bit, PicoSDK, USB 5 Gb/s: about 150, 75, 38 and 18 MS/s on 1, 2, 3 or 4 and more than 4 channels; 5000E+ 8 bit twice that (D4). 3000E on USB 2: 8 bit about 30, 15, 8 and 4 MS/s (D3). 5000D on USB 3: 125 MS/s at 8 bit or 62.5 MS/s at 12 to 16 bit, shared between channels; on USB 2 about 30 and 15 MS/s (D2).

## Power

| Family | Power choices | Channels on USB power | At open | When the supply is plugged in or out |
|---|---|---|---|---|
| ps3000a | 4-channel: USB, or USB and the AC adaptor; 2-channel USB 3 models on a USB 2 port need a two-step power up (G3 3.1, 4.2) | Not stated. D1 says only that the adaptor "may be required" on a USB port that gives less than 1200 mA. | `PICO_POWER_SUPPLY_NOT_CONNECTED` (4-channel) or `PICO_USB3_0_DEVICE_NON_USB3_0_PORT` (2-channel), then `ChangePowerSource` with the same status (G3 4.2, 4.36) | The driver returns the status; call `ChangePowerSource` with it. Data and settings are lost; set the unit up again (G3 3.1, 4.2) |
| ps4000a | 4000A, 4824: USB 3, or a double USB 2 cable. 4444: USB, or DC supply for probes that draw more current (G5 4.1) | All | 4000A, 4824: `PICO_USB3_0_DEVICE_NON_USB3_0_PORT` on a non-USB 3 port. 4444: `PICO_POWER_SUPPLY_NOT_CONNECTED` (G5 4.37) | As ps3000a (G5 4.1). `CurrentPowerSource` applies to the 4444 only; the others return OK (G5 4.4) |
| ps5000a | 5000A/B: AC adaptor, or double USB cable; 5000D 4-channel: USB port of at least 1200 mA, or the AC adaptor (G6 3.7, D2) | 4-channel models: A and B (and digital ports) only. "When three or four channels are enabled, you must use the AC adaptor." (G6 3.7, 4.1, 4.37; D2; F1) | `PICO_POWER_SUPPLY_NOT_CONNECTED` (4-channel) or `PICO_USB3_0_DEVICE_NON_USB3_0_PORT` (2-channel 5000D on USB 2) | The driver returns the status; call `ChangePowerSource`. On a change to USB power the driver turns C and D off and removes their triggers (G6 4.1). The device restarts and unsaved data is lost (G6 3.7) |
| ps6000a | Mains supply only | All | Nothing | Nothing |
| psospa | USB-C port of 3 A, or a USB port and a 5 V 3 A USB-C supply on the power port (D3, D4) | All | `psospaOpenUnit` fills `PICO_USB_POWER_DETAILS`: each port's voltage, current limit and whether a USB-C partner is on it; `PICO_POWER_MANAGER` when "power setup is invalid" (G9 3.29, 3.29.1, 3.29.2) | Not stated |
| ps2000, ps2000a, ps4000, ps6000 | One choice each (USB, or mains for the 6000) | All | Nothing | Nothing |

### A USB 3 scope on a USB 2 port

The guides treat it as a power mode only (G3 4.2, G5 4.1, G6 3.7): call `ChangePowerSource` with `PICO_USB3_0_DEVICE_NON_USB3_0_PORT`. It limits no channels. It limits streaming speed (D2, D3).

## What this crate does

- `OscilloscopeInfo.power` is the `PowerSource` (`External`, `Usb` or `UsbNonUsb3Port`) the unit runs on, or `None` for a unit with no choice. ps3000a, ps4000a and ps5000a read it from `CurrentPowerSource`, and keep the open status where that returns OK (a 4000A, or a 2-channel unit on a USB 2 port). psospa reads it from the open's power details: `External` with a USB-C partner on the power port, else `Usb`.
- `OscilloscopeInfo.supply_channels` lists the channels that run only on the external supply: C and D on 4-channel ps5000a units. `available_channels()` leaves them out unless the power is `External`. `valid_channel_ranges` still lists every channel the unit has.
- `check_config` (which `start` calls) refuses a config that turns on a channel the power does not run with `PICO_CHANNEL_DISABLED_DUE_TO_USB_POWERED`, then a resolution that allows fewer channels than are on with `PICO_INVALID_NUMBER_CHANNELS_FOR_RESOLUTION`.
- `OscilloscopeInfo.rate_limits` holds the fastest rate for each resolution and count of channels (the first n, from A), and `max_samples_per_second_for(config)` gives the one for a config.
- When a call returns a power status while the unit is open or streaming, the scope device calls `ChangePowerSource` with it, reads the new power into the info and sends `DeviceNotice::PowerChanged { from, to, available, stream }` on the runner's `notices`. `stream` is `Idle` (nothing streamed), `Restarts` (the new power runs the channels on, so the runner starts the stream again; the samples while it was down are lost) or `Stopped` (a channel on needs the lost power: the runner stops and does not try again; start it with channels from `available`). A status that names the power the unit has already is a refusal, not a change, and is handled as any other error.
- An open that returns `PICO_POWER_SUPPLY_CONNECTED`, as a 4444 with its supply does (G5 4.37), counts as open on the supply. Before, it failed the open.
- The runner now checks a config with `StreamDevice::check` before each start, and stops (target `Open`) rather than tries again every 500 ms when the check refuses it. This also stops a refused resolution from being tried again in the background.

## Needs hardware

- A 4-channel 5000D (or 5000A/B) on USB power and on its adaptor: the open status, `CurrentPowerSource`, C refused on USB power, and the adaptor plugged in and out while streaming (which call returns the status, and whether `ChangePowerSource` brings the stream back).
- A 4-channel 3000D on a USB port under 1200 mA: whether the driver refuses C and D (and with which status), so `supply_channels` can list them for ps3000a. Now none are listed.
- A 4000A on a USB 2 port with a double cable, and a 4444 with and without its supply.
- A 5000D and a 6000E for `rate_limits` from the stateless calls.
- A 3000E or 5000E on its USB-C supply, to see `power` read `External`. The 5464E MSO on a USB 2 port with no supply read `Usb` (2026-10-14).

## The 5464E MSO variant JSON

`psospaGetVariantDetails("5464E+MSO", PICO_JSON_DATA)`, 2026-10-14 (H1). The tests read the same file, `driver/src/oscilloscope/fixtures/psospa-5464E+MSO.json`.

<details>
<summary>JSON</summary>

```json
{
  "ArbitraryWaveformGenerator": {
    "NumValuesRange": [
      1,
      32768
    ],
    "TimePerCountPs": 5000,
    "ValuesRange": [
      -32767,
      32767
    ]
  },
  "AuxInputOutput": {
    "EtsSource": false,
    "TriggerInput": true,
    "TriggerRangeuV": [
      1650000,
      1650000
    ],
    "TriggerResolution": -1,
    "TriggerStepuV": 0
  },
  "BandwidthLimits": [
    20000000,
    60000000,
    100000000,
    200000000,
    350000000,
    500000000
  ],
  "CouplingTypes": [
    0,
    1,
    50
  ],
  "Defaults": {
    "BandwidthLimit": 200000000,
    "CouplingType": 0,
    "Range": 10,
    "VerticalResolution": 4
  },
  "DigitalPortSettings": {
    "InputRangeV": {
      "Max": 20.0,
      "Min": -20.0
    },
    "NumberOfDigitalChannelsPerPort": 8,
    "ThresholdRangeV": {
      "Max": 5.0,
      "Min": -5.0
    }
  },
  "EquivalentTimeSampling": {
    "AnalogueSources": [
      0,
      1,
      2,
      3
    ],
    "MaxCycles": 500,
    "MaxInterleave": 40
  },
  "HandleValue": 1,
  "MemoryDepth": 2147483648,
  "NumberOfAnalogueChannels": 4,
  "NumberOfDigitalPorts": 2,
  "RangeSettings": [
    {
      "ProbeSettings": [
        {
          "Max": 5000000,
          "Min": -5000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 1,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 50,
              "Max": 0.025,
              "Min": -0.025
            }
          ],
          "Type": 0
        },
        {
          "Max": 5000000,
          "Min": -5000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 1,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 50,
              "Max": 0.025,
              "Min": -0.025
            }
          ],
          "Type": 1
        },
        {
          "Max": 50000000,
          "Min": -50000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 1,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 50,
              "Max": 0.25,
              "Min": -0.25
            }
          ],
          "Type": 10
        }
      ],
      "Range": 0
    },
    {
      "ProbeSettings": [
        {
          "Max": 10000000,
          "Min": -10000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 1,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 50,
              "Max": 0.025,
              "Min": -0.025
            }
          ],
          "Type": 0
        },
        {
          "Max": 10000000,
          "Min": -10000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 1,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 50,
              "Max": 0.025,
              "Min": -0.025
            }
          ],
          "Type": 1
        },
        {
          "Max": 100000000,
          "Min": -100000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 1,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 50,
              "Max": 0.25,
              "Min": -0.25
            }
          ],
          "Type": 10
        }
      ],
      "Range": 1
    },
    {
      "ProbeSettings": [
        {
          "Max": 20000000,
          "Min": -20000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 1,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 50,
              "Max": 0.025,
              "Min": -0.025
            }
          ],
          "Type": 0
        },
        {
          "Max": 20000000,
          "Min": -20000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 1,
              "Max": 0.025,
              "Min": -0.025
            },
            {
              "Coupling": 50,
              "Max": 0.025,
              "Min": -0.025
            }
          ],
          "Type": 1
        },
        {
          "Max": 200000000,
          "Min": -200000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 1,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 50,
              "Max": 0.25,
              "Min": -0.25
            }
          ],
          "Type": 10
        }
      ],
      "Range": 2
    },
    {
      "ProbeSettings": [
        {
          "Max": 50000000,
          "Min": -50000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 1,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 50,
              "Max": 0.25,
              "Min": -0.25
            }
          ],
          "Type": 0
        },
        {
          "Max": 50000000,
          "Min": -50000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 1,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 50,
              "Max": 0.25,
              "Min": -0.25
            }
          ],
          "Type": 1
        },
        {
          "Max": 500000000,
          "Min": -500000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 1,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 50,
              "Max": 2.5,
              "Min": -2.5
            }
          ],
          "Type": 10
        }
      ],
      "Range": 3
    },
    {
      "ProbeSettings": [
        {
          "Max": 100000000,
          "Min": -100000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 1,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 50,
              "Max": 0.25,
              "Min": -0.25
            }
          ],
          "Type": 0
        },
        {
          "Max": 100000000,
          "Min": -100000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 1,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 50,
              "Max": 0.25,
              "Min": -0.25
            }
          ],
          "Type": 1
        },
        {
          "Max": 1000000000,
          "Min": -1000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 1,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 50,
              "Max": 2.5,
              "Min": -2.5
            }
          ],
          "Type": 10
        }
      ],
      "Range": 4
    },
    {
      "ProbeSettings": [
        {
          "Max": 200000000,
          "Min": -200000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 1,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 50,
              "Max": 0.25,
              "Min": -0.25
            }
          ],
          "Type": 0
        },
        {
          "Max": 200000000,
          "Min": -200000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 1,
              "Max": 0.25,
              "Min": -0.25
            },
            {
              "Coupling": 50,
              "Max": 0.25,
              "Min": -0.25
            }
          ],
          "Type": 1
        },
        {
          "Max": 2000000000,
          "Min": -2000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 1,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 50,
              "Max": 2.5,
              "Min": -2.5
            }
          ],
          "Type": 10
        }
      ],
      "Range": 5
    },
    {
      "ProbeSettings": [
        {
          "Max": 500000000,
          "Min": -500000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 1,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 50,
              "Max": 2.5,
              "Min": -2.5
            }
          ],
          "Type": 0
        },
        {
          "Max": 500000000,
          "Min": -500000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 1,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 50,
              "Max": 2.5,
              "Min": -2.5
            }
          ],
          "Type": 1
        },
        {
          "Max": 5000000000,
          "Min": -5000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 25.0,
              "Min": -25.0
            },
            {
              "Coupling": 1,
              "Max": 25.0,
              "Min": -25.0
            },
            {
              "Coupling": 50,
              "Max": 25.0,
              "Min": -25.0
            }
          ],
          "Type": 10
        }
      ],
      "Range": 6
    },
    {
      "ProbeSettings": [
        {
          "Max": 1000000000,
          "Min": -1000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 1,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 50,
              "Max": 2.5,
              "Min": -2.5
            }
          ],
          "Type": 0
        },
        {
          "Max": 1000000000,
          "Min": -1000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 1,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 50,
              "Max": 2.5,
              "Min": -2.5
            }
          ],
          "Type": 1
        },
        {
          "Max": 10000000000,
          "Min": -10000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 25.0,
              "Min": -25.0
            },
            {
              "Coupling": 1,
              "Max": 25.0,
              "Min": -25.0
            },
            {
              "Coupling": 50,
              "Max": 25.0,
              "Min": -25.0
            }
          ],
          "Type": 10
        }
      ],
      "Range": 7
    },
    {
      "ProbeSettings": [
        {
          "Max": 2000000000,
          "Min": -2000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 1,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 50,
              "Max": 2.5,
              "Min": -2.5
            }
          ],
          "Type": 0
        },
        {
          "Max": 2000000000,
          "Min": -2000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 1,
              "Max": 2.5,
              "Min": -2.5
            },
            {
              "Coupling": 50,
              "Max": 2.5,
              "Min": -2.5
            }
          ],
          "Type": 1
        },
        {
          "Max": 20000000000,
          "Min": -20000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 25.0,
              "Min": -25.0
            },
            {
              "Coupling": 1,
              "Max": 25.0,
              "Min": -25.0
            },
            {
              "Coupling": 50,
              "Max": 25.0,
              "Min": -25.0
            }
          ],
          "Type": 10
        }
      ],
      "Range": 8
    },
    {
      "ProbeSettings": [
        {
          "Max": 5000000000,
          "Min": -5000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 20.0,
              "Min": -20.0
            },
            {
              "Coupling": 1,
              "Max": 20.0,
              "Min": -20.0
            },
            {
              "Coupling": 50,
              "Max": 5.0,
              "Min": -5.0
            }
          ],
          "Type": 0
        },
        {
          "Max": 5000000000,
          "Min": -5000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 20.0,
              "Min": -20.0
            },
            {
              "Coupling": 1,
              "Max": 20.0,
              "Min": -20.0
            },
            {
              "Coupling": 50,
              "Max": 5.0,
              "Min": -5.0
            }
          ],
          "Type": 1
        },
        {
          "Max": 50000000000,
          "Min": -50000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 200.0,
              "Min": -200.0
            },
            {
              "Coupling": 1,
              "Max": 200.0,
              "Min": -200.0
            },
            {
              "Coupling": 50,
              "Max": 50.0,
              "Min": -50.0
            }
          ],
          "Type": 10
        }
      ],
      "Range": 9
    },
    {
      "ProbeSettings": [
        {
          "Max": 10000000000,
          "Min": -10000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 20.0,
              "Min": -20.0
            },
            {
              "Coupling": 1,
              "Max": 20.0,
              "Min": -20.0
            }
          ],
          "Type": 0
        },
        {
          "Max": 10000000000,
          "Min": -10000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 20.0,
              "Min": -20.0
            },
            {
              "Coupling": 1,
              "Max": 20.0,
              "Min": -20.0
            }
          ],
          "Type": 1
        },
        {
          "Max": 100000000000,
          "Min": -100000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 200.0,
              "Min": -200.0
            },
            {
              "Coupling": 1,
              "Max": 200.0,
              "Min": -200.0
            }
          ],
          "Type": 10
        }
      ],
      "Range": 10
    },
    {
      "ProbeSettings": [
        {
          "Max": 20000000000,
          "Min": -20000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 20.0,
              "Min": -20.0
            },
            {
              "Coupling": 1,
              "Max": 20.0,
              "Min": -20.0
            }
          ],
          "Type": 0
        },
        {
          "Max": 20000000000,
          "Min": -20000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 20.0,
              "Min": -20.0
            },
            {
              "Coupling": 1,
              "Max": 20.0,
              "Min": -20.0
            }
          ],
          "Type": 1
        },
        {
          "Max": 200000000000,
          "Min": -200000000000,
          "OffsetLimits": [
            {
              "Coupling": 0,
              "Max": 200.0,
              "Min": -200.0
            },
            {
              "Coupling": 1,
              "Max": 200.0,
              "Min": -200.0
            }
          ],
          "Type": 10
        }
      ],
      "Range": 11
    }
  ],
  "Series": "5000E",
  "VariantName": "5464E+MSO",
  "VerticalResolutions": [
    {
      "MaxSampleRateHz": 5000000000,
      "MaxTimebaseps": 1677312000,
      "Ranges": [
        {
          "HighestBandwidthLimitAvailable": 100000000,
          "Range": 0
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 1
        },
        {
          "HighestBandwidthLimitAvailable": 350000000,
          "Range": 2
        },
        {
          "HighestBandwidthLimitAvailable": 500000000,
          "Range": 3
        },
        {
          "HighestBandwidthLimitAvailable": 500000000,
          "Range": 4
        },
        {
          "HighestBandwidthLimitAvailable": 500000000,
          "Range": 5
        },
        {
          "HighestBandwidthLimitAvailable": 500000000,
          "Range": 6
        },
        {
          "HighestBandwidthLimitAvailable": 500000000,
          "Range": 7
        },
        {
          "HighestBandwidthLimitAvailable": 500000000,
          "Range": 8
        },
        {
          "HighestBandwidthLimitAvailable": 500000000,
          "Range": 9
        },
        {
          "HighestBandwidthLimitAvailable": 500000000,
          "Range": 10
        },
        {
          "HighestBandwidthLimitAvailable": 500000000,
          "Range": 11
        }
      ],
      "Resolution": 0,
      "TriggerResolution": 256,
      "ValuesRange": [
        -32512,
        32512
      ]
    },
    {
      "MaxSampleRateHz": 2500000000,
      "MaxTimebaseps": 1677312000,
      "Ranges": [
        {
          "HighestBandwidthLimitAvailable": 100000000,
          "Range": 0
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 1
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 2
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 3
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 4
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 5
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 6
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 7
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 8
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 9
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 10
        },
        {
          "HighestBandwidthLimitAvailable": 200000000,
          "Range": 11
        }
      ],
      "Resolution": 4,
      "TriggerResolution": 65536,
      "ValuesRange": [
        -32767,
        32767
      ]
    }
  ]
}
```

</details>
