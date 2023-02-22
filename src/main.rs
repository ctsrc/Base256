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

#![forbid(unsafe_code)]

#[cfg(not(any(feature = "encode", feature = "decode")))]
compile_error!("Building bin target requires that at least one encoder or decoder is enabled");

use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Write};

use anyhow::Result;
use clap::{Parser, ValueEnum};
#[cfg(any(feature = "decode_pgp", feature = "decode_eff"))]
use utf8_chars::BufReadCharsExt;

#[cfg(feature = "encode_eff")]
use base256::EffEncode;
#[cfg(any(feature = "encode_pgp", feature = "encode_eff"))]
use base256::Encode;
#[cfg(feature = "encode_pgp")]
use base256::PgpEncode;

#[cfg(any(feature = "decode_pgp", feature = "decode_eff"))]
use base256::Decode;
#[cfg(feature = "decode_eff")]
use base256::EffDecode;
#[cfg(feature = "decode_pgp")]
use base256::PgpDecode;

#[derive(Parser)]
#[command(author, version, about, long_about = None, name = "lastresort")]
struct Cli {
    /*
     * The decode feature can be enabled or disabled at compile-time.
     * If the decode feature is enabled at compile-time:
     * - There will be a "-d" flag available at run-time.
     * - The requirements for the "-d" flag itself, and its argument, will depend on:
     *   * Whether or not the encode feature is enabled at compile-time.
     *   * Which decoders were enabled at compile-time.
     */
    #[cfg(feature = "decode")]
    #[command(flatten)]
    decoding: CliDecoding,

    /*
     * The encode feature can be enabled or disabled at compile-time.
     * If the encode feature is enabled at compile-time:
     * - There will be an "-e" flag available at run-time.
     * - The requirements for the "-e" flag itself, and its argument, will depend on:
     *   * Whether or not the decode feature is enabled at compile-time.
     *   * Which encoders were enabled at compile-time.
     */
    #[cfg(feature = "encode")]
    #[command(flatten)]
    encoding: CliEncoding,

    /*
     * The input and output arguments are always available.
     */
    /// Read input from INPUT_FILE. Default is stdin; passing - also represents stdin
    #[arg(short, long, value_name = "INPUT_FILE")]
    input: Option<String>,
    /// Write output to OUTPUT_FILE. Default is stdout; passing - also represents stdout
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<String>,
}

/*
 * When both the decode and the encode features are enabled at compile-time:
 * - The "-d" flag is optional.
 * - The "-d" flag conflicts with the "-e" flag.
 * - The requirements for the argument to the "-d" flag
 *   depends on the decoders enabled at compile-time.
 */
#[cfg(all(feature = "decode", feature = "encode"))]
#[derive(clap::Args)]
struct CliDecoding {
    /*
     * When the decode_pgp feature is enabled at compile-time:
     * - The "-d" flag itself remains optional, and
     * - The "-d" flag can optionally take an argument to specify which decoder to use.
     */
    /// Decode data (default action is to encode data)
    #[cfg(feature = "decode_pgp")]
    #[arg(short, long, value_name = "DECODER", conflicts_with("encoder"))]
    decode: Option<Option<Decoder>>,

    /*
     * When the decode_pgp feature is DISABLED at compile-time:
     * - The "-d" flag itself remains optional, but
     * - The "-d" flag when used takes a REQUIRED argument to specify which decoder to use.
     */
    /// Decode data (default action is to encode data)
    #[cfg(all(feature = "decode_eff", not(feature = "decode_pgp")))]
    #[arg(short, long, value_name = "DECODER", conflicts_with("encoder"))]
    #[arg(required = false)]
    decode: Option<Decoder>,
}

/*
 * When the decode feature is enabled at compile-time,
 * but the encode feature is DISABLED at compile-time:
 * - The "-d" flag is REQUIRED.
 * - The requirements for the argument to the "-d" flag
 *   depends on the decoders enabled.
 */
#[cfg(all(feature = "decode", not(feature = "encode")))]
#[derive(clap::Args)]
struct CliDecoding {
    /*
     * When the decode_pgp feature is enabled at compile-time:
     * - The "-d" flag itself remains REQUIRED, but
     * - The "-d" flag can optionally take an argument to specify which decoder to use.
     */
    /// Decode data
    #[cfg(feature = "decode_pgp")]
    #[arg(short, long, value_name = "DECODER", required = true)]
    decode: Option<Option<Decoder>>,

    /*
     * When the decode_pgp feature is DISABLED at compile-time:
     * - The "-d" flag itself remains REQUIRED, and
     * - The "-d" flag takes a REQUIRED argument to specify which decoder to use.
     */
    /// Decode data
    #[cfg(all(feature = "decode_eff", not(feature = "decode_pgp")))]
    #[arg(short, long, value_name = "DECODER")]
    decode: Decoder,
}

/*
 * When both the encode and decode features are enabled at compile-time:
 * - The "-e" flag conflicts with the "-d" flag.
 * - The requirements for the "-e" flag, and its argument,
 *   depend on the encoders enabled at compile-time.
 */
