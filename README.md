# rust_hamming_distance

Hamming distance and bitwise hamming distance implementations in rust

[![Build Status](https://travis-ci.org/SingingTree/rust_hamming_distance.svg?branch=master)](https://travis-ci.org/SingingTree/rust_hamming_distance)

## Generalised implementations

Due to how type checking is done by the compiler (see for example RFCs: #586, #1053), more general
implementations of the bitwise hamming traits, such as those on IntoIterators yielding u8s,
cannot coexist with specific implementations, like those on u8. This can be worked around,
but is not yet implemented at this stage.