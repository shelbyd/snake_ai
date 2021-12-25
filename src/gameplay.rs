use rand::prelude::*;
use std::collections::VecDeque;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SnakeGame {
    pub score: usize,
    pub moves: usize,

    pub width: u8,
    pub height: u8,

    pub apple: Cell,
    pub body: VecDeque<Cell>,
    pub heading: Heading,
}

impl SnakeGame {
    pub fn random(width: u8, height: u8) -> SnakeGame {
        let mut game = SnakeGame {
            width,
            height,
            ..Default::default()
        };
        let mut rng = thread_rng();
        game.apple = game.gen_cell(&mut rng);
        game.body.push_back(game.gen_open_cell(&mut rng).unwrap());
        game.heading = [Heading::North, Heading::South, Heading::East, Heading::West]
            .choose(&mut rng)
            .unwrap()
            .clone();

        game
    }

    fn gen_cell(&self, rng: &mut impl Rng) -> Cell {
        let x = rng.gen_range(0..self.width);
        let y = rng.gen_range(0..self.height);
        Cell(x, y)
    }

    fn gen_open_cell(&self, rng: &mut impl Rng) -> Option<Cell> {
        let cell = self.gen_cell(rng);
        if self.cell_occupant(cell) == None {
            return Some(cell);
        }

        self.open_cells().choose(rng)
    }

    pub fn open_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..self.height)
            .flat_map(|row| (0..self.width).map(move |col| Cell(col, row)))
            .filter(|cell| self.cell_occupant(*cell) == None)
    }

    pub fn cell_occupant(&self, cell: Cell) -> Option<Occupant> {
        if self.apple == cell {
            return Some(Occupant::Apple);
        }
        let is_body = self.body.iter().any(|&c| c == cell);
        if is_body {
            return Some(Occupant::Body);
        }
        None
    }

    #[must_use]
    pub fn do_action(&mut self, action: Action) -> Option<Terminal> {
        self.moves += 1;

        self.heading = self.heading.after(action);

        let next_cell = match self.cell_delta(self.head(), self.heading) {
            Some(next_cell) => next_cell,
            None => return Some(Terminal::Died),
        };

        match self.cell_occupant(next_cell) {
            None => {
                self.body.pop_front();
                self.body.push_back(next_cell);
            }
            Some(Occupant::Apple) => {
                self.score += 1;
                self.body.push_back(self.apple);
                self.apple = match self.gen_open_cell(&mut rand::thread_rng()) {
                    Some(c) => c,
                    None => return Some(Terminal::Won),
                };
            }
            Some(Occupant::Body) => return Some(Terminal::Died),
        }

        None
    }

    pub fn head(&self) -> Cell {
        *self.body.iter().next_back().unwrap()
    }

    fn cell_delta(&self, cell: Cell, heading: Heading) -> Option<Cell> {
        let next = heading.move_(cell)?;
        if next.0 >= self.width || next.1 >= self.height {
            return None;
        }
        Some(next)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Cell(pub u8, pub u8);

impl Cell {
    pub fn heading_toward(self, other: Cell) -> Option<Heading> {
        use std::cmp::Ordering::*;

        match (self.0.cmp(&other.0), self.1.cmp(&other.1)) {
            (Equal, Equal) => None,
            (Less, _) => Some(Heading::East),
            (Greater, _) => Some(Heading::West),
            (_, Less) => Some(Heading::South),
            (_, Greater) => Some(Heading::North),
        }
    }

    pub fn taxicab_distance_to(self, other: Cell) -> u8 {
        abs_diff(self.0, other.0) + abs_diff(self.1, other.1)
    }

    pub fn neighbors(self) -> impl Iterator<Item = Cell> {
        Heading::iter().filter_map(move |heading| heading.move_(self))
    }
}

fn abs_diff(a: u8, b: u8) -> u8 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Action {
    TurnLeft,
    TurnRight,
    GoStraight,
}

impl Action {
    pub fn iter() -> impl Iterator<Item = Action> {
        [Action::GoStraight, Action::TurnLeft, Action::TurnRight].into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Terminal {
    Won,
    Died,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Occupant {
    Apple,
    Body,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Heading {
    North,
    South,
    East,
    West,
}

impl Default for Heading {
    fn default() -> Self {
        Heading::East
    }
}

impl Heading {
    pub fn iter() -> impl Iterator<Item = Heading> {
        [Heading::North, Heading::South, Heading::East, Heading::West].into_iter()
    }

    fn after(self, action: Action) -> Heading {
        use Heading::*;

        match action {
            Action::GoStraight => self,
            Action::TurnLeft => match self {
                North => West,
                West => South,
                South => East,
                East => North,
            },
            Action::TurnRight => match self {
                North => East,
                East => South,
                South => West,
                West => North,
            },
        }
    }

    pub fn move_(self, cell: Cell) -> Option<Cell> {
        match self {
            Heading::West => Some(Cell(cell.0.checked_sub(1)?, cell.1)),
            Heading::East => Some(Cell(cell.0.checked_add(1)?, cell.1)),

            Heading::North => Some(Cell(cell.0, cell.1.checked_sub(1)?)),
            Heading::South => Some(Cell(cell.0, cell.1.checked_add(1)?)),
        }
    }

    pub fn turn_towards(self, other: Heading) -> Option<Action> {
        if self == other {
            return Some(Action::GoStraight);
        }
        [Action::TurnRight, Action::TurnLeft]
            .into_iter()
            .filter(|a| self.after(*a) == other)
            .next()
    }
}
