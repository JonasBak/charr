use super::vector::{Vec2, Vec2f, Vec2i, Vec3, Vec3f, Vec3i, Vector};

fn inside(p0: &Vec2f, p1: &Vec2f, p2: &Vec2f, p: &Vec2f) -> bool {
    let d_p1_x = p1.0 - p0.0;
    let d_p1_y = p1.1 - p0.1;
    let d_p2_x = p2.0 - p0.0;
    let d_p2_y = p2.1 - p0.1;
    let d_x = p.0 as f32 - p0.0;
    let d_y = p.1 as f32 - p0.1;
    let c1 = d_x * d_p1_y - d_y * d_p1_x;
    let c2 = d_x * d_p2_y - d_y * d_p2_x;
    c1 * c2 <= -0.00001
}

fn basis(v0: &Vec2f, v1: &Vec2f, p: &Vec2f) -> Option<(f32, f32)> {
    if v0.cross(&v1) == 0.0 {
        return None;
    }
    if v0.0 != 0.0 {
        let b = (p.1 - v0.1 * p.0 / v0.0) / (v1.1 - v0.1 * v1.0 / v0.0);
        let a = (p.0 - b * v1.0) / v0.0;
        return Some((a, b));
    } else {
        let b = (p.1 - v1.1 * p.0 / v1.0) / (v0.1 - v1.1 * v0.0 / v1.0);
        let a = (p.0 - b * v0.0) / v1.0;
        return Some((a, b));
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Vertex(pub Vec3f, pub Color);

pub struct PixelBuffer {
    pub color: Color,
    pub depth: f32,
}

pub fn rasterize(
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    screen_buffer: &mut [Option<PixelBuffer>],
    width: i32,
    height: i32,
) {
    let center = Vec2((width as f32) / 2.0, (height as f32) / 2.0);
    let p0_proj = Vec2((v0.0).0, (v0.0).1); //.scal(100.0 / (100.0 + (v0.0).2));
    let p1_proj = Vec2((v1.0).0, (v1.0).1); //.scal(100.0 / (100.0 + (v1.0).2));
    let p2_proj = Vec2((v2.0).0, (v2.0).1); //.scal(100.0 / (100.0 + (v2.0).2));

    //let min_x = p0_proj
    //    .0
    //    .min(p1_proj.0)
    //    .min(p2_proj.0)
    //    .max(0.0)
    //    .min(width as f32 - 1.0)
    //    .round() as i32;
    //let max_x = p0_proj
    //    .0
    //    .max(p1_proj.0)
    //    .max(p2_proj.0)
    //    .max(0.0)
    //    .min(width as f32 - 1.0)
    //    .round() as i32;
    //let min_y = p0_proj
    //    .1
    //    .min(p1_proj.1)
    //    .min(p2_proj.1)
    //    .max(0.0)
    //    .min(height as f32 - 1.0)
    //    .round() as i32;
    //let max_y = p0_proj
    //    .1
    //    .max(p1_proj.1)
    //    .max(p2_proj.1)
    //    .max(0.0)
    //    .min(height as f32 - 1.0)
    //    .round() as i32;

    for y in 0..height {
        for x in 0..width {
            let p = Vec2(
                2.0 * x as f32 / width as f32 - 1.0,
                2.0 * y as f32 / height as f32 - 1.0,
            );

            if inside(&p0_proj, &p1_proj, &p2_proj, &p) && inside(&p1_proj, &p2_proj, &p0_proj, &p)
            {
                let mut depth = -1000.0;
                if let Some((a, b)) = basis(
                    &p1_proj.sub(&p0_proj),
                    &p2_proj.sub(&p0_proj),
                    &p.sub(&p0_proj),
                ) {
                    depth = (v0.0).2 + v1.0.sub(&v0.0).scal(a).2 + v2.0.sub(&v0.0).scal(b).2;
                }
                let mut pb = PixelBuffer {
                    color: v0.1,
                    depth: depth,
                };
                if let Some(tmp) = screen_buffer[(y * width + x) as usize].take() {
                    if tmp.depth < depth {
                        pb = tmp;
                    }
                }
                screen_buffer[(y * width + x) as usize] = Some(pb);
            }
        }
    }
}
