# `base256`(1) – Encode and decode data in base 256

Command-line utility for encoding and decoding arbitrary binary data
to and from easily typed words from the EFF autocomplete-friendly wordlist.

I actually wanted to use base 1296, because there are a total of 1296
words in the wordlist, but I soon realized that base 256 was far more
convenient, since base 256 corresponds to a single whole byte.

You might expect data encoded in base 256 to be more space efficient
than data encoded in base 16, but with this particulare set of symbols,
that is not the case! Likewise, you have to type more, not less, than
you would if you use my base 256 instead of base 16. So why?

The purpose of `base256` is to make manual input of binary data
onto a computer less error-prone compared to typing in the base 16 or
[base 64](https://en.wikipedia.org/wiki/Base64) encoding of said data.
Whereas manually typing out base 64 is painful, and base 16 makes it
easy to lose track of where you are while typing, `base256` attempts
to remedy both of these problems by using a 256 different words from
the EFF autocomplete-friendly wordlist.

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
