use super::LibraryResolution;
use lazy_static::lazy_static;
use libloading::{Error, Library};
use parking_lot::Mutex;
use pico_common::Driver;
use std::sync::{Arc, Weak};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

type LoadedFn = fn();
type LoadedDependency = (LoadedFn, Library);
pub type LoadedDependencies = Vec<Arc<LoadedDependency>>;

lazy_static! {
    // We only hold a weak reference so that they're dropped when there are no
    // more drivers loaded.
    static ref DEPENDENCY_CACHE: Mutex<HashMap<PathBuf, Weak<LoadedDependency>>> =
        Default::default();
}

/// Loads driver dependencies into memory so drivers can be loaded from
/// non-standard paths
///
/// Depending on which platform you're running on, most Pico drivers depend on one
/// or more external libraries. If we're not loading drivers from the platforms
/// default search paths, we can manually load the dependencies into memory so
/// that they can be found.
pub fn load_dependencies<P: AsRef<Path>>(path: P) -> LoadedDependencies {
    let parent = path.as_ref().parent().expect("Driver path has no parent");
    let to_load = Driver::get_dependencies_for_platform();
    let resolution = LibraryResolution::Custom(parent.to_path_buf());

    let mut output = Vec::with_capacity(to_load.len());

    for dependency in to_load {
        let path = resolution.get_path(dependency);
        if let Some(dep) = load_dependency(&path, dependency) {
            output.push(dep);
        }
    }

    output
}

fn load_dependency(path: &Path, dependency: Driver) -> Option<Arc<LoadedDependency>> {
    let mut cache = DEPENDENCY_CACHE.lock();

    let load = || {
        match dependency {
            Driver::IOMP5 => load_iomp5(path),
            Driver::PicoIPP => load_ipp(path),
            _ => panic!("This method should only be used to load dependencies"),
        }
        .ok()
        .map(|dep| {
            let strong = Arc::new(dep);
            #[allow(clippy::redundant_clone)]
            let weak = Arc::downgrade(&strong.clone());
            (strong, weak)
        })
    };

    match cache.get(path) {
        Some(weak) => match weak.upgrade() {
            Some(strong) => Some(strong),
            None => load().map(|(strong, weak)| {
                cache.insert(path.to_path_buf(), weak);
                strong
            }),
        },
        None => load().map(|(strong, weak)| {
            cache.insert(path.to_path_buf(), weak);
            strong
        }),
    }
}

fn load_iomp5<P: AsRef<Path>>(path: P) -> Result<(LoadedFn, Library), Error> {
    let path = path.as_ref().to_path_buf();
    let iomp5 = unsafe { Library::new(path)? };
    let sym = unsafe { iomp5.get(b"ompc_set_num_threads")? };

    Ok((*sym, iomp5))
}

fn load_ipp<P: AsRef<Path>>(path: P) -> Result<(LoadedFn, Library), Error> {
    let path = path.as_ref().to_path_buf();
    let ipp = unsafe { Library::new(path)? };
    let sym = unsafe { ipp.get(b"Pico_ippsAbs_32f")? };

    Ok((*sym, ipp))
}
