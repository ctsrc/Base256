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

use std::fs::{File, OpenOptions};
use std::io;
use std::io::{stdin, stdout, BufReader, BufWriter, Read};

use anyhow::Result;
use clap::{Parser, ValueEnum};

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation
include!(concat!(env!("OUT_DIR"), "/256.rs"));

#[derive(Parser)]
#[command(author, version, about, long_about = None, name = "lastresort")]
struct Cli {
    /// Codec to use
    #[arg(short, long, value_enum, default_value_t=Codec::Pgp)]
    codec: Codec,
    /// Decode data (default action is to encode data).
    #[arg(short, long)]
    decode: bool,
    /// Read input from INPUT_FILE. Default is stdin; passing - also represents stdin
    #[arg(short, long, value_name = "INPUT_FILE")]
    input: Option<String>,
    /// Write output to OUTPUT_FILE. Default is stdout; passing - also represents stdout
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<String>,
}

#[derive(ValueEnum, Clone)]
enum Codec {
    /// PGP Word List. The default codec
    Pgp,
    /// EFF Short Wordlist 2.0. The legacy codec
    Eff,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input: Box<dyn io::Read> = match cli.input {
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

    let mut output: Box<dyn io::Write> = match cli.output {
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

    if cli.decode {
        unimplemented!();
    } else {
        match cli.codec {
            Codec::Pgp => {
                let mut odd_even = 0;
                for byte in input_bytes {
                    if odd_even == 0 {
                        write!(output, "{} ", WL_PGPFONE_TWO_SYLLABLE[byte? as usize])?
                    } else {
                        write!(output, "{} ", WL_PGPFONE_THREE_SYLLABLE[byte? as usize])?
                    }
                    odd_even = (odd_even + 1) % 2;
                }
            }
            Codec::Eff => {
                for byte in input_bytes {
                    write!(output, "{} ", WL_AUTOCOMPLETE[byte? as usize])?
                }
            }
        }
    }

    Ok(())
}
