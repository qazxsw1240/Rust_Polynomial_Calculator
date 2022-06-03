use std::io::Write;
use std::str::FromStr;

use polynomial::Polynomial;

mod monomial;
mod polynomial;
mod substr;

fn main() {
    let mut output_stream = std::io::stdout();
    let input_stream = std::io::stdin();

    let mut buffer1 = String::new();
    let mut buffer2 = String::new();

    output_stream
        .write("Input the first polynomial with respect to x: ".as_bytes())
        .unwrap();
    output_stream.flush().unwrap();

    input_stream
        .read_line(&mut buffer1)
        .expect("Failed to read string");

    let polynomial1 = Polynomial::from_str(buffer1.trim()).expect("Invalid expression provided.");

    output_stream
        .write("Input the second polynomial with respect to x: ".as_bytes())
        .unwrap();
    output_stream.flush().unwrap();

    input_stream
        .read_line(&mut buffer2)
        .expect("Failed to read string");

    let polynomial2 = Polynomial::from_str(buffer2.trim()).expect("Invalid expression provided.");
    let sum = polynomial1.clone() + polynomial2.clone();
    let subtract = polynomial1.clone() - polynomial2.clone();

    println!("f1 + f2 = {}", sum.to_string());
    println!("f1 - f2 = {}", subtract.to_string());
}
