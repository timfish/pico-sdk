//! Downloads Pico's drivers from their public installers and turns them into the manifest this
//! crate compiles against, plus a `release/` directory of assets ready to publish.
//!
//! ```text
//! cargo run --example generate-manifest -- drivers-2026.07.22
//! ```
//!
//! Pico distribute their drivers three different ways, so this fetches from all three:
//!  - **Windows** — the `PicoSDK_<arch>_<ver>.exe` installers (a PE wrapping a CAB that holds an
//!    MSI whose embedded cabinet holds the DLLs).
//!  - **macOS** — the `PicoSDK[-ARM64]-<ver>.pkg` installers (a xar archive whose payload is a
//!    gzip-compressed cpio holding the dylibs).
//!  - **Linux** — the PicoScope 7 Debian repository (per-driver `.deb`s, each an `ar` archive
//!    whose `data.tar.xz` holds the `.so`).
//!
//! The URLs are hardcoded below; when Pico ship a new SDK, bump [`SDK_VERSION`] (and the Debian
//! codename if it changes) and re-run.
//!
//! What it produces:
//!  - `drivers/` — every extracted binary, laid out by platform and architecture. This is the
//!    complete set, including libraries this SDK has no bindings for (usbtc08, pl1000, ...).
//!  - `download/src/manifest.rs` — the committed catalogue, covering only the drivers this SDK
//!    wraps (plus `picoipp`, which a couple of them load at runtime).
//!  - `release/` — exactly the binaries the manifest references, named for upload:
//!    `gh release create drivers-2026.07.22 ./release/* --title "Pico drivers 2026.07.22"`
//!
//! The argument you pass becomes [`manifest::BUNDLE`](../src/manifest.rs): both the release tag
//! the assets hang off and the cache directory they download into. Give it its own dated tag so
//! it's independent of the library's own version — many library releases can reference one
//! driver release, and users only re-download when the drivers actually change.

use pico_common::Driver;
use ring::digest::{Context, SHA256};
use std::{
    fmt::Write as _,
    fs,
    io::{Cursor, Read},
    path::{Path, PathBuf},
    process::ExitCode,
};

/// The Pico SDK installer version the Windows and macOS URLs point at.
const SDK_VERSION: &str = "11.1.0.481";

/// Windows: `(rust arch, output dir, url token)`. URL is
/// `https://www.picotech.com/download/software/sr/PicoSDK_<token>_<ver>.exe`.
const WINDOWS_TARGETS: &[(&str, &str, &str)] = &[
    ("x86", "win/x86", "x86"),
    ("x86_64", "win/x64", "x64"),
    ("aarch64", "win/arm64", "ARM64"),
];

/// macOS: `(rust arch, output dir, url infix)`. URL is
/// `https://www.picotech.com/download/software/sr/PicoSDK-<infix><ver>.pkg`
/// (the Intel build has no arch in its name, hence the empty infix).
const MACOS_TARGETS: &[(&str, &str, &str)] = &[
    ("x86_64", "mac/x64", ""),
    ("aarch64", "mac/arm64", "ARM64-"),
];

/// Linux: `(rust arch, output dir, Debian arch)`. Drivers come from the PicoScope 7 apt repo.
const LINUX_TARGETS: &[(&str, &str, &str)] = &[
    ("x86_64", "linux/x64", "amd64"),
    ("aarch64", "linux/arm64", "arm64"),
    ("arm", "linux/armv7l", "armhf"),
];

const LINUX_REPO: &str = "https://labs.picotech.com/rc/picoscope7/debian";

/// One platform we extract drivers for.
struct Target {
    /// Matches `std::env::consts::OS`
    os: &'static str,
    /// Matches `std::env::consts::ARCH`
    arch: &'static str,
    /// Where its binaries live in the `drivers/` tree, e.g. `win/x64`
    out_dir: &'static str,
}

/// Where the binaries get published. Combined with `BUNDLE` and the asset name at runtime to
/// form each download URL. Must match `BASE_URL` in the generated manifest.
const BASE_URL: &str = "https://github.com/meatysolutions/pico-sdk/releases/download";

/// One extracted driver binary, already written to the `drivers/` tree and hashed.
struct Binary {
    driver: Driver,
    os: &'static str,
    arch: &'static str,
    file_name: String,
    version: String,
    sha256: String,
    size: u64,
    /// Where it landed in the `drivers/` tree, so it can be copied into `release/`
    source: PathBuf,
}

