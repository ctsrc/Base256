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

#[cfg(feature = "codec_eff")]
mod eff;

#[cfg(feature = "codec_eff")]
pub use eff::*;

#[cfg(feature = "encode")]
pub trait Encode<I: Iterator, C> {
    fn encode(self) -> C;
}

#[cfg(feature = "encode")]
#[cfg(test)]
mod test_cases_encode {
    use super::*;
    use std::io::{Cursor, Read};
    use test_case::test_case;

    #[cfg(feature = "codec_eff")]
    #[test_case(&[0x05u8; 3], &["acuteness"; 3] ; "eff encoder 0x05 0x05 0x05")]
    fn test_encoder(bytes: &[u8], expected_result: &[&str]) {
        let bytes = Cursor::new(bytes).bytes();
        let encoded = bytes
            .into_iter()
            .encode()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        //dbg!(&encoded);
        assert_eq!(encoded, expected_result);
    }
}
