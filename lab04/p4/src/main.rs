use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/hosts")?;
    let mut s1 = String::new();
    let mut s2 = String::new();

    let mut k;
    let mut ok = false;
    let mut verif;
    for i in s.lines() {
        k = 0;
        verif = false;
        if i.starts_with("#") {
            verif = true;
        } else {
            for j in i.chars() {
                if j != ' ' {
                    ok = false;
                    if k == 0 {
                        s1.push(j);
                    } else if k == 1 {
                        s2.push(j);
                    } else if k == 2 {
                        break;
                    }
                } else {
                    if ok == false {
                        k += 1;
                        ok = true;
                    }
                }
            }
        }
        if verif != true {
            println!("{s2} => {s1}");
        }
        s1.clear();
        s2.clear();
    }
    Ok(())
}
