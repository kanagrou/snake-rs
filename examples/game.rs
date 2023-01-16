use snake::Game;
use snake::renderer;
use std::thread;
use std::time;
use crossterm::event::{poll, read, Event};


fn main() -> crossterm::Result<()>{
    // TODO
    let mut game = Game::new(10,10);
    loop {
        let mut key_buf = Vec::new();
        if poll(time::Duration::from_millis(300))? {
            match read()? {
                Event::Key(event) => key_buf.push(event.code.into()),
                _ => {}
            }
        } else {
            //print! ("\x1B[2J\x1B[1;1H"); 
            game.update(key_buf);
            renderer::render_term(&game);
    
            if !game.is_ongoing() {
                break;
            }
        }
    }
    Ok(())
}