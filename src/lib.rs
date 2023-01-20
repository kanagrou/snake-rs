use std::ops;
use rand::Rng;

#[cfg(feature = "renderer")]
pub mod renderer;

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

impl From<crossterm::event::KeyCode> for Input {
    fn from(key_code: crossterm::event::KeyCode) -> Self {
        match key_code {
            crossterm::event::KeyCode::Left => Input::KeyLeft,
            crossterm::event::KeyCode::Right => Input::KeyRight,
            crossterm::event::KeyCode::Up => Input::KeyUp,
            crossterm::event::KeyCode::Down => Input::KeyDown,
            crossterm::event::KeyCode::Char(ch) => match ch {
                'a' => Input::KeyLeft,
                'd' => Input::KeyRight,
                'w' => Input::KeyUp,
                's' => Input::KeyDown,
                _ => Input::Other,
            },
            crossterm::event::KeyCode::Esc => Input::Quit,
            _ => Input::Other,
        }
    }
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
        self.dir = dir;
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
// point 1 (0,0)
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