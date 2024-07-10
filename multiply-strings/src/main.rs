struct Solution;
fn main() {
    println!("Use `cargo test` to test the implementation")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse() {
        assert_eq!("coucou".to_string(), reverse("uocuoc"));
    }

    #[test]
    fn test_multiply_by_digit_1() {
        assert_eq!(multiply_by_digit("321", '3'), "963".to_string());
    }

    #[test]
    fn test_multiply_by_digit_2() {
        assert_eq!(multiply_by_digit("005", '5'), "0052".to_string());
    }
    #[test]
    fn test_u32_to_char() {
        assert_eq!('5', u32_to_char(5));
    }

    #[test]
    fn test_add_1() {
        assert_eq!(add(vec!["503".to_string()]), "503".to_string());
    }
    #[test]
    fn test_add_2() {
        assert_eq!(
            add(vec!["503".to_string(), "506".to_string()]),
            "019".to_string()
        );
    }
    #[test]
    fn test_add_3() {
        assert_eq!(
            add(vec!["999".to_string(), "0001".to_string(), "1".to_string()]),
            "0002".to_string()
        );
    }
    #[test]
    fn test_multiply_by_factor_10_1() {
        assert_eq!(
            multiply_by_factor_10("9999".to_string(), 3),
            "0009999".to_string()
        );
    }
    #[test]
    fn test_multiply_by_factor_10_2() {
        assert_eq!(
            multiply_by_factor_10("9999".to_string(), 0),
            "9999".to_string()
        );
    }
    #[test]
    fn solution_1() {
        assert_eq!(
            Solution::multiply("2".to_string(), "3".to_string()),
            "6".to_string()
        );
    }
    #[test]
    fn test_solution_2() {
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
    }
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let rev_num1 = reverse(&num1);
        let rev_num2 = reverse(&num2);

        let all_products = rev_num2
            .char_indices()
            .map(|(i, c)| multiply_by_factor_10(multiply_by_digit(&rev_num1, c), i))
            .collect();
        return remove_trailing_zeros(reverse(&add(all_products)));
    }
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn multiply_by_digit(b: &str, p: char) -> String {
    let pd = p.to_digit(10).unwrap() as u32;
    let mut f = String::new();
    let mut rem: u32 = 0;
    for bd in b.chars().map(|c| c.to_digit(10).unwrap()) {
        let mut next_ch = bd * pd + rem;
        rem = next_ch / 10;
        next_ch %= 10;
        f.push(u32_to_char(next_ch));
    }
    if rem != 0 {
        f.push(u32_to_char(rem))
    }
    return f;
}

fn add(v: Vec<String>) -> String {
    let vec_digits = v
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut next_ch;
    let mut rem = 0;
    let mut f = String::new();
    for i in 0..vec_digits.iter().map(Vec::len).max().unwrap() {
        next_ch = rem;
        for v in &vec_digits {
            next_ch += v.get(i).unwrap_or(&0);
        }
        rem = next_ch / 10;
        next_ch %= 10;
        f.push(u32_to_char(next_ch));
    }
    if rem != 0 {
        f.push(u32_to_char(rem))
    }
    return f;
}

fn u32_to_char(n: u32) -> char {
    n.to_string().chars().next().unwrap()
}

fn multiply_by_factor_10(s: String, fac: usize) -> String {
    format!("{}{}", (0..fac).map(|_| '0').collect::<String>(), s)
}

fn remove_trailing_zeros(mut s: String) -> String {
    while s.starts_with('0') && s.len() != 1 {
        s.remove(0);
    }
    return s;
}
