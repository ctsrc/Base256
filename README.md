# `lastresort`(1) – Encode and decode data in base 256 easily typed words

[![Crates.io](https://img.shields.io/crates/v/base256?style=flat-square)](https://crates.io/crates/base256)
[![Crates.io](https://img.shields.io/crates/d/base256?style=flat-square)](https://crates.io/crates/base256)
[![Crates.io](https://img.shields.io/docsrs/base256?style=flat-square)](https://docs.rs/crate/base256/latest)
[![License](https://img.shields.io/badge/license-ISC-blue?style=flat-square)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/ctsrc/Base256?style=social)](https://github.com/ctsrc/Base256#start-of-content)

You might expect data encoded in base 256 to be more space efficient
than data encoded in [base 16](https://en.wikipedia.org/wiki/Hexadecimal)
or [base 64](https://en.wikipedia.org/wiki/Base64), but with this particular
set of symbols, that is not the case! As a natural consequence of this,
you have to type more, not less, when you manually input data
using this base 256 encoding instead of base 16 or some other
compact representation.

So, why this base 256 encoding?

Manually typing out base 64 is painful. Same goes for base 16.
Both of these (base 64 and base 16), and particularly base 16,
are prone to make you lose track of where you are
while reading and typing or speaking.

`lastresort` aims to make manual input of binary data onto a computer
by either keyboard or voice input less error-prone compared to
typing in the base 16 or base 64 encoding of said data.

Sample use-cases include:

- typing in your SSH private key on a computer that is running a
  live USB stick copy of its OS without persistent storage,
  and which doesn't have a usable webcam leaving you
  unable to enter the data using a QR-code.
- relaying the contents of binary files to another person
  over a voice channel.

Here is a quick example:

Imagine a file with three bytes in it. A hexadecimal dump of this file
might look like:

```text
00000000: 0505 05                                  ...
```

In other words, this file contains three bytes `0x05 0x05 0x05`.

In base64 this is:

```text
BQUF
```

That's not so bad, and it's quick and easy to type or to read out loud
in either base 16 or base 64 encoding.

But when the amount of data increases, it becomes more and more tricky
to manually type in the bytes by hand or to read them out loud in base 16
or in base 64.

In `lastresort` base 256 using the default codec PGP Word List, the bytes
from the example above are represented as:

```text
adult amulet adult
```

By now, you should see both:

- Why the name of this command is `lastresort`, and
- How in the case of greater amount of bytes, `lastresort` can be
  of great help :)

## Codecs

`lastresort` supports the following codecs:

- PGP Word List, the default codec
- EFF Short Wordlist 2.0, the legacy codec

### PGP Word List

> The PGP Word List [...] is a list of words for conveying data bytes
> in a clear unambiguous way via a voice channel. They are analogous
> in purpose to the NATO phonetic alphabet used by pilots,
> except a longer list of words is used, each word corresponding
> to one of the 256 distinct numeric byte values.
>
> [...]
>
> The list is actually composed of two lists [...]. Two lists are used
> because reading aloud long random sequences of human words
> usually risks three kinds of errors:
> 1) transposition of two consecutive words,
> 2) duplicate words, or
> 3) omitted words.
>
> To detect all three kinds of errors, the two lists are used alternately
> for the even-offset bytes and the odd-offset bytes in the byte sequence.

https://en.wikipedia.org/wiki/PGP_word_list

### EFF Short Wordlist 2.0

The EFF Short Wordlist 2.0 is a list of memorable and distinct words
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

In `lastresort`, 256 of the words from this list are used
when using the legacy codec.

## Example input and outputs using the different codecs

In the `sample_data/original` directory of this repository,
we have a file named `id_ed25519` containing the following
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

For the sake of brevity, the base 256 encoded output that results
when using the legacy encoder is omitted from this README.

The base 256 encoded output that results when using the default
encoder is shown in the subsection below.

### Example output using the PGP Word List encoder

If we run `lastresort` with the above private key `id_ed25519`
as input, and we use the PGP Word List codec:

```zsh
cargo run -- -e pgp -i sample_data/original/id_ed25519 | fold -w 78 -s
```

We get the following base 256 encoded output:

```text
button commando button commando button detergent crusade disable dogsled 
enchanting bison enrollment drumbeat dinosaur drifter escapade dwelling 
disbelief bison enterprise Dupont disruptive egghead detector eating dinosaur 
bison divisive crusade fascinate button commando button commando button 
armistice flagpole congregate crowfoot hemisphere flagpole hideaway drifter 
insincere fallout determine chatter impartial enlist exodus Geiger inception 
flytrap hazardous crusade detector cranky detector cranky detector crowfoot 
disable chopper inertia flagpole hesitate edict detector cranky detector 
cranky dinosaur flagpole hesitate classroom indigo enlist equation cranky 
detector cranky detector cranky detector cranky detector cranky detergent 
cranky detector cranky detector dreadful infancy cranky detector cranky 
detector highchair insincere flatfoot conformist framework inception enlist 
existence allow equation inverse enchanting eating everyday indulge 
enrollment drunken detector cranky detector crucial determine flytrap 
dinosaur chopper filament eating conformist cubic informant cranky hydraulic 
dropper component chopper graduate fragile embezzle dosage disbelief enlist 
conformist drainage impetus dragnet graduate dragnet crossover chopper 
inferno chatter distortion goggles component Geiger distortion crowfoot 
councilman fracture impetus choking enrollment checkup disbelief drunken 
detector cranky detector dosage handiwork inverse coherence egghead corporate 
Christmas impetus hotdog hemisphere fracture armistice hockey hamburger 
cranky detector cranky detector highchair insincere flatfoot conformist 
framework inception enlist existence drunken informant drifter Eskimo edict 
inferno dropper equation cranky detector cranky determine crucial graduate 
crusade consulting enlist Eskimo checkup direction inverse detector gremlin 
enrollment cement consulting flytrap guitarist dreadful distortion deckhand 
filament checkup document hamlet divisive flytrap divisive classroom 
consulting indulge confidence dosage holiness cement headwaters dosage 
detergent classic gravity hamlet consensus dropper conformist deckhand 
equation allow detector cranky detector crusade detector cranky impartial 
chairlift enrollment hockey coherence goggles graduate classroom distortion 
glucose gossamer briefcase hydraulic flagpole concurrent egghead Eskimo 
gremlin consensus involve inception chisel hamburger cubic corporate 
chairlift coherence chairlift disruptive eating equipment Glasgow document 
chisel disbelief inverse glossary gremlin disable drainage disable chopper 
concurrent eating hemisphere Glasgow enterprise endow exodus dogsled 
determine goggles corrosion cement hemisphere chatter councilman indoors 
headwaters flytrap hideaway endow indigo indoors armistice goldfish 
concurrent guidance congregate glucose disbelief edict hesitate gazelle 
coherence drunken headwaters deckhand inferno Christmas insincere framework 
corrosion endow graduate cranky detector cranky detector crusade disable 
egghead informant fallout existence highchair indigo drunken disable indulge 
hurricane endow hesitate egghead informant endow exodus Dupont hurricane 
flagpole conformist choking detergent cranky hamburger dreadful dinosaur 
crowfoot equation commence decadence allow commando button commando button 
commando crusade enchanting crumpled cannonball dropper enterprise crusade 
enchanting dwelling escapade deckhand cannonball drumbeat equipment dogsled 
examine cranky Eskimo crusade cannonball dragnet dinosaur endow commando 
button commando button commando allow
```

## Compression

The preferable way to deal with compression, when compression is desired,
is to separately compress the data first using for example gzip, xz, bzip2,
brotli or lzma, depending on your requirements and which compression tools
you have available on your devices.

Then use lastresort to encode the compressed data. On the other device
you then decode with lastresort and then decompress that with the decompression
tool corresponding to the compression tool you used.

Practical example using the file `sample_data/original/id_ed25519` as the
data we want to compress, encode, type in on another device and
decode and decompress:

- Compress data from some file with xz and then encode it.

  ```zsh
  xz < sample_data/original/id_ed25519 | lastresort | fold -w 78 -s | awk '{$1=$1};1'
  ```

- On the other device, input the encoded words into a text file and then decode and decompress that

  ```zsh
  mkdir -p ~/tmp/
  vim ~/tmp/id_ed25519_compressed_words.txt
  lastresort -d < ~/tmp/id_ed25519_compressed_words.txt | unxz > ~/tmp/id_ed25519
  ```

## Usage

```
lastresort [-d | --decode] [-i <INPUT_FILE>] [-o <OUTPUT_FILE>]
lastresort -h | --help
lastresort -V | --version
```

With no options, `lastresort` reads raw data from stdin
and writes encoded data as a continuous block
of space-separated human words to stdout.

### Options

`-d`, `--decode` `[<DECODER>]` Decode data (default action is to encode data).
Default: `pgp`. Possible values: `pgp`, `eff`.

`-e`, `--encoder` `<ENCODER>` Encoder to use.
Possible values: `pgp`, `eff`.
If encoder is not specified, the `pgp` encoder will be used.
Conflicts with option `-d`.

`-i`, `--input` `<INPUT_FILE>` Read input from `INPUT_FILE`.
Default is stdin; passing `-` also represents stdin.

`-o`, `--output` `<OUTPUT_FILE>` Write output to `OUTPUT_FILE`.
Default is stdout; passing `-` also represents stdout.

`-h`, `--help` Print usage summary and exit.

`-V`, `--version` Print version information and exit.

## See also

* `pgen`(1) on [crates.io](https://crates.io/crates/pgen) / [GitHub](https://github.com/ctsrc/Pgen)

## Installation

1. [Install Rust](https://www.rust-lang.org/en-US/install.html).
2. Run `cargo install base256`
