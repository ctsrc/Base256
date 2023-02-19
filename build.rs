/*
 * Copyright (c) 2018, 2023 Erik Nordstrøm <erik@nordstroem.no>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation

use std::fmt::{Debug, Formatter};
include!("src/decode/include/candidate_words.rs");

fn main() {
    #[cfg(any(
        feature = "wl_eff_encode",
        feature = "wl_eff_decode",
        feature = "wl_pgp_encode",
        feature = "wl_pgp_decode"
    ))]
    {
        use std::env;
        use std::fs::{read_to_string, File};
        use std::io::{BufRead, BufReader, Write};
        use std::path::Path;

        let out_dir = env::var("OUT_DIR").unwrap();

        let dest_path_enc = Path::new(&out_dir).join("wl_encode.rs");
        let mut f_dest_enc = File::create(&dest_path_enc).unwrap();

        let dest_path_dec = Path::new(&out_dir).join("wl_decode.rs");
        let mut f_dest_dec = File::create(&dest_path_dec).unwrap();

        #[cfg(any(feature = "wl_eff_encode", feature = "wl_eff_decode"))]
        {
            let f_src = BufReader::new(File::open("eff_short_wordlist_2_0.txt").unwrap());
            let mut words = vec![];
            for (i, line) in f_src.lines().take(1024).enumerate() {
                if i % 4 == 0 {
                    let line = line.unwrap();
                    let word = line.split('\t').nth(1).unwrap().to_string();
                    words.push(word);
                }
            }

            #[cfg(feature = "wl_eff_encode")]
            {
                writeln!(f_dest_enc, "/// EFF Short Wordlist 2.0 (encode)").unwrap();
                writeln!(f_dest_enc, "pub const WL_EFF_ENCODE: &[&str] = &{words:?};").unwrap();
            }

            #[cfg(feature = "wl_eff_decode")]
            {
                let words_lower: Vec<_> = words.iter().map(|w| w.to_lowercase()).collect();
                let mut words_decode: Vec<_> = words_lower
                    .iter()
                    .enumerate()
                    .map(|(pos, word)| WordlistDecodeEntry {
                        word,
                        byte: pos as u8,
                    })
                    .collect();
                words_decode.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let wl_subsets: Vec<(_, _)> =
                    words_decode
                        .clone()
                        .into_iter()
                        .fold(Vec::new(), |mut acc, entry| {
                            if acc.is_empty() {
                                acc.push((entry.word.len(), vec![entry]));
                            } else {
                                let curr_subset = acc.last_mut().unwrap();
                                if curr_subset.0 == entry.word.len() {
                                    curr_subset.1.push(entry);
                                } else {
                                    acc.push((entry.word.len(), vec![entry]));
                                }
                            }
                            acc
                        });
                let wl_subsets: Vec<_> = wl_subsets
                    .iter()
                    .map(|wl_subset| WordlistSubset {
                        word_len: wl_subset.0,
                        words: &wl_subset.1,
                    })
                    .collect();

                writeln!(f_dest_dec, "/// EFF Short Wordlist 2.0 (decode)").unwrap();
                writeln!(
                    f_dest_dec,
                    "const WL_EFF_DECODE: &[WordlistSubset] = &{wl_subsets:?};"
                )
                .unwrap();

                drop(wl_subsets);
            }
        }

        #[cfg(any(feature = "wl_pgp_encode", feature = "wl_pgp_decode"))]
        {
            let words_3_s = read_to_string("pgpfone_three_syllable_word_list.txt").unwrap();
            let words_3: Vec<_> = words_3_s.split(' ').collect();
            let words_2_s = read_to_string("pgpfone_two_syllable_word_list.txt").unwrap();
            let words_2: Vec<_> = words_2_s.split(' ').collect();

            #[cfg(feature = "wl_pgp_encode")]
            {
                writeln!(
                    f_dest_enc,
                    "/// PGP Word List (encode) -- PGPfone Three Syllable Word List"
                )
                .unwrap();
                writeln!(
                    f_dest_enc,
                    "pub const WL_PGP_ENCODE_THREE_SYLLABLE: &[&str] = &{words_3:?};"
                )
                .unwrap();

                writeln!(
                    f_dest_enc,
                    "/// PGP Word List (encode) -- PGPfone Two Syllable Word List"
                )
                .unwrap();
                writeln!(
                    f_dest_enc,
                    "pub const WL_PGP_ENCODE_TWO_SYLLABLE: &[&str] = &{words_2:?};"
                )
                .unwrap();
            }
        }
    }
}
