use std::io::Write;
use std::str::FromStr;

use monomial::Monomial;
use polynomial::Polynomial;

mod monomial;
mod polynomial;

fn main() {
    let mut output_stream = std::io::stdout();
    let input_stream = std::io::stdin();

    let mut buffer1 = String::new();
    let mut buffer2 = String::new();

    print!("x에 대한 첫번째 다항식 f1을 입력해 주세요: ");

    let _ = output_stream.flush();

    input_stream.read_line(&mut buffer1).expect("읽기에 실패했습니다.");

    let polynomial1 = Polynomial::from_str(buffer1.trim()).expect("잘못된 식입니다.");

    print!("x에 대한 첫번째 다항식 f2를 입력해 주세요: ");

    let _ = output_stream.flush();

    input_stream.read_line(&mut buffer2).expect("읽기에 실패했습니다.");

    let polynomial2 = Polynomial::from_str(buffer2.trim()).expect("잘못된 식입니다.");
    let sum = polynomial1.clone() + polynomial2.clone();
    let subtract = polynomial1.clone() - polynomial2.clone();

    println!("f1 + f2 = {}", sum.to_string());
    println!("f1 + f2 = {}", subtract.to_string());
}
