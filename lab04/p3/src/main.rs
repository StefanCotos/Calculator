use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let mut s = fs::read_to_string("src/text.txt")?;

    s = s.replace("dl", "domnul");
    s = s.replace("dna", "doamna");
    s = s.replace("ptr", "pentru");
    s = s.replace("pt", "pentru");

    //or
    //let mut s1 = String::new();
    /*for i in s.split(" ") {
        if i == "dl" {
            s1.push_str("domnul");
            s1.push(' ');
        } else if i == "dna" {
            s1.push_str("doamna");
            s1.push(' ');
        } else if i == "ptr" {
            s1.push_str("pentru");
            s1.push(' ');
        } else if i == "pt" {
            s1.push_str("pentru");
            s1.push(' ');
        } else {
            s1.push_str(i);
            s1.push(' ');
        }
    }*/
    println!("{s}");
    //fs::write("src/text.txt", &s1)?;
    Ok(())
}
