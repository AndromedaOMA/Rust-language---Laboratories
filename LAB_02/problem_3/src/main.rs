fn add_space(s: &mut String, mut n: i64) {
    while n != 0 {
        s.push(' ');
        n -= 1;
    }
}

fn add_str(s: &mut String, st: &str) {
    *s += &st;
}

fn add_integer(s: &mut String, value: i64) {
    let mut value = value.to_string();
    let mut index = value.len() - 1;
    let mut modified_value: String = Default::default();

    while index != 0 {
        if index % 3 == 0 {
            modified_value = (&value[0..index]).to_string();
            modified_value.push('_');
            modified_value.push_str(&value[index..]);
            value = modified_value.clone();
        }
        index -= 1;
    }

    *s += &modified_value;
}

fn add_float(s: &mut String, value: f64) {
    *s += &((value).to_string());
}

fn main() {
    //test
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
