#![feature(test)]
#![feature(uniform_paths)]

#[macro_use] extern crate assert_float_eq;
extern crate test;
extern crate hashbrown;

use core::hash::Hash;
use hashbrown::HashSet;
use std::slice::Windows;


fn intersection<T: Sized + Hash + Eq>(wx: Windows<T>, wy: Windows<T>) -> u64 {
    let len = *&wx.len() as usize;
    let hash: HashSet<&[T]> = wx.fold(
        HashSet::with_capacity(len),
        |mut acc, val| {
            acc.insert(val);
            acc
        }
    );

    let mut len: u64 = 0;
    for w in wy {
        if hash.contains(w) {
            len += 2;
        }
    }

    len
}

/**
    Calculates Sørensen–Dice coefficient
    https://en.wikipedia.org/wiki/Sørensen–Dice_coefficient
**/
#[inline(always)]
pub fn sorensen<T: Sized + Hash + Eq>(first: &[T], second: &[T]) -> f64 {
    let x_len = *&first.len() - 1;
    let y_len = *&second.len() - 1;

    if x_len + y_len < 10000 {
        let first_bigrams: Windows<T> = first.windows(2);
        let second_bigrams: Windows<T> = second.windows(2);

        let nx: usize = first_bigrams.len();
        let ny: usize = second_bigrams.len();
        let len = intersection(first_bigrams, second_bigrams);
        len as f64 / (nx as f64 + ny as f64)
    } else {
        let coll_x = first.chunks(500);
        let mut coll_y = second.chunks(500);

        let mut len = 0;
        for chunk in coll_x {
            let first_bigrams: Windows<T> = chunk.windows(2);
            let second_bigrams: Windows<T> = coll_y.next().unwrap().windows(2); // Option

            len += intersection(first_bigrams, second_bigrams);
        }

        len as f64 / (x_len as f64 + y_len as f64)
    }
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

    #[test]
    fn sorensen_distance_huge_test() {
        let str1 = (0..100000).map(|_| { 'a' }).collect::<Vec<char>>();
        let str2 = (0..100000).map(|_| { 'a' }).collect::<Vec<char>>();
        let result = sorensen(&str1, &str2);
        expect_float_relative_eq!(result, 1f64, 0.0003).ok();
    }

}