fn add_chars_n(s: String, c: char, n: i32) -> String {
    let mut sir = s;
    let mut i = 0;
    while i < n {
        sir.push(c);
        i += 1;
    }
    return sir;
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{s}");
}
