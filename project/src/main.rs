use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Args parsing example for the calculator project:")]
struct Args {
    #[arg(short, long, default_value = "2 * (3 + 5 / 4) - (5 ^^ 2 + 8) / 11")]
    expression: String,
}

#[derive(Debug)]
enum MyErrors {
    WrongExpression,
    WrongFunction,
    WrongOperator,
    WrongToken,
    _DivByZero,
    UnclosedParenthesis,
}

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Number(f64),
    Operator(String),
    Function(String),
    LParenthesis,
    RParenthesis,
}

#[derive(Debug)]
enum Expression {
    Number(f64),
    Operations {
        op: String,
        left_expr: Box<Expression>,
        right_expr: Box<Expression>,
    },
    Functions {
        func: String,
        expr: Box<Expression>,
    },
}

fn lexing(input: String) -> Result<Vec<TokenType>, MyErrors> {
    let mut tokens = Vec::new();
    let mut elements = input.chars().peekable();

    while let Some(&c) = elements.peek() {
        match c {
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&c) = elements.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        number.push(c);
                        elements.next();
                    } else {
                        break;
                    }
                }
                tokens.push(TokenType::Number(number.parse().unwrap()));
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(TokenType::Operator(c.to_string()));
                elements.next();
            }
            '^' => {
                elements.next();
                if let Some('^') = elements.peek() {
                    tokens.push(TokenType::Operator("^^".to_string()));
                    elements.next();
                } else {
                    return Err(MyErrors::WrongOperator);
                }
            }
            '(' => {
                tokens.push(TokenType::LParenthesis);
                elements.next();
            }
            ')' => {
                tokens.push(TokenType::RParenthesis);
                elements.next();
            }
            'a'..='z' => {
                let mut func = String::new();
                while let Some(&c) = elements.peek() {
                    if c.is_ascii_alphabetic() {
                        func.push(c);
                        elements.next();
                    } else {
                        break;
                    }
                }
                if func == "sqrt"
                    || func == "log"
                    || func == "cos"
                    || func == "sin"
                    || func == "tg"
                    || func == "ctg"
                {
                    tokens.push(TokenType::Function(func));
                } else {
                    return Err(MyErrors::WrongFunction);
                }
            }
            ' ' => {
                elements.next();
            }
            _ => {
                return Err(MyErrors::WrongExpression);
            }
        }
    }

    Ok(tokens)
}

fn parse(tokens: Vec<TokenType>) -> Result<Expression, MyErrors> {
    let mut tokens_iter = tokens.into_iter().peekable();
    parse_expression(&mut tokens_iter)
}

fn parse_expression(
    tokens_iter: &mut std::iter::Peekable<std::vec::IntoIter<TokenType>>,
) -> Result<Expression, MyErrors> {
    let mut left_expr = match parse_elements(tokens_iter) {
        Ok(left_expr) => left_expr,
        Err(err) => return Err(err),
    };

    while let Some(token) = tokens_iter.peek().cloned() {
        if let TokenType::Operator(op) = token {
            let precedence = match get_precedence(&op) {
                Ok(precedence) => precedence,
                Err(err) => return Err(err),
            };
            tokens_iter.next();
            let mut right_expr = match parse_elements(tokens_iter) {
                Ok(right_expr) => right_expr,
                Err(err) => return Err(err),
            };

            while let Some(next_token) = tokens_iter.peek().cloned() {
                if let TokenType::Operator(next_op) = next_token {
                    let next_precedence = match get_precedence(&next_op) {
                        Ok(next_precedence) => next_precedence,
                        Err(err) => return Err(err),
                    };
                    if next_precedence > precedence {
                        tokens_iter.next();
                        right_expr = Expression::Operations {
                            op: next_op,
                            left_expr: Box::new(right_expr),
                            right_expr: Box::new(match parse_elements(tokens_iter) {
                                Ok(expr) => expr,
                                Err(err) => return Err(err),
                            }),
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            left_expr = Expression::Operations {
                op,
                left_expr: Box::new(left_expr),
                right_expr: Box::new(right_expr),
            };
        } else {
            break;
        }
    }

    Ok(left_expr)
}

fn parse_elements(
    tokens_iter: &mut std::iter::Peekable<std::vec::IntoIter<TokenType>>,
) -> Result<Expression, MyErrors> {
    match tokens_iter.next() {
        Some(TokenType::Number(value)) => Ok(Expression::Number(value)),
        Some(TokenType::Function(func)) => {
            let operand = match parse_elements(tokens_iter) {
                Ok(operand) => operand,
                Err(err) => return Err(err),
            };
            Ok(Expression::Functions {
                func,
                expr: Box::new(operand),
            })
        }
        Some(TokenType::LParenthesis) => {
            let expr = match parse_expression(tokens_iter) {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            if let Some(TokenType::RParenthesis) = tokens_iter.next() {
                Ok(expr)
            } else {
                Err(MyErrors::UnclosedParenthesis)
            }
        }
        _ => Err(MyErrors::WrongToken),
    }
}

fn get_precedence(op: &str) -> Result<u8, MyErrors> {
    match op {
        "+" | "-" => Ok(1),
        "*" | "/" => Ok(2),
        "^^" => Ok(3),
        _ => Err(MyErrors::WrongOperator),
    }
}

fn main() {
    let input = Args::parse();
    let tokens = lexing(input.expression);

    match tokens {
        Ok(tokens) => {
            println!("Tokens: {:?}", tokens);
            let parsed_expr = parse(tokens);
            match parsed_expr {
                Ok(parsed_expr) => {
                    println!("Parsed Expression: {:?}", parsed_expr);
                }
                Err(err) => eprint!("{:?}",err),
            }
        }
        Err(err) => eprintln!("{:?}", err),
    }
}
