// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation

fn main() {
    #[cfg(any(feature = "wl_eff", feature = "wl_pgp"))]
    {
        use std::env;
        use std::fs::{read_to_string, File};
        use std::io::{BufRead, BufReader, Write};
        use std::path::Path;

        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("256.rs");
        let mut f_dest = File::create(&dest_path).unwrap();

        #[cfg(feature = "wl_eff")]
        {
            f_dest
                .write_all(b"pub const WL_AUTOCOMPLETE: &[&str] = &[")
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
            f_dest.write_all(b"];\n").unwrap();
        }

        #[cfg(feature = "wl_pgp")]
        {
            f_dest
                .write_all(b"pub const WL_PGPFONE_THREE_SYLLABLE: &[&str] = &[")
                .unwrap();
            let words = read_to_string("pgpfone_three_syllable_word_list.txt").unwrap();
            for word in words.split(' ') {
                write!(f_dest, "\"{word}\",").unwrap();
            }
            f_dest.write_all(b"];\n").unwrap();

            f_dest
                .write_all(b"pub const WL_PGPFONE_TWO_SYLLABLE: &[&str] = &[")
                .unwrap();
            let words = read_to_string("pgpfone_two_syllable_word_list.txt").unwrap();
            for word in words.split(' ') {
                write!(f_dest, "\"{word}\",").unwrap();
            }
            f_dest.write_all(b"];\n").unwrap();
        }
    }
}
