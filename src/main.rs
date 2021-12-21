mod agents;
use agents::*;

mod gameplay;
use gameplay::*;

fn main() {
    let mut game = SnakeGame::random(32, 18);
    let mut agent = agents::SimplePath::default();

    loop {
        let action = agent.action(&game);
        let term = game.do_action(action);

        if game.moves % 1000 == 0 {
            clearscreen::clear().expect("failed to clear screen");
            game.dbg_print();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        if let Some(terminal) = term {
            clearscreen::clear().expect("failed to clear screen");
            game.dbg_print();
            dbg!(terminal);
            break;
        }

    }
}
