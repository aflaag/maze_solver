use image::{RgbImage, Rgb};
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MazeCell {
    Wall,
    Path,
    TraversedPath,
    Start,
    End,
}

impl MazeCell {
    pub fn is_wall(&self) -> bool {
        *self == MazeCell::Wall
    }

    pub fn is_path(&self) -> bool {
        *self == MazeCell::Path
    }

    pub fn is_start(&self) -> bool {
        *self == MazeCell::Start
    }

    pub fn is_end(&self) -> bool {
        *self == MazeCell::End
    }
}

impl Default for MazeCell {
    fn default() -> Self {
        Self::Wall
    }
}

impl std::fmt::Display for MazeCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Wall => write!(f, "W"),
            Self::Path => write!(f, "P"),
            Self::TraversedPath => write!(f, " "),
            Self::Start => write!(f, "S"),
            Self::End => write!(f, "E"),
        }
    }
}

impl From<MazeCell> for Rgb<u8> {
    fn from(mazecell: MazeCell) -> Self {
        match mazecell {
            MazeCell::Wall => Rgb([0, 0, 0]),
            MazeCell::Path => Rgb([255, 255, 255]),
            MazeCell::TraversedPath => Rgb([0, 0, 255]),
            MazeCell::Start => Rgb([255, 0, 0]),
            MazeCell::End => Rgb([0, 255, 0]),
        }
    }
}

impl TryFrom<(u8, u8, u8)> for MazeCell {
    type Error = MazeError;

    fn try_from(pixel: (u8, u8, u8)) -> Result<Self, Self::Error> {
        match pixel {
            (0, 0, 0) => Ok(Self::Wall),
            (255, 255, 255) => Ok(Self::Path),
            (255, 0, 0) => Ok(Self::Start),
            (0, 255, 0) => Ok(Self::End),
            _ => Err(MazeError::InvalidPixelColor),
        }
    }
}

impl TryFrom<Rgb<u8>> for MazeCell {
    type Error = MazeError;

    fn try_from(pixel: Rgb<u8>) -> Result<Self, Self::Error> {
        match pixel.0 {
            [0, 0, 0] => Ok(Self::Wall),
            [255, 255, 255] => Ok(Self::Path),
            [255, 0, 0] => Ok(Self::Start),
            [0, 255, 0] => Ok(Self::End),
            _ => Err(MazeError::InvalidPixelColor),
        }
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

    pub fn print_over_image(&self, image: &mut RgbImage) { // TODO: should be generic
        self.path
            .iter()
            .for_each(|(x, y)| {
                image.put_pixel(*x as u32, *y as u32, self.maze[*y][*x].into())
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
