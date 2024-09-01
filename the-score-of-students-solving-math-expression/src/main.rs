#![allow(dead_code)]

pub struct Solution;
fn main() {
    println!("Use `cargo test` to test the implementation")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn score_of_students_1() {
        assert_eq!(
            Solution::score_of_students("7+3*1*2".into(), vec![20, 13, 42]),
            7
        );
    }
    #[test]
    fn score_of_students_2() {
        assert_eq!(
            Solution::score_of_students("3+5*2".into(), vec![13, 0, 10, 13, 13, 16, 16]),
            19
        );
    }
    #[test]
    fn score_of_students_3() {
        assert_eq!(
            Solution::score_of_students("6+0*1".into(), vec![12, 9, 6, 4, 8, 6]),
            10
        );
    }
    #[test]
    fn score_of_students_4() {
        assert_eq!(
            Solution::score_of_students("1+2*3+4".into(), vec![13, 21, 11, 15]),
            11
        );
    }
    #[test]
    fn score_of_students_5() {
        assert_eq!(
            Solution::score_of_students("6+3*6+2*9+9*4+9*9+5*9+6".into(), vec![]),
            0
        );
    }
}

// START OF LEETCODE COPY PASTE

use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(i32),
    Addition,
    Multiplication,
}
use Token::*;

impl Token {
    fn value(&self) -> i32 {
        if let Number(x) = self {
            return *x;
        } else {
            panic!("Not supposed to be everything other than Number(x)");
        }
    }
}

trait TokenSupport {
    fn parse(from: &str) -> Self;
    fn correct_eval(self, step_is_mul: bool) -> i32;
    fn all_eval(self, start_at: usize) -> HashSet<i32>;
}

impl TokenSupport for Vec<Token> {
    fn parse(from: &str) -> Self {
        let mut parsed = vec![];
        for ch in from.chars() {
            parsed.push(match ch {
                '0'..='9' => Number(ch.to_digit(10).unwrap() as i32),
                '+' => Addition,
                '*' => Multiplication,
                _ => panic!("Character not supposed to exist"),
            })
        }
        parsed
    }

    fn correct_eval(self, step_is_mul: bool) -> i32 {
        let searched_token = if step_is_mul {
            Multiplication
        } else {
            Addition
        };
        let oper = if step_is_mul {
            |a, b| a * b
        } else {
            |a, b| a + b
        };
        for (i, tok) in self.iter().enumerate().skip(1).step_by(2) {
            if *tok == searched_token {
                let evaluated = oper(self[i - 1].value(), self[i + 1].value());
                return [
                    &self[0..i - 1],
                    [Number(evaluated)].as_slice(),
                    &self[i + 2..],
                ]
                .concat()
                .correct_eval(step_is_mul);
            }
        }
        if step_is_mul {
            self.correct_eval(false)
        } else {
            assert_eq!(self.len(), 1);
            self[0].value()
        }
    }

    fn all_eval(self, start_at: usize) -> HashSet<i32> {
        dbg!(&self);
        if self.len() == 1 {
            return HashSet::from([self[0].value()]);
        }
        let mut answers = HashSet::new();
        let oper = if self[start_at] == Multiplication {
            |a, b| a * b
        } else {
            |a, b| a + b
        };
        let first_operation_executed = oper(self[start_at - 1].value(), self[start_at + 1].value());
        if first_operation_executed >= 1000 {
            return answers;
        }
        answers.extend(
            [
                &self[0..start_at - 1],
                [Number(first_operation_executed)].as_slice(),
                &self[start_at + 2..],
            ]
            .concat()
            .all_eval(1),
        );
        if start_at + 2 < self.len() {
            answers.extend(self.all_eval(start_at + 2));
        }
        answers
    }
}

impl Solution {
    pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
        let parsed = Vec::<Token>::parse(&s);
        let good_answer = parsed.clone().correct_eval(true);
        let bad_answers = parsed.all_eval(1);
        let mut total_score = 0;
        for answer in answers {
            if answer == good_answer {
                total_score += 5;
            } else if bad_answers.contains(&answer) {
                total_score += 2;
            }
        }
        total_score
    }
}
