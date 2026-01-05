use std::f32::consts::PI;

use bevy::math::{IVec2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
    East,
    North,
    West,
    South,
}

impl Dir {
    pub fn rotate_ccw(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
        }
    }

    pub fn rotate_cw(self) -> Self {
        match self {
            Self::East => Self::South,
            Self::North => Self::East,
            Self::West => Self::North,
            Self::South => Self::West,
        }
    }

    pub fn to_radians(self) -> f32 {
        match self {
            Dir::East => 0.,
            Dir::North => PI / 2.,
            Dir::West => PI,
            Dir::South => -PI / 2.,
        }
    }

    pub fn to_ivec(self) -> IVec2 {
        IVec2::from(self)
    }

    pub fn to_vec(self) -> Vec2 {
        Vec2::from(self)
    }
}

impl From<Dir> for IVec2 {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::East => Self::new(1, 0),
            Dir::North => Self::new(0, 1),
            Dir::West => Self::new(-1, 0),
            Dir::South => Self::new(0, -1),
        }
    }
}

impl From<Dir> for Vec2 {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::East => Self::new(1., 0.),
            Dir::North => Self::new(0., 1.),
            Dir::West => Self::new(-1., 0.),
            Dir::South => Self::new(0., -1.),
        }
    }
}
