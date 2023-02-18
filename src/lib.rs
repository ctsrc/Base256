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

#[cfg(not(any(
    feature = "encode",
    feature = "decode",
    feature = "wl_eff",
    feature = "wl_pgp"
)))]
compile_error!("Building lib target requires that at least one of the following features is enabled: encode; decode; wl_eff; wl_pgp");

#[cfg(feature = "encode")]
mod encode;

#[cfg(feature = "encode")]
pub use encode::*;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation
#[cfg(any(feature = "wl_eff", feature = "wl_pgp"))]
include!(concat!(env!("OUT_DIR"), "/256.rs"));

#[cfg(any(feature = "wl_eff", feature = "wl_pgp"))]
#[cfg(test)]
mod tests_word_lists_sorted_extent {
    use super::*;

    #[cfg(feature = "wl_eff")]
    #[test]
    /// The autocomplete wordlist based on the EFF Short Wordlist 2.0 is sorted.
    fn test_wl_autocomplete_is_sorted() {
        assert!(WL_AUTOCOMPLETE.windows(2).all(|w| w[0] <= w[1]));
    }

    #[cfg(feature = "wl_pgp")]
    #[test]
    /// The three syllable PGP word list is mostly sorted,
    /// except for the fact that the word "applicant" comes
    /// before the word "Apollo".
    fn test_wl_pgpfone_three_syllable_lowercase_is_mostly_sorted() {
        assert!(WL_PGPFONE_THREE_SYLLABLE.windows(2).all(|w| {
            //dbg!(w);
            if w[0] == "applicant" && w[1] == "Apollo" {
                true
            } else {
                w[0].to_lowercase() <= w[1].to_lowercase()
            }
        }));
    }

    #[cfg(feature = "wl_pgp")]
    #[test]
    /// The two syllable PGP word list is sorted.
    fn test_wl_pgpfone_two_syllable_lowercase_is_sorted() {
        assert!(WL_PGPFONE_TWO_SYLLABLE
            .windows(2)
            .all(|w| w[0].to_lowercase() <= w[1].to_lowercase()));
    }
}

#[cfg(feature = "wl_eff")]
#[cfg(test)]
#[test]
fn wl_eff_contains_256_words() {
    assert_eq!(WL_AUTOCOMPLETE.len(), 256);
}

#[cfg(feature = "wl_pgp")]
#[cfg(test)]
mod wl_eff_contains_256_words {
    use super::*;
    use test_case::test_case;

    #[test_case(WL_PGPFONE_THREE_SYLLABLE ; "three-syllable word list contains 256 words")]
    #[test_case(WL_PGPFONE_TWO_SYLLABLE ; "two-syllable word list contains 256 words")]
    fn wl_contains_256_words(wl: &[&str]) {
        assert_eq!(wl.len(), 256);
    }
}
