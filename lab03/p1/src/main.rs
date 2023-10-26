fn next_prime(x: u16) -> Option<u16> {
    let mut d;
    let mut nr_div;
    let mut flag = false;
    let mut y = x as u32;
    y += 1;
    while flag == false {
        d = 1;
        nr_div = 0;
        while d <= y {
            if y % d == 0 {
                nr_div += 1
            }
            d += 1
        }
        if nr_div == 2 {
            flag = true;
        } else {
            y += 1;
            if y > std::u16::MAX as u32 {
                return None;
            }
        }
    }
    Some(y as u16)
}

fn main() {
    let mut x = 2;
    while let Some(i) = next_prime(x) {
        println!("Urmatorul numar prim dupa {x} este {i}");
        x = i;
    }
    println!("Functia a returnat None")
}
