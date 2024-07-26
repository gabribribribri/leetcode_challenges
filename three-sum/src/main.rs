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
            vec![vec![-1, 0, 1], vec![-1, -1, 2],]
        )
    }
}

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut triplets = Vec::new();
        let mut m = nums.len() / 2;
        let mut l = m - 1;
        let mut r = m + 1;
        while m >= 1 && m <= nums.len() - 2 {
            // debugging
            for n in &nums {
                print!("{} ", n.abs());
            }
            println!();
            for i in 0..nums.len() {
                print!(
                    "{} ",
                    if i == l {
                        'l'
                    } else if i == m {
                        'm'
                    } else if i == r {
                        'r'
                    } else {
                        ' '
                    }
                )
            }
            println!();
            //-------------------------
            let triplet_value = nums[l] + nums[m] + nums[r];
            if triplet_value == 0 {
                triplets.push(vec![nums[l], nums[m], nums[r]]);
                if r == nums.len() - 1 {
                    m -= 1;
                    if m == 0 {
                        break;
                    }
                    l = m - 1;
                    r = m + 1;
                } else {
                    r += 1;
                }
            } else if triplet_value > 0 {
                if l == 0 {
                    m += 1;
                    if m == 0 {
                        break;
                    }
                    l = m - 1;
                    r = m + 1;
                } else {
                    l -= 1;
                }
            } else if triplet_value < 0 {
                if r == nums.len() - 1 {
                    m -= 1;
                    if m == 0 {
                        break;
                    }
                    l = m - 1;
                    r = m + 1;
                } else {
                    r += 1;
                }
            }
        }
        triplets.dedup();
        return triplets;
    }
}
