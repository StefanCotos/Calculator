fn addition(x: u32, y: u32) -> u32 {
    let n = x as u64;
    let m = y as u64;
    if n + m > std::u32::MAX as u64 {
        panic!("The result of addition doesn't fit in u32");
    }
    return (m + n) as u32;
}

fn multiplication(x: u32, y: u32) -> u32 {
    let n = x as u64;
    let m = y as u64;
    if n * m > std::u32::MAX as u64 {
        panic!("The result of multiplication doesn't fit in u32");
    }
    return (m * n) as u32;
}

fn main() {
    println!("{}", addition(30000000, 10000000));
    println!("{}", multiplication(112112, 2113));
    println!("{}", addition(3000000000, 3000000000));
    println!("{}", multiplication(112112, 221113));
}
