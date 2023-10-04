fn main() {
    let mut value = 99;
    while value >= 0 {
        if value == 0 {
            println!("No bottles of beer on the wall,");
            println!("No bottles of beer.");
            println!("Go to the store, buy some more,");
            println!("99 bottles of beer on the wall.");
        } else if value == 1 {
            println!("{value} bottles of beer on the wall,");
            println!("{value} bottles of beer.");
            println!("Take one down, pass it around");
            println!("No bottles of beer on the wall.");
            println!("");
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
