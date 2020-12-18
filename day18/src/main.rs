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
    Parenthesis(Expression),
}

#[derive(Debug, PartialEq)]
struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
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
                Some(Token::Parenthesis(expr)) => {
                    operands.push(expr.calculate());
                },
                None => break,
            }
        }

        assert_eq!(operands.len(), 1);
        operands[0]
    }
}

fn parse_sub_expression<'a, I>(tokens: &mut I) -> Result<Expression, InvalidExpressionError>
where
    I: Iterator<Item = &'a str>,
{
    let mut expr = Expression { tokens: vec![] };
    let mut oper: Option<Token> = None;

    loop {
        let t = tokens.next();
        match t {
            Some("+") => oper = Some(Token::Addition),
            Some("*") => oper = Some(Token::Multiplication),
            Some("(") => {
                let nested = parse_sub_expression(tokens)?;
                expr.tokens.push(Token::Parenthesis(nested));
                if let Some(op) = oper {
                    expr.tokens.push(op);
                    oper = None;
                }
            },
            Some(")") => {
                return Ok(expr);
            },
            Some(nr) => {
                if let Ok(number) = nr.parse::<isize>() {
                    expr.tokens.push(Token::Number(number));
                    if let Some(op) = oper {
                        expr.tokens.push(op);
                        oper = None;
                    }
                } else {
                    return Err(InvalidExpressionError);
                }
            },
            None => return Ok(expr),
        };
    }
}

fn parse_expression(line: &str) -> Result<Expression, InvalidExpressionError> {
    // Cheat and ensure every token is separated by whitespace
    let corrected = line.replace("(", "( ").replace(")", " )");
    let mut iter = corrected.split_ascii_whitespace();
    parse_sub_expression(&mut iter)
}

fn star_one(expressions: &Vec<Expression>) -> isize {
    expressions.iter().fold(0, |s, x| s + x.calculate())
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let expressions: Vec<Expression> = io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Could not read line"))
        .map(|x| parse_expression(&x).expect("Invalid operation in input file"))
        .collect();

    let ans = star_one(&expressions);
    println!("Star one: {}", ans);
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
            .map(|x| super::parse_expression(x).expect("Invalid operation in input file"))
            .collect();

        assert_eq!(expressions[0].tokens[0], super::Token::Number(1));
        assert_eq!(expressions[0].tokens[1], super::Token::Number(2));
        assert_eq!(expressions[0].tokens[2], super::Token::Addition);
        assert_eq!(expressions[0].tokens[3], super::Token::Number(3));
        assert_eq!(expressions[0].tokens[4], super::Token::Multiplication);
        assert_eq!(expressions[0].calculate(), 71);
        assert_eq!(expressions[1].calculate(), 51);

        let ans = super::star_one(&expressions);
        assert_eq!(ans, 71 + 51 + 26 + 437 + 12240 + 13632);
    }
}
