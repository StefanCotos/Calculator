use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/text.txt")?;

    let mut max = 0;
    let mut line_max = "";
    for i in s.lines() {
        if i.len() > max {
            max = i.len();
            line_max = i;
        }
    }
    println!("The longest line considering the number of bytes:\n {line_max}");

    max = 0;
    let mut line = String::new();
    let mut max_line = String::new();
    let mut k = 0;
    for i in s.chars() {
        if i != '\n' {
            k += 1;
            line.push(i);
        } else {
            if k > max {
                max = k;
                max_line.push_str(line.as_str());
            }
            k = 0;
            line.clear();
        }
    }
    if k > max {
        max_line.push_str(line.as_str());
    }
    println!("The longest line considering the number of actual characters:\n {max_line}");

    Ok(())
}
