#!/bin/bash

echo ">>>>>>>>>> COMPILE RUST BINARIES"
cargo build --release

# first case study only

echo ">>>>>>>>>> CASE STUDY 1, INITIAL VERSION OF THE SKETCH"
./target/release/case-study-tlgl

echo ">>>>>>>>>> CASE STUDY 1, REFINED VERSION OF THE SKETCH"
./target/release/case-study-tlgl -r
echo
