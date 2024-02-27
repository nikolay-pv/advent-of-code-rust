use std::{iter::zip, str::Chars};

const INPUT_TXT: &str = include_str!("input.txt");

fn main() {
    println!("Answer to first parts is {}", solve_first(&INPUT_TXT));
    println!("Answer to second parts is {}", solve_second(&INPUT_TXT));
}

fn solve_first(input: &str) -> i32 {
    zip(
        input.lines().into_iter().step_by(3),
        input.lines().into_iter().skip(1).step_by(3),
    ).enumerate().map(|(i, (left, right))|
        if less(left, right) {
            (i + 1) as i32
        } else {
            0
    }).sum()
}

fn solve_second(input: &str) -> i32 {
    input.len() as i32
}

fn less(left: &str, right: &str) -> bool {
    let mut lhs = Tokenizer::new(left);
    let mut rhs = Tokenizer::new(right);

    let mut l = lhs.next();
    let mut r = rhs.next();
    let mut close_left = false;
    let mut close_right = false;
    loop {
        if l == None {
            assert!(l != r);
            return true;
        }
        if r == None {
            return false;
        }
        match (l.unwrap(), r.unwrap()) {
            (Token::Comma, Token::Digit(_)) =>  { unreachable!(); },
            (Token::Comma, Token::ListStart) => { unreachable!(); },
            (Token::Digit(_), Token::Comma) =>  { unreachable!(); },
            (Token::ListStart, Token::Comma) => { unreachable!(); },
            // assuming a list
            (Token::ListStart, Token::Digit(_)) => {
                l = lhs.next();
                close_right = true;
            },
            (Token::Digit(_), Token::ListStart) => {
                r = rhs.next();
                close_left = true;
            },
            // double check that list doesn't need closing (edge case when it is empty)
            (Token::ListEnd, Token::Comma) => {
                if close_right {
                    l = lhs.next();
                    close_right = false;
                    continue;
                }
                return true;
            },
            (Token::Comma, Token::ListEnd) => {
                if close_left {
                    r = rhs.next();
                    close_left = false;
                    continue;
                }
                return false;
            },
            // one list is shorter than the other
            (Token::ListEnd, Token::Digit(_)) => { return true; },
            (Token::ListEnd, Token::ListStart) => { return true; },
            (Token::Digit(_), Token::ListEnd) => { return false; },
            (Token::ListStart, Token::ListEnd) => { return false; },
            // normal case
            (Token::Digit(a), Token::Digit(b)) => { 
                if a != b { 
                    return a < b;
                }
                if close_left {
                    l = Some(Token::ListEnd);
                    r = rhs.next();
                    close_left = false;
                } else if close_right {
                    l = lhs.next();
                    r = Some(Token::ListEnd);
                    close_right = false;
                } else {
                    l = lhs.next();
                    r = rhs.next();
                }
            },
            // (Token::ListStart, Token::ListStart) =>
            // (Token::ListEnd, Token::ListEnd) =>
            // (Token::Comma, Token::Comma) =>
            _ => {
                l = lhs.next();
                r = rhs.next();
            },
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    ListStart,
    ListEnd,
    Comma,
    Digit(i32),
}

struct Tokenizer<'a> {
    input: &'a str,
    it: Chars<'a>,
    read_count: usize,
}

impl Tokenizer<'_> {
    fn new(input: &str) -> Tokenizer<'_> {
        Tokenizer {
            input,
            it: input.chars().into_iter(),
            read_count: 0,
        }
    }

    fn next(&mut self) -> Option<Token> {
        while let Some(c) = self.it.next() {
            self.read_count += 1;
            return match c {
                ']' => Some(Token::ListEnd),
                '[' => Some(Token::ListStart),
                ',' => Some(Token::Comma),
                '0'..='9' => 
                { 
                    let mut number = c as i32 - '0' as i32;
                    while let Some(a) = self.it.next() {
                        self.read_count += 1;
                        match a {
                            '0'..='9' => { number = number * 10 + (a as i32 - '0' as i32); },
                            _ => {
                                self.read_count -= 1;
                                self.it = self.input.chars().into_iter();
                                self.it.nth(self.read_count - 1);
                                break;
                            },
                        }
                    }
                    Some(Token::Digit(number))
                },
                _ => panic!("Unknown token in input"),
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    const TEST_INPUT_TXT: &str = include_str!("input_test.txt");

    #[test]
    fn less_test() {
        assert_eq!(less(
            "[[1,[8]]",
            "[[10,3,8]]"), true);
        assert_eq!(less(
            "[[[5,[10,4]]]]",
            "[[[[]],[[8]]]"), false);
        assert_eq!(less(
            "[[1],2,4]",
            "[[1],[2],3]"), false);
    }

    #[test]
    fn part1() {
        assert_eq!(solve_first(&TEST_INPUT_TXT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_second(&TEST_INPUT_TXT), 185);
    }
}

