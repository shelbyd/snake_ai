use super::Agent;
use crate::{gameplay::*, TreeSearch};

use float_ord::FloatOrd;
use std::{cmp::Reverse, collections::VecDeque};

#[derive(Default)]
pub struct AveragePath {
    plan: VecDeque<Action>,
}

impl Agent for AveragePath {
    fn action(&mut self, game: &SnakeGame) -> Action {
        if let Some(a) = self.plan.pop_front() {
            return a;
        }

        self.plan = self.generate_plan(game);
        self.plan.pop_front().unwrap()
    }
}

impl AveragePath {
    fn generate_plan(&self, game: &SnakeGame) -> VecDeque<Action> {
        let search = TreeSearch::new(
            VecDeque::new(),
            |actions| Reverse(best_case_score(&actions, game)),
            |actions| {
                Action::iter()
                    .map(|a| actions.iter().cloned().chain([a]).collect())
                    .filter(|actions| game.do_many(actions).is_ok())
                    .collect()
            },
        );

        let must_be_better_than = std::cell::Cell::new(FloatOrd(f32::INFINITY));

        return search
            .take_while(|actions| best_case_score(&actions, game) <= must_be_better_than.get())
            .filter(|actions| game.do_many(actions).unwrap().score > game.score)
            .min_by_key(|actions| {
                let score = actual_score(&actions, game);
                must_be_better_than.set(std::cmp::min(score, must_be_better_than.get()));
                score
            })
            .expect("No path to apple");
    }
}

fn best_case_score(actions: &VecDeque<Action>, game: &SnakeGame) -> FloatOrd<f32> {
    let new_game = game.do_many(actions).unwrap();
    let best_case_moves = actions.len() + new_game.head().taxicab_distance_to(game.apple) as usize;
    let best_case_expected = best_case_moves as f32 + best_case_average_moves(&new_game);
    FloatOrd(best_case_expected)
}

fn best_case_average_moves(_game: &SnakeGame) -> f32 {
    // TODO(shelbyd): Calculate reachability of other cells.
    0.
}

fn actual_score(actions: &VecDeque<Action>, game: &SnakeGame) -> FloatOrd<f32> {
    let new_game = game.do_many(actions).unwrap();
    let actual_moves = actions.len() + new_game.head().taxicab_distance_to(game.apple) as usize;
    FloatOrd(actual_moves as f32 + average_moves(&new_game).unwrap_or(f32::INFINITY))
}

// Average number of moves required to reach open cells.
fn average_moves(game: &SnakeGame) -> Option<f32> {
    let move_counts = game
        .open_cells()
        .chain([game.apple])
        // TODO(shelbyd): Actual calculation.
        .map(|_cell| Some(0))
        .collect::<Option<Vec<_>>>()?;

    Some(move_counts.iter().sum::<usize>() as f32 / move_counts.len() as f32)
}
