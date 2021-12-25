mod agents;
use agents::*;

mod gameplay;
use gameplay::*;

mod keyed_queue;
pub use keyed_queue::*;

mod render;
use render::*;

mod tree_search;
use tree_search::*;

fn main() {
    let mut game = SnakeGame::random(10, 10);
    let mut agent = agents::average_path::AveragePath::default();
    let mut renderer = render::Terminal {
        sleep_time: std::time::Duration::from_millis(10),
        render_every: 1,
        ..Default::default()
    };

    loop {
        let action = agent.action(&game);
        if let Some(terminal) = game.do_action(action) {
            renderer.render(&game, true);
            dbg!(terminal);
            break;
        }

        renderer.render(&game, false);
    }
}
