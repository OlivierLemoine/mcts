extern crate rand;
use rand::distributions::{uniform::Uniform, Distribution};

pub enum PlayRes {
    Nothing,
    Win,
    Loose,
}

pub trait GameTest {
    fn play(&mut self, play: usize) -> PlayRes;
    fn valid_actions(&self) -> Vec<usize>;
}

#[derive(Clone)]
struct Tree {
    children: Vec<Tree>,
    plays: u32,
    wins: u32,
    action: usize,
}

impl Tree {
    fn best_child_index(&self) -> usize {
        let mut score = 0.0;
        self.children.iter().enumerate().fold(0, |acc, (index, t)| {
            if t.plays == 0 {
                score = 1.0;
                index
            } else if t.wins as f32 / t.plays as f32 > score {
                score = t.wins as f32 / t.plays as f32;
                index
            } else {
                acc
            }
        })
    }

    fn explore_index(&self, total_step: u32) -> usize {
        let sqrt2 = (2.0 as f32).sqrt();
        let tot_step_ln = (total_step as f32).ln();

        let mut score = 0.0;
        self.children.iter().enumerate().fold(0, |acc, (index, t)| {
            if t.plays == 0 {
                score = 100000.0;
                index
            } else {
                let new_score =
                    t.wins as f32 / t.plays as f32 + sqrt2 * (tot_step_ln / t.plays as f32).sqrt();
                if new_score > score {
                    score = new_score;
                    index
                } else {
                    acc
                }
            }
        })
    }

    fn select<T: GameTest>(&mut self, g: &mut T, total_step: u32) -> &mut Tree {
        if self.children.len() == 0 {
            self
        } else {
            let index = self.explore_index(total_step);
            g.play(index);
            self.children[index].select(g, total_step)
        }
    }

    fn expand(&mut self, actions: Vec<usize>) {
        for a in actions {
            self.children.push(Tree {
                children: Vec::new(),
                plays: 0,
                wins: 0,
                action: a,
            })
        }
    }

    fn simulate<T: GameTest>(&mut self, g: &mut T) -> bool {
        let actions = g.valid_actions();
        if actions.len() == 0 {
            self.plays += 1;
            return false;
        }
        let indice = Uniform::from(0..actions.len()).sample(&mut rand::thread_rng());
        let action = actions[indice];
        match g.play(action) {
            PlayRes::Nothing => self.simulate(g),
            PlayRes::Win => {
                self.plays += 1;
                self.wins += 1;
                true
            }
            PlayRes::Loose => {
                self.plays += 1;
                false
            }
        }
    }

    fn backprop(&mut self, plays: u32, wins: u32) {
        if self.children.len() != 0 {
            self.plays += plays;
            self.wins += wins;
            let index = self.best_child_index();
            self.children[index].backprop(plays, wins)
        }
    }
}

pub struct MCTS {
    tree: Tree,
    tot_step: u32,
}

impl MCTS {
    pub fn new() -> MCTS {
        MCTS {
            tree: Tree {
                children: Vec::new(),
                plays: 0,
                wins: 0,
                action: 0,
            },
            tot_step: 0,
        }
    }

    pub fn train<T: GameTest + Clone>(&mut self, g: &mut T) {
        let mut new_g: T = g.clone();

        let mut leaf = self.tree.select(&mut new_g, self.tot_step);
        leaf.expand(new_g.valid_actions());
        let mut acc_win = 0;
        let mut acc_play = 0;

        for l in &mut leaf.children {
            if l.simulate(&mut new_g.clone()) == true {
                acc_win += 1;
            }
            acc_play += 1;
        }

        self.tree.backprop(acc_play, acc_win);

        self.tot_step += 1;
    }

    pub fn best_move<T: GameTest + Clone>(&mut self, g: &mut T) -> usize {
        self.train(g);

        let best_index = self.tree.best_child_index();
        let mut new_tree = self.tree.children[best_index].clone();
        std::mem::swap(&mut self.tree, &mut new_tree);

        self.tree.action
    }
}
