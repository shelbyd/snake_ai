use crate::gameplay::*;
use rand::prelude::*;

pub mod average_path;
pub mod greedy;
pub mod tree_search;

pub mod simple_path;
pub use simple_path::*;

pub trait Agent {
    fn action(&mut self, game: &SnakeGame) -> Action;
}

#[derive(Default)]
pub struct Random;

impl Agent for Random {
    fn action(&mut self, _: &SnakeGame) -> Action {
        [Action::TurnLeft, Action::TurnRight, Action::GoStraight]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    }
}
