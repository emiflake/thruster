#![allow(unused_variables)]

extern crate image;
extern crate rand;
extern crate rayon;
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

#[macro_use]
extern crate enumset;

/// Acceleration structures for speeding up rendering
pub mod acceleration;

/// Algebra & Maths
pub mod algebra;
//pub mod app;
pub mod bxdf;
pub mod core;
pub mod denoise;
pub mod dither;
pub mod geometry;
pub mod key_state;
pub mod light;
pub mod logger;
pub mod parser;
pub mod profiler;
pub mod sampler;
pub mod support;
pub mod utils;
