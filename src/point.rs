#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn neighbors(&self, max_x: i32, max_y: i32) -> Vec<Point> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = self.x + dx;
                let ny = self.y + dy;
                if nx >= 0 && ny >= 0 && nx < max_x && ny < max_y {
                    neighbors.push(Point { x: nx, y: ny });
                }
            }
        }
        neighbors
    }
}
