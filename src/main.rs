extern crate mcts;
extern crate tictactoe;

use mcts::GameTest;
use std::io::Read;

#[derive(Clone)]
struct Game(tictactoe::Game, tictactoe::Check);

impl mcts::GameTest for Game {
    fn play(&mut self, p: usize) -> mcts::PlayRes {
        let res = self.0.play(p, self.1);
        self.1 = match self.1 {
            tictactoe::Check::None => tictactoe::Check::None,
            tictactoe::Check::O => tictactoe::Check::X,
            tictactoe::Check::X => tictactoe::Check::O,
        };
        match res {
            tictactoe::Check::None => mcts::PlayRes::Nothing,
            tictactoe::Check::O => mcts::PlayRes::Loose,
            tictactoe::Check::X => mcts::PlayRes::Win,
        }
    }

    fn valid_actions(&self) -> Vec<usize> {
        self.0.valid_moves()
    }
}

fn main() {
    let mut g = Game(tictactoe::Game::new(), tictactoe::Check::O);
    let mut m = mcts::MCTS::new();

    for _ in 0..100 {
        m.train(&mut g);
    }

    loop {
        println!("{}", g.0.show());

        let mut res = String::new();
        std::io::stdin().read_line(&mut res).unwrap();
        let int = match res.trim() {
            "7" => 0,
            "8" => 1,
            "9" => 2,
            "4" => 3,
            "5" => 4,
            "6" => 5,
            "1" => 6,
            "2" => 7,
            "3" => 8,
            _ => {
                println!("Not a valid entry");
                continue;
            }
        };

        match g.valid_actions().iter().find(|&&x| x == int) {
            None => {
                println!("Not a valid move");
                continue;
            }
            Some(_) => {
                g.play(int as usize);

                let mov = m.best_move(&mut g);
                g.play(mov);
            }
        }
    }
}
