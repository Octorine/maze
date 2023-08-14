mod maze;
mod union_find;
use maze::Maze;
fn main() {
    let mut maze = Maze::new(10, 10);
    maze.add_walls();
    maze.draw();
}
