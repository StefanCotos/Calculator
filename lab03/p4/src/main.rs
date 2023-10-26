use std::char;

#[derive(Debug)]
enum Errors {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(ch: char) -> Result<char, Errors> {
    if (ch < 'a' || ch > 'z') && (ch < 'A' || ch > 'Z') {
        return Err(Errors::NotLetter);
    }
    if ch >= 'A' && ch <= 'Z' {
        return Ok(ch);
    }
    let ch_upp = ch as u8 - 32;
    Ok(ch_upp as char)
}

fn to_lowercase(ch: char) -> Result<char, Errors> {
    if (ch < 'a' || ch > 'z') && (ch < 'A' || ch > 'Z') {
        return Err(Errors::NotLetter);
    }
    if ch >= 'a' && ch <= 'z' {
        return Ok(ch);
    }
    let ch_low = ch as u8 + 32;
    Ok(ch_low as char)
}

fn print_char(ch: char) -> Result<char, Errors> {
    if !ch.is_ascii_graphic() {
        return Err(Errors::NotPrintable);
    }
    Ok(ch)
}

fn char_to_number(ch: char) -> Result<u32, Errors> {
    if !ch.is_ascii() {
        return Err(Errors::NotAscii);
    }
    if ch < '0' || ch > '9' {
        return Err(Errors::NotDigit);
    }
    //let ch_num=ch as u8 - b'0';
    let ch_num = ch.to_digit(10).unwrap();
    Ok(ch_num as u32)
}

fn char_to_number_hex(ch: char) -> Result<u32, Errors> {
    if !ch.is_ascii() {
        return Err(Errors::NotAscii);
    }
    if (ch < '0' || ch > '9') && (ch < 'A' || ch > 'F') {
        return Err(Errors::NotBase16Digit);
    }
    let ch_hexnum = ch.to_digit(16).unwrap();
    Ok(ch_hexnum)
}

fn print_error(err: Errors) {
    match err {
        Errors::NotAscii => eprintln!("Input character is not ASCII!"),
        Errors::NotPrintable => eprintln!("Input character is not printable!"),
        Errors::NotLetter => eprintln!("Input character is not a letter!"),
        Errors::NotDigit => eprintln!("Input character is not a digit!"),
        Errors::NotBase16Digit => eprintln!("Input character is not a base16 digit!"),
    }
}

fn main() {
    match to_uppercase('1') {
        Ok(x) => println!("Letter uppercase:{x}"),
        Err(err) => print_error(err),
    }
    match to_lowercase('Z') {
        Ok(x) => println!("Letter lowercase:{x}"),
        Err(err) => print_error(err),
    }
    match print_char('\0') {
        Ok(x) => println!("{x} is printable"),
        Err(err) => print_error(err),
    }
    match char_to_number('4') {
        Ok(x) => println!("Digit:{x}"),
        Err(err) => print_error(err),
    }
    match char_to_number_hex('E') {
        Ok(x) => println!("Base16 Digit:{x}"),
        Err(err) => print_error(err),
    }
}
