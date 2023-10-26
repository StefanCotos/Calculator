#[derive(Debug)]
enum Error {
    Overflow,
}

fn addition(x: u32, y: u32) -> Result<u32, Error> {
    let n = x as u64;
    let m = y as u64;
    if n + m > std::u32::MAX as u64 {
        return Err(Error::Overflow);
    }
    Ok((m + n) as u32)
}

fn multiplication(x: u32, y: u32) -> Result<u32, Error> {
    let n = x as u64;
    let m = y as u64;
    if n * m > std::u32::MAX as u64 {
        return Err(Error::Overflow);
    }
    Ok((m * n) as u32)
}

fn test() -> Result<u32, Error> {
    let res_add = addition(1254233, 234533)?;
    let res_mul = multiplication(15372, 28373)?;
    Ok(res_mul - res_add)
}

fn main() {
    match test() {
        Ok(x) => println!("Result:{x}"),
        Err(err) => eprintln!("{:?}", err),
    }
}
