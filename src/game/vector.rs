use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::constants::GRID_SIZE;
use crate::game::direction::GridDirection;
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GridVector {
    pub x: i32,
    pub y: i32,
}

impl GridVector {
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_transform(transform: &Transform) -> Self {
        Self::from_vec3(&transform.translation)
    }

    pub fn from_vec3(vec: &Vec3) -> Self {
        Self::new((vec.x / GRID_SIZE) as i32, (vec.y / GRID_SIZE) as i32)
    }

    pub fn to_vec3(&self, z: f32) -> Vec3 {
        Vec3::new(self.x as f32 * GRID_SIZE, self.y as f32 * GRID_SIZE, z)
    }

    pub fn from_direction(direction: &GridDirection) -> Self {
        match direction {
            GridDirection::North => Self::new(0, 1),
            GridDirection::NorthEast => Self::new(1, 1),
            GridDirection::East => Self::new(1, 0),
            GridDirection::SouthEast => Self::new(1, -1),
            GridDirection::South => Self::new(0, -1),
            GridDirection::SouthWest => Self::new(-1, -1),
            GridDirection::West => Self::new(-1, 0),
            GridDirection::NorthWest => Self::new(-1, 1),
        }
    }

    pub fn vec_x(&self) -> f32 {
        (self.x as f32) * GRID_SIZE
    }
    pub fn vec_y(&self) -> f32 {
        (self.y as f32) * GRID_SIZE
    }

    pub fn distance_max(&self, rhs: &Self) -> i32 {
        let x_dist = (self.x - rhs.x).abs();
        let y_dist = (self.y - rhs.y).abs();
        x_dist.max(y_dist)
    }
}

impl Neg for GridVector {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(self.x, self.y)
    }
}

impl Add<GridVector> for GridVector {
    type Output = Self;
    #[inline]
    fn add(self, rhs: GridVector) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<GridVector> for GridVector {
    fn add_assign(&mut self, rhs: GridVector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<GridVector> for GridVector {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: GridVector) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<GridVector> for GridVector {
    fn sub_assign(&mut self, rhs: GridVector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i32> for GridVector {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i32> for GridVector {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
