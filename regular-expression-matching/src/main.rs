#![allow(dead_code)]

fn main() {
    println!("use `cargo test` to verify the implementation");
}
struct Solution;
#[cfg(test)]
mod testing {
    use super::*;

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

    // #[test]
    // fn rem_dup_char() {
    //     assert_eq!(
    //         "aaaaabbbc".chars().remove_duplicate_characters(),
    //         "abc".to_string()
    //     )
    // }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let regexp = p.parse(s);
        return regexp.is_match();
    }
}

struct RegExpToken {
    ch: char,
    repeat: bool,
    matched: bool,
}

struct RegExp {
    sub: String,
    re: Vec<RegExpToken>,
}

impl RegExp {
    fn is_match(&self) -> bool {
        let mut ri = 0;
        let mut si = 0;
        while si < self.sub.len() {
            if self.re[ri].repeat {
            } else {
                if self.sub_at(si) != self.re[ri].ch && self.sub_at(si) != '.' {
                    return false;
                }
            }
            ri += 1;
            if ri >= self.re.len() {
                return false;
            }
        }
        if ri != self.re.len() - 1 {
            return false;
        }
        return true;
    }

    fn sub_at(&self, si: usize) -> char {
        self.sub.chars().nth(si).unwrap()
    }

    fn check_for_repeating(&mut self, i: usize) -> bool {
        todo!()
    }
}

trait ToRegExp {
    fn parse(self, sub: String) -> RegExp;
}

impl ToRegExp for String {
    fn parse(self, sub: String) -> RegExp {
        let mut re: Vec<RegExpToken> = Vec::new();
        for ch in self.chars() {
            if ch == '*' {
                re.last_mut().unwrap().repeat = true;
            }
            re.push(RegExpToken {
                ch,
                repeat: false,
                matched: false,
            })
        }
        return RegExp { re, sub };
    }
}
