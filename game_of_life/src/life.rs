use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

// cuenta vecinos vivos alrededor de una celda
pub fn count_neighbors(fb: &Framebuffer, x: i32, y: i32) -> u8 {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let wrapped_x = (x + dx).rem_euclid(fb.width);
            let wrapped_y = (y + dy).rem_euclid(fb.height);

            if fb.get_color(wrapped_x, wrapped_y) == Color::WHITE {
                count += 1;
            }
        }
    }

    count
}

// calcula la siguiente generacion
pub fn update(fb: &mut Framebuffer) {
    // almaceno temporalmente el siguiente estado
    let mut next_state = Vec::new();

    for y in 0..fb.height {
        for x in 0..fb.width {
            let vecinos = count_neighbors(fb, x, y);

            let viva = fb.get_color(x, y) == Color::WHITE;

            let nueva_viva = match (viva, vecinos) {
                // sobrevive
                (true, 2) | (true, 3) => true,

                // nace
                (false, 3) => true,

                // muere o sigue muerta
                _ => false,
            };

            if nueva_viva {
                next_state.push(Color::WHITE);
            } else {
                next_state.push(Color::BLACK);
            }
        }
    }

    // actualizo el framebuffer
    let mut i = 0;

    for y in 0..fb.height {
        for x in 0..fb.width {
            fb.point(x, y, next_state[i]);

            i += 1;
        }
    }
}
