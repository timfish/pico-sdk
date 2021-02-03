# pico-download

Downloads Pico Technology oscilloscope drivers.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

This helper crate allows the distribution of software that can communicate with every Pico oscilloscope
without the need to ship every driver binary.

Supported platforms:
 - Windows (32 or 64 bit)
 - macOS (64 bit)
 - Linux (64 bit or ARM for Raspberry Pi)

All binaries are stored in Amazon AWS S3 storage and can be browsed [here](https://pico-drivers.s3.eu-west-2.amazonaws.com/).

Some of these binaries have been modified using `install_name_tool` or `rpath` after download from the
Pico website which allows them to be loaded dynamically from non-standard paths. Pico singing certificates remain.

File hashes are validated after download to ensure the hosted files have not been tampered with.
These hashes are generated using the `generate-hashes` example.

`download_drivers_to_cache()` downloads the passed drivers and their dependencies and `cache_resolution()`
returns a `Resolution` that can be used to resolve the downloaded binaries.

`let enumerator = DeviceEnumerator::with_resolution(cache_resolution());`

License: MIT
