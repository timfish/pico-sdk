# Streaming downsampling and sample types

What Pico's published programmer's guides say about streaming downsampling, data types and maximum ADC values, per driver family, and what this crate does with it. Page numbers are PDF pages.

## Guides

- **G1** ps2000pg.en-12, PicoScope 2000 Series: https://www.picotech.com/download/manuals/picoscope-2000-series-programmers-guide.pdf
- **G2** ps2000apg.en-12, PicoScope 2000 Series (A API): https://www.picotech.com/download/manuals/picoscope-2000-series-a-api-programmers-guide.pdf
- **G3** ps3000apg.en-15, PicoScope 3000 Series (A API): https://www.picotech.com/download/manuals/picoscope-3000-series-a-api-programmers-guide.pdf
- **G4** ps4000pg.en-10, PicoScope 4000 Series: https://www.picotech.com/download/manuals/picoscope-4000-series-programmers-guide.pdf
- **G5** ps4000apg.en-9, PicoScope 4000 Series (A API): https://www.picotech.com/download/manuals/picoscope-4000-series-a-api-programmers-guide.pdf
- **G6** ps5000apg.en r4, PicoScope 5000 Series (A API): https://www.picotech.com/download/manuals/picoscope-5000-series-a-api-programmers-guide.pdf
- **G7** ps6000pg.en-11, PicoScope 6000 Series: https://www.picotech.com/download/manuals/picoscope-6000-series-programmers-guide.pdf
- **G8** ps6000apg-9, PicoScope 6000 Series (A API): https://www.picotech.com/download/manuals/picoscope-6000-series-a-api-programmers-guide.pdf
- **G9** psospa-3, PicoScope 3000E Series (psospa API): https://www.picotech.com/download/manuals/picoscope-3000e-series-psospa-programmers-guide.pdf

## Per family

| Family | Where streaming downsampling runs | AVERAGE in streaming | SUM | int8 buffers | Max ADC value | This crate |
|---|---|---|---|---|---|---|
| ps2000 | Driver: aggregation merges samples in the driver (G1 §5.19 p39, glossary p63) | No: only min/max aggregation (`noOfSamplesPerAggregate`) | No | No | 32767 (G1 §2.3 p8) | Refuses any downsample (`RATIO_MODE_NOT_SUPPORTED`). Not a multiple of 256, so samples stay `I16`. |
| ps2000a | Not stated; RunStreaming says data is downsampled after it is collected from the device (G2 §3.18.1 p50, §3.38 p74), so the driver | Yes (G2 p49–50) | No | No | 32512 (G2 §2.3 p11) | AVERAGE (code 4). 8-bit raw as `I8` by `>> 8`. |
| ps3000a | Conflict: "dedicated hardware inside the scope" (G3 §4.22.1 p56), but RunStreaming says after collection (§4.41 p77) | Yes | No | No | 32512 (G3 §3.2 p12) | AVERAGE (code 4). 8-bit raw as `I8` by `>> 8`. |
| ps4000 | Driver, implied: aggregated after collection (G4 §5.37–5.38 p66–68) | Yes, only through `ps4000RunStreamingEx` and `ps4000SetDataBufferWithMode` (G4 p43, p68) | No | No | 32764; 4262: 32767 (G4 §2.2 p10) | AVERAGE (code 2) through the Ex calls; raw streams keep the plain calls. `I16`. |
| ps4000a | Conflict: "done by firmware on the device" (G5 §3.5 p13), but RunStreaming says after collection (§4.45 p81) | Yes (code 4) | No | No | 32767 (G5 §3.2 p11) | AVERAGE (code 4). `I16`. |
| ps5000a | Driver: "done at high speed by the driver" (G6 §4.22.1 p59) | Yes | No | No | 8-bit 32512; 12 to 16-bit 32767 (G6 §3.2 p11) | AVERAGE (code 4). At 8 bits, raw as `I8` by `>> 8`; else `I16`. |
| ps6000 | Conflict as ps3000a (G7 §3.16.1 p44 against §3.33 p63) | Yes (code 2) | No | No | 32512 (G7 §2.3 p10) | AVERAGE (code 2). 8-bit raw as `I8` by `>> 8`. |
| ps6000a | Scope hardware: raw rate past USB with downsampling on, raw samples stay on the device (G8 §2.6 p13, §2.6.3 p20, §3.21.1 p55, §3.41 p79) | Yes | Enum value only (16), not documented | `PICO_INT8_T` listed for SetDataBuffer; which modes and resolutions take it is not stated (G8 §3.45 p85) | GetAdcLimits: 8-bit 32512, 10-bit 32704, 12-bit 32736 (G8 §2.3 p10, §3.7 p32) | AVERAGE into `INT16`, SUM into `INT32`, 8-bit raw into `INT8`. |
| psospa | Scope hardware, as ps6000a (G9 §2.6 p13, §2.6.3 p20, §3.17.1 p48, §3.36 p73) | Yes | Enum value only (16), not documented | As ps6000a (G9 §3.40 p79) | 8-bit 32512, 10-bit 32704 (G9 §2.3 p10, §3.4 p28) | As ps6000a. |

