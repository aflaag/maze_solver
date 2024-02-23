#![allow(dead_code)]
#![allow(unused_variables)]

use image::{open, Rgb};
use maze_solver::maze::{ClampedAdd, ClampedMul, Maze, *};

fn lerp(first_color: Rgb<u8>, second_color: Rgb<u8>, t: f32) -> Rgb<u8> {
    first_color
        .clamped_mul(1.0 - t)
        .clamped_add(second_color.clamped_mul(t))
}

fn lerp_red_green(idx: f32, total: f32) -> Rgb<u8> {
    lerp(RED, GREEN, idx / total)
}

fn lerp_blue_cyan(idx: f32, total: f32) -> Rgb<u8> {
    lerp(BLUE, CYAN, idx / total)
}

fn alternated_blue_cyan(idx: f32, total: f32) -> Rgb<u8> {
    lerp(BLUE, CYAN, if idx % 2.0 == 0.0 { 0.0 } else { 1.0 })
}

fn main() {
    let mut image = open("maze.png").unwrap().into_rgb8();

    let mut maze = Maze::try_from(image.clone()).unwrap();

    maze.solve();

    maze.print_over_image(&mut image, lerp_blue_cyan);

    image.save("maze_solved.png").unwrap();
}
