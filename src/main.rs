#![deny(unsafe_code)]

mod window;
mod ray;

use minifb::Key;
use rayon::prelude::*;
use glam::{mat4, vec2, Vec2, Vec2Swizzles, vec3, vec4, Vec4Swizzles};
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

    let mut sphere = Sphere {
        center: vec3(0.0, 0.0, -20.0),
        radius: 13.0,
    };

    let sphere_ref = &sphere;
    let mut sphere_box = Box::new(*sphere_ref);
    scene.objects.push(sphere_box);

    let camera_pos = vec3(0.0, 0.0, 0.0);
    let camera_dir = vec3(0.0, 0.0, -1.0);
    let camera_up = vec3(0.0, 1.0, 0.0);


    let camera_z = camera_dir.normalize();
    let camera_x = camera_up.cross(camera_z).normalize();
    let camera_y = camera_z.cross(camera_x);


    let view_matrix = mat4(
        vec4(camera_x.x, camera_y.x, camera_z.x, 0.0),
        vec4(camera_x.y, camera_y.y, camera_z.y, 0.0),
        vec4(camera_x.z, camera_y.z, camera_z.z, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    );

    let aspect_ratio = width as f32 / height as f32;

    let mut buffer = (0..width as i32 * height as i32)
        .into_par_iter()
        .map(|i| {

            let x = i % width;
            let y = i / width;

            let ndc_x = (2.0 * x as f32 / (width - 1) as f32) - 1.0;
            let ndc_y = 1.0 - (2.0 * y as f32 / (height - 1) as f32);

            let ray_dir = (view_matrix * vec4(
                ndc_x * aspect_ratio,
                ndc_y,
                -1.0,
                0.0,
            ))
                .xyz()
                .normalize();

            let ray = Ray {
                origin: camera_pos,
                direction: ray_dir,
            };

            let color = scene.trace(&ray);

            color_u32_from_vec4(color)
        }).collect::<Vec<_>>();

    ray_canvas.update(&mut buffer);

    println!("Time taken to render: {}ms", start_time.elapsed().as_millis());

    ray_canvas.window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while ray_canvas.window.is_open() && !ray_canvas.window.is_key_down(Key::Escape) {
        ray_canvas.window.update();
    }
}