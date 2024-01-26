# Sorensen
[Sørensen–Dice coefficient](https://en.wikipedia.org/wiki/Sørensen–Dice_coefficient) implementation for Rust.

[![CI](https://github.com/co-yo-ne-da/sorensen/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/co-yo-ne-da/sorensen/actions/workflows/ci.yml)

##### v0.1.3

## Usage

 ```rust
    extern crate sorensen;
    
    use sorensen::distance;
    
    let string = "night";
    let string_to_compare = "nacht";
    let dst: f64 = distance(string.as_bytes(), string_to_compare.as_bytes()); // 0.25
 ```
