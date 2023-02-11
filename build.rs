use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("256.rs");
    let mut f_dest = File::create(&dest_path).unwrap();

    f_dest
        .write_all(b"const WL_AUTOCOMPLETE: &'static [&'static str] = &[")
        .unwrap();

    let f_src = BufReader::new(File::open("eff_short_wordlist_2_0.txt").unwrap());
    for (i, line) in f_src.lines().take(1024).enumerate() {
        if i % 4 == 0 {
            f_dest.write_all(b"\"").unwrap();

            f_dest
                .write_all(line.unwrap().split('\t').nth(1).unwrap().as_bytes())
                .unwrap();

            f_dest.write_all(b"\",").unwrap();
        }
    }

    f_dest.write_all(b"];").unwrap();
}
