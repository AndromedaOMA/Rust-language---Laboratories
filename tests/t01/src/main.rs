fn add1(mut number:i32) {
    number+=1;
    println!("neW_number={}", number);
}
// fn string_length(s:&String) -> usize {
//     s.len()
//     println!("s={}", s);
// }

fn main() {
    let number=23;
    add1(number);
    let no=number;
    println!("number={}", number);
    println!("no={}", no);

    let mut s1=String::from("Hello");
    let len=s1.len();
    let s2=&s1;
    let s3=&s1;
    println!("s2={}, {}", s2, len);
    println!("s3={}, {}", s3, len);
    s1.push_str(", world!");
    println!("s1={}, {}", s1, s1.len());
}