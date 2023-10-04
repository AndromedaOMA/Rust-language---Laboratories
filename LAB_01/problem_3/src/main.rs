fn main() {
    let mut value = 99;
    while value >= 1 {
        if value == 1 {
            println!("{value} bottles of beer on the wall,");
            println!("{value} bottles of beer.");
            println!("Take one down, pass it around");
            println!("No bottles of beer on the wall.");
        } else {
            println!("{value} bottles of beer on the wall,");
            println!("{value} bottles of beer.");
            println!("Take one down, pass it around");
            println!("{value} bottles of beer on the wall.");
            println!("");
        }
        value -= 1;
    }
}
