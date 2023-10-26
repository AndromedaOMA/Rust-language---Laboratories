use std::io;

#[derive(Debug)]
enum ERR {
    NotANumber,
}

fn check_number(number: &str) -> Result<f64, ERR> {
    //info source: https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse
    if let Ok(parsed) = number.trim().parse() {
        Ok(parsed)
    } else {
        Err(ERR::NotANumber)
    }
}

fn main() {
    println!("What is your weight in kilograms?");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();

    match check_number(&input_weight) {
        Ok(weight) => {
            println!("What is your height in meters?");
            //info source: https://fitech101.aalto.fi/programming-languages/rust/8-interaction-input-and-os/#:~:text=To%20read%20user%20input%20in,written%20on%20the%20command%20line.
            let mut input_height = String::new();
            io::stdin().read_line(&mut input_height).unwrap();

            match check_number(&input_height) {
                Ok(height) => {
                    //theory: https://mantracare.org/fitness/calculators/bmi-calculator/
                    let bmi = weight / (height * height);
                    if bmi < 18.5 {
                        println!("You are underweight.");
                    } else if bmi >= 18.5 && bmi < 25.0 {
                        println!("Your weight is normal.");
                    } else if bmi >= 25.0 && bmi < 30.0 {
                        println!("You are overweight.");
                    } else {
                        println!("You are obese.");
                    }
                }
                Err(err) => println!("{:?}", err),
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
