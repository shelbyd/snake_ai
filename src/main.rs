mod agents;
use agents::*;

mod gameplay;
use gameplay::*;

mod render;
use render::*;

fn main() {
    let mut game = SnakeGame::random(30, 30);
    let mut agent = agents::greedy::Greedy::default();
    let mut renderer = render::Terminal {
        sleep_time: std::time::Duration::from_millis(10),
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
