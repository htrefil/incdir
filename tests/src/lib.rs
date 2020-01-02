#![feature(proc_macro_hygiene)]

#[cfg(test)]
mod tests {
    use phf::Map;

    #[test]
    fn it_works() {
        let files: Map<&'static str, &'static [u8]> = incdir::include_dir!("tests/files");

        assert_eq!(*files.get("1.txt").unwrap(), b"first file");
        assert_eq!(*files.get("2/1.txt").unwrap(), b"second/first file");
        assert_eq!(*files.get("3.txt").unwrap(), b"third file");
    }
}
