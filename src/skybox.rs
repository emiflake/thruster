use crate::algebra::{Vec2, Vec3};
use crate::scene::RenderData;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skybox {
    /* +x, -x, +y, -y, +z, -z */
    pub handles: [String; 6],
}

impl Skybox {
    pub fn get_uv(&self, v: Vec3) -> Option<(String, Vec2)> {
        let abs = v.map_all(&|n| n.abs());

        let is_positive_x = v.x > 0.0;
        let is_positive_y = v.y > 0.0;
        let is_positive_z = v.z > 0.0;
        #[inline]
        fn convert_to_uv(c: Vec2, max_axis: f64) -> Vec2 {
            Vec2::new(0.5 * (c.x / max_axis + 1.0), 0.5 * (c.y / max_axis + 1.0))
        }

        if is_positive_x && abs.x >= abs.y && abs.x >= abs.z {
            Some((
                self.handles[0].clone(),
                convert_to_uv(Vec2::new(-v.z, v.y), abs.x),
            ))
        } else if !is_positive_x && abs.x >= abs.y && abs.x >= abs.z {
            Some((
                self.handles[1].clone(),
                convert_to_uv(Vec2::new(v.z, v.y), abs.x),
            ))
        } else if is_positive_y && abs.y >= abs.x && abs.y >= abs.z {
            Some((
                self.handles[2].clone(),
                convert_to_uv(Vec2::new(v.x, -v.z), abs.y),
            ))
        } else if !is_positive_y && abs.y >= abs.x && abs.y >= abs.z {
            Some((
                self.handles[3].clone(),
                convert_to_uv(Vec2::new(v.x, v.z), abs.y),
            ))
        } else if is_positive_z && abs.z >= abs.y && abs.z >= abs.x {
            Some((
                self.handles[4].clone(),
                convert_to_uv(Vec2::new(v.x, v.y), abs.z),
            ))
        } else if !is_positive_z && abs.z >= abs.y && abs.z >= abs.x {
            Some((
                self.handles[5].clone(),
                convert_to_uv(Vec2::new(-v.x, v.y), abs.z),
            ))
        } else {
            None
        }
    }

    pub fn calc_color(&self, scene: &RenderData, v: Vec3) -> Option<Vec3> {
        let (handle, uv) = self.get_uv(v)?;
        let img = scene.texture_map.get_image_by_handle(handle).ok()?;
        let rgb = img.get_pixel(
            ((1.0 - uv.x) * f64::from(img.width())) as u32 % img.width(),
            ((1.0 - uv.y) * f64::from(img.height())) as u32 % img.height(),
        );
        Some(Vec3::from_rgb(*rgb))
    }
}
