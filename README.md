# pico-sdk

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/meatysolutions/pico-sdk/Build%20&%20Package)
![GitHub](https://img.shields.io/github/license/meatysolutions/pico-sdk?style=flat)

### High performance, cross-platform, gap-free streaming from any Pico Technology oscilloscope

This package wraps all current Pico oscilloscope drivers in a high-level, common
API written in Rust. This API is compiled to native code and exposed to other
programming languages through simple C bindings.

## Issues, Feature Requests and Contributions

This library is still in the early stages of development so you should expect
breaking changes on any release before v1.0.0.

Please report any issues or feature requests in
[the issue tracker](https://github.com/meatysolutions/pico-sdk/issues). Pull
requests are welcome and encouraged!

## Building

The Rust code will build and test on any platform via `cargo build` and
`cargo test`.

To support multiple platforms, the language bindings require the native build
artifacts from every supported platform. This is achieved via GitHub Actions
with a pipelined build.

- On Windows, macOS and Linux we run `node scripts/build.js` which compiles the
  Rust code to native libraries.
- In the final step we run `node scripts/bindings.js` which copies the native
  libraries to the correct directories and builds/packages for each language.

You can build packages supporting only your current platform by running these
scripts yourself. However, the dotnet build currently fails if any native
artifacts are missing so you'll need to modify `PicoSDK.csproj` to negate this.

## Rust

![Crates.io](https://img.shields.io/crates/v/pico-sdk)

Add `pico-sdk` to dependencies in `Cargo.toml`:

```toml
[dependencies]
pico-sdk = "0.1.1"
```

Check out the usage examples in the [Rust README](rust/sdk).

## .NET

![Nuget](https://img.shields.io/nuget/v/PicoSDK)

Add the `PicoSDK` NuGet package:

```shell
dotnet add package PicoSDK
```

Check out the usage example in the [dotnet README](dotnet)

## Python

![PyPI](https://img.shields.io/pypi/v/pico_sdk)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/pico_sdk)

Install the `pico_sdk` package:

```shell
pip install pico_sdk
```

Check out the usage example in the [Python README](python)

## Node.js

![npm](https://img.shields.io/npm/v/pico-sdk)
![node-current](https://img.shields.io/node/v/pico-sdk)

Add the `pico-sdk` package:

```shell
npm i pico-sdk
```

Check out the usage example in the [node.js README](nodejs)
