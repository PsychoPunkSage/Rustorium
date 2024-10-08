// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub struct Robot(i32, i32, Direction);

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self(x, y, d)
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        match self.2 {
            Direction::North => Robot(self.0, self.1, Direction::East),
            Direction::East => Robot(self.0, self.1, Direction::South),
            Direction::South => Robot(self.0, self.1, Direction::West),
            Direction::West => Robot(self.0, self.1, Direction::North),
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.2 {
            Direction::North => Robot(self.0, self.1, Direction::West),
            Direction::East => Robot(self.0, self.1, Direction::North),
            Direction::South => Robot(self.0, self.1, Direction::East),
            Direction::West => Robot(self.0, self.1, Direction::South),
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.2 {
            Direction::North => Robot(self.0, self.1 + 1, self.2),
            Direction::East => Robot(self.0 + 1, self.1, self.2),
            Direction::South => Robot(self.0, self.1 - 1, self.2),
            Direction::West => Robot(self.0 - 1, self.1, self.2),
        }
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        instructions.chars().for_each(|cmd| match cmd {
            'L' => self = self.turn_left(),
            'R' => self = self.turn_right(),
            'A' => self = self.advance(),
            _ => (),
        });
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.2
    }
}
