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
        // println!("{:?}", m);
    }

    loop {
        println!("{}", g.0.show());

        let mut res = String::new();
        std::io::stdin().read_line(&mut res).unwrap();
        let int = match res.trim() {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
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
                m.apply_ext(&mut g, int as usize);

                println!("{:?}", m);

                m.play_best_move(&mut g);

                for _ in 0..10 {
                    m.train(&mut g);
                }

                println!("{:?}", m);
            }
        }
    }
}
