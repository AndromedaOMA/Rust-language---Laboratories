use base64::encode;
use clap::Parser;
use std::env;
use std::io;
use std::io::Write;

#[derive(Parser)]
struct Args {
    #[arg(long, short, default_value = "None")]
    input: String,

    #[arg(long, short, default_value = "None")]
    output: String,
}

fn main() {
    println!(
        "encoder, version {}, built for {}",
        env!("CARGO_PKG_VERSION"),
        env::consts::OS
    );

    let args = Args::parse();

    if args.input == "None" && args.output == "None" {
        println!("No input or output file specified");
        let mut input_file = String::new();
        io::stdin()
            .read_line(&mut input_file)
            .expect("Something went wrong reading the stdin");

        let mut input_bytes = input_file.as_bytes().to_vec();
        let output_file = encode(&mut input_bytes);

        io::stdout()
            .write_all(output_file.as_bytes())
            .expect("Something went wrong reading the stdout");
    } else if args.input == "None" || args.output == "None" {
        println!("Something went wrong reading the input/output");
    } else {
        let input = args.input;
        let output = args.output;

        let input_file =
            std::fs::read_to_string(&input).expect("Something went wrong reading the file");

        let mut input_bytes = input_file.as_bytes().to_vec();
        let output_file = encode(&mut input_bytes);

        std::fs::write(&output, output_file).expect("Something went wrong writing the file");
    }
}

//=========================================================================================================
//source: https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
//source: https://www.youtube.com/watch?v=B_UZu-jBYgw
//source: https://docs.rs/lazy_static/latest/lazy_static/
//source:
//source:
//=========================================================================================================
