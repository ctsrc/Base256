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

/// Base 256 encoder using PGP Word List
#[derive(Clone, Debug)]
pub struct PgpEncode<I: Iterator> {
    iter: I,
    odd_even: u8,
}

impl<I, E> Iterator for PgpEncode<I>
where
    I: Iterator<Item = Result<u8, E>>,
{
    type Item = Result<&'static str, E>;

    fn next(&mut self) -> Option<Self::Item> {
        let odd_even = self.odd_even;
        self.odd_even = (odd_even + 1) % 2;
        match self.iter.next()? {
            Ok(byte) => {
                if odd_even == 0 {
                    Some(Ok(crate::WL_PGPFONE_TWO_SYLLABLE[byte as usize]))
                } else {
                    Some(Ok(crate::WL_PGPFONE_THREE_SYLLABLE[byte as usize]))
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

impl<I: Iterator<Item = Result<u8, E>>, E> crate::Encode<I, PgpEncode<I>> for I {
    fn encode(self) -> PgpEncode<I> {
        PgpEncode {
            iter: self,
            odd_even: 0,
        }
    }
}

#[cfg(test)]
mod test_cases_encode {
    use super::super::Encode;
    use super::PgpEncode;
    use std::io::{Cursor, Read};
    use test_case::test_case;

    #[test_case(&[0x05u8; 3], &["adult", "amulet", "adult"] ; "data 0x05 0x05 0x05")]
    fn test_pgp_encoder(bytes: &[u8], expected_result: &[&str]) {
        let bytes = Cursor::new(bytes).bytes().into_iter();
        let encoded = Encode::<_, PgpEncode<_>>::encode(bytes)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        //dbg!(&encoded);
        assert_eq!(encoded, expected_result);
    }
}
