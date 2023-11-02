fn rot13(s: String) -> Option<String> {
    let mut j;
    let mut new_s = String::new();
    for i in s.chars() {
        if i.is_ascii() {
            j = (i as u8) - 13;
            if i >= 'A' && i <= 'Z' {
                if j < 65 {
                    j = 90 - (64 - j);
                }
                new_s.push(j as char);
            } else if i >= 'a' && i <= 'z' {
                if j < 97 {
                    j = 122 - (96 - j);
                }
                new_s.push(j as char);
            }
        } else {
            return None;
        }
    }
    Some(new_s)
}

fn main() {
    let s = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
    //let s1 = String::from("ABCD🙃abcd");
    println!("String : {}", s);
    print!("String after ROT13 : ");
    match rot13(s) {
        Some(new_s) => println!("{}", new_s),
        None => println!("Error: A non-ASCII character in string!"),
    }
}