impl Binary {
    /// Release assets share a flat namespace, so the target is folded into the name
    fn asset_name(&self) -> String {
        format!("{}-{}-{}", self.os, self.arch, self.file_name)
    }
}

fn main() -> ExitCode {
    let Some(bundle_id) = std::env::args().nth(1) else {
        eprintln!(
            "usage: cargo run --example generate-manifest -- <bundle-id>\n\
             \n\
             <bundle-id> identifies this driver set. It becomes the tag of the GitHub release\n\
             the binaries are published under and the cache directory they download into, so\n\
             give it its own dated tag, e.g. drivers-2026.07.22."
        );
        return ExitCode::FAILURE;
    };

    match run(&bundle_id) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("\nerror: {}", error);
            ExitCode::FAILURE
        }
    }
}

fn run(bundle_id: &str) -> Result<(), String> {
    let drivers_dir = Path::new("drivers");
    if drivers_dir.exists() {
        fs::remove_dir_all(drivers_dir).map_err(|e| format!("{}: {}", drivers_dir.display(), e))?;
    }

    let mut binaries = Vec::new();

    for &(arch, out_dir, token) in WINDOWS_TARGETS {
        let target = Target {
            os: "windows",
            arch,
            out_dir,
        };
        let url = format!(
            "https://www.picotech.com/download/software/sr/PicoSDK_{token}_{SDK_VERSION}.exe"
        );
        eprintln!("windows {arch}: {url}");
        let exe = fetch(&url)?;
        for (file_name, bytes) in extract_windows(&exe)? {
            record(
                &mut binaries,
                drivers_dir,
                &target,
                file_name,
                SDK_VERSION.to_string(),
                bytes,
            )?;
        }
    }

    for &(arch, out_dir, infix) in MACOS_TARGETS {
        let target = Target {
            os: "macos",
            arch,
            out_dir,
        };
        let url = format!(
            "https://www.picotech.com/download/software/sr/PicoSDK-{infix}{SDK_VERSION}.pkg"
        );
        eprintln!("macos {arch}: {url}");
        let pkg = fetch(&url)?;
        for (file_name, bytes) in extract_macos(&pkg)? {
            record(
                &mut binaries,
                drivers_dir,
                &target,
                file_name,
                SDK_VERSION.to_string(),
                bytes,
            )?;
        }
    }

    for &(arch, out_dir, deb_arch) in LINUX_TARGETS {
        let target = Target {
            os: "linux",
            arch,
            out_dir,
        };
        let packages_url =
            format!("{LINUX_REPO}/dists/picoscope/main/binary-{deb_arch}/Packages.gz");
        eprintln!("linux {arch}: {packages_url}");
        let packages = parse_packages(&gunzip(&fetch(&packages_url)?)?)?;

        for pkg in packages.iter().filter(|p| p.name.starts_with("lib")) {
            let deb = fetch(&format!("{LINUX_REPO}/{}", pkg.filename))?;
            let file_name = format!("{}.so", pkg.name);
            let bytes = extract_deb(&deb, &file_name)
                .map_err(|e| format!("{} ({}): {}", pkg.name, deb_arch, e))?;
            record(
                &mut binaries,
                drivers_dir,
                &target,
                file_name,
                pkg.version.clone(),
                bytes,
            )?;
        }
    }

    if binaries.is_empty() {
        return Err("extracted no drivers this SDK wraps - did the URLs or formats change?".into());
    }

    binaries.sort_by(|a, b| (a.driver, a.os, a.arch).cmp(&(b.driver, b.os, b.arch)));

    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("manifest.rs");
    fs::write(&manifest_path, generate_manifest(bundle_id, &binaries))
        .map_err(|e| format!("{}: {}", manifest_path.display(), e))?;

    let release_dir = PathBuf::from("release");
    write_release_dir(&binaries, &release_dir)?;

    report(&binaries, bundle_id, &manifest_path, &release_dir);
    Ok(())
}

