fn sum(val1: u32, val2: u32) -> u32 {
    let s = val1 + val2;
    if s > std::u16::MAX as u32 {
        panic!("Expecting sum of val1={val1} and val2={val2} to be type of u32");
    }
    s
}

fn mult(val1: u32, val2: u32) -> u32 {
    let m = val1 * val2;
    if m > std::u16::MAX as u32 {
        panic!("Expecting mult of val1={val1} and val2={val2} to be type of u32");
    }
    m
}

fn main() {
    let val1 = 10;
    let val2 = 20;
    let s = sum(val1, val2);
    println!("We got the sum {s}!");
    let m = mult(val1, val2);
    println!("We got the mult {m}!");

    let val1 = 1000;
    let val2 = 20303;
    let s = sum(val1, val2);
    println!("We got the sum {s}!");
    let m = mult(val1, val2);
    println!("We got the mult {m}!");
}
