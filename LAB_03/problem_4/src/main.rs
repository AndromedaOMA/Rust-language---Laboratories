#[derive(Debug)]
enum ERR {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(ch: char) -> Result<char, ERR> {
    if !(ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z') {
        Err(ERR::NotLetter)
    } else {
        if !(ch >= 'A' && ch <= 'Z') {
            let ascii_value = ch as u8;
            let upper_value = ascii_value - 32;
            let result = upper_value as char;
            return Ok(result);
        }
        return Ok(ch);
    }
}

fn to_lowercase(ch: char) -> Result<char, ERR> {
    if !(ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z') {
        Err(ERR::NotLetter)
    } else {
        if ch >= 'A' && ch <= 'Z' {
            let ascii_value = ch as u8;
            let upper_value = ascii_value + 32;
            let result = upper_value as char;
            return Ok(result);
        }
        return Ok(ch);
    }
}

fn print_char(ch: char) -> Result<char, ERR> {
    if !(ch as u8 >= 33 && ch as u8 <= 126) {
        Err(ERR::NotPrintable)
    } else {
        Ok(ch)
    }
}

fn char_to_number(ch: char) -> Result<i32, ERR> {
    let digit = ch as i32;

    if !ch.is_ascii() {
        Err(ERR::NotAscii)
    } else if !(digit >= 48 && digit <= 57) {
        Err(ERR::NotDigit)
    } else {
        Ok(digit - 48)
    }
}

fn char_to_number_hex(ch: char) -> Result<i32, ERR> {
    let digit = ch as i32;

    if !ch.is_ascii() {
        Err(ERR::NotAscii)
    } else if !(digit >= 48 && digit <= 57 || digit >= 65 && digit <= 70) {
        Err(ERR::NotDigit)
    } else {
        Ok(digit - 48)
    }
}

fn print_error(err: ERR) -> &'static str {
    match err {
        ERR::NotAscii => return "Char is NotAscii",
        ERR::NotDigit => return "Char is NotDigit",
        ERR::NotBase16Digit => return "Char is NotBase16Digit",
        ERR::NotLetter => return "Char is NotLetter",
        ERR::NotPrintable => return "Char is NotPrintable",
        _ => return "nothing",
    }
}

fn main() {
    let a: char = '7';
    let b: char = '2';
    let c: char = 'f';
    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        to_uppercase(a),
        to_lowercase(b),
        print_char(a),
        char_to_number(b),
        char_to_number_hex(c),
        print_error(ERR::NotAscii)
    );
}
