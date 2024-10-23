use std::fs::File;
use std::io::Write;
use rand::Rng;
use clap::Parser;


fn create_random_smt_expr() -> String {
    // simple function that outputs a random LISP-style SMT expression
    // (only supports not-nested expression, simplify, +, -, *)
    let mut rng = rand::thread_rng();

    let random_number_1 = rng.gen_range(0..3);
    let random_number_2 = rng.gen::<i32>();
    let random_number_3 = rng.gen::<i32>();

    let operator: String;
    match random_number_1 {
        0 => { operator = String::from("+"); },
        1 => { operator = String::from("-"); },
        _ => { operator = String::from("*"); },
    }

    return String::from(format!("(simplify ({} {} {}))", operator, random_number_2, random_number_3));
}

/// Simple program to generate random LISP-style SMT expressions and write them to a file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path to write to
    #[arg(short, long)]
    file: String,

    /// Number of LISP-style SMT expressions
    #[arg(short, long, default_value_t = 10)]
    number: u16,
}

fn main() {
    let args = Args::parse();

    let mut file_write = File::create(&args.file).expect("Cannot create file");

    for _i in 0..args.number {
        let smt_expr = create_random_smt_expr();

        writeln!(file_write, "{}", smt_expr).expect("Cannot write to file");
    }
    writeln!(file_write, "").expect("Cannot write to file");
    println!("Successfully wrote {} LISP-style SMT expression(s) to file: {}", args.number, args.file);
}
