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

#[cfg(feature = "encode_eff")]
mod eff;
#[cfg(feature = "encode_pgp")]
mod pgp;

#[cfg(feature = "encode_eff")]
pub use eff::*;
#[cfg(feature = "encode_pgp")]
pub use pgp::*;

#[cfg(feature = "wl_eff_encode")]
include!(concat!(env!("OUT_DIR"), "/wl_eff_encode.rs"));
#[cfg(feature = "wl_pgp_encode")]
include!(concat!(env!("OUT_DIR"), "/wl_pgp_encode.rs"));

/// Base 256 encoder trait
#[cfg(feature = "encode")]
pub trait Encode<I: Iterator, E> {
    fn encode(self) -> E;
}

#[cfg(any(feature = "wl_eff_encode", feature = "wl_pgp_encode"))]
#[cfg(test)]
mod tests_word_lists_sorted_extent {
    use super::*;

    #[cfg(feature = "wl_eff_encode")]
    #[test]
    /// EFF Short Wordlist 2.0 (encode) is sorted.
    fn test_wl_eff_encode_is_sorted() {
        assert!(WL_EFF_ENCODE.windows(2).all(|w| w[0] <= w[1]));
    }

    #[cfg(feature = "wl_pgp_encode")]
    #[test]
    /// PGP Word List (encode) – PGPfone Three Syllable Word List is mostly sorted,
    /// except for the fact that the word "applicant" comes before the word "Apollo".
    fn test_wl_pgp_encode_three_syllable_lowercase_is_mostly_sorted() {
        assert!(WL_PGP_ENCODE_THREE_SYLLABLE.windows(2).all(|w| {
            //dbg!(w);
            if w[0] == "applicant" && w[1] == "Apollo" {
                true
            } else {
                w[0].to_lowercase() <= w[1].to_lowercase()
            }
        }));
    }

    #[cfg(feature = "wl_pgp_encode")]
    #[test]
    /// PGP Word List (encode) – PGPfone Two Syllable Word List is sorted
    fn test_wl_pgp_encode_two_syllable_is_sorted() {
        assert!(WL_PGP_ENCODE_TWO_SYLLABLE
            .windows(2)
            .all(|w| w[0].to_lowercase() <= w[1].to_lowercase()));
    }
}

#[cfg(feature = "wl_eff_encode")]
#[cfg(test)]
#[test]
fn test_wl_eff_encode_contains_256_words() {
    assert_eq!(WL_EFF_ENCODE.len(), 256);
}

#[cfg(feature = "wl_pgp_encode")]
#[cfg(test)]
mod test_cases_wl_pgp_encode_contains_256_words {
    use super::*;
    use test_case::test_case;

    #[test_case(WL_PGP_ENCODE_THREE_SYLLABLE ; "three-syllable encode word list contains 256 words")]
    #[test_case(WL_PGP_ENCODE_TWO_SYLLABLE ; "two-syllable encode word list contains 256 words")]
    fn test(wl: &[&str]) {
        assert_eq!(wl.len(), 256);
    }
}
