mod agents;
use agents::*;

mod gameplay;
use gameplay::*;

mod render;
use render::*;

fn main() {
    let mut game = SnakeGame::random(16, 10);
    let mut agent = agents::SimplePath::default();
    let mut renderer = render::Terminal::default();

    loop {
        let action = agent.action(&game);
        if let Some(terminal) = game.do_action(action) {
            renderer.render(&game);
            dbg!(terminal);
            break;
        }

        renderer.render(&game);
    }
}
