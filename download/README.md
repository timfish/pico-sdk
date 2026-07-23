# pico-download

Downloads Pico Technology oscilloscope drivers.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

This helper crate allows the distribution of software that can communicate with every Pico oscilloscope
without the need to ship every driver binary.

Supported platforms:
 - Windows (x86, x86_64 or aarch64)
 - macOS (x86_64 or aarch64)
 - Linux (x86_64, aarch64 or arm for Raspberry Pi)

Pico do not build every driver for every platform, so use `driver_binary()` or `available_drivers()`
if you need to know up front whether a particular driver can be downloaded here.

## Bundles

Each version of this crate is built against one *bundle* of drivers, extracted from a release Pico
ship as a single archive. `BUNDLE` identifies it, and every binary in that bundle is listed in
`BINARIES` along with its upstream Pico version and SHA-256 hash. Those hashes are compiled into the
crate, so a download that has been tampered with in transit is rejected.

The bundle identifier is also part of the cache path, which means downloaded drivers are immutable:
two applications built against different versions of this crate get their own cache directory rather
than overwriting each other's drivers.

Binaries are published as [GitHub release assets](https://github.com/meatysolutions/pico-sdk/releases),
one release per bundle, under a dated tag of its own so that many library releases can reference the
same driver set. Set `PICO_DRIVERS_BASE_URL` to fetch them from a mirror instead; the compiled-in
hashes still gate what is accepted.

## Updating the drivers

`cargo run --example generate-manifest -- <bundle-id>` rebuilds the manifest from Pico's public
installers. It downloads the Windows `.exe`/MSI installers, the macOS `.pkg`s and the PicoScope 7
Debian repository, extracts every driver in pure Rust, writes `src/manifest.rs`, and stages the
assets in `release/` for upload:

```text
cargo run --example generate-manifest -- drivers-2026.07.22
gh release create drivers-2026.07.22 ./release/* --title "Pico drivers 2026.07.22"
```

The `<bundle-id>` becomes `BUNDLE`, so give each driver set its own dated tag. The installer version
the URLs point at is a constant at the top of the example; bump it when Pico ship a new SDK.

## Usage

`download_drivers_to_cache()` downloads the passed drivers and `cache_resolution()` returns a
`Resolution` that can be used to resolve the downloaded binaries. Drivers that are already cached
are left alone, so it's cheap to call on every start up.

`let enumerator = DeviceEnumerator::with_resolution(cache_resolution());`

License: MIT