/// Writes one extracted binary into the `drivers/` tree and, if this SDK has bindings for it,
/// records it for the manifest. Binaries the SDK doesn't wrap still land on disk but are skipped
/// for the manifest.
#[allow(clippy::too_many_arguments)]
fn record(
    binaries: &mut Vec<Binary>,
    drivers_dir: &Path,
    target: &Target,
    file_name: String,
    version: String,
    bytes: Vec<u8>,
) -> Result<(), String> {
    let dir = drivers_dir.join(target.out_dir);
    fs::create_dir_all(&dir).map_err(|e| format!("{}: {}", dir.display(), e))?;
    let source = dir.join(&file_name);
    fs::write(&source, &bytes).map_err(|e| format!("{}: {}", source.display(), e))?;

    if let Some(driver) = driver_for(lib_of(&file_name)) {
        binaries.push(Binary {
            driver,
            os: target.os,
            arch: target.arch,
            sha256: sha256_hex(&bytes),
            size: bytes.len() as u64,
            file_name,
            version,
            source,
        });
    }
    Ok(())
}

/// The library name a binary belongs to, e.g. `ps5000a.dll` / `libps5000a.dylib` /
/// `libps5000a.so` all map to `ps5000a`.
fn lib_of(file_name: &str) -> &str {
    file_name
        .strip_suffix(".dll")
        .or_else(|| {
            file_name
                .strip_suffix(".dylib")
                .map(|s| s.trim_start_matches("lib"))
        })
        .or_else(|| {
            file_name
                .strip_suffix(".so")
                .map(|s| s.trim_start_matches("lib"))
        })
        .unwrap_or(file_name)
}

/// Maps a Pico library name onto the `Driver` it backs. Libraries this SDK has no bindings for
/// (pl1000, the plain ps3000/ps5000, ...) return `None` and stay out of the manifest.
///
/// `picoipp` maps to `Driver::PicoIPP`: it isn't a device driver, but ps4000/ps6000 load it at
/// runtime, so it has to travel with them.
fn driver_for(lib: &str) -> Option<Driver> {
    Some(match lib {
        "ps2000" => Driver::PS2000,
        "ps2000a" => Driver::PS2000A,
        "ps3000a" => Driver::PS3000A,
        "ps4000" => Driver::PS4000,
        "ps4000a" => Driver::PS4000A,
        "ps5000a" => Driver::PS5000A,
        "ps6000" => Driver::PS6000,
        "ps6000a" => Driver::PS6000A,
        "psospa" => Driver::PSOSPA,
        "usbdrdaq" => Driver::DrDAQ,
        "usbtc08" => Driver::TC08,
        "picoipp" => Driver::PicoIPP,
        _ => return None,
    })
}

// ---- Windows: .exe -> outer CAB -> MSI -> embedded CAB -> DLLs -----------------------------

fn extract_windows(exe: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
    // The installer is a PE with two appended cabinets: a small one of setup scripts, then the
    // payload. Take the last MSCF signature.
    let payload_off = last_signature(exe, b"MSCF").ok_or("no cabinet found in installer")?;

    let mut outer = cab::Cabinet::new(Cursor::new(exe[payload_off..].to_vec()))
        .map_err(|e| format!("outer cab: {e}"))?;
    let outer_names = cab_entries(&outer);
    let msi_bytes = read_cab_file(&mut outer, outer_names.first().ok_or("empty outer cab")?)?;

    let mut package =
        msi::Package::open(Cursor::new(msi_bytes)).map_err(|e| format!("open msi: {e}"))?;

    // The DLLs live in the cabinet the MSI stores as a stream; find it by its signature rather
    // than by parsing the Media table.
    let stream_names: Vec<String> = package.streams().collect();
    let mut embedded = None;
    for name in &stream_names {
        let mut buf = Vec::new();
        package
            .read_stream(name)
            .map_err(|e| format!("read stream {name}: {e}"))?
            .read_to_end(&mut buf)
            .map_err(|e| format!("read stream {name}: {e}"))?;
        if buf.starts_with(b"MSCF") {
            embedded = Some(buf);
            break;
        }
    }
    let embedded = embedded.ok_or("no embedded cabinet in msi")?;

    let mut inner =
        cab::Cabinet::new(Cursor::new(embedded)).map_err(|e| format!("inner cab: {e}"))?;
    let mut out = Vec::new();
    for name in cab_entries(&inner) {
        if name.ends_with(".dll") {
            let bytes = read_cab_file(&mut inner, &name)?;
            out.push((name, bytes));
        }
    }
    Ok(out)
}

