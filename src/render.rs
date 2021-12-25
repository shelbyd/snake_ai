use crate::{Cell, Heading, Occupant, SnakeGame};
use std::time::Duration;

pub trait Renderer {
    fn render(&mut self, game: &SnakeGame, final_: bool);
}

pub struct Terminal {
    pub render_every: usize,
    pub sleep_time: Duration,
}

impl Renderer for Terminal {
    fn render(&mut self, game: &SnakeGame, final_: bool) {
        let should_render =
            final_ || (self.render_every == 0 || game.moves % self.render_every == 0);
        if !should_render {
            return;
        }

        clearscreen::clear().expect("failed to clear screen");
        dbg_print(game);
        std::thread::sleep(self.sleep_time);
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal {
            render_every: 1,
            sleep_time: Duration::from_millis(10),
        }
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
                Some(Occupant::Apple) => 'O',
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
