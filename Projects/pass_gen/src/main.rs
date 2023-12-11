use fastrand;
// use std::iter::repeat_with;

fn main() {
    //source: https://www.youtube.com/watch?v=AxPDP9Dj08U
    //and: https://docs.rs/fastrand/1.3.0/fastrand/
    let random_length = fastrand::usize(12..19);
    // let password: String = repeat_with(fastrand::alphanumeric).take(random_length).collect();

    let list_of_chars:Vec<char>= "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;':,./<>?".chars().collect();

    let password: String = (0..random_length)
        .map(|_| {
            let random_index = fastrand::usize(..list_of_chars.len());
            list_of_chars[random_index]
        })
        .collect();

    println!("Hello!!");
    println!("-> Here is the random password: {}", password);
    println!("Enjoy!!");
}
