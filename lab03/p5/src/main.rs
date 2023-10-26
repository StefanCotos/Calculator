use thiserror::Error;
#[derive(Error, Debug)]

enum Error {
    #[error("The number is negative!")]
    NegativeNumber,
}

fn sub(x: u32, y: u32) -> Result<u32, Error> {
    if y > x {
        return Err(Error::NegativeNumber);
    }
    Ok(x - y)
}

fn div(x: u32, y: u32) -> Option<u32> {
    if y == 0 {
        return None;
    }
    Some(x / y)
}

fn main() {
    match sub(0, 50) {
        Ok(x) => println!("Value:{x}"),
        Err(err) => eprintln!("{}", err),
    }
    match div(10, 0) {
        Some(x) => println!("Value:{x}"),
        None => println!("Error. Division by zero!"),
    }
}
