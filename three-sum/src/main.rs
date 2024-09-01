#![allow(dead_code)]
fn main() {
    println!("je suis dépité `cargo test`");
    println!("{}", 5 / 2)
}

#[cfg(test)]
mod caligula {
    use super::*;

    #[test]
    fn three_sum_01() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1],]
        )
    }
}

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut triplets = Vec::new();
        for i in 0..nums.len() - 2 {
            if (i > 0) && (nums[i] == nums[i - 1]) {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let curr_tripl = nums[i] + nums[l] + nums[r];
                if curr_tripl == 0 {
                    triplets.push(vec![nums[i], nums[l], nums[r]]);
                    while (r > l) && (nums[r] == nums[r - 1]) {
                        r -= 1;
                    }
                    while (r > l) && (nums[l] == nums[l + 1]) {
                        l += 1;
                    }
                    r -= 1;
                    l += 1;
                } else if curr_tripl > 0 {
                    r -= 1;
                } else if curr_tripl < 0 {
                    l += 1;
                }
            }
        }
        triplets
    }
}
