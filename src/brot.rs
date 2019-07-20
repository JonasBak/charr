use super::render::{rasterize, Color, Fragment, PixelBuffer, Vertex};
use super::vector::{Vec2, Vec2f, Vec2i, Vec3, Vec3f, Vec3i, Vector};

fn sigmoid(f: f64) -> f64 {
    return f.exp() / (f.exp() + 1.0);
}

fn iterate(r: f64, i: f64) -> f64 {
    let mut z = (0.0, 0.0);
    for n in 0..10000 {
        let new_z = (z.0 * z.0 - z.1 * z.1 + r, 2.0 * z.0 * z.1 + i);

        z = new_z;

        if z.0 * z.0 + z.1 * z.1 > 50.0 {
            return (n as f64).ln(); // + sigmoid(z.0 * z.0 + z.1 * z.1);
        }
    }
    return -1.0;
}
pub fn render_mandelbrot(v: &Fragment) -> PixelBuffer {
    let x = (v.0).0 * 1.5;
    let y = (v.0).1 * 1.5;
    let x = x / 14880.0;
    let y = y / 14880.0;
    let x = x + 0.2359;
    let y = y + 0.5244;
    let n = iterate(x as f64, y as f64);
    PixelBuffer {
        color: if n == -1.0 {
            Vec3(0.0, 0.0, 0.0)
        } else {
            let range = 3.0;
            let colors = [Vec3(1.0, 1.0, 0.0), Vec3(0.0, 0.0, 1.0)];
            let t = colors.len() as f64 * (n % range) / (range);
            // Vec3(0.0, 0.0, 1.0).add(&Vec3(0.0, 1.0, -1.0).scal(t));
            colors[t as usize].add(
                &colors[t as usize]
                    .sub(&colors[(t as usize + colors.len() - 1) % colors.len()])
                    .scal((t % 1.0) as f32),
            )
        },
        depth: (v.0).2,
    }
}
