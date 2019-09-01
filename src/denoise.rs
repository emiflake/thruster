//use oidn;

use image::{ImageBuffer, Rgba};

pub struct Denoiser;

impl Denoiser {
    fn create_input_vec(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<f32> {
        let mut vec = vec![0.0f32; (3 * image.width() * image.height()) as usize];
        for y in 0..image.height() {
            for x in 0..image.width() {
                let p = image.get_pixel(x, y);
                for c in 0..3 {
                    vec[3 * ((y * image.width() + x) as usize) + c] = p[c] as f32 / 255.0;
                }
            }
        }
        vec
    }

    fn create_output_image(
        (w, h): (u32, u32),
        out_vec: Vec<f32>,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut buf = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(w, h);

        for y in 0..h {
            for x in 0..w {
                let p = buf.get_pixel_mut(x, y);
                for c in 0..3 {
                    p[c] = (out_vec[3 * ((y * w + x) as usize) + c] * 255.0) as u8;
                }
                p[3] = 255;
            }
        }

        buf

        //ImageBuffer::from_vec(w, h, out_vec.iter().map(|p|*p as u8).collect()).expect("Could not make image buffer")
    }

    pub fn denoise(
        &self,
        image_buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (width, height) = (image_buf.width(), image_buf.height());

        let in_vec = Self::create_input_vec(image_buf.clone());
        let mut out_vec = vec![0.0f32; in_vec.len()];
        let mut device = oidn::Device::new();
        let mut filter = oidn::RayTracing::new(&mut device);
        filter
            .set_srgb(true)
            .set_img_dims(width as usize, height as usize);
        filter.execute(&in_vec[..], &mut out_vec[..]);
        if let Err(e) = device.get_error() {
            println!("Error denoising image: {}", e.1);
        }

        //image_buf

        Self::create_output_image((width, height), out_vec)
    }
}
