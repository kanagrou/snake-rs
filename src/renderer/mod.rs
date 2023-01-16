use crate::{Game, Point};
use std::io::stdout;
use crossterm::{cursor, ExecutableCommand};

const BORDER: char = '#';
const SNAKE: char = '*'; 
const FOOD: char = 'o';

pub fn render_term(game: &Game) -> crossterm::Result<()> {
    stdout()
        .execute(cursor::SavePosition)?;
    
    for i in 0..game.grid.width + 2 {
        for j in 0..game.grid.height + 2 { 
            let point = Point { x: j, y: i };

            if j == 0 || j == game.grid.height + 1 ||
                i == 0 || i == game.grid.width + 1{
                print!("{} ", BORDER);
            } else if game.snake.in_self(&point){
                print!("{} ", SNAKE);
            } else if &game.food.position == &point{
                print!("{} ", FOOD);
            } else {
                print!("  ");
            }
        }
        print!("\n");
    } 
    Ok(())
}