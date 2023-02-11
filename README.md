# `lastresort`(1) – Encode and decode data in base 256 easily typed words

[![Crates.io](https://img.shields.io/crates/v/base256?style=flat-square)](https://crates.io/crates/base256)
[![Crates.io](https://img.shields.io/crates/d/base256?style=flat-square)](https://crates.io/crates/base256)
[![License](https://img.shields.io/badge/license-ISC-blue?style=flat-square)](LICENSE)

You might expect data encoded in base 256 to be more space efficient
than data encoded in base 16, but with this particular set of symbols,
that is not the case! Likewise, you have to type more, not less, than
you would if you use my base 256 instead of base 16. So why?

The purpose of `lastresort` is to make manual input of binary data
onto a computer less error-prone compared to typing in the base 16 or
[base 64](https://en.wikipedia.org/wiki/Base64) encoding of said data.

Manually typing out base 64 is painful, and base 16 makes it
easy to lose track of where you are while typing. `lastresort` attempts
to remedy both of these problems by using human words.

Here is a quick example:

Imagine a file with three bytes in it. In hexadecimal this file
might look like:

```text
00000000: 0505 05                                  ...
```

In other words, this file contains three bytes `0x05 0x05 0x05`.

That's not so bad, and it's quick and easy to type even in hex.

But when the amount of data gets bigger, it gets more and more tricky
to manually type in the bytes by hand. When might you want to do so?

Sample use-cases include typing in your SSH private key on a computer
that is running a live USB stick copy of its OS without persistent
storage, and which doesn't have a useable webcamera leaving you
unable to enter the data using a QR-code.

Anyways, to continue with the example from above of the three bytes.

In base64 this is:

```text
BQUF
```

In `lastresort` base 256 using the default codec PGP Word List, it is:

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

> The PGP Word List ("Pretty Good Privacy word list", also called
> a biometric word list [...]) is a list of words for conveying data bytes
> in a clear unambiguous way via a voice channel. They are analogous
> in purpose to the NATO phonetic alphabet used by pilots,
> except a longer list of words is used, each word corresponding
> to one of the 256 distinct numeric byte values.
>
> [...]
>
> The list is actually composed of two lists, each containing
> 256 phonetically distinct words, in which each word represents
> a different byte value between 0 and 255. Two lists are used
> because reading aloud long random sequences of human words
> usually risks three kinds of errors:
> 1) transposition of two consecutive words,
> 2) duplicate words, or
> 3) omitted words.
>
> To detect all three kinds of errors, the two lists are used alternately
> for the even-offset bytes and the odd-offset bytes in the byte sequence.
> Each byte value is actually represented by two different words,
> depending on whether that byte appears at an even or an odd offset
> from the beginning of the byte sequence. The two lists are
> readily distinguished by the number of syllables;
> the even list has words of two syllables, the odd list has three.
> The two lists have a maximum word length of 9 and 11 letters, respectively.

https://en.wikipedia.org/wiki/PGP_word_list

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

### Example output using the PGP Word List codec

If we run `lastresort` with the above private key `id_ed25519`
as input, and we use the PGP Word List codec:

```zsh
cargo run -- -i sample_data/inputs/id_ed25519
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
disable chopper inertia flagpole hesitate edict detector cranky detector cranky
dinosaur flagpole hesitate classroom indigo enlist equation cranky detector
cranky detector cranky detector cranky detector cranky detergent cranky detector
cranky detector dreadful infancy cranky detector cranky detector highchair
insincere flatfoot conformist framework inception enlist existence allow
equation inverse enchanting eating everyday indulge enrollment drunken detector
cranky detector crucial determine flytrap dinosaur chopper filament eating
conformist cubic informant cranky hydraulic dropper component chopper graduate
fragile embezzle dosage disbelief enlist conformist drainage impetus dragnet
graduate dragnet crossover chopper inferno chatter distortion goggles component
Geiger distortion crowfoot councilman fracture impetus choking enrollment
checkup disbelief drunken detector cranky detector dosage handiwork inverse
coherence egghead corporate Christmas impetus hotdog hemisphere fracture
armistice hockey hamburger cranky detector cranky detector highchair insincere
flatfoot conformist framework inception enlist existence drunken informant
drifter Eskimo edict inferno dropper equation cranky detector cranky determine
crucial graduate crusade consulting enlist Eskimo checkup direction inverse
detector gremlin enrollment cement consulting flytrap guitarist dreadful
distortion deckhand filament checkup document hamlet divisive flytrap divisive
classroom consulting indulge confidence dosage holiness cement headwaters
dosage detergent classic gravity hamlet consensus dropper conformist deckhand
equation allow detector cranky detector crusade detector cranky impartial
chairlift enrollment hockey coherence goggles graduate classroom distortion
glucose gossamer briefcase hydraulic flagpole concurrent egghead Eskimo gremlin
consensus involve inception chisel hamburger cubic corporate chairlift
coherence chairlift disruptive eating equipment Glasgow document chisel
disbelief inverse glossary gremlin disable drainage disable chopper concurrent
eating hemisphere Glasgow enterprise endow exodus dogsled determine goggles
corrosion cement hemisphere chatter councilman indoors headwaters flytrap
hideaway endow indigo indoors armistice goldfish concurrent guidance congregate
glucose disbelief edict hesitate gazelle coherence drunken headwaters deckhand
inferno Christmas insincere framework corrosion endow graduate cranky detector
cranky detector crusade disable egghead informant fallout existence highchair
indigo drunken disable indulge hurricane endow hesitate egghead informant endow
exodus Dupont hurricane flagpole conformist choking detergent cranky hamburger
dreadful dinosaur crowfoot equation commence decadence allow commando button
commando button commando crusade enchanting crumpled cannonball dropper
enterprise crusade enchanting dwelling escapade deckhand cannonball drumbeat
equipment dogsled examine cranky Eskimo crusade cannonball dragnet dinosaur
endow commando button commando button commando allow
```

### Example output using the EFF Short Wordlist 2.0 codec

If we run `lastresort` with the above private key `id_ed25519`
as input, and we use the EFF Short Wordlist 2.0 codec:

```zsh
cargo run -- -c eff -i sample_data/inputs/id_ed25519
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
lastresort [-d | --decode] [-i <INPUT_FILE>] [-o <OUTPUT_FILE>]
lastresort -h | --help
lastresort -V | --version
```

With no options, `lastresort` reads raw data from stdin
and writes encoded data as a continuous block to stdout.

### Options

`-c`, `--codec` `<CODEC>` Codec to use. Default: `pgp`.
Possible values: `pgp`, `eff`.

`-d`, `--decode` Decode data (default action is to encode data).

`-i`, `--input` `<INPUT_FILE>` Read input from `INPUT_FILE`.
Default is stdin; passing `-` also represents stdin.

`-o`, `--output` `<OUTPUT_FILE>` Write output to `OUTPUT_FILE`.
Default is stdout; passing `-` also represents stdout.

`-h`, `--help` Print usage summary and exit.

`-V`, `--version` Print version information and exit.

## See also

* [`pgen`(1)](https://crates.io/crates/pgen)

## Installation

1. [Install Rust](https://www.rust-lang.org/en-US/install.html).
2. Run `cargo install base256`
