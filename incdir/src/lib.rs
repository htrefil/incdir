extern crate proc_macro;

use proc_macro::TokenStream;
use std::env;
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};
use syn::{Expr, Lit};

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
            .to_string()
    });
    let paths = files.iter().map(|path| path.to_str().unwrap().to_string());

    (quote::quote! {
        phf::phf_map! {
            #(#names => include_bytes!(#paths),)*
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
