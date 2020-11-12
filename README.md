# pico-sdk

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/meatysolutions/pico-sdk/Build%20&%20Package?label=Build%20%26%20Package&logo=github)](https://github.com/meatysolutions/pico-sdk/actions)
![GitHub](https://img.shields.io/github/license/meatysolutions/pico-sdk?style=flat)

### High performance, cross-platform, gap-free streaming from any Pico Technology oscilloscope

This package wraps all current Pico oscilloscope drivers in a high-level, common
API written in Rust. This API is compiled to native code and exposed to other
programming languages through simple C bindings.

## Issues, Feature Requests and Contributions

This library is still in the early stages of development so you should expect
breaking changes with any release before v1.0.0.

Please report any issues or feature requests in
[the issue tracker](https://github.com/meatysolutions/pico-sdk/issues). Pull
requests are welcome and encouraged!

## Building & Packaging

The Rust code will build and test on any platform via `cargo build` and
`cargo test`.

To support multiple platforms, the language bindings require the native build
artifacts from every supported platform and this is achieved via GitHub Actions
with multiple build steps.

- On Windows, macOS and Linux we run `node scripts/build.js` which compiles the
  Rust code to native libraries.
- In the final step we run `node scripts/bindings.js` which copies the native
  libraries to the correct directories and builds/packages for each language.
- On git tag creation, `node scripts/publish.js` publishes the packages to the
  respective package managers.

You can build packages supporting only your current platform by running these
scripts yourself. However, the dotnet build currently fails if any native
artifacts are missing so you'll need to modify `PicoSDK.csproj` to negate this.

## Rust

[![Crates.io](https://img.shields.io/crates/v/pico-sdk?color=dark-green&logo=rust)](https://crates.io/crates/pico-sdk)
[![docs.rs](https://docs.rs/pico-sdk/badge.svg?version=0.1.2)](https://docs.rs/pico-sdk/)

Add `pico-sdk` to dependencies in `Cargo.toml`:

```toml
[dependencies]
pico-sdk = "0.1.2"
```

Check out the usage examples in the [Rust README](rust/sdk).

## .NET

[![Nuget](https://img.shields.io/nuget/v/PicoSDK?color=dark-green&logo=nuget)](https://www.nuget.org/packages/PicoSDK/)

Add the `PicoSDK` NuGet package:

```shell
dotnet add package PicoSDK
```

Check out the usage example in the [dotnet README](dotnet)

## Python

[![PyPI](https://img.shields.io/pypi/v/pico_sdk?color=dark-green&logo=pypi) ![PyPI - Python Version](https://img.shields.io/pypi/pyversions/pico_sdk)](https://pypi.org/project/pico-sdk/)

Install the `pico_sdk` package:

```shell
pip install pico_sdk
```

Check out the usage example in the [Python README](python)

## Node.js

[![npm](https://img.shields.io/npm/v/pico-sdk?color=dark-green&logo=npm) ![node-current](https://img.shields.io/node/v/pico-sdk?color=blue)](https://www.npmjs.com/package/pico-sdk)

Add the `pico-sdk` package:

```shell
npm i pico-sdk
```

Check out the usage example in the [node.js README](nodejs)
