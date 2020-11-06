use super::Resolution;
use libloading::{Error, Library};
use pico_common::Driver;
use std::path::Path;
use std::sync::Arc;

type LoadedFn = fn();

fn load_iomp5<P: AsRef<Path>>(path: P) -> Result<(LoadedFn, Library), Error> {
    let path = path.as_ref().to_path_buf();
    let iomp5 = Library::new(path)?;
    let sym = unsafe { iomp5.get(b"ompc_set_num_threads")? };

    Ok((*sym, iomp5))
}

fn load_ipp<P: AsRef<Path>>(path: P) -> Result<(LoadedFn, Library), Error> {
    let path = path.as_ref().to_path_buf();
    let ipp = Library::new(path)?;
    let sym = unsafe { ipp.get(b"Pico_ippsAbs_32f")? };

    Ok((*sym, ipp))
}

/// Loads driver dependencies into memory so drivers can be loaded from
/// non-standard paths
///
/// Depending on which platform you're running on, most Pico drivers depend on one
/// or more external libraries. If we're not loading drivers from the platforms
/// default search paths, we can manually load the dependencies so that they can
/// be found.
#[derive(Clone)]
pub struct DependencyLoader {
    dependencies: Arc<Vec<(LoadedFn, Library)>>,
}

impl DependencyLoader {
    pub fn try_load(resolution: &Resolution) -> Result<Self, ()> {
        let mut dependencies = vec![];

        for dependency in Driver::get_dependencies_for_platform() {
            let path = resolution.get_path(dependency);

            match dependency {
                Driver::IOMP5 => {
                    if let Ok(dep) = load_iomp5(path) {
                        dependencies.push(dep);
                    } else {
                        return Err(());
                    }
                }
                Driver::PicoIPP => {
                    if let Ok(dep) = load_ipp(path) {
                        dependencies.push(dep);
                    } else {
                        return Err(());
                    }
                }
                _ => {}
            }
        }

        Ok(DependencyLoader {
            dependencies: Arc::new(dependencies),
        })
    }
}
