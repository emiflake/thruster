use crate::camera::PerspectiveCamera;
use crate::lightsource::PointLight;
use crate::material::{MatTex, Material};
use crate::parser;
use crate::shape::{Intersectable, Plane, Sphere, Triangle, Vec2, Vec3, Vertex};
use crate::skybox::Skybox;
use crate::texture_map;
use crate::thruster;

pub fn make_world<'a>() -> Result<thruster::Thruster<'a>, String> {
	let mut texture_map = texture_map::TextureMap::new();

	let checker_handle = texture_map.load_image_from_file("./textures/checker.png")?;
	let earth_handle = texture_map.load_image_from_file("./textures/earth.png")?;

	let skybox = Skybox::new([
		texture_map.load_image_from_file("./skybox/miramar/miramar_bk.png")?,
		texture_map.load_image_from_file("./skybox/miramar/miramar_ft.png")?,
		texture_map.load_image_from_file("./skybox/miramar/miramar_up.png")?,
		texture_map.load_image_from_file("./skybox/miramar/miramar_dn.png")?,
		texture_map.load_image_from_file("./skybox/miramar/miramar_rt.png")?,
		texture_map.load_image_from_file("./skybox/miramar/miramar_lf.png")?,
	]);
	// let skybox = Skybox::new([
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/negx.png")?,
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/posx.png")?,
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/posy.png")?,
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/negy.png")?,
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/posz.png")?,
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/negz.png")?,
	// ]);

	let checker_mattex = MatTex::from_handle(checker_handle, Vec2::new(1000.0, 1000.0));
	let earth_mattex = MatTex::from_handle(earth_handle, Vec2::new(1.0, 1.0));
	let plane_mat = Material::diffuse(checker_mattex);
	let earth_mat = Material::diffuse(earth_mattex);

	let red = MatTex::Color(Vec3::new(255.0, 0.0, 0.0));
	let red_mat = Material {
		c_diffuse: 0.7,
		c_reflection: 0.3,
		c_ambient: 0.0,
		texture: red,
	};
	let black = MatTex::Color(Vec3::new(255.0, 255.0, 255.0));
	let black_mat = Material {
		c_diffuse: 0.7,
		c_reflection: 0.3,
		c_ambient: 0.0,
		texture: black,
	};
	let green = MatTex::Color(Vec3::new(0.0, 255.0, 0.0));
	let green_mat = Material {
		c_diffuse: 0.7,
		c_reflection: 0.0,
		c_ambient: 0.3,
		texture: green,
	};

	let obj = parser::parse("./objs/teapot.obj".to_string());
	let mut scene: Vec<Box<dyn Intersectable + Sync>> = Vec::new();
	for (avt, bvt, cvt) in obj.triangles.iter() {
		scene.push(Box::new(Triangle {
			a: Vertex::from_parsed(avt),
			b: Vertex::from_parsed(bvt),
			c: Vertex::from_parsed(cvt),
			material: red_mat,
		}))
	}
	scene.extend::<Vec<Box<dyn Intersectable + Sync>>>(vec![
		Box::new(Plane {
			origin: Vec3::new(0.0, -1.0, 0.0),
			normal: Vec3::new(0.0, 1.0, 0.0),
			material: plane_mat,
		}),
		Box::new(Sphere {
			origin: Vec3::new(-75.0, 100.0, 50.0),
			radius: 50.0,
			material: red_mat,
		}),
		Box::new(Sphere {
			origin: Vec3::new(0.0, 100.0, 25.0),
			radius: 25.0,
			material: green_mat,
		}),
		Box::new(Sphere {
			origin: Vec3::new(75.0, 80.0, 50.0),
			radius: 50.0,
			material: earth_mat,
		}),
	]);

	#[allow(unused_mut)]
	let mut thruster = thruster::Thruster {
		camera: PerspectiveCamera::new(Vec3::new(0.0, 50.0, -200.0)),
		shapes: scene,
		lights: vec![Box::new(PointLight {
			origin: Vec3::new(1.0, 250.0, -30.0),
			color: Vec3::new(255.0, 255.0, 255.0),
		})],
		texture_map,
		skybox,
	};

	Ok(thruster)
}
