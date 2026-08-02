use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pixels: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background: Color) -> Self {
        Self {
            width,
            height,
            pixels: vec![background; (width * height) as usize],
        }
    }

    // pinta una celda del tablero
    pub fn point(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }

        let index = (y * self.width + x) as usize;
        self.pixels[index] = color;
    }

    // obtiene el color de una celda
    pub fn get_color(&self, x: i32, y: i32) -> Color {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Color::BLACK;
        }

        let index = (y * self.width + x) as usize;
        self.pixels[index]
    }
}
