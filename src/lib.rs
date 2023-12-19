pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl Digit {
    pub fn display(&self) -> [[bool; 3]; 5] {
        match self {
            Self::Zero => [
                [true, true, true],
                [true, false, true],
                [true, false, true],
                [true, false, true],
                [true, true, true],
            ],
            Self::One => [
                [false, false, true],
                [false, false, true],
                [false, false, true],
                [false, false, true],
                [false, false, true],
            ],
            Self::Two => [
                [true, true, true],
                [false, false, true],
                [true, true, true],
                [true, false, false],
                [true, true, true],
            ],
            Self::Three => [
                [true, true, true],
                [false, false, true],
                [true, true, true],
                [false, false, true],
                [true, true, true],
            ],
            Self::Four => [
                [true, false, true],
                [true, false, true],
                [true, true, true],
                [false, false, true],
                [false, false, true],
            ],
            Self::Five => [
                [true, true, true],
                [true, false, false],
                [true, true, true],
                [false, false, true],
                [true, true, true],
            ],
            Self::Six => [
                [true, true, true],
                [true, false, false],
                [true, true, true],
                [true, false, true],
                [true, true, true],
            ],
            Self::Seven => [
                [true, true, true],
                [false, false, true],
                [false, false, true],
                [false, false, true],
                [false, false, true],
            ],
            Self::Eight => [
                [true, true, true],
                [true, false, true],
                [true, true, true],
                [true, false, true],
                [true, true, true],
            ],
            Self::Nine => [
                [true, true, true],
                [true, false, true],
                [true, true, true],
                [false, false, true],
                [true, true, true],
            ],
        }
    }

    pub fn print(&self) {
        let map = self.display();
        for line in map {
            for cell in line {
                if cell {
                    print!("██");
                } else {
                    print!("  ");
                }
            }
            println!();
        }
    }
}
