mod color;
mod cube;
mod framebuffer;
mod light;
mod ray_intersect;

use color::Color;
use cube::Cube;
use framebuffer::Framebuffer;
use light::Light;
use nalgebra_glm::{cross, dot, normalize, Vec3};
use ray_intersect::{Intersect, Material, Object};
use raylib::prelude::*;
use std::f32::consts::PI;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FOV: f32 = PI / 3.0;
const AMBIENT: f32 = 0.18;

fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Object],
    light: &Light,
) -> Color {
    let mut closest = Intersect::empty();

    for object in objects {
        let hit = object.ray_intersect(ray_origin, ray_direction);
        if hit.is_intersecting && hit.distance < closest.distance {
            closest = hit;
        }
    }

    if !closest.is_intersecting {
        return Color::new(20, 24, 35);
    }

    let light_direction = normalize(&(light.position - closest.point));
    let diffuse_intensity = dot(&closest.normal, &light_direction).max(0.0);
    let lighting = AMBIENT + diffuse_intensity * light.intensity;

    closest
        .material
        .diffuse
        .multiply(light.color)
        .scale(lighting)
}

fn render(framebuffer: &mut Framebuffer, objects: &[Object], light: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let scale = (FOV / 2.0).tan();

    let camera = Vec3::new(3.0, 2.2, 4.5);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let forward = normalize(&(target - camera));
    let right = normalize(&cross(&forward, &Vec3::new(0.0, 1.0, 0.0)));
    let up = cross(&right, &forward);

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * (x as f32 + 0.5) / width - 1.0) * aspect_ratio * scale;
            let screen_y = (1.0 - 2.0 * (y as f32 + 0.5) / height) * scale;
            let ray_direction = normalize(&(forward + right * screen_x + up * screen_y));
            let pixel_color = cast_ray(&camera, &ray_direction, objects, light);

            framebuffer.point(x, y, pixel_color.to_raylib());
        }
    }
}

fn main() {
    let (mut window, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Ray Tracer Cube - Luz difusa")
        .build();

    let material = Material {
        diffuse: Color::new(70, 150, 230),
    };
    let objects: Vec<Object> = vec![Box::new(Cube {
        center: Vec3::new(0.0, 0.0, 0.0),
        size: 2.0,
        material,
    })];
    let light = Light {
        position: Vec3::new(4.0, 5.0, 3.0),
        color: Color::new(255, 255, 255),
        intensity: 0.9,
    };

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    render(&mut framebuffer, &objects, &light);
    let image = framebuffer.to_image();
    let texture = window
        .load_texture_from_image(&thread, &image)
        .expect("No se pudo crear la textura");

    window.set_target_fps(60);
    while !window.window_should_close() {
        let mut drawing = window.begin_drawing(&thread);
        drawing.clear_background(raylib::prelude::Color::BLACK);
        drawing.draw_texture(&texture, 0, 0, raylib::prelude::Color::WHITE);
    }
}
