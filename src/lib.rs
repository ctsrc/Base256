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

//! Encode and decode data in base 256

#![forbid(unsafe_code)]

#[cfg(not(any(
    feature = "encode",
    feature = "decode",
    feature = "wl_eff_encode",
    feature = "wl_pgp_encode"
)))]
compile_error!("Building lib target requires that at least one of the following features is enabled: encode; decode; wl_eff_encode; wl_pgp_encode");

#[cfg(any(
    feature = "decode",
    feature = "wl_eff_decode",
    feature = "wl_pgp_decode"
))]
mod decode;
#[cfg(any(
    feature = "encode",
    feature = "wl_eff_encode",
    feature = "wl_pgp_encode"
))]
mod encode;

#[cfg(any(
    feature = "decode",
    feature = "wl_eff_decode",
    feature = "wl_pgp_decode"
))]
pub use decode::*;
#[cfg(any(
    feature = "encode",
    feature = "wl_eff_encode",
    feature = "wl_pgp_encode"
))]
pub use encode::*;

#[cfg(all(feature = "encode", feature = "decode"))]
#[cfg(test)]
mod test_cases_encode {
    use super::{Decode, Encode};
    #[cfg(all(feature = "encode_eff", feature = "decode_eff"))]
    use super::{EffDecode, EffEncode};
    #[cfg(all(feature = "encode_pgp", feature = "decode_pgp"))]
    use super::{PgpDecode, PgpEncode};
    use std::io::{Cursor, Read};
    use test_case::test_case;
    use utf8_chars::BufReadCharsExt;

    #[cfg(all(feature = "encode_pgp", feature = "decode_pgp"))]
    #[test_case(&[0x00u8; 3] ; "data 0x00 0x00 0x00")]
    #[test_case(&*(0x00u8..=0xFF).collect::<Vec<_>>() ; "data 0x00..0xFF")]
    #[test_case(&*(0x01u8..=0xFF).collect::<Vec<_>>() ; "data 0x01..0xFF")]
    fn test_positive_roundtrip_pgp_codec(bytes_orig: &[u8]) {
        let bytes = Cursor::new(bytes_orig).bytes().into_iter();
        let mut encoded_words = Encode::<_, PgpEncode<_>>::encode(bytes)
            .collect::<Result<String, _>>()
            .unwrap();
        let mut cursor = Cursor::new(encoded_words);
        let words_chars = cursor.chars().into_iter();
        let decoded_bytes = Decode::<_, PgpDecode<_>>::decode(words_chars)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(bytes_orig, decoded_bytes);
    }

    #[cfg(all(feature = "encode_eff", feature = "decode_eff"))]
    #[test_case(&[0x00u8; 3] ; "data 0x00 0x00 0x00")]
    #[test_case(&*(0x00u8..=0xFF).collect::<Vec<_>>() ; "data 0x00..0xFF")]
    #[test_case(&*(0x01u8..=0xFF).collect::<Vec<_>>() ; "data 0x01..0xFF")]
    fn test_positive_roundtrip_eff_codec(bytes_orig: &[u8]) {
        let bytes = Cursor::new(bytes_orig).bytes().into_iter();
        let mut encoded_words = Encode::<_, EffEncode<_>>::encode(bytes)
            .collect::<Result<String, _>>()
            .unwrap();
        let mut cursor = Cursor::new(encoded_words);
        let words_chars = cursor.chars().into_iter();
        let decoded_bytes = Decode::<_, EffDecode<_>>::decode(words_chars)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(bytes_orig, decoded_bytes);
    }
}
