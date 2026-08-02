mod framebuffer;
mod life;
mod patterns;

use framebuffer::Framebuffer;
use life::update;
use patterns::load_initial_world;
use raylib::prelude::*;

fn main() {
    let escala = 8;

    let fb_width = 100;
    let fb_height = 100;

    let (mut rl, thread) = raylib::init()
        .size(fb_width * escala, fb_height * escala)
        .title("Game of Life")
        .build();

    rl.set_target_fps(8);

    let mut fb = Framebuffer::new(fb_width, fb_height, Color::BLACK);

    load_initial_world(&mut fb);

    while !rl.window_should_close() {
        update(&mut fb);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        for y in 0..fb.height {
            for x in 0..fb.width {
                let color = fb.get_color(x, y);

                d.draw_rectangle(x * escala, y * escala, escala, escala, color);
            }
        }
    }
}
