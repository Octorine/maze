use std::collections::HashSet;
#[derive(Debug)]
pub struct Maze {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
    pub edges: HashSet<Edge>,
}

impl Maze {
    pub fn has_edge(&self, p1: Point, p2: Point) -> bool {
        self.edges.contains(&Edge::new(p1, p2))
    }
    pub fn draw(&self) {
        let width = self.right - self.left;
        let height = self.bottom - self.top;
        // Draw the top row
        print!("+");
        for _ in 1..width - 2 {
            print!("--");
        }
        println!("+");
        // For each row
        for j in 1..height - 2 {
            // Draw the vertical bars
            print!("|");
            for i in 1..width - 2 {
                print!(" ");
                if self.has_edge((i, j), (i + 1, j)) {
                    print!("|");
                } else {
                    print!(" ")
                }
            }
            println!("|");
            // Draw the horizontal bars
            print!("|");
            for i in 1..width - 2 {
                if self.has_edge((i, j - 1), (i, j)) {
                    print!("-");
                } else {
                    print!(" ");
                }
                print!(" ");
            }
            println!("|");
        }
        // Draw the bottom row
        print!("+");
        for _ in 1..width - 2 {
            print!("--");
        }
        println!("+");
    }
}

pub type Point = (i32, i32);

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Edge {
    p1: Point,
    p2: Point,
}
impl Edge {
    pub fn new(p1: Point, p2: Point) -> Edge {
        Edge { p1, p2 }
    }
}
