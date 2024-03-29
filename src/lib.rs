use std::{ops, fmt};
use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
}

impl Grid {
    pub fn ofb(&self, p: &Point) -> bool {
        p.x < 0 || p.x > self.width -1 || p.y < 0 || p.y > self.height - 1
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Collision {
    Food,
    Wall,
    Body,
}

#[derive(Debug)]
pub enum Input {
    KeyUp,
    KeyDown,
    KeyRight,
    KeyLeft,
    Quit,
    Other
}

impl Direction {
    pub fn offset(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1},
            Direction::Left => Point { x: -1, y: 0},
            Direction::Right => Point { x: 1, y: 0},
            Direction::Down => Point { x: 0, y: 1}
        }
    }
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
}

pub struct Snake {
    pub body: Vec<Point>,
    growth: i32,
    dir: Direction
}

impl Snake {
    pub fn new(dir: Direction) -> Self {
        Self {
            body: Vec::new(),
            growth: 0,
            dir
        }
    }

    pub fn add_growth(&mut self, n: i32) {
        self.growth += n;
    }

    pub fn set_dir(&mut self, dir: Direction) {
        if self.dir != dir.opposite() {
            self.dir = dir;
        }
    }

    pub fn head(&self) -> Option<&Point> {
        self.body.get(0)
    }

    pub fn step(&mut self) {  
        self.body.insert(0, self.dir.offset() + self.head().expect("head").clone());
        
        if self.growth <= 0{
            self.body.pop();
        } else {
            self.growth -= 1;
        }
    }

    pub fn in_self(&self, point: &Point) -> bool {
        self.body.iter().any(|p| p == point)
    }
}

pub struct Food {
    pub position: Point,
    pub grid: Grid,
}

impl Food {
    pub fn new(grid: Grid) -> Self {
        Self {
            position: Point { x: 0, y: 0 },
            grid
        }
    }

    pub fn reset(&mut self) -> &Point {
        let y = rand::thread_rng().gen_range(0..self.grid.height);
        let x = rand::thread_rng().gen_range(0..self.grid.width);
        self.position = Point {x, y};
        &self.position
    }
}

pub enum GameState {
    Ongoing,
    Defeat,
    Win,
}


pub struct Game {
    pub grid: Grid,
    pub snake: Snake,
    pub food: Food,
    pub state: GameState,
}

impl Game {
    const BORDER: char = '#';
    const SNAKE: char = '*'; 
    const FOOD: char = 'o';

    pub fn new(width: i32, height: i32) -> Self {
        let grid = Grid { width, height };
        let mut food = Food::new(grid.clone());
        let mut snake = Snake::new(Direction::Right);
        snake.body.push(Point { x: width/2, y: height/2 });

        while snake.in_self(food.reset()){}

        Self {
            grid, snake, food, state: GameState::Ongoing
        }
    }
    
    fn check_collision(&self) -> Option<Collision> {
        let Some(snake_head) = self.snake.head() else {
            return None;
        };
        
        // Food collision
        if snake_head == &self.food.position {
            return Some(Collision::Food);
        }

        // Wall collision
        if self.grid.ofb(snake_head) {
            return Some(Collision::Wall);
        }

        // Body collision
        if self.snake.body.iter().skip(1).any(|p| p == snake_head) {
            return Some(Collision::Body);
        }

        None
    }

    fn handle_input(&mut self, input: &Input) {
        match input {
            Input::KeyDown => {self.snake.set_dir(Direction::Down)},
            Input::KeyLeft => {self.snake.set_dir(Direction::Left)},
            Input::KeyUp => {self.snake.set_dir(Direction::Up)},
            Input::KeyRight => {self.snake.set_dir(Direction::Right)},
            Input::Quit => self.state = GameState::Defeat,
            Input::Other => {},
            
        }
    }

    pub fn score(&self) -> i32 {
        self.snake.body.len() as i32
    }

    pub fn is_ongoing(&self) -> bool {
        match self.state {
            GameState::Ongoing => true,
            _ => false,
        }
    }

    pub fn update(&mut self, inputs: Vec<Input>) {
        // Inputs

        inputs.iter().for_each(|i| self.handle_input(i));

        self.snake.step();

        // Collisions
        if let Some(collision) = self.check_collision() {
            match collision {
                Collision::Food => {
                    self.snake.add_growth(1);
                    while self.snake.in_self(self.food.reset()){}
                },
                Collision::Wall => self.state = GameState::Defeat,
                Collision::Body => self.state = GameState::Defeat,
            }
        };
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for i in 0..self.grid.width + 2 {
            for j in 0..self.grid.height + 2 { 
                let point = Point { x: j - 1, y: i - 1 };
                if j == 0 || j == self.grid.height + 1 ||
                    i == 0 || i == self.grid.width + 1{
                    result.push(Game::BORDER);
                } else if self.snake.in_self(&point){
                    result.push(Game::SNAKE);
                } else if &self.food.position == &point{
                    result.push(Game::FOOD);
                } else {
                    result.push(' ');
                }
                result.push(' ');
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}