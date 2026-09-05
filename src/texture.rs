use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    color_a: Color,
    color_b: Color,
    squares: u32,
}

impl Texture {
    pub const fn new(color_a: Color, color_b: Color, squares: u32) -> Self {
        Self {
            color_a,
            color_b,
            squares,
        }
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let x = (u.clamp(0.0, 0.9999) * self.squares as f32) as u32;
        let y = (v.clamp(0.0, 0.9999) * self.squares as f32) as u32;

        if (x + y) % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}
