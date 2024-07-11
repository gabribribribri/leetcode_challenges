#![allow(dead_code)]
fn main() {
    println!("use `cargo test` to verify the implementation");
}
struct Solution;
#[cfg(test)]
mod testing {
    use crate::Solution;

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
        let mut je_suis_un_vecteur = vec![50];
        assert_eq!(Some(&50), je_suis_un_vecteur.last());
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut s_chars = s.chars().peekable();
        let mut tokens = RegExpToken::parse_string(p).peekable();
        while let Some(tok) = tokens.next() {
            match tok {
                RegExpToken::Every(every_tok) => match *every_tok {
                    RegExpToken::Specific(ch_to_match) => {
                        while let Some(sub_ch) = s_chars.peek() {
                            println!("{:?} {:?}", sub_ch, ch_to_match);
                            if *sub_ch != ch_to_match {
                                println!("break !");
                                break;
                            } else {
                                s_chars.next();
                            }
                        }
                    }
                    RegExpToken::Any => {
                        while let Some(sub_ch) = s_chars.next() {
                            if let Some(tok) = tokens.peek() {
                                if tok.match_single_char(sub_ch) {
                                    break;
                                }
                            }
                        }
                    }
                    RegExpToken::Every(_) => panic!("oh no no no this is bad"),
                },
                _ => {
                    if let Some(char_to_match) = s_chars.next() {
                        if !tok.match_single_char(char_to_match) {
                            println!("return case 1 {:?} {:?}", tok, char_to_match);
                            return false;
                        }
                    } else {
                        println!("return case 2 : {:?}", tok);
                        return false;
                    }
                }
            }
        }
        println!("has come to the end");
        return s_chars.next().is_none();
        // return true;
    }
}

#[derive(Debug)]
enum RegExpToken {
    Specific(char),
    Any,
    Every(Box<RegExpToken>),
}

impl RegExpToken {
    fn match_single_char(&self, ch: char) -> bool {
        match self {
            RegExpToken::Specific(ch_to_match) => return *ch_to_match == ch,
            RegExpToken::Any => true,
            RegExpToken::Every(_) => panic!("This isn't supposed to happen"),
        }
    }

    fn parse_string(s: String) -> impl Iterator<Item = RegExpToken> {
        let mut regexp_token_vec: Vec<RegExpToken> = Vec::new();
        let mut s_iter = s.chars().peekable();
        while let Some(ch) = s_iter.next() {
            if ch == '*' {
                continue;
            }
            if let Some(next_ch) = s_iter.peek() {
                if *next_ch == '*' {
                    println!("Is an every {:?}", next_ch);
                    regexp_token_vec.push(Self::Every(Box::new(Self::is_specific_or_any(ch))));
                } else {
                    if RegExpToken::is_last_item_specific_ext(&regexp_token_vec, ch) {
                        continue;
                    }
                    regexp_token_vec.push(Self::is_specific_or_any(ch));
                }
            } else {
                if RegExpToken::is_last_item_specific_ext(&regexp_token_vec, ch) {
                    continue;
                }
                println!("Is not an every");
                regexp_token_vec.push(Self::is_specific_or_any(ch));
            }
        }
        println!("{:?}", &regexp_token_vec);

        return regexp_token_vec.into_iter();
    }

    fn is_specific_or_any(ch: char) -> RegExpToken {
        if ch == '.' {
            Self::Any
        } else {
            Self::Specific(ch)
        }
    }

    fn is_last_item_specific_ext(regexp_token_vec: &Vec<RegExpToken>, current: char) -> bool {
        if let Some(last_token) = regexp_token_vec.last() {
            println!("INSIDE FIRST CHECKING NIGHTMARE");
            dbg!(last_token);
            if let RegExpToken::Every(last_inner_token) = last_token {
                dbg!(last_inner_token);
                if let RegExpToken::Specific(last_inner_inner_token) = **last_inner_token {
                    dbg!(last_inner_inner_token);
                    if last_inner_inner_token == current {
                        println!("yep. we did it.");
                        return true;
                    }
                }
            }
        }
        return false;
    }
}
