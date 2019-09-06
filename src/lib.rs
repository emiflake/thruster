#![allow(unused_variables)]

extern crate image;
extern crate rand;
extern crate scoped_threadpool;

extern crate glutin;
#[macro_use]
extern crate glium;

#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

extern crate oidn;

#[macro_use]
extern crate serde_derive;
extern crate ron;
extern crate serde;

// C++'s nth-element provider in Rust
// (!?)
extern crate pdqselect;

pub mod acceleration;
pub mod algebra;
pub mod app;
pub mod camera;
pub mod denoise;
pub mod dither;
pub mod key_state;
pub mod lightsource;
pub mod material;
pub mod parser;
pub mod profiler;
pub mod render_config;
pub mod scene;
pub mod shape;
pub mod skybox;
pub mod support;
pub mod texture_map;
pub mod utils;
