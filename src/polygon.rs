use crate::framebuffer::{Color, Framebuffer};
use crate::line::draw_line;

pub fn fill_polygon(fb: &mut Framebuffer, points: &[(i32, i32)], color: Color) {
    if points.len() < 3 {
        return;
    }

    let min_y = points.iter().map(|(_, y)| *y).min().unwrap().max(0);
    let max_y = points
        .iter()
        .map(|(_, y)| *y)
        .max()
        .unwrap()
        .min(fb.height as i32 - 1);

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        // Cruces de esta fila con las aristas del poligono.
        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];

            if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                let x = x0 as f32 + (y - y0) as f32 * (x1 - x0) as f32 / (y1 - y0) as f32;
                intersections.push(x.round() as i32);
            }
        }

        intersections.sort_unstable();

        // Se rellena entre pares de cruces.
        for pair in intersections.chunks(2) {
            if pair.len() != 2 {
                continue;
            }

            for x in pair[0]..=pair[1] {
                fb.set_pixel(x, y, color);
            }
        }
    }
}

pub fn draw_polygon_outline(fb: &mut Framebuffer, points: &[(i32, i32)], color: Color) {
    for i in 0..points.len() {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % points.len()];
        draw_line(fb, x0, y0, x1, y1, color);
    }
}