fn cab_entries<R: Read + std::io::Seek>(cabinet: &cab::Cabinet<R>) -> Vec<String> {
    cabinet
        .folder_entries()
        .flat_map(|folder| folder.file_entries().map(|f| f.name().to_string()))
        .collect()
}

fn read_cab_file<R: Read + std::io::Seek>(
    cabinet: &mut cab::Cabinet<R>,
    name: &str,
) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    cabinet
        .read_file(name)
        .map_err(|e| format!("cab entry {name}: {e}"))?
        .read_to_end(&mut buf)
        .map_err(|e| format!("cab entry {name}: {e}"))?;
    Ok(buf)
}

// ---- macOS: .pkg (xar) -> Payload (gzip cpio) -> dylibs -----------------------------------

fn extract_macos(pkg: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
    let mut xar = apple_xar::reader::XarReader::new(Cursor::new(pkg.to_vec()))
        .map_err(|e| format!("open pkg: {e}"))?;

    let payload_path = xar
        .files()
        .map_err(|e| format!("read pkg toc: {e}"))?
        .into_iter()
        .map(|(name, _)| name)
        .find(|name| name.ends_with("Payload"))
        .ok_or("no Payload in pkg")?;

    let payload = xar
        .get_file_data_from_path(&payload_path)
        .map_err(|e| format!("read payload: {e}"))?
        .ok_or("empty payload")?;

    let mut reader = cpio_archive::reader(Cursor::new(gunzip(&payload)?))
        .map_err(|e| format!("open cpio: {e}"))?;

    let mut out = Vec::new();
    while let Some(header) = reader.read_next().map_err(|e| format!("cpio: {e}"))? {
        let name = header.name().to_string();
        let is_regular_file = header.mode() & 0o170000 == 0o100000;

        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .map_err(|e| format!("cpio {name}: {e}"))?;

        // Each library dir holds the real, versioned dylib (libps5000a.2.dylib) plus an
        // unversioned symlink to it - and libpicoipp/ additionally holds its own libiomp5.dylib
        // dependency. So take every regular .dylib and name it after the file itself (with the
        // version stripped), never after the directory.
        if is_regular_file && name.contains("/Libraries/") {
            if let Some(file_name) = Path::new(&name)
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(dylib_output_name)
            {
                out.push((file_name, bytes));
            }
        }
    }
    Ok(out)
}

/// The unversioned name a dylib should be saved as, or `None` if it isn't a dylib.
///
/// Strips the version components Pico put in the real file name so it can be loaded by the plain
/// name: `libps5000a.2.dylib` and `libpicoipp.1.dylib` become `libps5000a.dylib` and
/// `libpicoipp.dylib`, while an already-unversioned `libiomp5.dylib` is left alone.
fn dylib_output_name(basename: &str) -> Option<String> {
    let stem = basename.strip_suffix(".dylib")?;
    let mut parts: Vec<&str> = stem.split('.').collect();
    while parts.len() > 1 && parts.last().unwrap().bytes().all(|b| b.is_ascii_digit()) {
        parts.pop();
    }
    Some(format!("{}.dylib", parts.join(".")))
}

// ---- Linux: Debian repo -> .deb (ar) -> data.tar.xz -> .so --------------------------------

struct DebPackage {
    name: String,
    version: String,
    filename: String,
}

fn parse_packages(packages: &[u8]) -> Result<Vec<DebPackage>, String> {
    let text = std::str::from_utf8(packages).map_err(|e| format!("Packages not utf-8: {e}"))?;
    let mut out = Vec::new();

    for stanza in text.split("\n\n") {
        let mut name = None;
        let mut version = None;
        let mut filename = None;
        for line in stanza.lines() {
            if let Some(v) = line.strip_prefix("Package: ") {
                name = Some(v.trim().to_string());
            } else if let Some(v) = line.strip_prefix("Version: ") {
                version = Some(v.trim().to_string());
            } else if let Some(v) = line.strip_prefix("Filename: ") {
                filename = Some(v.trim().to_string());
            }
        }
        if let (Some(name), Some(version), Some(filename)) = (name, version, filename) {
            out.push(DebPackage {
                name,
                version,
                filename,
            });
        }
    }
    Ok(out)
}

