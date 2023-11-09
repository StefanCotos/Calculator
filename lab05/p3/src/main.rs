use serde_derive::Deserialize;
use std::{fs, io};

#[derive(Debug)]
enum Error {
    NotSpecificPhoneNumber,
}

#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn students(p: &[Student; 4]) -> Result<(&Student, &Student), Error> {
    let mut min = 1000;
    let mut max = 0;
    let mut s1 = 0;
    let mut s2 = 0;
    for i in 0..p.len() {
        if p[i].phone.len() != 10 {
            return Err(Error::NotSpecificPhoneNumber);
        }
        if p[i].age > max {
            max = p[i].age;
            s1 = i;
        }
        if p[i].age < min {
            min = p[i].age;
            s2 = i;
        }
    }

    Ok((&p[s1], &p[s2]))
}

fn main() -> Result<(), io::Error> {
    let content: String = fs::read_to_string("src/students.json")?;
    let p: [Student; 4] = serde_json::from_str(&content).unwrap();
    match students(&p) {
        Ok(students) => {
            println!(
                "The oldest student: {} {} {}",
                students.0.name, students.0.phone, students.0.age
            );
            println!(
                "The youngest student: {} {} {}",
                students.1.name, students.1.phone, students.1.age
            );
        }
        Err(err) => eprintln!("{:?}", err),
    }

    Ok(())
}
