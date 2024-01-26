#[macro_use] extern crate assert_float_eq;
extern crate hashbrown;

use core::hash::Hash;
use hashbrown::HashSet;


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
pub fn distance<T: Sized + Hash + Eq>(x: &[T], y: &[T]) -> f64 {
    let wx = x.windows(2);
    let wy = y.windows(2);

    let nx = wx.len();
    let ny = wy.len();

    let mut hash_set: HashSet<&[T]> = HashSet::with_capacity(nx as usize);
    for item in wx {
        hash_set.insert(item);
    }

    let mut len: u64 = 0;
    for w in wy {
        if hash_set.contains(w) {
            len += 2;
        }
    }

    len as f64 / (nx as f64 + ny as f64)
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