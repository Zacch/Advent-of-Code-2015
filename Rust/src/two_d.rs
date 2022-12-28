use std::ops;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point { Point { x, y } }
    pub fn up(&self) -> Point { Point::new(self.x, self.y + 1) }
    pub fn down(&self) -> Point { Point::new(self.x, self.y - 1) }
    pub fn left(&self) -> Point { Point::new(self.x - 1, self.y) }
    pub fn right(&self) -> Point { Point::new(self.x + 1, self.y) }

    pub fn n(&self) -> Point { Point::new(self.x, self.y + 1) }
    pub fn s(&self) -> Point { Point::new(self.x, self.y - 1) }
    pub fn w(&self) -> Point { Point::new(self.x - 1, self.y) }
    pub fn e(&self) -> Point {
        Point::new(self.x + 1, self.y)
    }

    pub fn ne(&self) -> Point { Point::new(self.x + 1, self.y + 1) }
    pub fn se(&self) -> Point { Point::new(self.x + 1, self.y - 1) }
    pub fn sw(&self) -> Point { Point::new(self.x - 1, self.y - 1) }
    pub fn nw(&self) -> Point { Point::new(self.x - 1, self.y + 1) }

    pub fn turn_cw(&self) -> Point { Point::new(-self.y, self.x) }
    pub fn turn_ccw(&self) -> Point { Point::new(self.y, -self.x) }

    pub fn neighbours(&self) -> Vec<Point> {
        vec![self.e(), self.w(), self.n(), self.s(), self.ne(), self.nw(), self.se(), self.sw()]
    }

    pub fn manhattan_distance(&self, p: &Point) -> i32 {
        i32::abs(self.x - p.x) + i32::abs(self.y - p.y)
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point { Point::new(self.x + rhs.x, self.y + rhs.y) }
}

impl ops::Add<&Point> for Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point { Point::new(self.x + rhs.x, self.y + rhs.y) }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub bottom: i32,
    pub right: i32,
}

impl Rect {
    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.left && p.x < self.right && p.y >= self.bottom && p.y < self.top
    }
}

impl Rect {
    pub fn contains_inclusive(&self, p: Point) -> bool {
        p.x >= self.left && p.x <= self.right && p.y >= self.bottom && p.y <= self.top
    }
}
