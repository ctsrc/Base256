/*
 * Copyright (c) 2023 Erik Nordstrøm <erik@nordstroem.no>
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

use super::WordlistSubset;

/// Base 256 decoder using PGP Word List
#[derive(Clone, Debug)]
pub struct PgpDecode<I: Iterator> {
    iter: I,
    odd_even: u8,
    candidate_wl_subsets_remaining: Vec<WordlistSubset<'static>>,
    prev_match_len: usize,
    curr_match_len: usize,
}

impl<I> Iterator for PgpDecode<I>
where
    I: Iterator<Item = Result<char, std::io::Error>>,
{
    type Item = Result<u8, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        for word_byte in self.iter.by_ref() {
            // We immediately return the error if one is encountered.
            let Ok(word_char) = word_byte else { return Some(Err(word_byte.unwrap_err())) };

            let word_chars: Vec<_> = word_char.to_lowercase().collect();

            // We skip space, newline and carriage return characters
            if word_chars == [' '] || word_chars == ['\n'] || word_chars == ['\r'] {
                continue;
            }

            self.curr_match_len += word_chars.len();
            //dbg!(self.curr_match_len);
            //dbg!(&self.candidate_wl_subsets_remaining);

            // Remove subsets that are too short from the current set of possible matches.
            let first_subset_remaining = self
                .candidate_wl_subsets_remaining
                .partition_point(|wl| wl.word_len < self.curr_match_len);
            self.candidate_wl_subsets_remaining =
                self.candidate_wl_subsets_remaining[first_subset_remaining..].to_owned();

            for subset in self.candidate_wl_subsets_remaining.iter_mut() {
                // Find first word in subset that matches so far
                // TODO: Use partition_point()
                let mut subset_words_idx_low = 0;
                for entry in subset.words {
                    let word_remainder_to_match = &entry.word[self.prev_match_len..];
                    if word_remainder_to_match.starts_with(&*word_chars) {
                        //dbg!(entry.word, &word_chars, word_remainder_to_match);
                        break;
                    }
                    subset_words_idx_low += 1;
                }
                //dbg!(subset.words);
                subset.words = &subset.words[subset_words_idx_low..];
                //dbg!(subset.words);

                // Find last word in subset that matches so far
                // TODO: Use partition_point()
                let mut subset_words_idx_high = 0;
                for entry in subset.words {
                    let word_remainder_to_match = &entry.word[self.prev_match_len..];
                    if !word_remainder_to_match.starts_with(&*word_chars) {
                        //dbg!(entry.word, &word_chars, word_remainder_to_match);
                        break;
                    }
                    subset_words_idx_high += 1;
                }
                //dbg!(subset.words);
                subset.words = &subset.words[..subset_words_idx_high];
                //dbg!(subset.words);
            }

            // Remove empty subsets
            self.candidate_wl_subsets_remaining = self
                .candidate_wl_subsets_remaining
                .clone()
                .into_iter()
                .filter(|wl| !wl.words.is_empty())
                .collect();

            self.prev_match_len = self.curr_match_len;

            // No candidates remaining means input data was not valid
            if self.candidate_wl_subsets_remaining.is_empty() {
                return Some(Err(std::io::Error::from(std::io::ErrorKind::InvalidData)));
            }

            // Check for exact match
            if self.candidate_wl_subsets_remaining.len() == 1 {
                //dbg!(&self.candidate_wl_subsets_remaining);
                if self.candidate_wl_subsets_remaining[0].words.len() == 1
                    && self.curr_match_len == self.candidate_wl_subsets_remaining[0].word_len
                {
                    let ret_byte = self.candidate_wl_subsets_remaining[0].words[0].byte;

                    self.odd_even = (self.odd_even + 1) % 2;
                    self.candidate_wl_subsets_remaining = if self.odd_even == 0 {
                        super::WL_PGP_DECODE_TWO_SYLLABLE
                    } else {
                        super::WL_PGP_DECODE_THREE_SYLLABLE
                    }
                    .to_vec();
                    self.prev_match_len = 0;
                    self.curr_match_len = 0;

                    return Some(Ok(ret_byte));
                }
            }
        }
        None
    }
}

impl<I: Iterator<Item = Result<char, D>>, D> crate::Decode<I, PgpDecode<I>> for I {
    fn decode(self) -> PgpDecode<I> {
        PgpDecode {
            iter: self,
            odd_even: 0,
            candidate_wl_subsets_remaining: super::WL_PGP_DECODE_TWO_SYLLABLE.to_vec(),
            prev_match_len: 0,
            curr_match_len: 0,
        }
    }
}

#[cfg(test)]
mod test_cases_decode {
    use super::super::Decode;
    use super::PgpDecode;
    use std::fs::File;
    use std::io::{BufReader, Cursor};
    use std::path::Path;
    use test_case::test_case;
    use utf8_chars::BufReadCharsExt;

    #[test_case("adult amulet adult "; "words spaced")]
    #[test_case("a  dult amu let adu   lt "; "words extra space")]
    #[test_case("adultamuletadult"; "words mushed")]
    #[test_case("adult amulet \nadult "; "words spaced wrapped")]
    #[test_case("adultamuletad\nult"; "words mushed wrapped")]
    #[test_case("ADULT AMULET ADULT "; "words spaced uppercase")]
    #[test_case("Adult AMUlet aDULT "; "words spaced mixed-case")]
    fn test_positive_pgp_decoder_0x05_0x05_0x05(words: &str) {
        let mut cursor = Cursor::new(words);
        let words_chars = cursor.chars().into_iter();
        let decoded_bytes = Decode::<_, PgpDecode<_>>::decode(words_chars)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(decoded_bytes, &[0x05u8; 3]);
    }

    #[test_case("id_ed25519.txt")]
    #[test_case("id_ed25519-fold_w_78.txt")]
    #[test_case("id_ed25519-fold_w_78_s.txt")]
    #[test_case("id_ed25519-fold_w_78_s-trimmed.txt")]
    fn test_positive_pgp_decoder_sample_data_file_id_ed25519<P: AsRef<Path>>(fpath_encoded: P) {
        let fpath_original_id_ed25519 = "sample_data/original/id_ed25519";
        let expected_bytes = std::fs::read(fpath_original_id_ed25519).unwrap();

        let fpath_encoded = Path::new("sample_data/encoded/pgp").join(fpath_encoded);
        let mut input_encoded = BufReader::new(File::open(fpath_encoded).unwrap());

        let decoded_bytes = Decode::<_, PgpDecode<_>>::decode(input_encoded.chars())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(decoded_bytes, expected_bytes);
    }
}
