/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   texture_map.rs                                     :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/25 16:39:42 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/26 23:09:57 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use std::collections::HashMap;

use std::fs::File;
use std::io::BufReader;

use crate::material::MatTex;
use crate::scene::Scene;
use crate::shape::SceneObject;

#[derive(Debug, Clone)]
pub struct TextureMap {
    pub textures: HashMap<String, image::RgbImage>,
}

impl TextureMap {
    pub fn new() -> Self {
        TextureMap {
            textures: HashMap::new(),
        }
    }

    pub fn preload_all_in_scene(&mut self, scene: &Scene) {
        let mut paths: Vec<String> = Vec::new();

        for shape in &scene.shapes {
            match &shape.mat().texture {
                MatTex::Texture { handle, .. } => paths.push(handle.clone()),
                _ => {}
            }
        }

        for path in &scene.skybox.handles {
            paths.push(path.clone());
        }

        for path in &paths {
            self.load_image_from_file(path)
                .expect(&format!("Failed to load {} while preloading", path));
        }
    }

    pub fn load_image_from_file(&mut self, filename: &str) -> Result<(), String> {
        let mut f = File::open(filename)
            .map_err(|_| format!("Could not open file of image {}	", filename))?;
        let reader = BufReader::new(&mut f);
        let img = image::load(reader, image::ImageFormat::PNG)
            .map_err(|_| "Could not load image as PNG")?;
        let index = self.textures.len();
        self.textures.insert(filename.to_owned(), img.to_rgb());
        Ok(())
    }

    pub fn get_image_by_handle(
        &self,
        handle: String,
    ) -> std::result::Result<&image::RgbImage, String> {
        match self.textures.get(&handle).as_ref() {
            Some(img_ref) => Ok(img_ref),
            None => Err(format!(
                "Could not get image from that handle ({})!",
                handle
            )),
        }
    }
}
