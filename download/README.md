# pico-download

<!-- cargo-rdme start -->

Downloads Pico Technology oscilloscope and data logger drivers.

With this crate, your software can use every Pico instrument, but you do not ship the driver binaries with it.
It downloads the drivers that you ask for to a cache directory, and it does not download them again.
It covers every driver this SDK wraps: the oscilloscopes, the data loggers (usbtc08, pl1000, picohrdl, usbpt104, usbdrdaq, plcm3) and `picoipp`, which ps4000 and ps6000 load at runtime.

## Quick start

```rust
use pico_common::Driver;
use pico_download::{available_drivers, cache_resolution, download_drivers_to_cache};
use pico_driver::DriverLoad;

let available = available_drivers();
let drivers: Vec<Driver> = [Driver::PS2000A, Driver::TC08]
    .into_iter()
    .filter(|driver| available.contains(driver))
    .collect();

// Downloads only the drivers that are not in the cache.
download_drivers_to_cache(&drivers)?;

for driver in drivers {
    driver.load(&cache_resolution())?;
    println!("loaded {driver}");
}
```

## How the parts fit

```text
 &[Driver]                      the drivers you ask for
        │
        ▼
 download_drivers_to_cache()    adds picoipp for ps4000 and ps6000
        │
        ▼
 driver_binary() ──► DriverBinary   file name, size and SHA-256 for this platform
        │
        ▼
 download_url()                 BASE_URL or PICO_DRIVERS_BASE_URL, then BUNDLE
        │
        ▼
 SHA-256 check                  a bad file gives DriverDownloadError::HashMismatch
        │
        ▼
 get_cache_dir()                one directory for each bundle and platform
        │
        ▼
 cache_resolution() ──► LibraryResolution   pico_driver loads the drivers from here
```

## Platforms

- Windows: x86, x86_64 and aarch64.
- macOS: x86_64 and aarch64.
- Linux: x86_64, aarch64 and arm (Raspberry Pi).

Pico do not build every driver for every platform.
Use [`available_drivers`](https://docs.rs/pico-download/latest/pico_download/fn.available_drivers.html) or [`driver_binary`](https://docs.rs/pico-download/latest/pico_download/manifest/fn.lookup.html) to find if a driver is available.

- Windows x86_64, macOS x86_64 and Linux x86_64 have every driver.
- Windows x86 has no psospa.
- Windows aarch64, macOS aarch64 and Linux aarch64 have no ps4000, ps6000, usbdrdaq or picoipp.
- Linux arm has ps2000, ps4000, ps6000 and the six data logger drivers only.

## Bundles and the cache

Each version of this crate uses one *bundle* of drivers, which comes from the public Pico installers.
`BUNDLE` is the name of the bundle.
`BINARIES` lists each binary in the bundle, with its Pico version and its SHA-256 hash.
The hashes are in the compiled crate, so the crate does not accept a changed download.

The bundle name is part of the cache path.
So two applications that use different versions of this crate get different cache directories, and they do not change the files of each other.

The binaries are assets of a [GitHub release](https://github.com/timfish/pico-sdk/releases), with one release for each bundle.
Set the `PICO_DRIVERS_BASE_URL` environment variable to download them from a mirror.
The crate still checks each file against its hash.

## Main types

- [`download_drivers_to_cache`](https://docs.rs/pico-download/latest/pico_download/fn.download_drivers_to_cache.html), [`download_drivers`](https://docs.rs/pico-download/latest/pico_download/fn.download_drivers.html): download drivers and their dependencies.
- [`cache_resolution`](https://docs.rs/pico-download/latest/pico_download/fn.cache_resolution.html): tells [`pico_driver`](https://docs.rs/pico_driver/latest/pico_driver/) to load the drivers from the cache.
- [`DriverDownloadError`](https://docs.rs/pico-download/latest/pico_download/enum.DriverDownloadError.html): why a download failed.
- [`available_drivers`](https://docs.rs/pico-download/latest/pico_download/fn.available_drivers.html), [`driver_binary`](https://docs.rs/pico-download/latest/pico_download/manifest/fn.lookup.html), [`DriverBinary`](https://docs.rs/pico-download/latest/pico_download/manifest/struct.DriverBinary.html): the binaries for each platform.
- [`get_cache_dir`](https://docs.rs/pico-download/latest/pico_download/paths/fn.get_cache_dir.html), [`cached_driver_path`](https://docs.rs/pico-download/latest/pico_download/fn.cached_driver_path.html): where the drivers go.
- [`download_url`](https://docs.rs/pico-download/latest/pico_download/fn.download_url.html), `BASE_URL`, `BASE_URL_ENV_VAR`: where the drivers come from.

## Dependencies

- [`pico_common`](https://docs.rs/pico_common/latest/pico_common/) gives [`Driver`](https://docs.rs/pico_common/latest/pico_common/driver/enum.Driver.html).
- [`pico_driver`](https://docs.rs/pico_driver/latest/pico_driver/) gives [`LibraryResolution`](https://docs.rs/pico_driver/latest/pico_driver/resolution/enum.LibraryResolution.html), which loads the drivers from the cache.

<!-- cargo-rdme end -->

## Update the drivers

`cargo run --example generate-manifest -- <bundle-id>` makes the manifest again from the public Pico installers.
It downloads the Windows `.exe` and MSI installers, the macOS `.pkg` files and the PicoScope 7 Debian repository.
It extracts each driver in Rust, writes `src/manifest.rs`, and puts the assets for upload in `release/`:

```text
cargo run --example generate-manifest -- drivers-2026.10.25
gh release create drivers-2026.10.25 ./release/* --title "Pico drivers 2026.10.25"
```

The `drivers` workflow (`.github/workflows/drivers.yml`) does the same on a GitHub runner: `gh workflow run drivers.yml -f tag=drivers-2026.10.25 --ref <branch>`.
It refuses a tag that is already there.
It checks each hash in the manifest against its release file, publishes the release and commits `src/manifest.rs` to the branch it ran on.
The job summary shows the table of drivers for each platform.

The `<bundle-id>` becomes `BUNDLE`, so give each driver set its own dated tag.
A constant at the top of the example sets the installer version of the URLs.
Change it when Pico release a new SDK.
The set of drivers comes from `pico_common::Driver`, so the next run adds a new variant to the manifest.
The run ends with a table of the drivers that each platform has and the drivers that it does not have.
The tests in `src/lib.rs` hold the expected set of drivers for each platform.
Change them when Pico add or remove a build.

License: MIT
