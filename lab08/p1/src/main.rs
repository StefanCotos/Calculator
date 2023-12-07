use std::{collections::HashMap, fs};

fn aff(sir: String, max: usize) -> String {
    let mut s = String::from(sir);
    if s.len() < max {
        for _ in s.len()..8 {
            s.push(' ');
        }
    }
    return s;
}

fn punctuation(cuv: &str) -> String {
    let mut new_cuv = String::new();
    for i in cuv.chars() {
        if i.is_ascii_alphanumeric() {
            new_cuv.push(i);
        }
    }
    return new_cuv;
}

fn main() {
    let s = fs::read_to_string("src/text.txt").unwrap();
    let s1 = s.to_lowercase();
    let mut _j = String::new();
    let mut map = HashMap::<String, i32>::new();

    for i in s1.split(' ') {
        _j = punctuation(i);
        if i != "" {
            map.entry(_j).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    let mut vector: Vec<_> = map.into_iter().collect();
    vector.sort_by(|x, y| y.1.cmp(&x.1));

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
