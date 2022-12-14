use std::cmp::Ordering;

use itertools::{
    EitherOrBoth::{Both, Left, Right},
    Itertools,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Int(usize),
    List(Vec<Value>),
}

pub enum Token {
    Open,
    Close,
    Comma,
    Int(usize),
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            '[' => tokens.push(Token::Open),
            ']' => tokens.push(Token::Close),
            ',' => tokens.push(Token::Comma),
            _ => {
                let mut value = c.to_string();
                while let Some(&c) = chars.peek() {
                    if !c.is_numeric() {
                        break;
                    }
                    value.push(chars.next().unwrap());
                }
                tokens.push(Token::Int(value.parse().unwrap()));
            }
        }
    }

    tokens
}

impl Value {
    pub fn parse(input: &str) -> Self {
        let tokens = tokenize(input);
        let mut tokens = tokens.iter();

        match tokens.next() {
            Some(Token::Open) => Self::parse_list(&mut tokens),
            Some(Token::Int(value)) => Self::Int(*value),
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_list<'a, I>(tokens: &mut I) -> Self
    where
        I: Iterator<Item = &'a Token>,
    {
        let mut list = Vec::new();

        while let Some(token) = tokens.next() {
            match token {
                Token::Int(value) => list.push(Value::Int(*value)),
                Token::Open => list.push(Value::parse_list(tokens)),
                Token::Comma => continue,
                Token::Close => break,
            }
        }

        Value::List(list)
    }
}

pub fn parse_pairs(input: &str) -> Vec<(Value, Value)> {
    input
        .split("\n\n")
        .map(|pair| {
            let (pair1, pair2) = pair.split_once('\n').unwrap();
            let pair1 = Value::parse(pair1);
            let pair2 = Value::parse(pair2);
            (pair1, pair2)
        })
        .collect()
}

pub fn compare_pair(pair: &(Value, Value)) -> Ordering {
    match pair {
        (Value::Int(a), Value::Int(b)) => a.cmp(b),
        (Value::List(a), Value::List(b)) => {
            for subpair in a.iter().zip_longest(b.iter()) {
                match subpair {
                    Both(a, b) => match compare_pair(&(a.clone(), b.clone())) {
                        Ordering::Equal => continue,
                        order => return order,
                    },
                    Right(_) => return Ordering::Less,
                    Left(_) => return Ordering::Greater,
                }
            }
            Ordering::Equal
        }
        (a, Value::List(_)) => compare_pair(&(Value::List(vec![a.clone()]), pair.1.clone())),
        (Value::List(_), b) => compare_pair(&(pair.0.clone(), Value::List(vec![b.clone()]))),
    }
}

pub fn count_ordered_pairs(input: &str) -> usize {
    let pairs = parse_pairs(input);
    pairs
        .iter()
        .enumerate()
        .filter(|&(_, pair)| {
            // println!("{:?} {:?}", pair, are_ordered(pair));
            compare_pair(pair) == Ordering::Less
        })
        .map(|(i, _)| i + 1)
        .sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parse_pair() {
        let value = Value::parse("[1,2]");
        assert_eq!(value, Value::List(vec![Value::Int(1), Value::Int(2)]));
    }

    #[test]
    fn test_parse_nested() {
        let value = Value::parse("[1,[2,3]]");
        assert_eq!(
            value,
            Value::List(vec![
                Value::Int(1),
                Value::List(vec![Value::Int(2), Value::Int(3)])
            ])
        );
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_ordered_pairs(input), 13);
    }
}
