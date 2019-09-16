use crate::algebra::prelude::*;
//use crate::core::medium::{HomogeneousMedium, Medium};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    /// Time ray was cast at
    pub time: f64,
    //pub medium: Box<dyn Medium + 'a>,
    pub min_t: f64,
    pub max_t: f64,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Point3::ORIGIN,
            direction: Vec3::ORIGIN,
            time: 0.0,
            //medium: Box::new(HomogeneousMedium::default()),
            min_t: 0.0,
            max_t: std::f64::INFINITY,
        }
    }
}

impl Ray {
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

impl Transformable for Ray {
    fn apply_t(self, trans: &Transform) -> Self {
        Ray {
            origin: self.origin.apply_t(trans),
            direction: self.direction.apply_t(trans),
            ..self
        }
    }
}
