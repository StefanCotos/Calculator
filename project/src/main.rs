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
}

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Number(f64),
    Operator(String),
    Function(String),
    OpenParenthesis,
    CloseParenthesis,
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
                if let Some(&'^') = elements.peek() {
                    tokens.push(TokenType::Operator("^^".to_string()));
                    elements.next();
                } else {
                    return Err(MyErrors::WrongOperator);
                }
            }
            '(' => {
                tokens.push(TokenType::OpenParenthesis);
                elements.next();
            }
            ')' => {
                tokens.push(TokenType::CloseParenthesis);
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

fn main() {
    let input = Args::parse();
    let tokens = lexing(input.expression);

    match tokens {
        Ok(tokens) => println!("Tokens: {:?}", tokens),
        Err(err) => eprintln!("{:?}", err),
    }
}
