#[derive(Clone, Copy, PartialEq)]
pub enum Check {
    None,
    X,
    O,
}

#[derive(Clone)]
pub struct Game {
    b: Vec<Check>,
}

impl Game {
    pub fn new() -> Game {
        let mut res = Game {
            b: Vec::with_capacity(9),
        };
        for _ in 0..9 {
            res.b.push(Check::None);
        }
        res
    }

    pub fn show(&self) -> String {
        let mut res = String::new();

        let mut index = 0;

        for i in &self.b {
            if index % 3 == 0 {
                res.push('\n');
            }

            res.push(match i {
                Check::None => '.',
                Check::O => '0',
                Check::X => 'X',
            });

            index += 1;
        }

        res
    }

    pub fn play(&mut self, at: usize, piece: Check) -> Check {
        self.place(at, piece).check_win()
    }

    pub fn place(&mut self, at: usize, piece: Check) -> &mut Game {
        self.b[at] = piece;
        self
    }

    pub fn check_win(&self) -> Check {
        if self.b[0] == self.b[1] && self.b[0] == self.b[2] {
            self.b[0]
        } else if self.b[3] == self.b[4] && self.b[3] == self.b[5] {
            self.b[3]
        } else if self.b[6] == self.b[7] && self.b[6] == self.b[8] {
            self.b[6]
        } else if self.b[0] == self.b[3] && self.b[0] == self.b[6] {
            self.b[0]
        } else if self.b[1] == self.b[4] && self.b[1] == self.b[7] {
            self.b[1]
        } else if self.b[2] == self.b[5] && self.b[2] == self.b[8] {
            self.b[2]
        } else if self.b[0] == self.b[4] && self.b[0] == self.b[8] {
            self.b[0]
        } else if self.b[2] == self.b[4] && self.b[2] == self.b[6] {
            self.b[2]
        } else {
            Check::None
        }
    }

    pub fn valid_moves(&self) -> Vec<usize> {
        let mut i = 0;
        self.b
            .iter()
            .map(|v| {
                i += 1;
                match v {
                    Check::None => i - 1,
                    _ => 100,
                }
            })
            .filter(|&v| v != 100)
            .collect()
    }
}
