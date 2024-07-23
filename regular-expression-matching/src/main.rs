#![allow(dead_code)]

fn main() {
    println!("use `cargo test` to verify the implementation");
}
struct Solution;
#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn regexp_01() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }
    #[test]
    fn regexp_02() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    }
    #[test]
    fn regexp_03() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }
    #[test]
    fn regexp_04() {
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
    }
    #[test]
    fn regexp_05() {
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*ip*.".to_string()),
            true
        );
    }
    #[test]
    fn regexp_06() {
        assert_eq!(
            Solution::is_match("aaa".to_string(), "a*a".to_string()),
            true
        );
    }
    #[test]
    fn regexp_07() {
        assert_eq!(
            Solution::is_match("aaaaaaa".to_string(), "a*aaaaa".to_string()),
            true
        );
    }
    #[test]
    fn regexp_08() {
        assert_eq!(
            Solution::is_match("abcdef".to_string(), "abcdef".to_string()),
            true
        );
    }
    #[test]
    fn regexp_09() {
        assert_eq!(
            Solution::is_match("abcdef".to_string(), "ab.def".to_string()),
            true
        );
    }

    #[test]
    fn regexp_10() {
        assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
    }
    #[test]
    fn regexp_11() {
        assert_eq!(
            Solution::is_match(
                "aaaaaaaaaaaaab".to_string(),
                "a*a*a*a*a*a*a*a*a*c".to_string()
            ),
            false
        );
    }
    #[test]
    fn regexp_12() {
        assert_eq!(
            Solution::is_match("aaaaaaaaaaaaab".to_string(), "a*c".to_string()),
            false
        );
    }
    #[test]
    fn regexp_13() {
        assert_eq!(
            Solution::is_match(
                "aaaaaaaaaaaaaaaaaaab".to_string(),
                "a*a*a*a*a*a*a*a*a*a*".to_string()
            ),
            false
        );
    }
}

use std::{cell::RefCell, collections::HashMap};

impl Solution {
    //p: pattern: regex
    //s: subject
    pub fn is_match(s: String, p: String) -> bool {
        _is_match(s.as_str(), p.as_str(), &RefCell::new(HashMap::new()))
    }
}

fn _is_match<'a>(
    s: &'a str,
    p: &'a str,
    dict: &RefCell<HashMap<(&'a str, &'a str), bool>>,
) -> bool {
    if let Some(r) = dict.borrow().get(&(s, p)) {
        return *r;
    }
    if p.is_empty() {
        let r = s.is_empty();
        dict.borrow_mut().insert((s, p), r);
        return r;
    }

    if let Some('*') = p.chars().nth(1) {
        if let Some(first_ch) = s.chars().next() {
            if first_ch == p.chars().next().unwrap() || p.chars().next().unwrap() == '.' {
                let r = _is_match(s, &p[2..], &dict)
                    || _is_match(&s[1..], p, &dict)
                    || _is_match(&s[1..], &p[2..], &dict);
                dict.borrow_mut().insert((s, p), r);
                return r;
            }
        }
        let r = _is_match(s, &p[2..], &dict);
        dict.borrow_mut().insert((s, p), r);
        return r;
    } else if let Some(first_ch) = s.chars().next() {
        if first_ch == p.chars().next().unwrap() || p.chars().next().unwrap() == '.' {
            let r = _is_match(&s[1..], &p[1..], &dict);
            dict.borrow_mut().insert((s, p), r);
            return r;
        }
    }
    dict.borrow_mut().insert((s, p), false);
    return false;
}
