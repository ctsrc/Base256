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

#[cfg(not(any(feature = "encode", feature = "decode")))]
compile_error!("Building bin target requires that at least one encoder or decoder is enabled");

use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

use anyhow::Result;
use clap::{Parser, ValueEnum};

#[cfg(feature = "encode_eff")]
use base256::EffCodecEncode;
#[cfg(any(feature = "encode_pgp", feature = "encode_eff"))]
use base256::Encode;
#[cfg(feature = "encode_pgp")]
use base256::PgpCodecEncode;

#[derive(Parser)]
#[command(author, version, about, long_about = None, name = "lastresort")]
struct Cli {
    /// Decode data (default action is to encode data)
    #[cfg(all(feature = "decode_pgp", feature = "encode"))]
    #[arg(short, long, value_name = "DECODER", conflicts_with("encoder"))]
    decode: Option<Option<Decoder>>,
    /// Decode data (default action is to encode data)
    #[cfg(all(
        not(feature = "decode_pgp"),
        feature = "decode_eff",
        feature = "encode"
    ))]
    #[arg(short, long, value_name = "DECODER", required = false)]
    decode: Option<Decoder>,
    /// Decode data
    #[cfg(all(feature = "decode_pgp", not(feature = "encode")))]
    #[arg(short, long, value_name = "DECODER", required = true)]
    decode: Option<Option<Decoder>>,
    /// Decode data
    #[cfg(all(
        feature = "decode_eff",
        not(feature = "decode_pgp"),
        not(feature = "encode")
    ))]
    #[arg(short, long, value_name = "DECODER")]
    decode: Decoder,
    /// Encoder to use
    #[cfg(all(feature = "encode_pgp", feature = "decode"))]
    #[arg(short, long, conflicts_with("decode"))]
    encoder: Option<Encoder>,
    /// Encoder to use
    #[cfg(all(
        feature = "encode_eff",
        not(feature = "encode_pgp"),
        feature = "decode"
    ))]
    #[arg(
        short,
        long,
        conflicts_with("decode"),
        required_unless_present("decode")
    )]
    encoder: Option<Encoder>,
    /// Encoder to use
    #[cfg(all(feature = "encode_pgp", not(feature = "decode")))]
    #[arg(short, long)]
    encoder: Option<Encoder>,
    /// Encoder to use
    #[cfg(all(
        feature = "encode_eff",
        not(feature = "encode_pgp"),
        not(feature = "decode")
    ))]
    #[arg(short, long)]
    encoder: Encoder,
    /// Read input from INPUT_FILE. Default is stdin; passing - also represents stdin
    #[arg(short, long, value_name = "INPUT_FILE")]
    input: Option<String>,
    /// Write output to OUTPUT_FILE. Default is stdout; passing - also represents stdout
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<String>,
}

#[derive(ValueEnum, Clone)]
enum Encoder {
    /// PGP Word List. The default encoder
    #[cfg(feature = "encode_pgp")]
    Pgp,
    /// EFF Short Wordlist 2.0. The legacy encoder
    #[cfg(feature = "encode_eff")]
    Eff,
}

#[derive(ValueEnum, Clone)]
enum Decoder {
    /// PGP Word List. The default decoder
    #[cfg(feature = "decode_pgp")]
    Pgp,
    /// EFF Short Wordlist 2.0. The legacy decoder
    #[cfg(feature = "decode_eff")]
    Eff,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input: Box<dyn Read> = match cli.input {
        None => Box::new(stdin()),
        Some(path) => {
            if path == "-" {
                Box::new(stdin())
            } else {
                Box::new(BufReader::new(File::open(path)?))
            }
        }
    };
    let input_bytes = input.bytes();

    let mut output: Box<dyn Write> = match cli.output {
        None => Box::new(stdout()),
        Some(path) => {
            if path == "-" {
                Box::new(stdout())
            } else {
                let file = OpenOptions::new().create(true).write(true).open(path)?;
                Box::new(BufWriter::new(file))
            }
        }
    };

    #[cfg(feature = "decode")]
    {
        #[cfg(not(any(feature = "decode_pgp", feature = "decode_eff")))]
        compile_error!("Building bin target with decoding feature enabled requires that at least one decoder is enabled");

        let decoder = cli.decode;

        #[cfg(all(
            feature = "decode_eff",
            not(feature = "decode_pgp"),
            not(feature = "encode")
        ))]
        let decoder = Some(Some(decoder)); // TODO: Something less messy for this feature set

        if let Some(decoder) = decoder {
            // If support for the PGP decoder was compiled, then it is the default decoder..
            #[cfg(feature = "decode_pgp")]
            let decoder = decoder.unwrap_or(Decoder::Pgp);
            // ..otherwise, the decoder has to be provided as a cli arg.
            #[cfg(not(any(feature = "decode_pgp", feature = "encode")))]
            let decoder = match decoder {
                Some(decoder) => decoder,
                None => {
                    unreachable!("This match arm should never be reached due to clap parse rules.");
                }
            };

            match decoder {
                #[cfg(feature = "decode_pgp")]
                Decoder::Pgp => {
                    todo!();
                }
                #[cfg(feature = "decode_eff")]
                Decoder::Eff => {
                    todo!();
                }
            }
            return Ok(());
        }
    }

    #[cfg(not(any(feature = "encode_pgp", feature = "encode_eff")))]
    compile_error!("Building bin target with encoding feature enabled requires that at least one encoder is enabled");

    #[cfg(any(feature = "encode_pgp", feature = "encode_eff"))]
    {
        // If support for the PGP encoder was compiled, then it is the default encoder..
        #[cfg(feature = "encode_pgp")]
        let encoder = cli.encoder.unwrap_or(Encoder::Pgp);
        // ..otherwise, the encoder has to be provided as a cli arg.
        #[cfg(all(feature = "decode", not(feature = "encode_pgp")))]
        let encoder = match cli.encoder {
            Some(encoder) => encoder,
            None => {
                unreachable!("This match arm should never be reached due to clap parse rules.");
            }
        };
        #[cfg(not(any(feature = "decode", feature = "encode_pgp")))]
        let encoder = cli.encoder;

        match encoder {
            #[cfg(feature = "encode_pgp")]
            Encoder::Pgp => {
                for word in Encode::<_, PgpCodecEncode<_>>::encode(input_bytes) {
                    write!(output, "{} ", word?)?
                }
            }
            #[cfg(feature = "encode_eff")]
            Encoder::Eff => {
                for word in Encode::<_, EffCodecEncode<_>>::encode(input_bytes) {
                    write!(output, "{} ", word?)?
                }
            }
        }
    }

    Ok(())
}
