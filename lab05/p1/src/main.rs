use std::{fs, io};

#[derive(Debug)]
enum Error {
    NotSpecificPhoneNumber,
}

#[derive(Debug, Copy, Clone)]
struct Student<'a> {
    name: &'a str,
    phone: &'a str,
    age: i32,
}

fn students(s: &String) -> Result<(Student, Student), Error> {
    let mut k;
    let mut student: [Student<'_>; 2] = [Student {
        name: "",
        phone: "",
        age: 0,
    }; 2];

    let mut min = 1000;
    let mut max = 0;
    for i in s.lines() {
        k = 0;
        let mut name = "";
        let mut phone = "";
        let mut age;
        let mut number = 0;
        for j in i.split(',') {
            k += 1;
            if k == 1 {
                name = j;
            } else if k == 2 {
                phone = j;
                if phone.len() != 10 {
                    return Err(Error::NotSpecificPhoneNumber);
                }
            } else if k == 3 {
                for a in j.chars() {
                    number = number * 10 + a.to_digit(10).unwrap();
                }
                age = number as i32;
                if age > max {
                    max = age;
                    student[0].name = name;
                    student[0].phone = phone;
                    student[0].age = age;
                }
                if age < min {
                    min = age;
                    student[1].name = name;
                    student[1].phone = phone;
                    student[1].age = age;
                }
            }
        }
    }
    Ok((student[0], student[1]))
}

fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("src/text.txt")?;

    match students(&s) {
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
