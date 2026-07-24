use crate::LibraryResolution;
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

/// Loads a driver's runtime dependencies into memory so it can be loaded from a non-standard
/// path.
///
/// Some Pico drivers (ps4000 and ps6000) load an external library by bare name at runtime. When
/// we're not loading from the platform's default search paths, we preload that library from the
/// driver's own directory so the dynamic loader can find it. Drivers with no dependencies get an
/// empty result and load nothing.
pub fn load_dependencies<P: AsRef<Path>>(driver: Driver, path: P) -> LoadedDependencies {
    let parent = path.as_ref().parent().expect("Driver path has no parent");
    let to_load = driver.dependencies();
    let resolution = LibraryResolution::Custom(parent.to_path_buf());

    let mut output = Vec::with_capacity(to_load.len());

    for &dependency in to_load {
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

fn load_ipp<P: AsRef<Path>>(path: P) -> Result<(LoadedFn, Library), Error> {
    let path = path.as_ref().to_path_buf();
    let ipp = unsafe { Library::new(path)? };
    let sym = unsafe { ipp.get(b"Pico_ippsAbs_32f")? };

    Ok((*sym, ipp))
}
