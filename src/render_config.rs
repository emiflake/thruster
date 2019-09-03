pub struct RenderConfig {
    pub reflection_spp: i32,
    pub refraction_spp: i32,
    pub shadow_spp: i32,
    pub distributed_tracing: bool,
    pub recursion_depth: i32,
    pub denoise: bool,
    pub dither: bool,
    pub multi_thread: bool,
    pub reflections: bool,
    pub refractions: bool,
    pub shadows: bool,
    pub textures: bool,
    pub skybox: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            reflection_spp: 3,
            refraction_spp: 3,
            shadow_spp: 3,
            distributed_tracing: true,
            recursion_depth: 3,
            denoise: true,
            dither: false,
            multi_thread: true,

            reflections: true,
            refractions: true,
            shadows: true,
            textures: true,
            skybox: true,
        }
    }
}

use imgui::*;
impl RenderConfig {
    pub fn draw_ui(&mut self, ui: &mut imgui::Ui) {
        imgui::Window::new(&ui, im_str!("Config"))
            .size([400.0, 175.0], Condition::FirstUseEver)
            .scroll_bar(true)
            .build(|| {
                ui.checkbox(im_str!("Parallel Rendering"), &mut self.multi_thread);
                ui.checkbox(
                    im_str!("Distributed tracing"),
                    &mut self.distributed_tracing,
                );
                ui.slider_int(im_str!("Shadow SPP"), &mut self.shadow_spp, 1, 15)
                    .build();
                ui.slider_int(im_str!("Reflection SPP"), &mut self.reflection_spp, 1, 15)
                    .build();
                ui.slider_int(im_str!("Refraction SPP"), &mut self.refraction_spp, 1, 15)
                    .build();
                ui.slider_int(im_str!("Recursion depth"), &mut self.recursion_depth, 1, 15)
                    .build();
                ui.checkbox(im_str!("Dither"), &mut self.dither);
                ui.checkbox(im_str!("Reflections"), &mut self.reflections);
                ui.checkbox(im_str!("Refractions"), &mut self.refractions);
                ui.checkbox(im_str!("Shadows"), &mut self.shadows);
                ui.checkbox(im_str!("Textures"), &mut self.textures);
                ui.checkbox(im_str!("Skybox"), &mut self.skybox);
            });
    }
}
