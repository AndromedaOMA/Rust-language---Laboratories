fn add_space(s: &mut String, mut n: i64) {
    while n != 0 {
        s.push(' ');
        n -= 1;
    }
    print!("{}", s);
}

fn add_str(s: &mut String, st: &str) {
    // let mut iterator = 0;
    // let l = s.len();
    // while iterator <= l {
    //     s.push(st[iterator]);
    //     iterator += 1;
    // }

    *s += &st;

    print!("{}", s);
}

// fn add_integer(mut s: String, value: i32) {
//     let aux = value as String;
//     let iterator = 0;
//     while value != NULL {
//         if iterator == 3 {
//             aux.push('_');
//             iterator = 0;
//         } else {
//             aux.push((aux % 10) as String);
//         }
//         iterator += 1;
//         value /= 10;
//     }
// }

fn add_float(mut s: String, value: u32) {}

fn main() {
    //test
    let mut name:String = String::from("Joe");
    add_space(&mut name,5);
    add_str(&mut name,"hello");
}
