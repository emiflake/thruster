pub struct RenderConfig {
    pub reflection_spp: i32,
    pub refraction_spp: i32,
    pub shadow_spp: i32,
    pub distributed_tracing: bool,
    pub recursion_depth: i32,
    pub denoise: bool,
    pub dither: bool,
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
        }
    }
}
