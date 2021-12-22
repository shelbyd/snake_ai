use super::Agent;
use crate::gameplay::*;

#[derive(Default)]
pub struct Greedy;

impl Agent for Greedy {
    fn action(&mut self, game: &SnakeGame) -> Action {
        let heading = game.head().heading_toward(game.apple).unwrap();
        game.heading
            .turn_towards(heading)
            .unwrap_or(Action::TurnLeft)
    }
}
