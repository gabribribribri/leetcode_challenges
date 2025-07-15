#![allow(dead_code)]

fn main() {
    println!("Use `cargo test` to test implementation");
}

#[cfg(test)]
mod garglurglug {
    use super::*;

    #[test]
    fn missing_number_1() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2)
    }

    #[test]
    fn missing_number_2() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2)
    }

    #[test]
    fn missing_number_3() {
        assert_eq!(Solution::missing_number(vec![9,6,4,2,3,5,7,0,1]), 8)
    }
}

struct Solution;

use std::ops::BitXor;

impl Solution {
    // Traditionnal boring solution
    // pub fn missing_number(nums: Vec<i32>) -> i32 {
    //     let mut flag = vec![0u8; nums.len()/8+1];

    //     for n in nums {
    //         flag[n as usize/8] |= 0b1 << n as usize%8;
    //     }
        
    //     for (i, byte) in flag.iter().enumerate() {
    //         if byte.trailing_ones() != 8 || i == flag.len()-1{
    //             return (i as i32*8)+(byte.trailing_ones() as i32);
    //         }
    //     }
    //     unreachable!()
    // }

    // Cool XOR Trick
    pub fn missing_number(nums: Vec<i32>) -> i32 {
       return nums.iter().fold(0, BitXor::bitxor)
              ^ (0..=nums.len() as i32).fold(0, BitXor::bitxor)
    }
}
