use maze_solver::maze::Maze;
use image::open;

fn main() {
    let mut image = open("maze.png").unwrap().into_rgb8();

    let mut maze: Maze<201, 201> = Maze::try_from(&image).unwrap();

    maze.solve();

    maze.print_over_image(&mut image);

    image.save("maze_solved.png").unwrap();
}
