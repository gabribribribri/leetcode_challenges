#![allow(dead_code)]

fn main() {
    println!("TU DOIS FAIRE `cargo test` OH");
}

#[cfg(test)]
mod bouglibalgaga {
    use super::*;
    #[test]
    fn int_to_roman_01() {
        assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
    }
    #[test]
    fn int_to_roman_02() {
        assert_eq!(Solution::int_to_roman(3749), String::from("MMMDCCXLIX"));
    }
    #[test]
    fn int_to_reman_03() {
        assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
    }
}
struct Solution;
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman = String::new();
        for (i, dig) in ALL_DIGITS.iter().enumerate() {
            let how_much = num / dig.value;
            println!("{how_much}");
            match how_much {
                0 => (),
                1..=3 => {
                    for _ in 0..how_much {
                        roman.push(dig.one);
                    }
                }
                4 => {
                    roman.push(dig.one);
                    roman.push(dig.five);
                }
                5..=8 => {
                    roman.push(dig.five);
                    for _ in 0..how_much - 5 {
                        roman.push(dig.one);
                    }
                }
                9 => {
                    roman.push(dig.one);
                    roman.push(ALL_DIGITS[i - 1].one);
                }
                _ => panic!("supposed to be in 0..10"),
            }
            num -= how_much * dig.value;
        }
        roman
    }
}
struct RomanDigit {
    value: i32,
    one: char,
    five: char,
}

const ALL_DIGITS: [RomanDigit; 4] = [
    RomanDigit {
        value: 1000,
        one: 'M',
        five: '5',
    },
    RomanDigit {
        value: 100,
        one: 'C',
        five: 'D',
    },
    RomanDigit {
        value: 10,
        one: 'X',
        five: 'L',
    },
    RomanDigit {
        value: 1,
        one: 'I',
        five: 'V',
    },
];
