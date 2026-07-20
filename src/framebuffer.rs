use crate::png;
use std::io;

pub type Color = [u8; 3];

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background: Color) -> Self {
        Self {
            width,
            height,
            pixels: vec![background; width * height],
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            return;
        }

        let image_y = self.height - 1 - y;
        self.pixels[image_y * self.width + x] = color;
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }

    pub fn write_png(&self, path: &str) -> io::Result<()> {
        png::write_png(path, self)
    }
}