#[cfg(all(feature = "encode", feature = "decode"))]
#[derive(clap::Args)]
struct CliEncoding {
    /*
     * When the encode_pgp feature is enabled at compile-time:
     * - The "-e" flag itself is optional, and
     * - The "-e" flag takes a REQUIRED argument to specify which decoder to use.
     */
    /// Encoder to use
    #[cfg(feature = "encode_pgp")]
    #[arg(short, long, conflicts_with("decode"))]
    encoder: Option<Encoder>,

    /*
     * When the encode_pgp feature is DISABLED at compile-time:
     * - The "-e" flag itself is REQUIRED unless the "-d" flag is used, and
     * - The "-e" flag takes a REQUIRED argument to specify which decoder to use.
     */
    /// Encoder to use
    #[cfg(all(feature = "encode_eff", not(feature = "encode_pgp")))]
    #[arg(
        short,
        long,
        conflicts_with("decode"),
        required_unless_present("decode")
    )]
    encoder: Option<Encoder>,
}

/*
 * When the encode feature is enabled at compile-time,
 * but the decode feature is DISABLED at compile-time:
 * - The requirements for the "-e" flag, and its argument,
 *   depend on the encoders enabled at compile-time.
 */
#[cfg(all(feature = "encode", not(feature = "decode")))]
#[derive(clap::Args)]
struct CliEncoding {
    /*
     * When the encode_pgp feature is enabled at compile-time:
     * - The "-e" flag itself is optional, and
     * - The "-e" flag takes a REQUIRED argument to specify which decoder to use.
     */
    /// Encoder to use
    #[cfg(feature = "encode_pgp")]
    #[arg(short, long)]
    encoder: Option<Encoder>,

    /*
     * When the encode_pgp feature is DISABLED at compile-time:
     * - The "-e" flag itself is REQUIRED, and
     * - The "-e" flag takes a REQUIRED argument to specify which decoder to use.
     */
    /// Encoder to use
    #[cfg(all(feature = "encode_eff", not(feature = "encode_pgp")))]
    #[arg(short, long)]
    encoder: Encoder,
}

#[cfg(feature = "encode")]
#[derive(ValueEnum, Clone)]
enum Encoder {
    /// PGP Word List. The default encoder
    #[cfg(feature = "encode_pgp")]
    Pgp,
    /// EFF Short Wordlist 2.0. The legacy encoder
    #[cfg(feature = "encode_eff")]
    Eff,
}

#[cfg(feature = "decode")]
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

    let mut input: Box<dyn BufRead> = match cli.input {
        None => Box::new(stdin().lock()),
        Some(path) => {
            if path == "-" {
                Box::new(stdin().lock())
            } else {
                Box::new(BufReader::new(File::open(path)?))
            }
        }
    };

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

        let decoder = cli.decoding.decode;

        #[cfg(all(
            feature = "decode_eff",
            not(feature = "decode_pgp"),
            not(feature = "encode")
        ))]
        let decoder = Some(Some(decoder)); // TODO: Something less messy for this feature set

        if let Some(decoder) = decoder {
            let input_chars = input.chars();

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
                    for byte in Decode::<_, PgpDecode<_>>::decode(input_chars) {
                        output.write_all(&[byte?])?;
                    }
                }
                #[cfg(feature = "decode_eff")]
                Decoder::Eff => {
                    for byte in Decode::<_, EffDecode<_>>::decode(input_chars) {
                        output.write_all(&[byte?])?;
                    }
                }
            }
            return Ok(());
        }
    }

    #[cfg(feature = "encode")]
    {
        let input_bytes = input.bytes();

        #[cfg(not(any(feature = "encode_pgp", feature = "encode_eff")))]
        compile_error!("Building bin target with encoding feature enabled requires that at least one encoder is enabled");

        let encoder = cli.encoding.encoder;

        // If support for the PGP encoder was compiled, then it is the default encoder..
        #[cfg(feature = "encode_pgp")]
        let encoder = encoder.unwrap_or(Encoder::Pgp);
        // ..otherwise, the encoder has to be provided as a cli arg.
        #[cfg(all(feature = "decode", not(feature = "encode_pgp")))]
        let encoder = match encoder {
            Some(encoder) => encoder,
            None => {
                unreachable!("This match arm should never be reached due to clap parse rules.");
            }
        };
        #[cfg(not(any(feature = "decode", feature = "encode_pgp")))]
        let encoder = encoder;

        let mut did_write_any_words = false;

        match encoder {
            #[cfg(feature = "encode_pgp")]
            Encoder::Pgp => {
                let mut encoded = Encode::<_, PgpEncode<_>>::encode(input_bytes);
                if let Some(word) = encoded.next() {
                    did_write_any_words = true;
                    write!(output, "{}", word?)?;
                }
                for word in encoded {
                    write!(output, " {}", word?)?
                }
            }
            #[cfg(feature = "encode_eff")]
            Encoder::Eff => {
                let mut encoded = Encode::<_, EffEncode<_>>::encode(input_bytes);
                if let Some(word) = encoded.next() {
                    did_write_any_words = true;
                    write!(output, "{}", word?)?;
                }
                for word in encoded {
                    write!(output, " {}", word?)?
                }
            }
        }

        if did_write_any_words {
            write!(output, "\n")?;
        }
    }

    Ok(())
}
