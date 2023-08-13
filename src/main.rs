mod maze;
use maze::{Edge, Maze};
fn main() {
    Maze {
	width: 2,
	height: 2,
        edges: vec![Edge::new((0, 0), (0, 1)),
		    Edge::new((1, 0), (1, 1)),
		    Edge::new((0, 0), (1, 0)),
		    Edge::new((0, 1), (1, 1))

	].into_iter().collect(),
    }
    .draw();
}
