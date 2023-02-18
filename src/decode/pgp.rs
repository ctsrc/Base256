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

/// Base 256 decoder using PGP Word List
#[derive(Clone, Debug)]
pub struct PgpDecode<I: Iterator> {
    iter: I,
    odd_even: u8,
}

impl<I, D> Iterator for PgpDecode<I>
where
    I: Iterator<Item = Result<char, D>>,
{
    type Item = Result<u8, D>;

    fn next(&mut self) -> Option<Self::Item> {
        let odd_even = self.odd_even;
        self.odd_even = (odd_even + 1) % 2;
        todo!();
    }
}

impl<I: Iterator<Item = Result<char, D>>, D> crate::Decode<I, PgpDecode<I>> for I {
    fn decode(self) -> PgpDecode<I> {
        PgpDecode {
            iter: self,
            odd_even: 0,
        }
    }
}

#[cfg(test)]
mod test_cases_decode {
    use super::super::Decode;
    use super::PgpDecode;
    use std::io::Cursor;
    use test_case::test_case;
    use utf8_chars::BufReadCharsExt;

    #[test_case("adult amulet adult ", &[0x05u8; 3]; "words spaced")]
    #[test_case("a dult amu let adu lt ", &[0x05u8; 3]; "words extra space")]
    #[test_case("adultamuletadult", &[0x05u8; 3]; "words mushed")]
    #[test_case("adult amulet \nadult ", &[0x05u8; 3]; "words spaced wrapped")]
    #[test_case("adultamuletad\nult", &[0x05u8; 3]; "words mushed wrapped")]
    #[test_case("ADULT AMULET ADULT ", &[0x05u8; 3]; "words spaced uppercase")]
    #[test_case("Adult AMUlet aDULT ", &[0x05u8; 3]; "words spaced mixed-case")]
    fn test_pgp_decoder_positive(words: &str, expected_bytes: &[u8]) {
        let mut cursor = Cursor::new(words);
        let words_chars = cursor.chars().into_iter();
        let decoded_bytes = Decode::<_, PgpDecode<_>>::decode(words_chars)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(decoded_bytes, expected_bytes);
    }
}
