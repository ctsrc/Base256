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

#[derive(Clone, Debug)]
pub struct EffCodecEncode<I: Iterator> {
    iter: I,
}

impl<I, E> Iterator for EffCodecEncode<I>
where
    I: Iterator<Item = Result<u8, E>>,
{
    type Item = Result<&'static str, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next()? {
            Ok(byte) => Some(Ok(crate::WL_AUTOCOMPLETE[byte as usize])),
            Err(e) => Some(Err(e)),
        }
    }
}

impl<I: Iterator<Item = Result<u8, E>>, E> crate::Encode<I, EffCodecEncode<I>> for I {
    fn encode(self) -> EffCodecEncode<I> {
        EffCodecEncode { iter: self }
    }
}
