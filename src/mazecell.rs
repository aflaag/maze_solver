use crate::maze::MazeError;
use image::Rgb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MazeCell {
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
