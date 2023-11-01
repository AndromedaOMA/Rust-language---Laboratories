#[derive(PartialEq,Debug)]
enum ERR {
    NotAscii,
}

fn ROT13(s: String) -> Result<String, ERR> {
    let mut final_string: String = String::from("");
    let mut result;
    for i in s.chars() {
        if !(i >= 'a' && i <= 'z' || i >= 'A' && i <= 'Z') {
            return Err(ERR::NotAscii);
        } else if !(i >= 'a' && i <= 'n' || i >= 'A' && i <= 'N') {
            let ascii_value = i as u8;
            let ch_plus = ascii_value - 13;
            result = ch_plus as char;
        } else {
            let ascii_value = i as u8;
            let ch_plus = ascii_value + 13;
            result = ch_plus as char;
        }
        final_string.push(result);
    }

    Ok(final_string)
}

fn main() {
    let s: String = String::from("Rebeca");
    print!("{:?}", ROT13(s));
}
