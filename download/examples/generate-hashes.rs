use enum_iterator::IntoEnumIterator;
use http_req::request;
use pico_common::Driver;
use ring::digest::{Context, Digest, SHA256};
use std::io::{Cursor, Read, Result};

pub fn get_binary_name(driver: Driver, os: &str) -> String {
    match os {
        "windows" => format!("{}.dll", driver),
        "macos" => format!("lib{}.dylib", driver),
        "linux" => format!("lib{}.so", driver),
        _ => panic!("unknown platform"),
    }
}

pub fn get_remote_url(driver: Driver, os: &str, arch: &str) -> String {
    let name = get_binary_name(driver, os);

    format!(
        "https://pico-drivers.s3.eu-west-2.amazonaws.com/{}/{}/{}",
        os, arch, name
    )
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn main() {
    let platforms: &[(&str, &[&str])] = &[
        ("windows", &["x86", "x86_64"]),
        ("macos", &["x86_64"]),
        ("linux", &["x86_64", "arm"]),
    ];

    println!("#![allow(unreachable_patterns)]");
    println!("use pico_common::Driver;");
    println!("/// This file is auto-generated from the generate-hashes example");
    println!("pub fn get_expected_driver_hash(driver: Driver) -> &'static str {{");
    platforms.iter().for_each(|(os, arches)| {
        arches.iter().for_each(|arch| {
            println!(
                "    #[cfg(all(target_os = \"{}\", target_arch = \"{}\"))]",
                os, arch
            );
            println!("    match driver {{");

            Driver::into_enum_iter().for_each(|driver| {
                let url = get_remote_url(driver, os, arch);

                let buf = vec![];
                let mut cur = Cursor::new(buf);
                let response = request::get(&url, &mut cur).unwrap();
                if response.status_code().is_success() {
                    cur.set_position(0);
                    let digest = sha256_digest(cur).unwrap();
                    println!("        Driver::{:?} => \"{:?}\",", driver, digest);
                }
            });

            println!("        _ => \"\"");
            println!("    }}");
        })
    });

    // Pico do not yet distribute drivers for macOS aarch64 so throw an error at compile time
    println!("    #[cfg(all(target_os = \"macos\", target_arch = \"aarch64\"))]");
    println!("    compile_error!(\"Pico do not yet distribute drivers for macOS aarch64. You'll need to target x86_64 for now!\");");

    println!("}}");
}
