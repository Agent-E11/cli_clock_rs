use cli_clock::Digit::*;

fn main() {
    let digits = [Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine];

    for digit in digits {
        digit.print();
        println!();
    }
}
