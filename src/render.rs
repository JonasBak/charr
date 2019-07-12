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
    c1 * c2 <= 0.0
}

fn basis(v0: &Vec2f, v1: &Vec2f, p: &Vec2f) -> Option<(f32, f32)> {
    if v0.cross(&v1) == 0.0 {
        return None;
    }
    let (a, b) = if v0.0 != 0.0 {
        let b = (p.1 - v0.1 * p.0 / v0.0) / (v1.1 - v0.1 * v1.0 / v0.0);
        let a = (p.0 - b * v1.0) / v0.0;
        (a, b)
    } else {
        let b = (p.1 - v1.1 * p.0 / v1.0) / (v0.1 - v1.1 * v0.0 / v1.0);
        let a = (p.0 - b * v0.0) / v1.0;
        (a, b)
    };
    if a < 0.0 || a > 1.0 || b < 0.0 || b > 1.0 {
        return None;
    }
    return Some((a, b));
}

pub type Color = Vec3f;

pub struct Vertex(pub Vec3f, pub Color);
pub struct Fragment(pub Vec2f, pub Color);

pub struct PixelBuffer {
    pub color: Color,
    pub depth: f32,
}

pub fn rasterize(
    vertex_shader: impl Fn(&Vertex) -> Fragment,
    fragment_shader: impl Fn(&Fragment, f32) -> PixelBuffer,
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    screen_buffer: &mut [Option<PixelBuffer>],
    width: i32,
    height: i32,
) {
    let p0_frag = vertex_shader(v0); //.scal(100.0 / (100.0 + (v0.0).2));
    let p1_frag = vertex_shader(v1); //.scal(100.0 / (100.0 + (v1.0).2));
    let p2_frag = vertex_shader(v2); //.scal(100.0 / (100.0 + (v2.0).2));

    for y in 0..height {
        for x in 0..width {
            let p = Vec2(
                2.0 * x as f32 / width as f32 - 1.0,
                2.0 * y as f32 / height as f32 - 1.0,
            );

            if inside(&p0_frag.0, &p1_frag.0, &p2_frag.0, &p)
                && inside(&p1_frag.0, &p2_frag.0, &p0_frag.0, &p)
            {
                if let Some((a, b)) = basis(
                    &p1_frag.0.sub(&p0_frag.0),
                    &p2_frag.0.sub(&p0_frag.0),
                    &p.sub(&p0_frag.0),
                ) {
                    let depth = (v0.0).2 + v1.0.sub(&v0.0).scal(a).2 + v2.0.sub(&v0.0).scal(b).2;
                    let color = (p0_frag.1)
                        .add(&p1_frag.1.sub(&p0_frag.1).scal(a))
                        .add(&p2_frag.1.sub(&p0_frag.1).scal(b));

                    let fragment = Fragment(p, color);

                    let mut pb = fragment_shader(&fragment, depth);
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
}
