use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

#[derive(Debug)]
struct InvalidExpressionError;

#[derive(Debug, PartialEq)]
enum Token {
    Number(isize),
    Addition,
    Multiplication,
    LeftParenthesis,
}

#[derive(Debug, PartialEq)]
struct Expression {
    advanced: bool,
    precedence: Vec<Vec<Token>>,
    tokens: Vec<Token>,
}

impl Expression {
    fn new(advanced: bool) -> Self {
        if !advanced {
            Expression {
                advanced,
                precedence: vec![vec![Token::Addition,Token::Multiplication]],
                tokens: vec![],
            }
        }
        else {
            Expression {
                advanced,
                precedence: vec![vec![Token::Multiplication],vec![Token::Addition]],
                tokens: vec![],
            }
        }
    }

    fn compare_precedence(&self, left: &Token, right: &Token) -> Ordering {
        // TODO: probably should do caching
        let left_idx = self.precedence.iter().enumerate().find_map(|(idx, vec)| if vec.contains(left) { Some(idx) } else { None }).unwrap();
        let right_idx = self.precedence.iter().enumerate().find_map(|(idx, vec)| if vec.contains(right) { Some(idx) } else { None }).unwrap();
        left_idx.cmp(&right_idx)
    }

    fn from_line(line: &str, advanced: bool) -> Result<Self, InvalidExpressionError>
    {
        let mut result = Expression::new(advanced);

        // Cheat and ensure every token is separated by whitespace
        let corrected = line.replace("(", "( ").replace(")", " )");
        let mut tokens = corrected.split_ascii_whitespace();
        let mut operstack: Vec<Token> = vec![];
    
        // Shunting Yard algorithm by Dijkstra
        loop {
            let t = tokens.next();
            match t {
                Some(raw_oper) if raw_oper == "+" || raw_oper == "*" => {
                    let oper = match raw_oper {
                        "+" => Token::Addition,
                        "*" => Token::Multiplication,
                        _ => unreachable!(),
                    };
                    while operstack.len() > 0 {
                        let last = operstack.last().unwrap();
                        if *last == Token::LeftParenthesis {
                            break;
                        }
                        match result.compare_precedence(last, &oper) {
                            Ordering::Less => break,
                            // We only have left-associative operators + and * so both greater and equal cases are the same
                            _ => {
                                let last = operstack.pop().unwrap();
                                result.tokens.push(last);
                            }
                        }
                    }
                    operstack.push(oper);
                },
                Some("(") => operstack.push(Token::LeftParenthesis),
                Some(")") => {
                    while operstack.len() > 0 && operstack.last().unwrap() != &Token::LeftParenthesis {
                        let last = operstack.pop().unwrap();
                        result.tokens.push(last);
                    }
                    if operstack.len() == 0 {
                        return Err(InvalidExpressionError);
                    }
                    if operstack.last().unwrap() == &Token::LeftParenthesis {
                        operstack.pop();
                    }
                },
                Some(raw_number) => {
                    if let Ok(number) = raw_number.parse::<isize>() {
                        result.tokens.push(Token::Number(number));
                    } else {
                        return Err(InvalidExpressionError);
                    }
                },
                None => break,
            };
        }

        while operstack.len() > 0 {
            let last = operstack.pop().unwrap();
            if last == Token::LeftParenthesis {
                return Err(InvalidExpressionError);
            }
            result.tokens.push(last);
        }

        Ok(result)
    }
    
    fn calculate(&self) -> isize {
        let mut operands: Vec<isize> = vec![];
        let mut generator = self.tokens.iter();
        loop {
            let t = generator.next();
            match t {
                Some(Token::Number(val)) => {
                    operands.push(*val);
                },
                Some(Token::Addition) => {
                    let left_oper = operands.pop().unwrap();
                    let right_oper = operands.pop().unwrap();
                    // Cannot inline the double pop due to the fact that operands cannot be borrowed twice in the same call
                    operands.push(left_oper + right_oper);
                },
                Some(Token::Multiplication) => {
                    let left_oper = operands.pop().unwrap();
                    let right_oper = operands.pop().unwrap();
                    // Cannot inline the double pop due to the fact that operands cannot be borrowed twice in the same call
                    operands.push(left_oper * right_oper);
                },
                Some(_) => unreachable!(),
                None => break,
            }
        }

        assert_eq!(operands.len(), 1);
        operands[0]
    }
}

fn sum_expressions(expressions: &Vec<Expression>) -> isize {
    expressions.iter().fold(0, |s, x| s + x.calculate())
}


fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let expressions: Vec<Expression> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .map(|x| Expression::from_line(&x, false).expect("Invalid operation in input file"))
        .collect();

    let ans = sum_expressions(&expressions);
    println!("Star one: {}", ans);

    let file = File::open("./input").expect("Unreadable input file ./input");
    let expressions: Vec<Expression> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .map(|x| Expression::from_line(&x, true).expect("Invalid operation in input file"))
        .collect();

    let ans = sum_expressions(&expressions);
    println!("Star two: {}", ans);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_star_one() {
        let expressions: Vec<super::Expression> = TEST_DATA
            .lines()
            .map(|x| super::Expression::from_line(&x, false).expect("Invalid operation in test data"))
            .collect();

        assert_eq!(expressions[0].tokens[0], super::Token::Number(1));
        assert_eq!(expressions[0].tokens[1], super::Token::Number(2));
        assert_eq!(expressions[0].tokens[2], super::Token::Addition);
        assert_eq!(expressions[0].tokens[3], super::Token::Number(3));
        assert_eq!(expressions[0].tokens[4], super::Token::Multiplication);
        assert_eq!(expressions[0].calculate(), 71);
        assert_eq!(expressions[1].calculate(), 51);

        let ans = super::sum_expressions(&expressions);
        assert_eq!(ans, 71 + 51 + 26 + 437 + 12240 + 13632);
    }

    #[test]
    fn test_star_two() {
        let expressions: Vec<super::Expression> = TEST_DATA
            .lines()
            .map(|x| super::Expression::from_line(&x, true).expect("Invalid operation in test data"))
            .collect();

        assert_eq!(expressions[0].calculate(), 231);
        assert_eq!(expressions[1].calculate(), 51);
        assert_eq!(expressions[2].calculate(), 46);

        let ans = super::sum_expressions(&expressions);
        assert_eq!(ans, 231 + 51 + 46 + 1445 + 669060 + 23340);
    }
}
