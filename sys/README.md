# pico-sys-dynamic

Unsafe dynamically loaded bindings for every Pico Technology oscilloscope driver. These
were generated from official header files with `bindgen` before manual clean up.

```rust
use pico_sys_dynamic::ps2000::PS2000Loader;

let ps2000 = unsafe { PS2000Loader::new("./path/ps2000.dll").unwrap() };
let handle = unsafe { ps2000.ps2000_open_unit() };
```

License: MIT
