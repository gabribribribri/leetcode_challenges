fn main() {
    println!("je suis dépité `cargo test`");
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
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut triplets = Vec::new();
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                for k in 0..nums.len() {
                    if i == j || i == k || j == k {
                        continue;
                    }
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut trip = vec![nums[i], nums[j], nums[k]];
                        trip.sort();
                        if !triplets.contains(&trip) {
                            triplets.push(trip);
                        }
                    }
                }
            }
        }
        triplets
    }
}
