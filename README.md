# `lastresort`(1) – Encode and decode data in base 256

[![Crates.io](https://img.shields.io/crates/v/base256?style=flat-square)](https://crates.io/crates/base256)
[![Crates.io](https://img.shields.io/crates/d/base256?style=flat-square)](https://crates.io/crates/base256)
[![License](https://img.shields.io/badge/license-ISC-blue?style=flat-square)](LICENSE)

Command-line utility for encoding and decoding arbitrary binary data
to and from easily typed words.

You might expect data encoded in base 256 to be more space efficient
than data encoded in base 16, but with this particular set of symbols,
that is not the case! Likewise, you have to type more, not less, than
you would if you use my base 256 instead of base 16. So why?

The purpose of `lastresort` is to make manual input of binary data
onto a computer less error-prone compared to typing in the base 16 or
[base 64](https://en.wikipedia.org/wiki/Base64) encoding of said data.
Whereas manually typing out base 64 is painful, and base 16 makes it
easy to lose track of where you are while typing, `lastresort` attempts
to remedy both of these problems by using human words.

Sample use-cases include typing in your SSH private key on a computer
that is running a live USB stick copy of its OS without persistent
storage, and which doesn't have a useable webcamera leaving you
unable to enter the data using a QR-code.

## Codecs

`lastresort` supports the following codecs:

- EFF Short Wordlist 2.0, the legacy codec

### EFF Short Wordlist 2.0

The EFF Short Wordlist 2.0 is a list of 1,296 memorable and distinct words
with a few additional features making the words easy to type:

> * Each word has a unique three-character prefix. This means that future
>   software could auto-complete words in the passphrase after the user
>   has typed the first three characters
> * All words are at least an edit distance of 3 apart. This means that
>   future software could correct any single typo [...] (and in many cases
>   more than one typo).

https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases

This wordlist is also used in the `pgen` passphrase generator.
See https://github.com/ctsrc/pgen for more about `pgen`.

In `lastresort`, 256 of the words from this list are used when using this codec.

## Example input and outputs using the different codecs

In the `sample_data` directory of this repository, we have
a file named `id_ed25519` containing the following
OpenSSH ed25519 private key:

```text
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAAMwAAAAtzc2gtZW
QyNTUxOQAAACCdE5ZT2FyAqO/5dfMJHZ2LsKdK95x1Jo/kJB8es4O2HQAAAJiy+V66svle
ugAAAAtzc2gtZWQyNTUxOQAAACCdE5ZT2FyAqO/5dfMJHZ2LsKdK95x1Jo/kJB8es4O2HQ
AAAEAAr0Ou+od9Jnc+qb0VTq4zt3gF60+0ITRlL3HybqGLG50TllPYXICo7/l18wkdnYuw
p0r3nHUmj+QkHx6zg7YdAAAAEGVyaWtuQGxpYmVyYXRpb24BAgMEBQ==
-----END OPENSSH PRIVATE KEY-----
```

The corresponding base 256 encoded outputs using each of the different
codecs available in `lastresort` are shown is the subsections below.

### Example output using the EFF Short Wordlist 2.0 codec

If we run `lastresort` with the above private key `id_ed25519`
as input, and we use the EFF Short Wordlist 2.0 codec:

```zsh
cargo run -- < sample_data/inputs/id_ed25519
```

We get the following base 256 encoded output:

```text
carrot carrot carrot carrot carrot dealer depot dictionary directory
dryer badass dugout dustpan depot dryer ebook ebook dimple badass
dustpan eagerness directory eggnog daughter ecosystem depot badass
dominoes depot elsewhere carrot carrot carrot carrot carrot alchemy
eskimo clay dealer federal eskimo fettuccine dryer geyser ergonomic
deepness chopsticks foamless embroidery eliminator faucet fox
etiquette falcon depot daughter daughter daughter daughter daughter
dealer dictionary codeword gainfully eskimo femur educator daughter
daughter daughter daughter depot eskimo femur crawfish frosting
embroidery dwelling daughter daughter daughter daughter daughter
daughter daughter daughter daughter dealer daughter daughter daughter
daughter dresser garlic daughter daughter daughter daughter fox geyser
estrogen cinnamon exfoliate fox embroidery elbow alchemy dwelling
geographer dryer ecosystem educator gearbox dugout dwelling daughter
daughter daughter deepness deepness etiquette depot codeword embroidery
ecosystem cinnamon device geographer daughter fleshiness dugout cement
codeword etiquette evolution dresser dizziness dimple embroidery
cinnamon double fondue dominoes etiquette dominoes crawfish codeword
gearbox chopsticks dizziness fictitious cement faucet dizziness dealer
couch euthanize fondue clubhouse dugout cinnamon dimple dwelling
daughter daughter daughter dizziness fabric geographer cage eggnog
cohabitate cohabitate fondue gainfully federal euthanize alchemy
frosting exfoliate daughter daughter daughter daughter fox geyser
estrogen cinnamon exfoliate fox embroidery elbow dwelling geographer
dryer ecosystem educator gearbox dugout dwelling daughter daughter
daughter deepness deepness etiquette depot codeword embroidery
ecosystem cinnamon device geographer daughter fleshiness dugout cement
codeword etiquette evolution dresser dizziness dimple embroidery
cinnamon double fondue dominoes etiquette dominoes crawfish codeword
gearbox chopsticks dizziness fictitious cement faucet dizziness dealer
couch euthanize fondue clubhouse dugout cinnamon dimple dwelling
alchemy daughter daughter daughter depot daughter daughter foamless
chalkboard dugout frosting cage fictitious etiquette crawfish dizziness
fettuccine estrogen cage fleshiness eskimo chalkboard eggnog ecosystem
fleshiness clubhouse geyser fox clay exfoliate device cohabitate
chalkboard cage chalkboard directory ecosystem eagerness federal double
clay dimple geographer eskimo fleshiness dictionary double dictionary
codeword chalkboard ecosystem federal federal dustpan elsewhere
eliminator directory deepness fictitious copier cement federal
chopsticks couch garlic faucet etiquette fettuccine elsewhere frosting
garlic alchemy finalist chalkboard foamless clay fettuccine dimple
educator femur falcon cage dwelling faucet dimple gearbox cohabitate
geyser exfoliate copier elsewhere etiquette daughter daughter daughter
daughter depot dictionary eggnog geographer ergonomic elbow fox
frosting dwelling dictionary gearbox finalist elsewhere femur eggnog
geographer elsewhere eliminator eagerness finalist eskimo cinnamon
clubhouse dealer daughter exfoliate dresser depot dealer dwelling
cybernetic cybernetic alchemy carrot carrot carrot carrot carrot depot
dryer deliverer badass dugout dustpan depot dryer ebook ebook dimple
badass dustpan eagerness directory eggnog daughter ecosystem depot
badass dominoes depot elsewhere carrot carrot carrot carrot carrot
alchemy 
```

## Usage

```
lastresort [-d | --decode]
lastresort -h | --help
lastresort -V | --version
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