fn extract_deb(deb: &[u8], so_name: &str) -> Result<Vec<u8>, String> {
    // Most packages ship data.tar.xz, but a few (libpicocv) use data.tar.gz
    let mut archive = ar::Archive::new(Cursor::new(deb));
    let mut compressed = None;
    while let Some(entry) = archive.next_entry() {
        let mut entry = entry.map_err(|e| format!("ar: {e}"))?;
        let id = String::from_utf8_lossy(entry.header().identifier()).to_string();
        if let Some(compression) = id.strip_prefix("data.tar.") {
            let mut buf = Vec::new();
            entry
                .read_to_end(&mut buf)
                .map_err(|e| format!("read {id}: {e}"))?;
            compressed = Some((compression.to_string(), buf));
            break;
        }
    }
    let (compression, buf) = compressed.ok_or("no data.tar.* in deb")?;
    let data_tar = match compression.as_str() {
        "xz" => xz_decompress(&buf)?,
        "gz" => gunzip(&buf)?,
        other => return Err(format!("unsupported data.tar.{other}")),
    };

    // The package ships libX.so (a symlink) pointing at the real libX.so.2.0.0. Resolve the
    // symlink, then extract the file it points at.
    let target = tar_symlink_target(&data_tar, so_name)?
        .ok_or_else(|| format!("{so_name} not found or not a symlink"))?;
    tar_read_file(&data_tar, &target)?.ok_or_else(|| format!("symlink target {target} missing"))
}

/// The basename a `basename` symlink points at, if present in the tar.
fn tar_symlink_target(tar: &[u8], basename: &str) -> Result<Option<String>, String> {
    let mut archive = tar::Archive::new(Cursor::new(tar));
    for entry in archive.entries().map_err(|e| format!("tar: {e}"))? {
        let entry = entry.map_err(|e| format!("tar: {e}"))?;
        let path = entry.path().map_err(|e| format!("tar path: {e}"))?;
        if path.file_name().map(|n| n == basename).unwrap_or(false) {
            if let Some(link) = entry.link_name().map_err(|e| format!("tar link: {e}"))? {
                return Ok(link.file_name().map(|n| n.to_string_lossy().to_string()));
            }
        }
    }
    Ok(None)
}

/// The contents of the file whose basename is `basename`.
fn tar_read_file(tar: &[u8], basename: &str) -> Result<Option<Vec<u8>>, String> {
    let mut archive = tar::Archive::new(Cursor::new(tar));
    for entry in archive.entries().map_err(|e| format!("tar: {e}"))? {
        let mut entry = entry.map_err(|e| format!("tar: {e}"))?;
        let path = entry.path().map_err(|e| format!("tar path: {e}"))?;
        if path.file_name().map(|n| n == basename).unwrap_or(false) {
            let mut buf = Vec::new();
            entry
                .read_to_end(&mut buf)
                .map_err(|e| format!("tar read {basename}: {e}"))?;
            return Ok(Some(buf));
        }
    }
    Ok(None)
}

// ---- shared helpers -----------------------------------------------------------------------

/// Downloads `url` into memory. `http_req` follows redirects itself (installer and repo URLs
/// bounce through object storage).
fn fetch(url: &str) -> Result<Vec<u8>, String> {
    let mut body = Vec::new();
    let response = http_req::request::get(url, &mut body).map_err(|e| format!("{url}: {e}"))?;
    let status = response.status_code();
    if !status.is_success() {
        return Err(format!("{url}: HTTP {}", status));
    }
    Ok(body)
}

fn last_signature(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .enumerate()
        .filter(|(_, w)| *w == needle)
        .map(|(i, _)| i)
        .next_back()
}

fn gunzip(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    flate2::read::GzDecoder::new(Cursor::new(bytes))
        .read_to_end(&mut out)
        .map_err(|e| format!("gunzip: {e}"))?;
    Ok(out)
}

fn xz_decompress(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    xz2::read::XzDecoder::new(Cursor::new(bytes))
        .read_to_end(&mut out)
        .map_err(|e| format!("xz: {e}"))?;
    Ok(out)
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut context = Context::new(&SHA256);
    context.update(bytes);

    let mut hex = String::with_capacity(64);
    for byte in context.finish().as_ref() {
        let _ = write!(hex, "{:02x}", byte);
    }
    hex
}

