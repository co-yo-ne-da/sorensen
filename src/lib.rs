#![feature(test)]
#![feature(uniform_paths)]

#[macro_use] extern crate assert_float_eq;
extern crate test;
extern crate hashbrown;

use core::hash::Hash;
use hashbrown::HashSet;
use std::slice::{ Windows, Chunks };


#[inline(always)]
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

#[inline(always)]
fn short_length<T: Sized + Hash + Eq>(wx: Windows<T>, wy: Windows<T>) -> f64 {
    let nx: usize = wx.len();
    let ny: usize = wy.len();
    let len = intersection(wx, wy);

    len as f64 / (nx as f64 + ny as f64)
}

#[inline(always)]
fn long_length<T: Sized + Hash + Eq>(cx: Chunks<T>, cy: &mut Chunks<T>) -> f64 {
    let mut len = 0;
    for chunk in cx {
        let wx: Windows<T> = chunk.windows(2);
        match cy.next() {
            Some(ch) => {
                let wy = ch.windows(2);
                len += intersection(wx, wy);
            },
            None => {}
        };
    }

    len as f64
}

/**
    Calculates Sørensen–Dice coefficient
    https://en.wikipedia.org/wiki/Sørensen–Dice_coefficient

    Examples:

    ```rust
    extern crate sorensen;

    use sorensen::distance;

    let string = "night";
    let string_to_compare = "nacht";
    let dst: f64 = distance(string.as_bytes(), string_to_compare.as_bytes()); // 0.25

    ```
**/
#[inline(always)]
pub fn distance<T: Sized + Hash + Eq>(x: &[T], y: &[T]) -> f64 {
    let x_len = *&x.len() - 1;
    let y_len = *&y.len() - 1;

    if x_len + y_len < 10000 {
        short_length(x.windows(2), y.windows(2))
    } else {
        let len = long_length(x.chunks(500), &mut y.chunks(500));
        len / (x_len as f64 + y_len as f64)
    }
}


#[cfg(test)]
mod tests {
    extern crate rand;
    use super::*;

    #[test]
    fn sorensen_distance_test() {
        let result = distance("night".as_bytes(), "nacht".as_bytes());
        assert_float_absolute_eq!(result, 0.25f64)
    }

    #[test]
    fn sorensen_distance_irrational_test() {
        let result = distance("seal".as_bytes(), "sale".as_bytes());
        assert_float_absolute_eq!(result, 0.3333333333333333f64)
    }

    #[test]
    fn sorensen_distance_unicode_test() {
        let result = distance("Sørensen".as_bytes(), "Sørensen".as_bytes());
        assert_float_absolute_eq!(result, 1f64)
    }

    #[test]
    fn sorensen_distance_repetative_test() {
        let result = distance("whoawhoa".as_bytes(), "whoawhoa".as_bytes());
        assert_float_absolute_eq!(result, 1f64)
    }

    #[test]
    fn sorensen_distance_zero_test() {
        let result = distance("abca".as_bytes(), "xyza".as_bytes());
        assert_float_absolute_eq!(result, 0f64)
    }

    #[test]
    fn sorensen_distance_exact_test() {
        let result = distance("night".as_bytes(), "night".as_bytes());
        assert_float_absolute_eq!(result, 1f64)
    }

    #[test]
    fn sorensen_distance_huge_test() {
        let str1 = (0..100000).map(|_| { 'a' }).collect::<Vec<char>>();
        let str2 = (0..100000).map(|_| { 'a' }).collect::<Vec<char>>();
        let result = distance(&str1, &str2);
        expect_float_relative_eq!(result, 1f64, 0.00003).ok();
    }

}