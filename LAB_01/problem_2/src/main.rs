

fn co_prime_numbers(mut value1: i32, mut value2: i32) -> bool {
    let mut rest;
    while value2 != 0 {
        rest = value1 % value2;
        value1 = value2;
        value2 = rest;
    }
    if value1 == 1 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut value1: i32 = 0;
    let mut value2: i32;
    while value1 <= 100 {
        value2 = 0;
        while value2 <= 100 {
            if co_prime_numbers(value1, value2) == true {
                print!("({value1},{value2}); ");
            }
            value2 += 1;
        }
        value1 += 1;
    }
}
