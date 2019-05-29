extern crate rand;
use rand::distributions::Distribution;
use std::rc::Rc;

pub enum PlayRes {
    Nothing,
    Win,
    Loose,
}

pub trait GameTest {
    fn play(&mut self, play: usize) -> PlayRes;
}

pub struct Tree {
    children_count: u32,
    children: Vec<Tree>,
    plays: u32,
    wins: u32,
}

impl Tree {
    pub fn new(children_count: u32) -> Tree {
        Tree {
            children_count,
            children: Vec::new(),
            plays: 0,
            wins: 0,
        }
    }

    pub fn update_tree(&mut self, at: usize) -> Tree{
        self.children.remove(at)
    }

    pub fn best_move(&mut self, g: &mut GameTest) -> usize {
        let path = self.path_to_best();
        let leaf = self.fetch_leaf(&path, 0);
        leaf.populate();

        self.simulate(g, &path, 0);

        if path.len() == 0 {
            0
        } else {
            *path.last().unwrap()
        }
    }

    pub fn simulate(&mut self, g: &mut GameTest, path: &Vec<usize>, at: usize) -> bool {
        match path.get(at) {
            None => {
                let i = rand::distributions::Uniform::from(0..self.children_count)
                    .sample(&mut rand::thread_rng()) as usize;
                match g.play(i) {
                    PlayRes::Nothing => self.simulate(g, path, at + 1),
                    PlayRes::Win => true,
                    PlayRes::Loose => false,
                }
            }
            Some(&index) => {
                let res = match g.play(index) {
                    PlayRes::Nothing => self.children[index].simulate(g, path, at + 1),
                    PlayRes::Win => true,
                    PlayRes::Loose => false,
                };

                if res == true {
                    self.wins += 1
                }

                self.plays += 1;

                res
            }
        }
    }

    fn fetch_leaf(&mut self, path: &Vec<usize>, at: usize) -> &mut Tree {
        match path.get(at) {
            None => self,
            Some(&index) => self.children[index].fetch_leaf(path, at + 1),
        }
    }

    pub fn populate(&mut self) {
        for _ in 0..self.children_count {
            self.children.push(Tree::new(self.children_count));
        }
    }

    fn path_to_best(&self) -> Vec<usize> {
        let (index, score) = self.children.iter().enumerate().fold(
            (None, 0),
            |(prev_index, prev_score), (new_index, child)| {
                let new_score = if child.plays != 0 {
                    child.wins / child.plays
                } else {
                    999999999
                };
                match prev_index {
                    None => (Some(new_index), new_score),
                    Some(prev_i) => {
                        if new_score > prev_score {
                            (Some(new_index), new_score)
                        } else {
                            (Some(prev_i), prev_score)
                        }
                    }
                }
            },
        );

        match index {
            None => vec![],
            Some(i) => append_to_vec(self.children[i].path_to_best(), i),
        }
    }
}

fn append_to_vec<T>(mut vec: Vec<T>, val: T) -> Vec<T> {
    vec.push(val);
    vec
}
