#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl std::ops::Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}


#[derive(Clone, Copy, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn to_vec2(self) -> Vec2 {
        match self {
            Dir::Up => Vec2 { x: 0, y: 1 },
            Dir::Down => Vec2 { x: 0, y: -1 },
            Dir::Left => Vec2 { x: -1, y: 0 },
            Dir::Right => Vec2 { x: 1, y: 0 },
        }
    }
    
    pub const ALL: [Self; 4] = [
        Self::Up,
        Self::Down,
        Self::Left,
        Self::Right,
    ];
}

impl std::ops::Add<Dir> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Dir) -> Self::Output {
        self + rhs.to_vec2()
    }
}