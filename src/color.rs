use raylib::prelude::Color as RayColor;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }

    pub fn multiply(self, other: Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }

    pub fn scale(self, value: f32) -> Self {
        Self {
            r: self.r * value,
            g: self.g * value,
            b: self.b * value,
        }
    }

    pub fn to_raylib(self) -> RayColor {
        RayColor::new(
            (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.clamp(0.0, 1.0) * 255.0) as u8,
            255,
        )
    }
}
