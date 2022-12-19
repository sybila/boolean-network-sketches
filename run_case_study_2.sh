#!/bin/bash

echo ">>>>>>>>>> COMPILE RUST BINARIES"
cargo build --release

# second case study only

echo ">>>>>>>>>> CASE STUDY 2, SKETCH WITH FIXED-POINT PROPERTIES ONLY"
./target/release/case-study-arabidopsis -f

echo ">>>>>>>>>> CASE STUDY 2, SKETCH WITH COMPLEX PROPERTIES"
./target/release/case-study-arabidopsis -p
echo
