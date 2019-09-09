use crate::algebra::prelude::*;
use crate::core::medium::{HomogeneousMedium, Medium};

#[derive(Debug)]
pub struct Ray<'a> {
    pub origin: Point3,
    pub direction: Vec3,
    /// Time ray was cast at
    pub time: f64,
    pub medium: Box<dyn Medium + 'a>,

    pub min_t: f64,
    pub max_t: f64,
}

impl<'a> Default for Ray<'a> {
    fn default() -> Self {
        Self {
            origin: Point3::ORIGIN,
            direction: Vec3::ORIGIN,
            time: 0.0,
            medium: Box::new(HomogeneousMedium::default()),
            min_t: 0.0,
            max_t: std::f64::INFINITY,
        }
    }
}

impl<'a> Ray<'a> {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            ..Self::default()
        }
    }

    pub fn new_with_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            time,
            ..Self::new(origin, direction)
        }
    }
}
