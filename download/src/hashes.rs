use pico_common::Driver;
/// This file is auto-generated from the generate-hashes example
pub fn get_expected_driver_hash(driver: Driver) -> &'static str {
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    match driver {
        Driver::PS2000 => "SHA256:d6e7b63c7225dde8f6f879898584d92167db20464609d79fb26ce702fbf1f7b5",
        Driver::PS2000A => {
            "SHA256:6d3c1406a75895283982b42e2cab5c9afd6cd047e5a8572427f490dbd6d5d580"
        }
        Driver::PS3000A => {
            "SHA256:812fa751c7629f55384292fc83ebad4b046ede03d31d95f1907c52fdf12e7e07"
        }
        Driver::PS4000 => "SHA256:a1d4f03afbe92eab043f15f56b821af6ad4e3187992e572b7099871445f0ddac",
        Driver::PS4000A => {
            "SHA256:faaaacff46c8c2d26ccac75e95caf74c350ca5079ed506a1627a3f2cdedc9f27"
        }
        Driver::PS5000A => {
            "SHA256:357720b818ab8ff0e0aac208370022944a46eafb096e9ce80cb8758425ad4a4e"
        }
        Driver::PS6000 => "SHA256:19dcc3de482e8b9e1bf4e119c3aea69f24997351395f926b0c39a28242b6fe4a",
        Driver::PicoIPP => {
            "SHA256:3dc1520a40d220a3a51b8feb1cc55c5172c9cdd253545b44afb0f69c73bec80e"
        }
        _ => "",
    }
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    match driver {
        Driver::PS2000 => "SHA256:87aa4cce1d3aee32dd547e0449c7edba440f12e652b01cdf123c1fa8fab8dc8c",
        Driver::PS2000A => {
            "SHA256:29ddeaa0e76d24c64e87c5ac62a10c1de9d28a8d9190cf3d41bdab89dd667486"
        }
        Driver::PS3000A => {
            "SHA256:ba80a0f502083290ff0b5c4b9e016c0359bd618406de98418d7e32bcd16611cd"
        }
        Driver::PS4000 => "SHA256:c0b23100cd6aa96b404e61dec7545c7e6d2d11ccaf9d4c3d5231617cf1a2113f",
        Driver::PS4000A => {
            "SHA256:569530588decbba0ad5cfa61876c0a45691b11e10c66b086dba4c9185c0f4aeb"
        }
        Driver::PS5000A => {
            "SHA256:303e2f287d916f234a4e74ad891b50c417b9cc43050fd5fa5c03959fa1c2ac0c"
        }
        Driver::PS6000 => "SHA256:cd7f9ab9ce6551e869c2a9040297d7d758626ef35ff53dab5f6eac97ae19d984",
        Driver::PicoIPP => {
            "SHA256:a26381ac9bcdb3364b52fa934b337e8486e58bc7f0755f8d2d73dc237d2d1958"
        }
        _ => "",
    }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    match driver {
        Driver::PS2000 => "SHA256:f7b6e71931f7e458e79dc8b5b243b729e848436f0a11f29890ea4d1c2d67f15a",
        Driver::PS2000A => {
            "SHA256:0fae6f5a64ed873b343cd37d19659f7ca7f2b2f449951916d7c1b86ce877e849"
        }
        Driver::PS3000A => {
            "SHA256:17103fa43fc92726b2c57b004f160de7b0dbaa853e0a5903de0a2033daa9a5dc"
        }
        Driver::PS4000 => "SHA256:23856ea1bd93299fd343dfbcee3d88eb1c39299e7d2046a19bf9faf5051a4ce7",
        Driver::PS4000A => {
            "SHA256:cf6bbc7d54fe7cd18243dbd6a3f30bb53dec40d36bda523dd43bb9cd953059e7"
        }
        Driver::PS5000A => {
            "SHA256:dac03cf4aa9d24f9828ada0cee14624c78a44f33f3493f4c66ea1c0614468b57"
        }
        Driver::PS6000 => "SHA256:5da7622446aed202724ac7754190a68e0bb1165a52bf5a1792e5f664d536a578",
        Driver::PicoIPP => {
            "SHA256:23d90a67a0d545e8377a60ef11434d4a744afaa0bf0b7b4f0322a1f75bc26ac8"
        }
        Driver::IOMP5 => "SHA256:f76e798755b75f5dc2faa6474375a4c185da65b0ed0528336bd83b4e58d20256",
        _ => "",
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    match driver {
        Driver::PS2000 => "SHA256:9c3859644b08bb9462bdbfa06782081a8e9a0374b0f6b1c335c21f3dfb02a01d",
        Driver::PS2000A => {
            "SHA256:e0a6f4945946f512b543dee27a1aba38f0373f060d8d239726824ddd902fb1c5"
        }
        Driver::PS3000A => {
            "SHA256:887de4943817d698203e0bc814290c4b9a86c1cd89323eefd764c157fd959665"
        }
        Driver::PS4000 => "SHA256:5eb3c0f122cc9acc5d49023b7dba9ab77a9c38d66ec93d202f8b8dfa0356d182",
        Driver::PS4000A => {
            "SHA256:3fcb22d9d0da7e2612692d84b7476c2bef044ac86ffd73ce0f339f9a7e091d5b"
        }
        Driver::PS5000A => {
            "SHA256:4e15a1ebd358a71bfefd51844a2390f96efea7abe2e40c8e89121e9ba538f28c"
        }
        Driver::PS6000 => "SHA256:90275b12fd6bb78df36a45f80d5e7cc8360f795039b7a7fc7b2e1ee4ad7af9aa",
        Driver::PicoIPP => {
            "SHA256:69ba217f477c5bd7dcca9d3fac0efa17ff8cdca59c27c927ea8f2ac606781b60"
        }
        Driver::IOMP5 => "SHA256:64a85a2b94c307d95ed9a08a613bf72261afeb6b1e0a295eb6e410f565f5fd69",
        _ => "",
    }
    #[cfg(all(target_os = "linux", target_arch = "arm"))]
    match driver {
        Driver::PS2000 => "SHA256:c016a02417ea3a5e509f50db6b02ee356b7f495e267491562968ec7e68d03c43",
        Driver::PS2000A => {
            "SHA256:0592e0452dd4df2f320d7d2e719acfc8d0668ba73606c7212e1579a6ce8f1ef0"
        }
        Driver::PS3000A => {
            "SHA256:d0bb5cf52765f9243801b888b51d6296284d4caa6d3198991077ece43efa528b"
        }
        Driver::PS4000 => "SHA256:835fda4f3608e2d423d2665c379a9135cd0385ad91b06ebff25a9742c98427c5",
        Driver::PS4000A => {
            "SHA256:cc94889fe114ce1d67fb4262e9aab9aee14d3013b748152949b279f23f6d7cc9"
        }
        Driver::PS5000A => {
            "SHA256:af58a1c0bfedea56d5ce9b2c53d69ec1e41f6098d1d03ce9b14579277893327f"
        }
        Driver::PS6000 => "SHA256:32ca7e7eebac2c1772423d3b5b74eaac5559bac8b33454f1757d3cfd4b788a14",
        Driver::PicoIPP => {
            "SHA256:cde63e4aaafd567acc6c991c403a600fd0b8e4274c85ccad4eed04952b51daad"
        }
        _ => "",
    }
}
