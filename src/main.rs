mod framebuffer;
mod line;
mod png;
mod polygon;

use framebuffer::{Color, Framebuffer};
use polygon::{draw_polygon_outline, fill_polygon};

type Point = (i32, i32);

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BACKGROUND: Color = [0, 0, 0];
const WHITE: Color = [255, 255, 255];
const YELLOW: Color = [255, 220, 0];
const BLUE: Color = [0, 90, 255];
const RED: Color = [230, 30, 30];
const GREEN: Color = [0, 180, 80];

const POLYGON_1: &[Point] = &[
    (165, 380),
    (185, 360),
    (180, 330),
    (207, 345),
    (233, 330),
    (230, 360),
    (250, 380),
    (220, 385),
    (205, 410),
    (193, 383),
];

const POLYGON_2: &[Point] = &[(321, 335), (288, 286), (339, 251), (374, 302)];

const POLYGON_3: &[Point] = &[(377, 249), (411, 197), (436, 249)];

const POLYGON_4: &[Point] = &[
    (413, 177),
    (448, 159),
    (502, 88),
    (553, 53),
    (535, 36),
    (676, 37),
    (660, 52),
    (750, 145),
    (761, 179),
    (672, 192),
    (659, 214),
    (615, 214),
    (632, 230),
    (580, 230),
    (597, 215),
    (552, 214),
    (517, 144),
    (466, 180),
];

const POLYGON_5_HOLE: &[Point] = &[(682, 175), (708, 120), (735, 148), (739, 170)];

fn main() {
    let mut fb = Framebuffer::new(WIDTH, HEIGHT, BACKGROUND);

    // Poligono 1: amarillo.
    fill_polygon(&mut fb, POLYGON_1, YELLOW);
    // Poligono 2: azul.
    fill_polygon(&mut fb, POLYGON_2, BLUE);
    // Poligono 3: rojo.
    fill_polygon(&mut fb, POLYGON_3, RED);
    // Poligono 4: verde.
    fill_polygon(&mut fb, POLYGON_4, GREEN);

    // Poligono 5: agujero.
    fill_polygon(&mut fb, POLYGON_5_HOLE, BACKGROUND);

    draw_polygon_outline(&mut fb, POLYGON_1, WHITE);
    draw_polygon_outline(&mut fb, POLYGON_2, WHITE);
    draw_polygon_outline(&mut fb, POLYGON_3, WHITE);
    draw_polygon_outline(&mut fb, POLYGON_4, WHITE);
    draw_polygon_outline(&mut fb, POLYGON_5_HOLE, WHITE);

    fb.write_png("out.png").expect("no se pudo escribir out.png");
    println!("Imagen generada: out.png");
}
