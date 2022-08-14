use maze_solver::maze::{Maze, ClampedAdd, ClampedMul};
use image::{open, Rgb};

fn lerp(first_color: Rgb<u8>, second_color: Rgb<u8>, t: f32) -> Rgb<u8> {
    first_color.clamped_mul(1.0 - t).clamped_add(second_color.clamped_mul(t))
}

fn main() {
    let mut image = open("maze.png").unwrap().into_rgb8();

    let mut maze: Maze<201, 201> = Maze::try_from(&image).unwrap();

    maze.solve();

    maze.print_over_image(&mut image, lerp);

    image.save("maze_solved.png").unwrap();
}