Buffers: in the older APIs `SetDataBuffer` takes one `int16_t` buffer and the ratio mode, for every mode but aggregation, which needs `SetDataBuffers` (G2 §3.40–3.41 p77–78, G3 §4.44–4.45 p81–82, G4 §5.41/5.43 p73–75, G5 §4.49–4.50 p87–88, G6 §4.47–4.48 p86–87, G7 §3.35/3.37 p68–70). `overviewBufferSize` is "the same as the bufferLth value passed to" SetDataBuffer (G2 p75, G3 p77, G4 p67/69, G6 p81, G7 p64), so this crate sizes both by the delivered (downsampled) rate. ps6000a and psospa have no overview buffer parameter.

## 8-bit samples

0x7F00 = 127 × 256, so 8-bit codes sit in the top byte of the 16-bit word (inferred from the max values; the guides do not say "shifted"). For an 8-bit stream with no downsample the crate delivers `Samples::I8`: from `INT8` buffers on ps6000a and psospa, else `>> 8` of the `i16` buffer when the unit's max is a multiple of 256. A code with low bits set logs one warning a stream. The ps2000 max is 32767, so it stays `I16`.

The multiplier of `I8` samples is the range over the 8-bit limit (`max >> 8`, 127). Averages stay `I16`, as the mean keeps a fraction of a code in the low byte.

## SUM

On a 5464E (psospa) the driver refuses SUM in streaming with `RATIO_MODE_NOT_SUPPORTED` (2026-10-14), so psospa and ps6000a offer AVERAGE only. The code below stays for a driver that streams SUM.

SUM gives `Samples::I32`, unscaled, with a multiplier of range / (max ADC × ratio), so `scale_sample` gives the group's mean and the sum keeps every bit. A scaled float would lose bits past 2^24 and an `I16` would overflow. The guides do not say whether the sum is of 16-bit-scaled values or raw codes, nor the ratio limit; the multiplier assumes 16-bit-scaled values (one of 32512 at 8 bits).

## Needs hardware

- Whether ps6000a and psospa take `INT8` buffers for RAW streaming at 8 bits, and what the codes are (±127 assumed). Pico's published Python streaming examples use `INT16` at 8 bits.
- SUM: refused in streaming on a 5464E (psospa). ps6000a not checked.
- Whether AVERAGE at 8 bits gives a fraction in the low byte on each family.
- Whether 8-bit codes in `i16` buffers are always multiples of 256 (2208A, ps3000a, ps6000).
- Whether `ps4000RunStreamingEx` is in the installed ps4000 driver.

## Checked on hardware (2026-10-14)

| Scope | Test | Result |
|---|---|---|
| 2208A (ps2000a) | 8-bit raw at 1 MS/s | `I8`, 100% of samples, every i16 code a multiple of 256 |
| 2208A | AVERAGE ×16, ×256 | `I16` at 62.5 kS/s and 3.9 kS/s; fractions in the low byte (16 and 72 distinct codes where raw gives 3) |
| 5464E (psospa) | 8-bit raw at 1 MS/s | native `INT8` buffers, `I8`, 99.3% of samples |
| 5464E | 8-bit AVERAGE ×16 | `I16` at 62.2 kS/s; noise 0.64 mV → 0.16 mV (√16) |
| 5464E | 16-bit AVERAGE ×16 | noise 0.145 mV → 0.065 mV |
| 5464E | SUM ×16 | refused: `RATIO_MODE_NOT_SUPPORTED` |
| 5464E | 12, 14, 15 bit | the unit lists only 8 and 16 bit; these are now refused (they fell back to 16 bit before) |
| 5464E | 8 and 16 bit at 31.25 MS/s, one channel | 8 bit keeps every sample; 16 bit keeps about 65% over USB 2 |
