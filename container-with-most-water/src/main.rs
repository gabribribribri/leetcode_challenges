#![allow(dead_code)]

fn main() {
    println!("bah nan euh `cargo test`");
}

#[cfg(test)]
mod gloobiboulga {
    use super::*;
    #[test]
    fn max_area_01() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
    #[test]
    fn max_area_02() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
struct Solution;

use std::cmp::{max, min};
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut biggest = 0;
        while r > l {
            biggest = max(biggest, min(height[r], height[l]) * (r as i32 - l as i32));
            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        return biggest;
    }
}
