fn is_prime(value: u16) -> bool {
    if value < 2 {
        return false;
    } else if value > 2 && value % 2 == 0 {
        return false;
    } else {
        let mut i = 3;
        while i * i <= value {
            if value % i == 0 {
                return false;
            }
            i += 2;
        }
    }
    true
}

// #[allow(unused)]
fn next_prime(mut x: u32) -> Option<u16> {
    while !is_prime(x) {
        if x > std::u16::MAX {
            return None;
        }
        x += 1 as u16;
    }
    Some(x)
}

fn main() {
    let mut value: u16 = 65500;
    // let mut r: u16 = 1;
    // while r != 0 {
    //     r = next_prime(value).unwrap_or(0);
    //     println!("We got the next_prime number: {r}");
    //     value += 1;
    // }
    let res =next_prime(value);
    println!("We got the next_prime number: {}", res);
}
