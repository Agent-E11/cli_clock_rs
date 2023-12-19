pub trait BlockChar3By5 {
    fn char_grid(&self) -> [[bool; 3]; 5];
}

impl BlockChar3By5 for char {
    fn char_grid(&self) -> [[bool; 3]; 5] {
        if self.is_numeric() {
            match self {
                '0' => [
                    [true , true , true ],
                    [true , false, true ],
                    [true , false, true ],
                    [true , false, true ],
                    [true , true , true ],
                ],
                '1' => [
                    [false, false, true ],
                    [false, false, true ],
                    [false, false, true ],
                    [false, false, true ],
                    [false, false, true ],
                ],
                '2' => [
                    [true , true , true ],
                    [false, false, true ],
                    [true , true , true ],
                    [true , false, false],
                    [true , true , true ],
                ],
                '3' => [
                    [true , true , true ],
                    [false, false, true ],
                    [true , true , true ],
                    [false, false, true ],
                    [true , true , true ],
                ],
                '4' => [
                    [true , false, true ],
                    [true , false, true ],
                    [true , true , true ],
                    [false, false, true ],
                    [false, false, true ],
                ],
                '5' => [
                    [true , true , true ],
                    [true , false, false],
                    [true , true , true ],
                    [false, false, true ],
                    [true , true , true ],
                ],
                '6' => [
                    [true , true , true ],
                    [true , false, false],
                    [true , true , true ],
                    [true , false, true ],
                    [true , true , true ],
                ],
                '7' => [
                    [true , true , true ],
                    [false, false, true ],
                    [false, false, true ],
                    [false, false, true ],
                    [false, false, true ],
                ],
                '8' => [
                    [true , true , true ],
                    [true , false, true ],
                    [true , true , true ],
                    [true , false, true ],
                    [true , true , true ],
                ],
                '9' => [
                    [true , true , true ],
                    [true , false, true ],
                    [true , true , true ],
                    [false, false, true ],
                    [true , true , true ],
                ],
                _ => [
                    [true , true , true ],
                    [true , true , true ],
                    [true , true , true ],
                    [true , true , true ],
                    [true , true , true ],
                ]
            }

        } else {
            [
                [true, true, true],
                [true, true, true],
                [true, true, true],
                [true, true, true],
                [true, true, true],
            ]
        }
    }
}
