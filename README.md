# incdir
Compile-time including of directories.

This crate works in a similar fashion as the `include_bytes!` macro in Rust, except it includes
a whole directory and stores them in a perfect hash function map from the [phf](https://crates.io/crates/phf) crate.

All pathnames in the directory processed by the `include_dir!` macro must be valid UTF-8.

# Usage
```
[dependencies]
incdir = "0.1.0"
phf = { version = "*", features = ["macros"] }
```

```
#![feature(proc_macro_hygiene)]
use phf::Map;

static TEXTURES: Map<&'static str, &'static [u8]> = incdir::include_dir!("textures");

fn main() {
    // The file is stored in "files/player.png", the directory prefix is stripped in the map.
    let player = TEXTURES.get("player.png").unwrap();
    // Stored in "textures/world/grass.png".
    let grass = TEXTURES.get("world/grass.png").unwrap()
}
```

## License
[MIT](LICENSE)