fn write_release_dir(binaries: &[Binary], release_dir: &Path) -> Result<(), String> {
    if release_dir.exists() {
        fs::remove_dir_all(release_dir).map_err(|e| format!("{}: {}", release_dir.display(), e))?;
    }
    fs::create_dir_all(release_dir).map_err(|e| format!("{}: {}", release_dir.display(), e))?;

    for binary in binaries {
        let destination = release_dir.join(binary.asset_name());
        fs::copy(&binary.source, &destination)
            .map_err(|e| format!("{}: {}", destination.display(), e))?;
    }
    Ok(())
}

fn generate_manifest(bundle_id: &str, binaries: &[Binary]) -> String {
    let mut out = String::new();

    out.push_str(
        "//! Catalogue of the Pico driver binaries published for this crate version.\n\
         //!\n\
         //! AUTO-GENERATED by the `generate-manifest` example - do not edit by hand.\n\
         use pico_common::Driver;\n\
         \n\
         /// Identifies the driver set this crate version was built against.\n\
         ///\n\
         /// This is both the release tag the binaries are published under and the cache\n\
         /// directory they are downloaded into, so two versions of this crate that were built\n\
         /// against different driver sets never share a cache entry.\n",
    );
    let _ = writeln!(out, "pub const BUNDLE: &str = \"{}\";\n", bundle_id);

    let _ = writeln!(
        out,
        "/// Where the binaries for `BUNDLE` are published.\n\
         pub const BASE_URL: &str = \"{}\";\n",
        BASE_URL
    );

    out.push_str(
        "/// A driver binary published for one target.\n\
         #[derive(Debug, Clone, Copy, Eq, PartialEq)]\n\
         pub struct DriverBinary {\n    \
         pub driver: Driver,\n    \
         /// Matches `std::env::consts::OS`\n    \
         pub os: &'static str,\n    \
         /// Matches `std::env::consts::ARCH`\n    \
         pub arch: &'static str,\n    \
         /// The name the binary must have on disk to be loadable\n    \
         pub file_name: &'static str,\n    \
         /// The name the binary is published under, which is unique across targets\n    \
         pub asset_name: &'static str,\n    \
         /// The upstream Pico version this binary was built from\n    \
         pub version: &'static str,\n    \
         /// Lowercase hex SHA-256 of the binary\n    \
         pub sha256: &'static str,\n    \
         pub size: u64,\n\
         }\n\
         \n\
         /// Every published binary, sorted by driver then target.\n\
         pub const BINARIES: &[DriverBinary] = &[\n",
    );

    for binary in binaries {
        let _ = writeln!(
            out,
            "    DriverBinary {{\n        \
             driver: Driver::{:?},\n        \
             os: \"{}\",\n        \
             arch: \"{}\",\n        \
             file_name: \"{}\",\n        \
             asset_name: \"{}\",\n        \
             version: \"{}\",\n        \
             sha256: \"{}\",\n        \
             size: {},\n    \
             }},",
            binary.driver,
            binary.os,
            binary.arch,
            binary.file_name,
            binary.asset_name(),
            binary.version,
            binary.sha256,
            binary.size,
        );
    }

    out.push_str(
        "];\n\
         \n\
         /// Looks up the binary published for a driver on a given target, if there is one.\n\
         ///\n\
         /// Pico do not build every driver for every target, so `None` here means the driver is\n\
         /// genuinely unavailable rather than that something went wrong.\n\
         pub fn lookup(driver: Driver, os: &str, arch: &str) -> Option<&'static DriverBinary> {\n    \
         BINARIES\n        \
         .iter()\n        \
         .find(|b| b.driver == driver && b.os == os && b.arch == arch)\n\
         }\n",
    );

    out
}

fn report(binaries: &[Binary], bundle_id: &str, manifest_path: &Path, release_dir: &Path) {
    let missing: Vec<_> = enum_iterator::all::<Driver>()
        .filter(|driver| !binaries.iter().any(|b| b.driver == *driver))
        .collect();

    println!(
        "\nWrote {} covering {} binaries to {}",
        bundle_id,
        binaries.len(),
        manifest_path.display()
    );
    println!("Release assets ready in {}", release_dir.display());

    if !missing.is_empty() {
        println!(
            "\nNot published for any target: {}",
            missing
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    println!(
        "\nTo publish:\n  gh release create {0} --title \"Pico drivers {0}\" ./{1}/*",
        bundle_id,
        release_dir.display()
    );
}
