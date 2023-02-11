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

use std::fs::File;
use std::io;
use std::io::{stdin, BufReader, Read};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation
include!(concat!(env!("OUT_DIR"), "/256.rs"));

#[derive(Parser)]
#[command(author, version, about, long_about = None, name = "lastresort")]
struct Cli {
    /// Decode data (default action is to encode data).
    #[arg(short, long)]
    decode: bool,
    /// Read input from INPUT_FILE. Default is stdin; passing - also represents stdin
    #[arg(short, long, value_name = "INPUT_FILE")]
    input: Option<String>,
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

    if cli.decode {
        unimplemented!();
    } else {
        for byte in input_bytes {
            print!("{} ", WL_AUTOCOMPLETE[byte? as usize])
        }
    }

    Ok(())
}
