fn is_prime(value: i32) -> bool {
    if value < 2 {
        return false;
    } else if value % 2 == 0 {
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

fn main() {
    let mut value: i32 = 0;
    println!("The prime numbers from 0 to 100 are:");
    while value != 100 {
        if is_prime(value) {
            print!("{value} ");
        }
        value += 1;
    }
}
