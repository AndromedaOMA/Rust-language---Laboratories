use std::fs;
use std::path::Path;

fn compute_the_calibration_number(content: String) -> i32 {
    let mut calibration_value: i32 = 0;
    for line in content.lines() {
        let mut value: i32 = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                value = value * 10 + c.to_digit(10).unwrap() as i32;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                value = value * 10 + c.to_digit(10).unwrap() as i32;
                break;
            }
        }
        // println!("{}", value);
        calibration_value += value;
    }
    calibration_value
}

fn main() {    
    let content = fs::read_to_string(Path::new("input.txt")).expect("Can't read the file");
    // println!("{:?}", content);

    let calibration_value=compute_the_calibration_number(content);
    println!("{}",calibration_value);
}
