fn add_space(s: &mut String, mut n: i64) {
    while n != 0 {
        s.push(' ');
        n -= 1;
    }
}

fn add_str(s: &mut String, st: &str) {
    *s += &st;
}

fn convert_i_to_string(s: &mut String, mut value: i64) {
    let ch: char = ((value % 10) as u8 + b'0') as char;
    s.push(ch);
    value /= 10;
    if value == 0 {
        let mut reverse_s: String = Default::default();
        let mut index = s.len() - 1;
        while index != 0 {
            reverse_s += &s[index..index + 1];
            index -= 1;
        }
        reverse_s += &s[index..index + 1];
        *s = reverse_s;

        return;
    }
    convert_i_to_string(s, value);
}

fn add_integer(s: &mut String, value: i64) {
    // let mut string_value = value.to_string();
    let mut string_value: String = Default::default();
    convert_i_to_string(&mut string_value, value);

    let mut index = string_value.len() - 1;
    let mut modified_value: String = Default::default();

    while index != 0 {
        if index % 3 == 0 {
            modified_value = (&string_value[0..index]).to_string();
            modified_value.push('_');
            modified_value.push_str(&string_value[index..]);
            string_value = modified_value.clone();
        }
        index -= 1;
    }

    *s += &modified_value;
}

//-----------------------------------------
//     ---Am sa va rog sa luati in considerare si logica de la acest cod comentat. N-am avut destul  timp sa termin implementarea!---

// fn convert_f_to_string(s: &mut String, mut value: f64) {
//     let mut precision = 0;
//     while value.sqrt().floor() != value.sqrt() {
//         value *= 10.0;
//         precision += 1;
//     }

//     let mut string_value: String = Default::default();
//     convert_i_to_string(&mut string_value, value as i64);
//     let _sufix: &str =
//         &string_value[&string_value.len() - precision..&string_value.len() - precision + 1];
//     *s += &string_value;
// }

// fn add_float(s: &mut String, value: f64) {
//     // *s += &((value).to_string());
//     let mut string_value: String = Default::default();

//     convert_f_to_string(&mut string_value, value);

//     s.push_str(&string_value);
// }

//---------------
//sau
//---------------

// fn add_float(s: &mut String, mut precision: i64, value: f64) {
//     // *s += &((value).to_string());

//     let mut _pow = 1;
//     while precision != 0 {
//         _pow *= 10;
//         precision -= 1;
//     }

//     add_integer(s, value as i64);
//     s.push('.');

//     let mut fraction = value.fract();
//     fraction *= _pow as f64;
//     add_integer(s, fraction as i64);
// }

//----------------------------------------------

fn add_float(s: &mut String, value: f64, mut precision: i64) {
    // *s += &((value).to_string());

    let integer = value.floor() as i64;
    let mut integer_string: String = Default::default();
    convert_i_to_string(&mut integer_string, integer);

    let mut _pow: i64 = 1;
    while precision != 0 {
        _pow *= 10;
        precision -= 1;
    }

    let fraction = (value.fract() * _pow as f64) as i64;
    let mut fraction_string: String = Default::default();
    convert_i_to_string(&mut fraction_string, fraction);

    s.push_str(&integer_string);
    s.push('.');
    s.push_str(&fraction_string);
}

fn main() {
    let mut s: String = Default::default();

    add_space(&mut s, 41);
    add_str(&mut s, "I 💚\n");
    add_space(&mut s, 41);
    add_str(&mut s, "RUST.\n\n");
    add_space(&mut s, 5);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "create");
    add_space(&mut s, 5);
    add_integer(&mut s, 306237968);
    add_space(&mut s, 10);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "lastest");
    add_space(&mut s, 9);
    add_str(&mut s, "is\n");
    add_space(&mut s, 10);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 9);
    add_str(&mut s, "has");
    add_space(&mut s, 11);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038, 4);
    add_str(&mut s, ".");

    print!("{}", s);
}
