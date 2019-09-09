use crate::algebra::prelude::*;
use std::ops::{Add, Index, Mul};

/// Compute the dot product between two elements
pub fn dot<A: Index<usize, Output = f64>, B: Index<usize, Output = f64>>(a: &A, b: &B) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Interpolate between two elements
pub fn lerp<T: Mul<f64, Output = T> + Add<Output = T> + Copy>(t: f64, a: &T, b: &T) -> T {
    *a * (1.0 - t) + *b * t
}

/// Calculate the cross-vector
pub fn cross<A: Index<usize, Output = f64>, B: Index<usize, Output = f64>>(a: &A, b: &B) -> Vec3 {
    Vec3::new(
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    )
}

/// Make an object clampable between two instances of itself
/// # Example:
/// ```
/// use thruster::algebra::prelude::*;
/// println!("{}", (5f64).clamp_to(2.0, 10.0));
/// //=>  2.0
/// println!("{}", (14f64).clamp_to(2.0, 10.0));
/// //=> 10.0
/// ```
pub trait Clampable {
    fn clamp_to(self, min: Self, max: Self) -> Self;
}

impl Clampable for f64 {
    fn clamp_to(self, min: f64, max: f64) -> f64 {
        if self > max {
            return max;
        }
        if self < min {
            return min;
        }
        self
    }
}

pub fn spherical_direction(sin_theta: f64, cos_theta: f64, phi: f64) -> Vec3 {
    Vec3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
}

pub fn spherical_direction_axes(
    sin_theta: f64,
    cos_theta: f64,
    phi: f64,
    x: &Vec3,
    y: &Vec3,
    z: &Vec3,
) -> Vec3 {
    sin_theta * phi.cos() * *x + sin_theta * phi.sin() * *y + cos_theta * *z
}

pub fn spherical_theta(v: &Vec3) -> f64 {
    v.z.clamp_to(-1.0, 1.0).acos()
}

pub fn spherical_phi(v: &Vec3) -> f64 {
    let p = v.x.atan2(v.y);
    if p < 0.0 {
        p + 2.0 * std::f64::consts::PI
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_test() {
        assert_eq!(
            comb::dot(&Vec3::new(1.0, 5.0, 3.0), &Vec3::new(2.0, 0.0, 0.0)),
            2.0
        );
        assert_eq!(
            comb::dot(&Vec3::new(0.0, 1.0, 0.0), &Vec3::new(0.0, 1.0, 0.0)),
            1.0
        );
        assert_eq!(
            comb::dot(&Vec3::new(0.0, 0.0, 1.0), &Vec3::new(0.0, 1.0, 0.0)),
            0.0
        );
        assert_eq!(
            comb::dot(&Vec3::new(0.0, -1.0, 0.0), &Vec3::new(0.0, 1.0, 0.0)),
            -1.0
        );
    }

    #[test]
    fn lerp() {
        let a = Vec3::new(0.0, 100.0, 0.0);
        let b = Vec3::new(100.0, 0.0, 0.0);
        let c = 0.5;

        let res = comb::lerp(c, &a, &b);
        assert_eq!(res, Vec3::new(50.0, 50.0, 0.0));
    }
}
