use crossterm::ExecutableCommand;
use snake::Game;
use snake::renderer;
use std::io::stdout;
use std::time;
use crossterm::event::{poll, read, Event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor;

fn main() -> crossterm::Result<()>{
    // TODO
    let mut game = Game::new(10,10);
    
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(cursor::Hide)?;
    loop {
        let mut key_buf = Vec::new();
        if poll(time::Duration::from_millis(300))? {
            match read()? {
                Event::Key(event) => key_buf.push(event.code.into()),
                _ => {}
            }
        }
        game.update(key_buf);
        renderer::render_term(&game)?;

        if !game.is_ongoing() {
            break;
        }
    }
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}