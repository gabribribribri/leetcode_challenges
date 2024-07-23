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

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {}
}
