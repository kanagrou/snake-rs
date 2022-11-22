use std::ops;

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
        p.x < 0 || p.x > self.width || p.y < 0 || p.y > self.height
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

pub enum Input {
    KeyUp,
    KeyDown,
    KeyRight,
    KeyLeft,
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
}

pub enum Event {
    Defeat,
    Win,
}

pub struct Game<'a> {
    pub grid: Grid,
    pub snake: Snake,
    pub food: Food,
    pub event_handler: &'a dyn Fn(Event),
}

impl <'a>Game<'a> {
    pub fn new(width: i32, height: i32, event_handler: &'a dyn Fn(Event)) -> Self {
        let grid = Grid { width, height };
        let food = Food::new(grid.clone());
        let mut snake = Snake::new(Direction::Right);
        snake.body.push(Point { x: width/2, y: height/2 });

        Self {
            grid, snake, food, event_handler
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
        if self.snake.body.iter().any(|p| p == snake_head) {
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
        }
    }

    pub fn score(&self) -> i32 {
        self.snake.body.len() as i32
    }

    pub fn update(&mut self, inputs: Vec<Input>) {
        // Inputs

        inputs.iter().for_each(|i| self.handle_input(i));

        // Collisions
        if let Some(collision) = self.check_collision() {
            match collision {
                Collision::Food => self.snake.add_growth(1),
                Collision::Wall => (self.event_handler)(Event::Defeat),
                Collision::Body => (self.event_handler)(Event::Defeat)
            }
        };
    }
}