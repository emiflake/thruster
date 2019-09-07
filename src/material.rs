use crate::algebra::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MatTex {
    Color(Vec3),
    Texture { handle: String, scaling: Vec2 },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Transparency {
    pub index_of_refraction: f64,
    pub blurriness: f64,
    pub amount: f64,
}

impl Transparency {
    pub fn is_transparent(&self) -> bool {
        self.amount > 0.0
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Reflectivity {
    pub amount: f64,
    pub blurriness: f64,
}

impl Reflectivity {
    pub fn is_reflective(&self) -> bool {
        self.amount > 0.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Material {
    pub texture: MatTex,
    pub c_diffuse: f64,
    pub c_ambient: f64,
    pub reflectivity: Reflectivity,
    pub transparency: Transparency,
}

impl Material {
    pub fn draw_ui(&mut self, ui: &imgui::Ui) {
        let mut diffuse = self.c_diffuse as f32;
        let mut ambient = self.c_ambient as f32;
        let mut reflection = self.reflectivity.amount as f32;
        ui.text("-> Material");
        ui.separator();
        ui.input_float(im_str!("Diffuse amount"), &mut diffuse)
            .build();
        ui.input_float(im_str!("Ambient amount"), &mut ambient)
            .build();
        ui.input_float(im_str!("Reflection amount"), &mut reflection)
            .build();
        self.c_diffuse = f64::from(diffuse);
        self.c_ambient = f64::from(ambient);
        self.reflectivity.amount = f64::from(reflection);
    }
}
