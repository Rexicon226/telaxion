#![deny(unsafe_code)]

mod window;
mod ray;

use minifb::Key;
use rayon::prelude::*;
use glam::{vec2, Vec2, Vec2Swizzles, vec3};
use std::time::Instant;
use rand::Rng;
use crate::ray::scene::{Ray, Scene, Sphere};
use crate::window::window::{color_u32_from_vec4, RayCanvas, WindowInfo};

fn main() {
    let start_time = Instant::now();

    let width: i32 = 1280;
    let height: i32 = 720;

    let window_options: WindowInfo = WindowInfo {
        buffer: vec![0; (width * height) as usize],
        height,
        width,
        title: "Ray Tracing - ESC to exit".to_string(),
    };

    let mut ray_canvas = RayCanvas::new(window_options).unwrap();

    let mut scene = Scene {
        objects: vec![],
    };

    scene.objects.push(Box::new(Sphere {
        center: vec3(0.0, 0.0, 20.0),
        radius: 13.0,
    }));

    let aspect_ratio = width as f32 / height as f32;

    let mut buffer = (0..width as i32 * height as i32)
        .into_par_iter()
        .map(|i| {
            let x = i % width as i32;
            let y = i / width as i32;

            let mut rng = rand::thread_rng();

            let u = (x as f32 + rng.gen::<f32>()) / (width - 1) as f32;
            let v = (y as f32 + rng.gen::<f32>()) / (height - 1) as f32;


            let ray = Ray {
                origin: vec3(0.0, 0.0, 0.0),
                direction: (vec3(
                    (2.0 * u - 1.0) * aspect_ratio,
                    1.0 - 2.0 * v,
                    1.0 / (0.5 * std::f32::consts::PI),
                ) - vec3(0.0, 0.0, 0.5)).normalize(),
            };

            let color = scene.trace(&ray);

            color_u32_from_vec4(color)
        }).collect::<Vec<_>>();

    ray_canvas.window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while ray_canvas.window.is_open() && !ray_canvas.window.is_key_down(Key::Escape) {
        let start_time = Instant::now();

        ray_canvas.update(&mut buffer);

        println!("{} ms", start_time.elapsed().as_millis());
    }
}