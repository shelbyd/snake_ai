use super::Agent;
use crate::gameplay::*;

#[derive(Default)]
pub struct SimplePath;

impl Agent for SimplePath {
    fn action(&mut self, game: &SnakeGame) -> Action {
        use Action::*;
        use Heading::*;

        assert!(game.height % 2 == 0);

        match (game.head(), game.heading) {
            (Cell(0, 0), h) => h.turn_towards(East).unwrap_or(TurnLeft),
            (Cell(0, _), h) => h.turn_towards(North).unwrap_or(TurnLeft),
            (Cell(_, y), h) if y + 1 == game.height => h.turn_towards(West).unwrap_or(TurnLeft),

            (Cell(x, y), h) if y % 2 == 0 => {
                let dir = if x + 1 == game.width { South } else { East };
                h.turn_towards(dir)
                    .unwrap_or_else(|| h.turn_towards(West).unwrap())
            }
            (Cell(x, y), h) if y % 2 == 1 => {
                let dir = if x == 1 { South } else { West };
                h.turn_towards(dir).unwrap_or(TurnRight)
            }

            unhandled => {
                unimplemented!("unhandled: {:?}", unhandled);
            }
        }
    }
}
