use super::Agent;
use crate::gameplay::*;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet, VecDeque},
};

#[derive(Default)]
pub struct TreeSearch {
    plan: VecDeque<Action>,
}

impl Agent for TreeSearch {
    fn action(&mut self, game: &SnakeGame) -> Action {
        if let Some(a) = self.plan.pop_front() {
            return a;
        }

        self.plan = generate_plan(game);
        self.plan.pop_front().unwrap()
    }
}

fn generate_plan(game: &SnakeGame) -> VecDeque<Action> {
    let mut queue = BinaryHeap::new();

    queue.push(State {
        game: game.clone(),
        actions: VecDeque::new(),
    });

    loop {
        let state = match queue.pop() {
            Some(s) => s,
            None => return [Action::GoStraight].into_iter().collect(),
        };
        if state.game.score > game.score {
            return state.actions;
        }

        while let Some(head) = queue.peek() {
            if head == &state {
                queue.pop();
            } else {
                break;
            }
        }

        for action in Action::iter() {
            let mut new_state = state.clone();
            new_state.actions.push_back(action);
            match new_state.game.do_action(action) {
                None | Some(Terminal::Won) => {}
                Some(Terminal::Died) => continue,
            }

            if new_state.is_closed_off() {
                continue;
            }
            queue.push(new_state);
        }
    }
}

#[derive(Clone)]
struct State {
    game: SnakeGame,
    actions: VecDeque<Action>,
}

impl State {
    fn distance_to_apple(&self) -> u8 {
        self.game.head().taxicab_distance_to(self.game.apple)
    }

    fn is_closed_off(&self) -> bool {
        !self.can_reach_at_least(self.game.body.len() + 1)
    }

    fn can_reach_at_least(&self, n: usize) -> bool {
        let mut reached = HashSet::new();
        let mut front = VecDeque::new();
        front.push_back(self.game.head());

        while let Some(cell) = front.pop_front() {
            if !reached.insert(cell) {
                continue;
            }
            if reached.len() >= n {
                return true;
            }
            front.extend(Heading::iter().filter_map(|h| h.move_(cell)).filter(|cell| {
                match self.game.cell_occupant(*cell) {
                    None => true,
                    Some(Occupant::Body) => false,
                    Some(Occupant::Apple) => true,
                }
            }));
        }

        reached.len() >= n
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
            .then_with(|| self.game.score.cmp(&other.game.score))
            .then_with(|| {
                self.distance_to_apple()
                    .cmp(&other.distance_to_apple())
                    .reverse()
            })
            .then_with(|| self.actions.len().cmp(&other.actions.len()).reverse())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for State {}
