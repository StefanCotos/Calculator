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
    DivByZero,
    UnclosedParenthesis,
    UnopenedParenthesis,
    EmptyParenthesis,
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
                    if c.is_ascii_alphabetic() || c == '(' {
                        func.push(c);
                        if c == '(' {
                            break;
                        }
                        elements.next();
                    } else {
                        break;
                    }
                }
                if func == "sqrt("
                    || func == "ln("
                    || func == "cos("
                    || func == "sin("
                    || func == "tg("
                    || func == "ctg("
                {
                    func.pop();
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

fn parsing(tokens: Vec<TokenType>) -> Result<Expression, MyErrors> {
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
            if let Some(TokenType::RParenthesis) = tokens_iter.peek() {
                Err(MyErrors::EmptyParenthesis)
            } else {
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
        }
        Some(TokenType::RParenthesis) => Err(MyErrors::UnopenedParenthesis),
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

fn res_print_op(result: f64, left_value: f64, right_value: f64, op: &str, input: &mut String) {
    let op_sqrt = format!("sqrt({}{}{})", left_value, op, right_value);
    let op_ln = format!("ln({}{}{})", left_value, op, right_value);
    let op_cos = format!("cos({}{}{})", left_value, op, right_value);
    let op_sin = format!("sin({}{}{})", left_value, op, right_value);
    let op_tg = format!("tg({}{}{})", left_value, op, right_value);
    let op_ctg = format!("ctg({}{}{})", left_value, op, right_value);
    let op_str = format!("({}{}{})", left_value, op, right_value);
    let op_str0 = format!("{}{}{}", left_value, op, right_value);
    let res_str = format!("({})", result);
    let res_str2 = format!("{}", result);
    if input.contains(&op_sqrt)
        || input.contains(&op_ln)
        || input.contains(&op_cos)
        || input.contains(&op_sin)
        || input.contains(&op_tg)
        || input.contains(&op_ctg)
    {
        *input = input.replacen(&op_str, &res_str, 1);
    } else if input.contains(&op_str) {
        *input = input.replacen(&op_str, &res_str2, 1);
    } else if input.contains(&op_str0) {
        *input = input.replacen(&op_str0, &res_str2, 1);
    }
    println!("= {input}");
}

fn res_print_func(result: f64, operand_value: f64, func: &str, input: &mut String) {
    let op_str = format!("{}({})", func, operand_value);
    let res_str = format!("{}", result);
    *input = input.replacen(&op_str, res_str.as_str(), 1);
    println!("={input}");
}

fn resolving(expr: &Expression, input: &mut String) -> Result<f64, MyErrors> {
    match expr {
        Expression::Number(value) => Ok(*value),
        Expression::Operations {
            op,
            left_expr,
            right_expr,
        } => {
            let left_value = match resolving(left_expr, input) {
                Ok(left_value) => left_value,
                Err(err) => return Err(err),
            };
            let right_value = match resolving(right_expr, input) {
                Ok(right_value) => right_value,
                Err(err) => return Err(err),
            };

            match op.as_str() {
                "+" | "-" | "*" | "/" | "^^" => {
                    let mut result = 0.0;
                    if op.as_str() == "+" {
                        result = left_value + right_value;
                    } else if op.as_str() == "-" {
                        result = left_value - right_value;
                    } else if op.as_str() == "*" {
                        result = left_value * right_value;
                    } else if op.as_str() == "/" {
                        if right_value == 0.0 {
                            return Err(MyErrors::DivByZero);
                        }
                        result = left_value / right_value;
                    } else if op.as_str() == "^^" {
                        result = left_value.powf(right_value);
                    };

                    res_print_op(result, left_value, right_value, op, input);
                    Ok(result)
                }
                _ => Err(MyErrors::WrongOperator),
            }
        }
        Expression::Functions { func, expr } => {
            let operand_value = match resolving(expr, input) {
                Ok(operand_value) => operand_value,
                Err(err) => return Err(err),
            };

            match func.as_str() {
                "sqrt" | "ln" | "cos" | "sin" | "tg" | "ctg" => {
                    let mut result = 0.0;
                    if func.as_str() == "sqrt" {
                        result = operand_value.sqrt();
                    } else if func.as_str() == "ln" {
                        result = operand_value.ln();
                    } else if func.as_str() == "cos" {
                        result = operand_value.cos();
                    } else if func.as_str() == "sin" {
                        result = operand_value.sin();
                    } else if func.as_str() == "tg" {
                        result = operand_value.tan();
                    } else if func.as_str() == "ctg" {
                        if operand_value.tan() == 0.0 {
                            return Err(MyErrors::DivByZero);
                        }
                        result = 1.0 / operand_value.tan();
                    };

                    res_print_func(result, operand_value, func, input);
                    Ok(result)
                }
                _ => Err(MyErrors::WrongFunction),
            }
        }
    }
}

fn main() {
    let input = Args::parse();
    let mut input0 = Args::parse().expression.replace(' ', "");
    let tokens = lexing(input.expression);

    match tokens {
        Ok(tokens) => {
            let parsed_expr = parsing(tokens);
            match parsed_expr {
                Ok(parsed_expr) => {
                    println!("Expression: {}", Args::parse().expression);
                    let result = resolving(&parsed_expr, &mut input0);
                    match result {
                        Ok(result) => println!("Result: {}", result),
                        Err(err) => eprint!("{:?}", err),
                    }
                }
                Err(err) => eprint!("{:?}", err),
            }
        }
        Err(err) => eprintln!("{:?}", err),
    }
}
