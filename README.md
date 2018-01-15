# `base256`(1) – Encode and decode data in base 256

Command-line utility for encoding and decoding arbitrary binary data
to and from easily typed words from the EFF autocomplete-friendly wordlist.

I actually wanted to use base 1296, because there are a total of 1296
words in the wordlist, but I soon realized that base 256 was far more
convenient, since base 256 corresponds to a single whole byte.

## Usage

```
base256 [-d | --decode]
base256 -h | --help
base256 -V | --version
```

Reads from `stdin` and writes to `stdout`.

### Options

`-d`, `--decode` Decode data (default action is to encode data).

`-h`, `--help` Show help and exit.

`-V`, `--version` Print version information and exit.

## See also

* [`pgen`(1)](https://crates.io/crates/pgen)

## Installation

1. [Install Rust](https://www.rust-lang.org/en-US/install.html).
2. Run `cargo install base256`
