use raylib::prelude::*;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::BLACK; width * height],
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn to_image(&self) -> Image {
        let mut image = Image::gen_image_color(
            self.width as i32,
            self.height as i32,
            Color::BLACK,
        );

        for y in 0..self.height {
            for x in 0..self.width {
                image.draw_pixel(x as i32, y as i32, self.pixels[y * self.width + x]);
            }
        }

        image
    }
}
