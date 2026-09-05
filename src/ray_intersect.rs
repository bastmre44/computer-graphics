use crate::color::Color;
use crate::texture::Texture;
use nalgebra_glm::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub texture: Texture,
}

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
    pub u: f32,
    pub v: f32,
}

impl Intersect {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        distance: f32,
        material: Material,
        u: f32,
        v: f32,
    ) -> Self {
        Self {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
            u,
            v,
        }
    }

    pub fn empty() -> Self {
        Self {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: f32::INFINITY,
            is_intersecting: false,
            material: Material {
                diffuse: Color::new(0, 0, 0),
                texture: Texture::new(Color::new(0, 0, 0), Color::new(0, 0, 0), 1),
            },
            u: 0.0,
            v: 0.0,
        }
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}

pub type Object = Box<dyn RayIntersect>;
