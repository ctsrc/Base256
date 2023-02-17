#!/usr/bin/env zsh

set -euxo pipefail

rm -f \
  sample_data/encoded/pgp/id_ed25519-fold_w_78_s.txt \
  sample_data/encoded/pgp/id_ed25519-fold_w_78.txt \
  sample_data/encoded/eff/id_ed25519-fold_w_78.txt \
  sample_data/encoded/eff/id_ed25519-fold_w_78_s.txt \
  sample_data/encoded/eff/id_ed25519-fold_w_78_s-trimmed.txt \
  sample_data/encoded/pgp/id_ed25519-fold_w_78_s-trimmed.txt \
  sample_data/encoded/pgp/id_ed25519.txt \
  sample_data/encoded/eff/id_ed25519.txt \
  target/release/lastresort

cargo build --release

./target/release/lastresort -e pgp -i sample_data/original/id_ed25519 | fold -w 78 -s > sample_data/encoded/pgp/id_ed25519-fold_w_78_s.txt

./target/release/lastresort -e pgp -i sample_data/original/id_ed25519 | fold -w 78 > sample_data/encoded/pgp/id_ed25519-fold_w_78.txt

./target/release/lastresort -e eff -i sample_data/original/id_ed25519 | fold -w 78 > sample_data/encoded/eff/id_ed25519-fold_w_78.txt

./target/release/lastresort -e eff -i sample_data/original/id_ed25519 | fold -w 78 -s > sample_data/encoded/eff/id_ed25519-fold_w_78_s.txt

./target/release/lastresort -e eff -i sample_data/original/id_ed25519 | fold -w 78 -s | awk '{$1=$1};1' > sample_data/encoded/eff/id_ed25519-fold_w_78_s-trimmed.txt

./target/release/lastresort -e pgp -i sample_data/original/id_ed25519 | fold -w 78 -s | awk '{$1=$1};1' > sample_data/encoded/pgp/id_ed25519-fold_w_78_s-trimmed.txt

./target/release/lastresort -e pgp -i sample_data/original/id_ed25519 > sample_data/encoded/pgp/id_ed25519.txt

./target/release/lastresort -e eff -i sample_data/original/id_ed25519 > sample_data/encoded/eff/id_ed25519.txt
