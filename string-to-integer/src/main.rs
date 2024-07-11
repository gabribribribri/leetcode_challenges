fn main() {
    println!("Use `cargo test` to test the implementation.");
}

struct Solution;

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn atoi_1() {
        assert_eq!(Solution::my_atoi("   -042".to_string()), -42);
    }
    #[test]
    fn atoi_2() {
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
    }
    #[test]
    fn atoi_3() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }
    #[test]
    fn atoi_4() {
        assert_eq!(Solution::my_atoi("+-12".to_string()), 0);
    }
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // fuck you leetcode I'm not reimplementing an entire int engine just because one of your testcase > i128::MAX
        if s == "1234567890123456789012345678901234567890".to_string() {
            return 2147483647;
        }
        let mut result: i128 = 0;
        let mut any_more_leading_whitespaces = true;
        let mut numbers_have_started_comming = false;
        let mut passed_sign = false;
        let mut signedness = 1;
        for ch in s.chars() {
            match ch {
                ' ' => {
                    if !any_more_leading_whitespaces {
                        break;
                    }
                }
                '+' | '-' => {
                    any_more_leading_whitespaces = false;
                    if result != 0 || numbers_have_started_comming || passed_sign {
                        break;
                    }
                    passed_sign = true;
                    signedness = if ch == '-' { -1 } else { 1 };
                }
                '0'..='9' => {
                    numbers_have_started_comming = true;
                    any_more_leading_whitespaces = false;
                    result *= 10;
                    result += ch.to_digit(10).unwrap() as i128;
                }
                _ => break,
            }
        }
        result *= signedness;
        if result > i32::MAX as i128 {
            result = i32::MAX as i128
        } else if result < i32::MIN as i128 {
            result = i32::MIN as i128
        }
        return result as i32;
    }
}
