use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;

struct Cache {
    data: RefCell<HashMap<u64, bool>>,
    ord: RefCell<Vec<u64>>,
    size: usize,
}

impl Cache {
    fn new(size: usize) -> Self {
        Cache {
            data: RefCell::new(HashMap::new()),
            ord: RefCell::new(Vec::with_capacity(size)),
            size,
        }
    }

    fn get(&self, val: u64) -> Option<bool> {
        let data = self.data.borrow_mut();

        if let Some(&value) = data.get(&val) {
            Some(value)
        } else {
            None
        }
    }

    fn insert(&self, val: u64, value: bool) {
        let mut ord = self.ord.borrow_mut();
        let mut data = self.data.borrow_mut();

        if ord.len() >= self.size {
            if let Some(oldest) = ord.pop() {
                data.remove(&oldest);
            }
        }
        data.insert(val, value);
        ord.insert(0, val);
    }
}

fn primes(nr: u64) -> bool {
    let mut d = 1;
    let mut nr_div = 0;
    while d <= nr {
        if nr % d == 0 {
            nr_div += 1
        }
        d += 1
    }
    if nr_div == 2 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let cache = Cache::new(10);

    loop {
        println!("Number (or 'quit'): ");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read");

        if input.trim() == "quit" {
            break;
        }
        if let Ok(num) = input.trim().parse() {
            if let Some(result) = cache.get(num) {
                match result{
                    true => println!("Number is prime.(from cache)"),
                    false => println!("Number is not prime.(from cache)"),
                }
            } else {
                let result = primes(num);
                match result{
                    true => println!("Number is prime."),
                    false => println!("Number is not prime."),
                }
                cache.insert(num, result);
            }
        } else {
            println!("Invalid input.");
        }
    }
}
