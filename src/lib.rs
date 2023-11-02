mod union_find;

use rand::Rng;

use std::collections::HashSet;
use union_find::Forest;

// A maze consisting of square cells numbered from 0 to WIDTH - 1 and
// 0 to HEIGHT - 1.
#[derive(Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub edges: HashSet<Edge>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        Maze {
            width,
            height,
            edges: HashSet::new(),
        }
    }
    pub fn has_edge(&self, p1: Point, p2: Point) -> bool {
        self.edges.contains(&Edge::new(p1, p2)) || self.edges.contains(&Edge::new(p2, p1))
    }
    pub fn draw(&self) {
        let width = self.width;
        let height = self.height;
        // Draw the top row
        print!("+");
        for _ in 0..width - 1 {
            print!("--");
        }
        if width > 0 {
            print!("-");
        }
        println!("+");
        // For each row
        for j in 0..height - 1 {
            // Draw the vertical bars.  One at the beginning, one at
            // the end, and one wherever there's an edge between
            // neighboring cells.
            print!("|");
            for i in 0..width - 1 {
                print!(" ");
                if self.has_edge((i, j), (i + 1, j)) {
                    print!("|");
                } else {
                    print!(" ")
                }
            }
            if width > 0 {
                print!(" ");
            }
            println!("|");
            // Draw the horizontal bars
            print!("|");
            for i in 0..width - 1 {
                if self.has_edge((i, j + 1), (i, j)) {
                    print!("-");
                } else {
                    print!(" ");
                }
                print!(" ");
            }
            if self.has_edge((width - 1, j + 1), (width - 1, j)) {
                print!("-");
            } else {
                print!(" ");
            }
            println!("|");
        }
        // Draw the verttical bars for the bottom row, but no
        // horizontal bars underneath.
        print!("|");
        for i in 0..width - 1 {
            print!(" ");
            if self.has_edge((i, height - 1), (i + 1, height - 1)) {
                print!("|");
            } else {
                print!(" ")
            }
        }
        if width > 0 {
            print!(" ");
        }
        println!("|");
        // Draw the bottom row
        print!("+");
        for _ in 0..width - 1 {
            print!("--");
        }
        if width > 0 {
            print!("-");
        }
        println!("+");
    }
    pub fn add_walls(&mut self) {
        let mut forest = Forest::new();
        let mut edge_set = vec![];
        let width = self.width;
        let height = self.height;
        let mut distinct = width * height;
        for j in 0..self.height {
            for i in 0..self.width {
                let tree_num = (width * j + i) as usize;
                forest.push(tree_num);
                if i < width - 1 && j < height - 1 {
                    edge_set.push(Edge::new((i, j), (i + 1, j)));
                    edge_set.push(Edge::new((i, j), (i, j + 1)));
                } else if i < width - 1 {
                    edge_set.push(Edge::new((i, j), (i + 1, j)));
                } else if j < height - 1 {
                    edge_set.push(Edge::new((i, j), (i, j + 1)));
                }
            }
        }
        let mut rng = rand::thread_rng();
        while distinct > 1 {
            let edge_index = rng.gen_range(0..edge_set.len());
            let edge = &edge_set[edge_index];
            let p1 = edge.p1.0 + width * edge.p1.1;
            let p2 = edge.p2.0 + width * edge.p2.1;
            if forest.find(p1) != forest.find(p2) {
                edge_set.remove(edge_index);
                forest.union(p1, p2);
                distinct -= 1;
            }
        }
        self.edges = edge_set.clone().into_iter().collect();
    }
}

pub type Point = (usize, usize);

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
