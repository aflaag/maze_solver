use crate::mazecell::MazeCell;
use image::{RgbImage, Rgb};

pub trait ClampedAdd<Rhs = Self> {
    type Output;

    fn clamped_add(&self, rhs: Rhs) -> Self::Output;
}

pub trait ClampedMul<Rhs = Self> {
    type Output;

    fn clamped_mul(&self, rhs: Rhs) -> Self::Output;
}

impl ClampedMul<f32> for Rgb<u8> {
    type Output = Rgb<u8>;

    fn clamped_mul(&self, rhs: f32) -> Self::Output {
        let rgb = self.0;

        let raw_r = (rgb[0] as f32 * rhs).round();
        let raw_g = (rgb[1] as f32 * rhs).round();
        let raw_b = (rgb[2] as f32 * rhs).round();

        let r = if raw_r > u8::MAX as f32 {
            u8::MAX
        } else if raw_r < u8::MIN as f32 {
            u8::MIN
        } else {
            raw_r as u8
        };

        let g = if raw_g > u8::MAX as f32 {
            u8::MAX
        } else if raw_g < u8::MIN as f32 { 
            u8::MIN
        } else {
            raw_g as u8
        };

        let b = if raw_b > u8::MAX as f32 {
            u8::MAX
        } else if raw_b < u8::MIN as f32 {
            u8::MIN
        } else {
            raw_b as u8
        };

        Rgb([r, g, b])
    }
}

impl ClampedAdd for Rgb<u8> {
    type Output = Rgb<u8>;

    fn clamped_add(&self, rhs: Rgb<u8>) -> Self::Output {
        let rgb_self = self.0;
        let rgb_rhs = rhs.0;

        let raw_r = rgb_self[0].checked_add(rgb_rhs[0]);
        let raw_g = rgb_self[1].checked_add(rgb_rhs[1]);
        let raw_b = rgb_self[2].checked_add(rgb_rhs[2]);

        let r = if let Some(r) = raw_r {
            r
        } else {
            u8::MAX
        };

        let g = if let Some(g) = raw_g {
            g
        } else {
            u8::MAX
        };

        let b = if let Some(b) = raw_b {
            b
        } else {
            u8::MAX
        };

        Rgb([r, g, b])
     }
}

pub struct Maze<const W: usize, const H: usize> {
    maze: [[MazeCell; W]; H],
    start: (usize, usize),
    end: (usize, usize),
    path: Vec<(usize, usize)>,
    is_solved: bool,
}

impl<const W: usize, const H: usize> Maze<W, H> {
    fn get_directions((x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut directions = Vec::new();

        if x != 0 {
            directions.push((x - 1, y));
        }

        if x != W - 1 {
            directions.push((x + 1, y));
        }

        if y != 0 {
            directions.push((x, y - 1));
        }

        if y != H - 1 {
            directions.push((x, y + 1));
        }

        directions
    }

    fn solve_internals(&mut self, (x, y): (usize, usize)) {
        if (x, y) == self.end {
            self.is_solved = true;
            self.path.pop();

            return;
        }

        let mut directions = Self::get_directions((x, y));

        directions.retain(|(d_x, d_y)| self.maze[*d_y][*d_x].is_path() || self.maze[*d_y][*d_x].is_end());

        for direction in directions {
            let (d_x, d_y) = direction;

            self.maze[d_y][d_x] = MazeCell::TraversedPath;

            self.path.push(direction);

            self.solve_internals(direction);

            if self.is_solved {
                return;
            }

            self.maze[d_y][d_x] = MazeCell::Path;

            self.path.pop();
        }
    }

    pub fn solve(&mut self) {
        self.solve_internals(self.start);
    }

    pub fn path(&self) -> Vec<(usize, usize)> {
        self.path.clone()
    }

    // fn lerp(first_color: Rgb<u8>, second_color: Rgb<u8>, t: f32) -> Rgb<u8> {
    //     first_color.clamped_mul(1.0 - t).clamped_add(second_color.clamped_mul(t))
    // }

    // TODO: should be generic
    pub fn print_over_image<F>(&self, image: &mut RgbImage, f: F)
    where
        Rgb<u8>: ClampedMul<f32> + ClampedAdd,
        F: Fn(Rgb<u8>, Rgb<u8>, f32) -> Rgb<u8>,
    {
        let len = self.path.len() as f32;

        let red = Rgb([255, 0, 0]);
        let green = Rgb([0, 255, 0]);

        self.path
            .iter()
            .enumerate()
            .for_each(|(idx, (x, y))| {
                image.put_pixel(*x as u32, *y as u32, f(red, green, idx as f32 / len).into())
            })
    }
}

impl<const W: usize, const H: usize> TryFrom<&RgbImage> for Maze<W, H> {
    type Error = MazeError;

    fn try_from(image: &RgbImage) -> Result<Self, Self::Error> {
        if image.width() as usize != W || image.height() as usize != H {
            return Err(MazeError::IncompatibleDimensons)
        }

        let mut maze = [[MazeCell::default(); W]; H];

        if image.enumerate_pixels().any(|(_, _, pixel)| MazeCell::try_from(*pixel).is_err()) {
            return Err(MazeError::InvalidPixelColor);
        }

        image
            .enumerate_rows()
            .zip(maze.iter_mut())
            .for_each(|((_, pixel_row), row)| {
                pixel_row
                    .zip(row.iter_mut())
                    .for_each(|((_, _, pixel), element)| *element = MazeCell::try_from(*pixel).unwrap());
            });

        let start = image
            .enumerate_pixels()
            .position(|(_, _, pixel)| MazeCell::try_from(*pixel).unwrap() == MazeCell::Start);

        let end = image
            .enumerate_pixels()
            .position(|(_, _, pixel)| MazeCell::try_from(*pixel).unwrap() == MazeCell::End);

        match (start, end) {
            (Some(start_pos), Some(end_pos)) => {
                let start = (start_pos % W, start_pos / H);
                let end = (end_pos % W, end_pos / H);

                Ok(Self { maze, start, end, path: vec![start], is_solved: false })
            },
            (_, _) => Err(MazeError::MissingEndpoints)
        }
    }
}

#[derive(Debug)]
pub enum MazeError {
    InvalidPixelColor,
    IncompatibleDimensons,
    MissingEndpoints,
    TooManyEndpoints,
}

impl std::fmt::Display for MazeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidPixelColor => writeln!(f, "There are invalid colors inside the image."),
            Self::IncompatibleDimensons => writeln!(f, "The image has incompatible declared dimensions."),
            Self::MissingEndpoints => writeln!(f, "There are no endpoints (start and end)."),
            Self::TooManyEndpoints => writeln!(f, "There are too many endpoints (start and end)."),
        }
    }
}

impl std::error::Error for MazeError {}
