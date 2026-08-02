use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

// glider clasico
pub fn glider() -> Vec<(i32, i32)> {
    vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]
}

// oscilador simple
pub fn blinker() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (2, 0)]
}

// oscilador tipo sapo
pub fn toad() -> Vec<(i32, i32)> {
    vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)]
}

// coloca un patron en el tablero
pub fn place_pattern(fb: &mut Framebuffer, pattern: &[(i32, i32)], start_x: i32, start_y: i32) {
    for &(dx, dy) in pattern {
        fb.point(start_x + dx, start_y + dy, Color::WHITE);
    }
}

pub fn load_initial_world(fb: &mut Framebuffer) {
    place_pattern(fb, &glider(), 4, 4);
    place_pattern(fb, &glider(), 20, 14);
    place_pattern(fb, &blinker(), 7, 32);
    place_pattern(fb, &toad(), 22, 31);
    place_pattern(fb, &block(), 42, 8);
    place_pattern(fb, &beehive(), 55, 9);
    place_pattern(fb, &boat(), 70, 8);
    place_pattern(fb, &tub(), 86, 10);
    place_pattern(fb, &beacon(), 8, 58);
    place_pattern(fb, &loaf(), 25, 57);
    place_pattern(fb, &lwss(), 50, 55);
    place_pattern(fb, &mwss(), 69, 54);
    place_pattern(fb, &hwss(), 6, 80);
    place_pattern(fb, &pulsar(), 39, 75);
    place_pattern(fb, &pentadecathlon(), 78, 73);

    place_pattern(fb, &gosper_glider_gun(), 34, 28);
}

//block
// estructura estable
pub fn block() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (0, 1), (1, 1)]
}

//beehive
// estructura estable tipo colmena
// estructura estable tipo colmena
pub fn beehive() -> Vec<(i32, i32)> {
    vec![(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)]
}

//boat
// estructura estable tipo bote
pub fn boat() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)]
}

//tub
// estructura estable tipo tina
pub fn tub() -> Vec<(i32, i32)> {
    vec![(1, 0), (0, 1), (2, 1), (1, 2)]
}

//beacon
// oscilador de periodo 2
pub fn beacon() -> Vec<(i32, i32)> {
    vec![
        (0, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 2),
        (2, 3),
        (3, 3),
    ]
}

// estructura estable tipo pan
pub fn loaf() -> Vec<(i32, i32)> {
    vec![(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)]
}

// nave ligera que se desplaza horizontalmente
pub fn lwss() -> Vec<(i32, i32)> {
    vec![
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (0, 1),
        (4, 1),
        (4, 2),
        (0, 3),
        (3, 3),
    ]
}

pub fn mwss() -> Vec<(i32, i32)> {
    vec![
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (1, 1),
        (5, 1),
        (5, 2),
        (1, 3),
        (4, 3),
        (3, 4),
    ]
}

pub fn hwss() -> Vec<(i32, i32)> {
    vec![
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (1, 1),
        (6, 1),
        (6, 2),
        (1, 3),
        (5, 3),
        (3, 4),
        (4, 4),
    ]
}

pub fn pulsar() -> Vec<(i32, i32)> {
    vec![
        (2, 0),
        (3, 0),
        (4, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (0, 2),
        (5, 2),
        (7, 2),
        (12, 2),
        (0, 3),
        (5, 3),
        (7, 3),
        (12, 3),
        (0, 4),
        (5, 4),
        (7, 4),
        (12, 4),
        (2, 5),
        (3, 5),
        (4, 5),
        (8, 5),
        (9, 5),
        (10, 5),
        (2, 7),
        (3, 7),
        (4, 7),
        (8, 7),
        (9, 7),
        (10, 7),
        (0, 8),
        (5, 8),
        (7, 8),
        (12, 8),
        (0, 9),
        (5, 9),
        (7, 9),
        (12, 9),
        (0, 10),
        (5, 10),
        (7, 10),
        (12, 10),
        (2, 12),
        (3, 12),
        (4, 12),
        (8, 12),
        (9, 12),
        (10, 12),
    ]
}

pub fn pentadecathlon() -> Vec<(i32, i32)> {
    vec![
        (1, 0),
        (1, 1),
        (0, 2),
        (2, 2),
        (1, 3),
        (1, 4),
        (1, 5),
        (1, 6),
        (0, 7),
        (2, 7),
        (1, 8),
        (1, 9),
    ]
}

pub fn gosper_glider_gun() -> Vec<(i32, i32)> {
    vec![
        (0, 4),
        (0, 5),
        (1, 4),
        (1, 5),
        (10, 4),
        (10, 5),
        (10, 6),
        (11, 3),
        (11, 7),
        (12, 2),
        (12, 8),
        (13, 2),
        (13, 8),
        (14, 5),
        (15, 3),
        (15, 7),
        (16, 4),
        (16, 5),
        (16, 6),
        (17, 5),
        (20, 2),
        (20, 3),
        (20, 4),
        (21, 2),
        (21, 3),
        (21, 4),
        (22, 1),
        (22, 5),
        (24, 0),
        (24, 1),
        (24, 5),
        (24, 6),
        (34, 2),
        (34, 3),
        (35, 2),
        (35, 3),
    ]
}
