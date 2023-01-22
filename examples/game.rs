use crossterm::ExecutableCommand;
use snake::Game;
use std::io::stdout;
use std::time;
use crossterm::event::{poll, read, Event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor;

fn main() -> crossterm::Result<()>{
    // TODO
    let mut game = Game::new(10,10);
    
    let start_time = time::Instant::now();

    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(cursor::Hide)?;
    loop {
        let mut key_buf = Vec::new();
        if poll(time::Duration::from_millis(300))? {
            match read()? {
                Event::Key(event) => key_buf.push(match event.code {
                    crossterm::event::KeyCode::Left => snake::Input::KeyLeft,
                    crossterm::event::KeyCode::Right => snake::Input::KeyRight,
                    crossterm::event::KeyCode::Up => snake::Input::KeyUp,
                    crossterm::event::KeyCode::Down => snake::Input::KeyDown,
                    crossterm::event::KeyCode::Char(ch) => match ch {
                        'a' => snake::Input::KeyLeft,
                        'd' => snake::Input::KeyRight,
                        'w' => snake::Input::KeyUp,
                        's' => snake::Input::KeyDown,
                        _ => snake::Input::Other,
                    },
                    crossterm::event::KeyCode::Esc => snake::Input::Quit,
                    _ => snake::Input::Other,
                }),
                _ => {}
            }
        }
        game.update(key_buf);

        stdout()
        .execute(cursor::MoveToRow(0))?;
        println!("{}", game);
        println!("Score: {}", game.snake.body.len());
        println!("Time: {}", start_time.elapsed().as_secs());

        if !game.is_ongoing() {
            break;
        }
    }
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}