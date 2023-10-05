//Exercitiul 1
fn primes(nr: i32) -> bool {
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

//Exercitiul2
fn coprime(mut a: i32, mut b: i32) -> bool {
    let mut r;
    while b != 0 {
        r = a % b;
        a = b;
        b = r;
    }
    if a == 1 {
        return true;
    } else {
        return false;
    }
}

fn sing() {
    let mut i = 99;
    while i > 0 {
        if i == 1 {
            println!("{} bottles of beer on the wall,", i);
            println!("{} bottle of beer.", i);
            println!("Take one down, pass it around,")
        } else {
            println!("{} bottles of beer on the wall,", i);
            println!("{} bottles of beer.", i);
            println!("Take one down, pass it around,");
            println!("{} bottles of beer on the wall,", i - 1);
            println!();
        }
        i -= 1
    }
    println!("No bottles of beer on the wall,");
    println!("No bottles of beer.");
    println!("Go to the store, buy some more,");
    println!("{} bottles of beer on the wall.", 99);
}

fn main() {
    //Ex1
    println!("Numerele prime pana la 100:");
    for i in 0..100 {
        if primes(i) == true {
            print!("{},", i)
        }
    }
    println!();

    //Ex2
    println!("Perechile de numere coprime pana la 100:");
    for i in 0..100 {
        for j in 0..100 {
            if coprime(i, j) == true {
                print!("({},{});", i, j);
            }
        }
    }
    println!();

    //Ex3
    sing()
}
