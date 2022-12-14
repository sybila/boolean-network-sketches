#!/bin/bash

echo ">>>>>>>>>> COMPILE RUST BINARIES"
cargo build --release

# second case study only

echo
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo ">>>>>>>>>> CASE STUDY 2, VARIANT OF THE SKETCH WITH FIXED-POINT PROPERTIES ONLY"
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo
./target/release/case-study-arabidopsis
echo

echo
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo ">>>>>>>>>> CASE STUDY 2, VARIANT OF THE SKETCH WITH COMPLEX PROPERTIES"
echo ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
echo
./target/release/case-study-arabidopsis -m
echo
