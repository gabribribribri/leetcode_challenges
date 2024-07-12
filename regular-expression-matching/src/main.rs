#![allow(dead_code)]

fn main() {
    println!("use `cargo test` to verify the implementation");
}
struct Solution;
#[cfg(test)]
mod testing {
    use crate::{MyExtension, Solution};

    #[test]
    fn regexp_1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }
    #[test]
    fn regexp_2() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    }
    #[test]
    fn regexp_3() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }
    #[test]
    fn regexp_4() {
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
    }
    #[test]
    fn regexp_5() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()),
            true
        );
    }
    #[test]
    fn regexp_6() {
        assert_eq!(
            Solution::is_match("aaa".to_string(), "a*a".to_string()),
            true
        );
    }
    #[test]
    fn regexp_7() {
        assert_eq!(
            Solution::is_match("aaaaaaa".to_string(), "a*aaaaa".to_string()),
            true
        );
    }

    #[test]
    fn vector_last() {
        let je_suis_un_vecteur = vec![50];
        assert_eq!(Some(&50), je_suis_un_vecteur.last());
    }

    #[test]
    fn rem_dup_char() {
        assert_eq!(
            "aaaaabbbc".chars().remove_duplicate_characters(),
            "abc".to_string()
        )
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let sp = s.chars().remove_duplicate_characters();
        let pp = RegExpToken::parse(p);

        let mut repeat_until_next_match_found = false;
        let mut pi = 0;
        let mut si = 0;
        while si < sp.len() {
            if pp[pi].required {
                if pp[pi].ch != sp.chars().nth(si).unwrap() {
                    return false;
                }
                si += 1;
            } else {
                if pp[pi].ch == sp.chars().nth(si).unwrap() {
                    si += 1;
                }
            }
            //RegExp bounds check
            pi += 1;
            if pi >= pp.len() {
                return false;
            }
        }
        return true;
    }
}

struct RegExpToken {
    ch: char,
    required: bool,
}

impl RegExpToken {
    fn parse(p: String) -> Vec<Self> {
        let mut required = true;
        let mut parsed = Vec::new();
        for ch in p.chars().rev() {
            match ch {
                '*' => required = false,
                _ => {
                    parsed.push(RegExpToken { ch, required });
                    required = false;
                }
            }
        }
        parsed.reverse();
        return parsed;
    }
}

trait MyExtension: Iterator<Item = char> {
    fn remove_duplicate_characters(self) -> String
    where
        Self: Sized,
    {
        let mut reversed = Vec::new();
        for ch in self {
            if let Some(last_ch) = reversed.last() {
                if *last_ch == ch {
                    continue;
                }
            }
            reversed.push(ch);
        }
        return reversed.into_iter().collect();
    }
}

impl<I: Iterator<Item = char>> MyExtension for I {}
