use maze::Maze;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let width: usize = args.get(1).and_then(|num| num.parse().ok()).unwrap_or(1);
    let height: usize = args.get(2).and_then(|num| num.parse().ok()).unwrap_or(1);
    let mut maze = Maze::new(width, height);
    maze.add_walls();
    maze.draw();
}
