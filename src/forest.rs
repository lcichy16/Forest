use crate::point::Point;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Tree,
    Burning,
    Burned,
}

pub struct Forest {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,
}

impl Forest {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Cell::Empty; width]; height];
        Self { width, height, grid }
    }

    pub fn grow(&mut self, perc: f64) {
        let mut rng = thread_rng();
        let total = (self.width * self.height) as f64 * perc / 100.0;
        let mut planted = 0;

        while (planted as f64) < total {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            if self.grid[y][x] == Cell::Empty {
                self.grid[y][x] = Cell::Tree;
                planted += 1;
            }
        }
    }

    pub fn start_fire(&mut self) {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            if self.grid[y][x] == Cell::Tree {
                self.grid[y][x] = Cell::Burning;
                break;
            }
        }
    }

    pub fn spread_fire(&mut self) {
        let mut new_burning = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == Cell::Burning {
                    for neighbor in Point::new(x as i32, y as i32).neighbors(self.width as i32, self.height as i32) {
                        let nx = neighbor.x as usize;
                        let ny = neighbor.y as usize;
                        if self.grid[ny][nx] == Cell::Tree {
                            new_burning.push((nx, ny));
                        }
                    }
                    self.grid[y][x] = Cell::Burned;
                }
            }
        }

        for (x, y) in new_burning {
            self.grid[y][x] = Cell::Burning;
        }
    }

    pub fn burned_percentage(&self) -> f64 {
        let mut total = 0;
        let mut burned = 0;

        for row in &self.grid {
            for &cell in row {
                if cell == Cell::Tree || cell == Cell::Burned {
                    total += 1;
                }
                if cell == Cell::Burned {
                    burned += 1;
                }
            }
        }

        if total == 0 {
            0.0
        } else {
            (burned as f64 / total as f64) * 100.0
        }
    }

    pub fn has_burning_trees(&self) -> bool {
        self.grid.iter().any(|row| row.iter().any(|&c| c == Cell::Burning))
    }
}
