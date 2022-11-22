use crate::Game;
use std::io::stdout;
use crossterm::{cursor, ExecutableCommand};

const BORDER: char = '#';

pub fn render_term(game: &Game) -> crossterm::Result<()> {
    stdout()
        .execute(cursor::SavePosition)?;
    
    for i in 0..game.grid.width + 2 {
        for j in 0..game.grid.height + 2 { 
            // i: y j: x
            if j == 0 || j == game.grid.height + 1 ||
                i == 0 || i == game.grid.width + 1{
                print!("{} ", BORDER);
            } else {
                print!("  ");
            }
        }
        print!("\n");
    }
    
        
    Ok(())

    
}