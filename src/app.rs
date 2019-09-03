use crate::key_state::Keystate;
use crate::profiler::Profiler;
use crate::scene::Scene;
use crate::shape::SceneObject;
use crate::support;
use crate::texture_map::TextureMap;

use glium::Surface;

use imgui::Context;
use imgui::*;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::fs::OpenOptions;
use std::io::Write;

use crate::algebra::Vec3;

use std::thread;
#[allow(unused_imports)]
use std::time::{Duration, Instant};

pub struct App {
    pub scene: Scene,
    pub texture_map: TextureMap,
}

impl App {
    pub fn new(scene: Scene, mut texture_map: TextureMap) -> App {
        texture_map.preload_all_in_scene(&scene);
        Self { scene, texture_map }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut event_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new()
            .with_dimensions(glutin::dpi::LogicalSize::new(1280.0, 720.0));
        let cb = glutin::ContextBuilder::new().with_vsync(true);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        let mut imgui = Context::create();
        imgui.set_ini_filename(None);
        let mut platform = WinitPlatform::init(&mut imgui);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
        }

        // Raw image rendering output
        let program = support::get_program(&display);
        let (vertex_buffer, index_buffer) = support::get_buffers(&display);

        let mut keystate = Keystate::default();

        // * Variables during looping * //
        let mut profiler = Profiler::new(100);
        let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &display).unwrap();
        let mut last_frame = Instant::now();
        let mut closed = false;
        let mut dimensions: [i32; 2] = [3840, 2160];

        while !closed {
            let gl_window = display.gl_window();
            let window = gl_window.window();

            let now = Instant::now();
            let delta = now - last_frame;
            let delta_time = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
            profiler.record_delay(delta_time);

            // Event handling
            event_loop.poll_events(|event| {
                platform.handle_event(imgui.io_mut(), &window, &event);
                keystate.handle_event(&event);

                match event {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        _ => (),
                    },
                    _ => (),
                }
            });

            // IMGUI PREPARE
            let io = imgui.io_mut();
            last_frame = io.update_delta_time(last_frame);
            let mut ui = imgui.frame();
            profiler.draw_ui(delta_time, &mut ui);

            self.scene.config.draw_ui(&mut ui);

            //imgui::Window::new(&ui, im_str!("Object Viewer"))
            //.size([350.0, 650.0], Condition::FirstUseEver)
            //.position([800.0, 50.0], Condition::FirstUseEver)
            //.scroll_bar(true)
            //.build(|| {
            //if imgui::Ui::button(&ui, im_str!("Save"), [100.0, 25.0]) {
            //let ron_str = ron::ser::to_string_pretty(
            //&self.scene,
            //ron::ser::PrettyConfig::default(),
            //)
            //.expect("Could not generate RON");
            //let mut f = std::fs::OpenOptions::new()
            //.write(true)
            //.create(true)
            //.open("./cfg.ron")
            //.unwrap();
            //f.write_all(ron_str.as_bytes())
            //.expect("Could not save 'cfg.ron'");
            //}
            //ui.separator();

            //for (i, sceneobject) in self.scene.shapes.iter_mut().enumerate() {
            //ui.push_id(imgui::ImId::Int(i as i32));
            //sceneobject.draw_ui(&ui);
            //sceneobject.mat_mut().draw_ui(&ui);
            //ui.pop_id();
            //}
            //});

            imgui::Window::new(&ui, im_str!("Screenshot"))
                .size([300.0, 150.0], Condition::FirstUseEver)
                .position([50.0, 500.0], Condition::FirstUseEver)
                .build(|| {
                    ui.input_int(im_str!("Width"), &mut dimensions[0]).build();
                    ui.input_int(im_str!("Height"), &mut dimensions[1]).build();
                    if dimensions[0] <= 0 {
                        dimensions[0] = 640;
                    }
                    if dimensions[1] <= 0 {
                        dimensions[1] = 480;
                    }
                    if imgui::Ui::button(&ui, im_str!("Take Screenshot"), [175.0, 50.0]) {
                        let before = self.scene.config.denoise;
                        self.scene.config.denoise = true;
                        self.scene
                            .screenshot(
                                "screenshot.png",
                                f64::from(dimensions[0]),
                                f64::from(dimensions[1]),
                                &self.texture_map,
                            )
                            .expect("Could not take screenshot");
                        self.scene.config.denoise = before;

                        std::process::Command::new("open")
                            .arg("screenshot.png")
                            .output()
                            .expect("failed to execute process");
                    }
                });

            if keystate.is_key_down(glutin::VirtualKeyCode::Escape) {
                closed = true;
            }

            handle_keys(&keystate, &mut self.scene, delta_time as f64);

            let image = self.scene.new_render(640.0, 360.0, &self.texture_map);
            let image_dimensions = image.dimensions();
            let raw_pixels = image.into_raw();

            let mut target = display.draw();
            target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);

            let image =
                glium::texture::RawImage2d::from_raw_rgba_reversed(&raw_pixels, image_dimensions);
            let opengl_texture =
                glium::texture::CompressedSrgbTexture2d::new(&display, image).unwrap();

            let uniforms = uniform! {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32]
                ],
                tex: &opengl_texture
            };
            target
                .draw(
                    &vertex_buffer,
                    &index_buffer,
                    &program,
                    &uniforms,
                    &Default::default(),
                )
                .unwrap();

            // IMGUI RENDER
            let draw_data = ui.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Could not render imgui");

            target.finish().expect("Failed to swap buffers");
            thread::sleep(std::time::Duration::from_millis(16));
        }

        Ok(())
    }
}

pub fn handle_keys(keystate: &Keystate, scene: &mut Scene, dt: f64) {
    let speed = if keystate.is_key_down(glutin::VirtualKeyCode::LShift) {
        250.0 * dt
    } else {
        10.0 * dt
    };

    if keystate.is_key_down(glutin::VirtualKeyCode::A) {
        scene.camera.translate(Vec3::new(-speed, 0.0, 0.0));
    }
    if keystate.is_key_down(glutin::VirtualKeyCode::D) {
        scene.camera.translate(Vec3::new(speed, 0.0, 0.0));
    }
    if keystate.is_key_down(glutin::VirtualKeyCode::S) {
        scene.camera.translate(Vec3::new(0.0, 0.0, speed));
    }
    if keystate.is_key_down(glutin::VirtualKeyCode::W) {
        scene.camera.translate(Vec3::new(0.0, 0.0, -speed));
    }
    if keystate.is_key_down(glutin::VirtualKeyCode::E) {
        scene.camera.translate(Vec3::new(0.0, speed, 0.0));
    }
    if keystate.is_key_down(glutin::VirtualKeyCode::Q) {
        scene.camera.translate(Vec3::new(0.0, -speed, 0.0));
    }
    if keystate.is_key_down(glutin::VirtualKeyCode::Left) {
        scene.camera.rotate(Vec3::new(0.0, -0.05, 0.0));
    }
    if keystate.is_key_down(glutin::VirtualKeyCode::Right) {
        scene.camera.rotate(Vec3::new(0.0, 0.05, 0.0));
    }
}
