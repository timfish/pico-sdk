#[cfg(not(target_os = "windows"))]
pub fn is_kernel_driver_installed() -> bool {
    true
}

#[cfg(target_os = "windows")]
pub fn is_kernel_driver_installed() -> bool {
    use std::{
        ffi::OsStr,
        iter::once,
        mem::size_of,
        os::windows::ffi::OsStrExt,
        ptr::{null, null_mut},
    };

    use winapi::um::setupapi::*;

    let flags = DIGCF_ALLCLASSES;
    let wide: Vec<u16> = OsStr::new("USB").encode_wide().chain(once(0)).collect();

    let dev_info = unsafe { SetupDiGetClassDevsW(null(), wide.as_ptr(), null_mut(), flags) };

    let mut dev_info_data = SP_DEVINFO_DATA {
        cbSize: size_of::<SP_DEVINFO_DATA>() as u32,
        ..Default::default()
    };

    let mut i = 0;
    while unsafe { SetupDiEnumDeviceInfo(dev_info, i, &mut dev_info_data) } > 0 {
        if unsafe { SetupDiBuildDriverInfoList(dev_info, &mut dev_info_data, SPDIT_COMPATDRIVER) }
            > 0
        {
            let mut j = 0;
            loop {
                let mut drv_info_data = SP_DRVINFO_DATA_V2_W {
                    cbSize: std::mem::size_of::<SP_DRVINFO_DATA_V2_W>() as u32,
                    ..Default::default()
                };

                if unsafe {
                    SetupDiEnumDriverInfoW(
                        dev_info,
                        &mut dev_info_data,
                        SPDIT_COMPATDRIVER,
                        j,
                        &mut drv_info_data,
                    )
                } == 0
                {
                    break;
                }

                let mfg = drv_info_data.MfgName;
                if String::from_utf16_lossy(&mfg)
                    .trim_matches(char::from(0))
                    .contains("Pico Technology Ltd")
                {
                    return true;
                }

                j += 1;
            }
        }

        i += 1;
    }

    false
}
