use snake::Game;
use snake::Event;
use snake::renderer;

fn event_handler(event: Event) {

}

fn main() {
    // TODO
    let game = Game::new(10,10, &event_handler);
    renderer::render_term(&game);
}