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

/// Base 256 encoder using EFF Short Wordlist 2.0
#[derive(Clone, Debug)]
pub struct EffEncode<I: Iterator> {
    iter: I,
}

impl<I, E> Iterator for EffEncode<I>
where
    I: Iterator<Item = Result<u8, E>>,
{
    type Item = Result<&'static str, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next()? {
            Ok(byte) => Some(Ok(crate::WL_EFF_ENCODE[byte as usize])),
            Err(e) => Some(Err(e)),
        }
    }
}

impl<I: Iterator<Item = Result<u8, E>>, E> crate::Encode<I, EffEncode<I>> for I {
    fn encode(self) -> EffEncode<I> {
        EffEncode { iter: self }
    }
}

#[cfg(test)]
mod test_cases_encode {
    use super::super::Encode;
    use super::EffEncode;
    use std::io::{Cursor, Read};
    use test_case::test_case;

    #[test_case(&[0x05u8; 3], &["acuteness"; 3] ; "data 0x05 0x05 0x05")]
    fn test_positive_eff_encoder(bytes: &[u8], expected_words: &[&str]) {
        let bytes = Cursor::new(bytes).bytes().into_iter();
        let encoded_words = Encode::<_, EffEncode<_>>::encode(bytes)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(encoded_words, expected_words);
    }
}
