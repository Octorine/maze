mod maze;
use maze::{Maze, Edge};
fn main() {
    Maze{left: 0, right: 10, top: 0, bottom: 10, edges: vec!(Edge::new((1,1), (2, 1))).into_iter().collect()}.draw();
}
