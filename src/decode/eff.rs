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

/// Base 256 decoder using EFF Short Wordlist 2.0
#[derive(Clone, Debug)]
pub struct EffDecode<I: Iterator> {
    iter: I,
    candidate_wl_subsets_remaining: Vec<WordlistSubset<'static>>,
    prev_match_len: usize,
    curr_match_len: usize,
}

impl<I, D> Iterator for EffDecode<I>
where
    I: Iterator<Item = Result<char, D>>,
{
    type Item = Result<u8, D>;

    fn next(&mut self) -> Option<Self::Item> {
        for word_byte in self.iter.by_ref() {
            // We immediately return the error if one is encountered.
            let Ok(word_char) = word_byte else { return Some(Err(word_byte.err().unwrap())) };

            let word_char: Vec<_> = word_char.to_lowercase().collect();

            // We skip space, newline and carriage return characters
            if word_char == [' '] || word_char == ['\n'] || word_char == ['\r'] {
                continue;
            }

            self.curr_match_len += word_char.len();
            dbg!(self.curr_match_len);

            // Remove subsets that are too short from the current set of possible matches.
            let first_subset_remaining = self
                .candidate_wl_subsets_remaining
                .partition_point(|wl| wl.word_len < self.curr_match_len);
            self.candidate_wl_subsets_remaining =
                self.candidate_wl_subsets_remaining[first_subset_remaining..].to_owned();

            dbg!(&self.candidate_wl_subsets_remaining);
            if self.candidate_wl_subsets_remaining.len() <= 1 {
                break;
            }
        }
        None
    }
}

impl<I: Iterator<Item = Result<char, D>>, D> crate::Decode<I, EffDecode<I>> for I {
    fn decode(self) -> EffDecode<I> {
        EffDecode {
            iter: self,
            candidate_wl_subsets_remaining: super::WL_EFF_DECODE.to_vec(),
            prev_match_len: 0,
            curr_match_len: 0,
        }
    }
}

#[cfg(test)]
mod test_cases_decode {
    use super::super::Decode;
    use super::EffDecode;
    use std::io::Cursor;
    use test_case::test_case;
    use utf8_chars::BufReadCharsExt;

    #[test_case("acuteness acuteness acuteness "; "words spaced")]
    #[test_case("acute ness a cute ness acuten ess "; "words extra space")]
    #[test_case("acutenessacutenessacuteness"; "words mushed")]
    #[test_case("acuteness acuteness \nacuteness "; "words spaced wrapped")]
    #[test_case("acutenessacut\nenessacuteness"; "words mushed wrapped")]
    #[test_case("ACUTENESS ACUTENESS ACUTENESS "; "words spaced uppercase")]
    #[test_case("Acuteness ACUTEness acuteNESS "; "words spaced mixed-case")]
    fn test_eff_decoder_positive_0x05_0x05_0x05(words: &str) {
        let mut cursor = Cursor::new(words);
        let words_chars = cursor.chars().into_iter();
        let decoded_bytes = Decode::<_, EffDecode<_>>::decode(words_chars)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(decoded_bytes, &[0x05u8; 3]);
    }
}
