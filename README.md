# Sorensen
[Sørensen–Dice coefficient](https://en.wikipedia.org/wiki/Sørensen–Dice_coefficient) implementation for Rust.

[![Build Status](https://travis-ci.org/ponyloop/sorensen.svg?branch=master)](https://travis-ci.org/ponyloop/sorensen)

Work in progress

## Usage

 ```rust
    extern crate sorensen;
    
    use sorensen::distance;
    
    let string = "night";
    let string_to_compare = "nacht";
    let dst: f64 = distance(string.as_bytes(), string_to_compare.as_bytes()); // 0.25
 ```