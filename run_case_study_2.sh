#!/bin/bash

echo ">>>>>>>>>> COMPILE RUST BINARIES"
cargo build --release

# second case study only

echo
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo ">>>>>>>>>> CASE STUDY 2, VARIANT OF THE SKETCH WITH FIXED-POINT PROPERTIES ONLY"
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
./target/release/case-study-arabidopsis -f
echo

echo
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo ">>>>>>>>>> CASE STUDY 2, VARIANT OF THE SKETCH WITH COMPLEX PROPERTIES"
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
./target/release/case-study-arabidopsis -p
echo
