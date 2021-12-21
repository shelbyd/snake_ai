use crate::{Cell, Heading, Occupant, SnakeGame};

pub trait Renderer {
    fn render(&mut self, game: &SnakeGame);
}

#[derive(Default)]
pub struct Terminal;

impl Renderer for Terminal {
    fn render(&mut self, game: &SnakeGame) {
        clearscreen::clear().expect("failed to clear screen");
        dbg_print(game);
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

pub fn dbg_print(game: &SnakeGame) {
    use std::fmt::Write;

    let mut s = String::with_capacity((game.width as usize + 2) * (game.height as usize + 2));
    write!(&mut s, "{}/{}\n", game.score, game.moves).unwrap();

    for _ in 0..(game.width + 2) {
        s.push('#');
    }
    s.push('\n');

    for row in 0..game.height {
        s.push('#');
        for col in 0..game.width {
            let cell = Cell(col, row);
            let c = match game.cell_occupant(cell) {
                None => ' ',
                Some(Occupant::Body) => {
                    if game.head() == cell {
                        match game.heading {
                            Heading::North => '^',
                            Heading::South => 'v',
                            Heading::East => '>',
                            Heading::West => '<',
                        }
                    } else {
                        '+'
                    }
                }
                Some(Occupant::Apple) => '*',
            };
            s.push(c);
        }
        s.push('#');
        s.push('\n');
    }

    for _ in 0..(game.width + 2) {
        s.push('#');
    }

    eprintln!("{}", s);
}
