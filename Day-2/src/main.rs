enum Digit {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
}

impl Digit {
    fn as_str(&self) -> &'static str {
        match self {
            Digit::ZERO => "zero",
            Digit::ONE => "one",
            Digit::TWO => "two",
            Digit::THREE => "three",
            Digit::FOUR => "four",
            Digit::FIVE => "five",
            Digit::SIX => "six",
            Digit::SEVEN => "seven",
            Digit::EIGHT => "eight",
            Digit::NINE => "nine",
        }
    }
}

fn main() {
    println!("{}", Digit::ZERO.as_str());
}
