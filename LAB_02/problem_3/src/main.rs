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

// fn convert_f_to_string(s: &mut String, mut value: f64) {
//     let ch: char = ((value % 10.0) as u8 + b'0') as char;
//     s.push(ch);
//     value /= 10.0;
//     if value == 0.0 {
//         return;
//     }
//     convert_f_to_string(s, value);
// }

fn add_float(s: &mut String, value: f64) {
    *s += &((value).to_string());
    // let mut string_value: String = Default::default();
    // convert_f_to_string(&mut string_value, value);
    // *s += &string_value;
}

fn main() {
    let mut s: String = String::from("");

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
    add_float(&mut s, 2.038);
    add_str(&mut s, ".");

    print!("{}", s);
}
