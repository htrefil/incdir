//! Compile-time including of directories.
//!
//! This crate works in a similar fashion as the `include_bytes!` macro in Rust, except it includes
//! a whole directory and stores them in a perfect hash function map from the [phf](https://crates.io/crates/phf) crate.  
//! For cross-platform consistency, backslashes in paths stored in the map are replaced with forward slashes.
//!
//! All pathnames in the directory processed by the `include_dir!` macro must be valid UTF-8.
//! # Usage
//! ```toml
//! [dependencies]
//! incdir = "0.1.0"
//! phf = { version = "*", features = ["macros"] }
//! ```
//!
//! ```ignore
//! #![feature(proc_macro_hygiene)]
//! use phf::Map;
//!
//! static TEXTURES: Map<&'static str, &'static [u8]> = incdir::include_dir!("textures");
//!
//! fn main() {
//!     // The file is stored in "files/player.png", the directory prefix is stripped in the map.
//!     let player = TEXTURES.get("player.png").unwrap();
//!     // Stored in "textures/world/grass.png".
//!     let grass = TEXTURES.get("world/grass.png").unwrap()
//! }
//! ```
extern crate proc_macro;

use proc_macro::TokenStream;
use std::env;
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};
use syn::{Expr, Lit};

/// The proc macro that does the hard work.
/// Returns a `phf::Map<&'static str, &'static [u8]>`.
#[proc_macro]
pub fn include_dir(input: TokenStream) -> TokenStream {
    let path = match syn::parse_macro_input!(input as Expr) {
        Expr::Lit(lit) => match lit.lit {
            Lit::Str(lit) => lit.value(),
            _ => panic!("expected a string literal"),
        },
        _ => panic!("expected a string literal"),
    };

    let prefix = env::current_dir().unwrap().join(Path::new(&path));
    let files = match collect_files(&prefix) {
        Ok(files) => files,
        Err(err) => panic!("failed to read directory: {}", err),
    };
    let names = files.iter().map(|path| {
        path.iter()
            .skip(prefix.iter().count())
            .collect::<PathBuf>()
            .to_str()
            .unwrap()
            .replace('\\', "/")
            .to_string()
    });
    let paths = files.iter().map(|path| path.to_str().unwrap().to_string());

    (quote::quote! {
        {
            let map: phf::Map<&'static str, &'static [u8]> = {
                phf::phf_map! {
                    #(#names => include_bytes!(#paths),)*
                }
            };

            map
        }
    })
    .into()
}

fn collect_files(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_file() {
            files.push(path);
        } else {
            files.extend_from_slice(&collect_files(&path)?);
        }
    }

    Ok(files)
}
