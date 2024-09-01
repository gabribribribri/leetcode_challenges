#![allow(dead_code)]
struct Solution;
fn main() {
    println!("welp. not even a cargo test. sowwy");
}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common_prefix = String::new();
        for i in 0.. {
            let mut common_char: Option<char> = None;
            for s in strs.iter() {
                if let Some(ch) = s.chars().nth(i) {
                    if ch != common_char.unwrap_or(ch) {
                        return common_prefix;
                    }
                    common_char = Some(ch);
                } else {
                    return common_prefix;
                }
            }
            common_prefix.push(common_char.unwrap())
        }
        common_prefix
    }
}
