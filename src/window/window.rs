use glam::Vec4;
use minifb::{Window, WindowOptions};

pub struct WindowInfo {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub buffer: Vec<u32>,
}

pub struct RayCanvas {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub buffer: Vec<u32>,
    pub window: Window,
}

impl RayCanvas {
    pub fn new(window_info: WindowInfo) -> Result<RayCanvas, String> {
        let window = Window::new(
            &window_info.title,
            window_info.width as usize,
            window_info.height as usize,
            WindowOptions::default(),
        )
        .map_err(|e| e.to_string())?;

        Ok(RayCanvas {
            width: window_info.width,
            height: window_info.height,
            title: window_info.title,
            buffer: window_info.buffer,
            window,
        })
    }

    pub fn update(&mut self, buffer: &mut Vec<u32>) {
        self.window
            .update_with_buffer(&buffer, self.width as usize, self.height as usize)
            .unwrap();
    }
}

fn srgb_oetf(x: f32) -> f32 {
    if x <= 0.0031308 {
        x * 12.92
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}

pub fn color_u32_from_vec4(v: Vec4) -> u32 {
    let convert = |f: f32| -> u32 { (f.clamp(0.0, 1.0) * 255.0).round() as u32 };

    convert(srgb_oetf(v.z))
        | convert(srgb_oetf(v.y)) << 8
        | convert(srgb_oetf(v.x)) << 16
        | convert(v.w) << 24
}