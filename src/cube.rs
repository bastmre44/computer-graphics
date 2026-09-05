use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let half = self.size / 2.0;
        let min = self.center - Vec3::new(half, half, half);
        let max = self.center + Vec3::new(half, half, half);

        let mut near = f32::NEG_INFINITY;
        let mut far = f32::INFINITY;
        let mut hit_axis = 0;

        for axis in 0..3 {
            if ray_direction[axis].abs() < 0.000001 {
                if ray_origin[axis] < min[axis] || ray_origin[axis] > max[axis] {
                    return Intersect::empty();
                }
                continue;
            }

            let mut t1 = (min[axis] - ray_origin[axis]) / ray_direction[axis];
            let mut t2 = (max[axis] - ray_origin[axis]) / ray_direction[axis];

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }

            if t1 > near {
                near = t1;
                hit_axis = axis;
            }
            far = far.min(t2);

            if near > far {
                return Intersect::empty();
            }
        }

        if near <= 0.0 {
            return Intersect::empty();
        }

        let point = ray_origin + ray_direction * near;
        let mut normal = Vec3::zeros();
        normal[hit_axis] = if ray_direction[hit_axis] > 0.0 {
            -1.0
        } else {
            1.0
        };

        let local = point - min;
        let (u, v) = match hit_axis {
            0 => (local.z / self.size, local.y / self.size),
            1 => (local.x / self.size, local.z / self.size),
            _ => (local.x / self.size, local.y / self.size),
        };

        Intersect::new(point, normal, near, self.material, u, v)
    }
}
