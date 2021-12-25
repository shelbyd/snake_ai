use super::Agent;
use crate::{gameplay::*, KeyedQueue};

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
        let mut queue = KeyedQueue::new();
        queue.insert(Reverse(0), (game.clone(), VecDeque::<Action>::new()));

        let mut best: Option<(f32, VecDeque<Action>)> = None;

        while let Some((game, actions)) = queue.pop() {
            for action in Action::iter() {
                let mut new_game = game.clone();
                match new_game.do_action(action) {
                    Some(Terminal::Died) => continue,
                    None | Some(Terminal::Won) => {}
                }

                let mut actions = actions.clone();
                actions.push_back(action);

                let just_scored = new_game.score > game.score;
                if just_scored {
                    if let Some(moves) = average_moves(&new_game) {
                        best = best
                            .into_iter()
                            .chain(std::iter::once((
                                actions.len() as f32 + moves,
                                actions.clone(),
                            )))
                            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
                    }
                } else {
                    let best_case_moves = actions.len() + best_case_moves_to_go(&new_game, false);

                    let best_case_expected =
                        best_case_moves as f32 + best_case_average_moves(&new_game);
                    if let Some((best, _)) = &best {
                        if best_case_expected > *best {
                            continue;
                        }
                    }

                    queue.insert(Reverse(best_case_moves), (new_game, actions));
                }
            }
        }

        best.unwrap().1
    }
}

fn best_case_moves_to_go(game: &SnakeGame, just_scored: bool) -> usize {
    if just_scored {
        0
    } else {
        game.head().taxicab_distance_to(game.apple) as usize
    }
}

// Average number of moves required to reach open cells.
fn average_moves(game: &SnakeGame) -> Option<f32> {
    let mut move_counts = game
        .open_cells()
        .chain(std::iter::once(game.apple))
        .map(|cell| Some(0))
        .collect::<Option<Vec<_>>>()?;

    Some(move_counts.iter().sum::<usize>() as f32 / move_counts.len() as f32)
}

fn best_case_average_moves(game: &SnakeGame) -> f32 {
    // TODO(shelbyd): Calculate reachability of other cells.
    0.
}
