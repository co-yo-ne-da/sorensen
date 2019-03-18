#![feature(test)]
#![feature(uniform_paths)]

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
pub fn sorensen<T: Sized + Ord + Hash + Clone>(first: &[T], second: &[T]) -> f64 {
    let hash: HashSet<&[T]> = first.windows(2).collect();
    let windows: Windows<T> = second.windows(2);

    let nx: f64 = hash.len() as f64;
    let ny: f64 = windows.len() as f64;

    let mut len: u64 = 0;

    for w in windows {
        if hash.contains(w) {
            len += 1;
        }
    }

    2f64 * len as f64 / (nx + ny)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn sorensen_distance_test() {
        let result = sorensen("night".as_bytes(), "nacht".as_bytes());
        assert_eq!(result, 0.25f64)
    }

    #[test]
    fn sorensen_distance_exact_test() {
        let result = sorensen("night".as_bytes(), "night".as_bytes());
        assert_eq!(result, 1f64)
    }

    #[bench]
    fn intersection_short_bench(b: &mut Bencher) {
        b.iter(|| {
            sorensen(
                "Hello".as_bytes(),
                "World".as_bytes()
            )
        })
    }

    #[bench]
    fn sorensen_distance_long_bench(b: &mut Bencher) {
        b.iter(|| {
            sorensen(
                "Loremipsumdolorsiamet".as_bytes(),
                "Loremipsumdolorsiameeeet".as_bytes()
            )
        })

    }

    #[bench]
    fn intersection_10000_long_bench(b: &mut Bencher) {
        let str1 = (0..10000).map(|_| { 'a' }).collect::<String>();
        let str2 = (0..10000).map(|_| { 'h' }).collect::<String>();

        b.iter(|| {
            sorensen(
                str1.as_str().as_bytes(),
                str2.as_str().as_bytes()
            )
        })
    }

    #[bench]
    fn intersection_1000000_long_bench(b: &mut Bencher) {
        let str1 = (0..1000000).map(|_| { 'a' }).collect::<String>();
        let str2 = (0..1000000).map(|_| { 'h' }).collect::<String>();

        b.iter(|| {
            sorensen(
                str1.as_str().as_bytes(),
                str2.as_str().as_bytes()
            )
        })
    }


}