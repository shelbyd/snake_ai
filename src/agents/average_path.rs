use super::Agent;
use crate::{gameplay::*, TreeSearch};

use float_ord::FloatOrd;
use std::{
    cmp::Reverse,
    collections::{HashMap, VecDeque},
};

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
                if game.do_many(actions).unwrap().score > game.score {
                    return Vec::new();
                }

                Action::iter()
                    .map(|a| actions.iter().cloned().chain([a]).collect())
                    .filter(|actions| game.do_many(actions).is_ok())
                    .collect()
            },
        );

        let must_be_better_than = std::cell::Cell::new(FloatOrd(f32::INFINITY));

        let result = search
            .take_while(|actions| best_case_score(&actions, game) <= must_be_better_than.get())
            .filter(|actions| game.do_many(actions).unwrap().score > game.score)
            .min_by_key(|actions| {
                let score = actual_score(&actions, game);
                must_be_better_than.set(std::cmp::min(score, must_be_better_than.get()));
                score
            })
            .expect("No path to apple");
        result
    }
}

fn best_case_score(actions: &VecDeque<Action>, game: &SnakeGame) -> FloatOrd<f32> {
    let new_game = game.do_many(actions).unwrap();
    let best_case_moves = actions.len() + new_game.head().taxicab_distance_to(game.apple) as usize;
    FloatOrd(best_case_moves as f32 + best_case_average_moves(&new_game))
}

fn best_case_average_moves(game: &SnakeGame) -> f32 {
    let move_counts = game
        .open_cells()
        .chain([game.apple])
        .map(|cell| game.head().taxicab_distance_to(cell) as usize)
        .collect::<Vec<_>>();

    move_counts.iter().sum::<usize>() as f32 / move_counts.len() as f32
}

fn actual_score(actions: &VecDeque<Action>, game: &SnakeGame) -> FloatOrd<f32> {
    let new_game = game.do_many(actions).unwrap();
    assert_eq!(new_game.head(), game.apple);
    FloatOrd(actions.len() as f32 + average_moves(&new_game))
}

// Average number of moves required to reach open cells.
fn average_moves(game: &SnakeGame) -> f32 {
    let mut front = VecDeque::new();
    front.push_back(game.head());

    let mut min_steps: HashMap<Cell, usize> = HashMap::new();
    while let Some(cell) = front.pop_front() {
        let steps = *min_steps.get(&cell).unwrap_or(&0);
        for neighbor in cell.neighbors() {
            match game.cell_occupant(neighbor) {
                None | Some(Occupant::Apple) => {}
                Some(Occupant::Body) => continue,
            }
            if min_steps.contains_key(&neighbor) {
                continue;
            }
            front.push_back(neighbor);
            min_steps.insert(neighbor, steps + 1);
        }
    }

    let total = game
        .non_body_cells()
        .map(|cell| {
            min_steps
                .get(&cell)
                .map(|steps| *steps as f32)
                .unwrap_or(f32::INFINITY)
        })
        .sum::<f32>();
    total / game.non_body_cells().count() as f32
}
