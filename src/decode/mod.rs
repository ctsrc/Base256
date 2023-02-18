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

#[cfg(feature = "decode_eff")]
mod eff;
#[cfg(feature = "decode_pgp")]
mod pgp;

#[cfg(feature = "decode_eff")]
pub use eff::*;
#[cfg(feature = "decode_pgp")]
pub use pgp::*;

// Note: Decode list entry struct is currently not exported, as it is rather specific to the implementation.
// Note: The list entry struct implementation is included at compile-time so that both the build script
//       and the crate itself can share that code.
#[cfg(any(feature = "wl_eff_decode", feature = "wl_pgp_decode"))]
include!("include/wordlist_entry.rs");

// Note: Decode lists are currently not exported, as they are rather specific to the implementation.
// Note: The decode lists are generated at compile-time by the build script.
#[cfg(any(feature = "wl_eff_decode", feature = "wl_pgp_decode"))]
include!(concat!(env!("OUT_DIR"), "/wl_decode.rs"));

/// Base 256 decoder trait
#[cfg(feature = "decode")]
pub trait Decode<I: Iterator, D> {
    fn decode(self) -> D;
}
