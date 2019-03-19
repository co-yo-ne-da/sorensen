#![feature(test)]
#![feature(uniform_paths)]

#[macro_use] extern crate assert_float_eq;
extern crate test;
extern crate hashbrown;

use core::hash::Hash;
use hashbrown::HashSet;
use std::slice::Windows;


/**
    Calculates Sørensen–Dice coefficient
    https://en.wikipedia.org/wiki/Sørensen–Dice_coefficient
**/
#[inline(always)]
pub fn sorensen<T: Sized + Hash + Eq>(first: &[T], second: &[T]) -> f64 {
    let first_bigrams: Windows<T> = first.windows(2);
    let second_bigrams: Windows<T> = second.windows(2);

    let nx: f64 = first_bigrams.len() as f64;
    let ny: f64 = second_bigrams.len() as f64;

    let hash: HashSet<&[T]> = first_bigrams.fold(
        HashSet::with_capacity(nx as usize),
        |mut acc, val| {
            acc.insert(val);
            acc
        }
    );

    let mut len: f64 = 0f64;
    for w in second_bigrams {
        if hash.contains(w) {
            len += 2f64;
        }
    }

    len / (nx + ny)
}


#[cfg(test)]
mod tests {
    extern crate rand;
    use super::*;


    #[test]
    fn sorensen_distance_test() {
        let result = sorensen("night".as_bytes(), "nacht".as_bytes());
        assert_float_absolute_eq!(result, 0.25f64)
    }

    #[test]
    fn sorensen_distance_irrational_test() {
        let result = sorensen("seal".as_bytes(), "sale".as_bytes());
        assert_float_absolute_eq!(result, 0.3333333333333333f64)
    }

    #[test]
    fn sorensen_distance_unicode_test() {
        let result = sorensen("Sørensen".as_bytes(), "Sørensen".as_bytes());
        assert_float_absolute_eq!(result, 1f64)
    }

    #[test]
    fn sorensen_distance_repetative_test() {
        let result = sorensen("whoawhoa".as_bytes(), "whoawhoa".as_bytes());
        assert_float_absolute_eq!(result, 1f64)
    }

    #[test]
    fn sorensen_distance_zero_test() {
        let result = sorensen("abca".as_bytes(), "xyza".as_bytes());
        assert_float_absolute_eq!(result, 0f64)
    }

    #[test]
    fn sorensen_distance_exact_test() {
        let result = sorensen("night".as_bytes(), "night".as_bytes());
        assert_float_absolute_eq!(result, 1f64)
    }
}