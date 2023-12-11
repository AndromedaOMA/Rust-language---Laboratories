use fastrand;
// use std::iter::repeat_with;

fn main() {
    //source: https://www.youtube.com/watch?v=AxPDP9Dj08U
    //and: https://docs.rs/fastrand/1.3.0/fastrand/
    let random_length = fastrand::usize(12..19);
    
    // let password: String = repeat_with(fastrand::alphanumeric).take(random_length).collect();

    //============================================LISTS OF CHARS==========================================
    let list_of_alphanumeric: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars()
            .collect();
    let list_of_special_chars: Vec<char> = "!?#@".chars().collect();
    let list_of_uppercase: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    //====================================================================================================


    //============================================RANDOM PASSWORD==========================================
    let password: String = (0..random_length)
        .map(|current_index| {
            if current_index == 0 {
                let random_index = fastrand::usize(..list_of_uppercase.len());
                list_of_uppercase[random_index].to_string()
            } else if current_index == 1 {
                let random_index = fastrand::usize(..list_of_special_chars.len());
                list_of_special_chars[random_index].to_string()
            } else {
                let random_index = fastrand::usize(..list_of_alphanumeric.len());
                list_of_alphanumeric[random_index].to_string()
            }
        })
        .collect();
    //====================================================================================================

    println!("Hello!!");
    println!("-> Here is the random password: {}", password);
    println!("Enjoy!!");
}
