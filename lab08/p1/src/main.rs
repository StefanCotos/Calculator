use std::{collections::HashMap, fs};

fn aff(sir: &str, max: usize) -> String {
    let mut s = String::from(sir);
    if s.len() < max {
        for _ in s.len()..8 {
            s.push(' ');
        }
    }
    return s;
}

fn main() {
    let s = fs::read_to_string("src/text.txt").unwrap();
    let s1 = s.to_lowercase();

    let mut map = HashMap::<&str, i32>::new();

    for i in s1.split('.') {
        for j in i.split(' ') {
            if j != "" {
                map.entry(j).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }

    let mut vector = Vec::<(&&str, &i32)>::new();
    for i in &map {
        vector.push(i);
    }
    vector.sort_unstable_by(|x, y| *&y.1.cmp(*&x.1));

    let mut max = 0;
    for i in &vector {
        if i.0.len() > max {
            max = i.0.len();
        }
    }
    for i in vector {
        println!("{} => {}", aff(i.0, max), i.1);
    }
}
