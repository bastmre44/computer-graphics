use crate::bmp;
use std::io;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<[u8; 3]>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background: [u8; 3]) -> Self {
        Self {
            width,
            height,
            pixels: vec![background; width * height],
        }
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: [u8; 3]) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            return;
        }

        let index = y * self.width + x;
        self.pixels[index] = color;
    }

    pub fn pixels(&self) -> &[[u8; 3]] {
        &self.pixels
    }

    pub fn write_bmp(&self, path: &str) -> io::Result<()> {
        bmp::write_bmp(path, self)
    }
}
