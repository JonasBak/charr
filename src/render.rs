use super::vector::{Vec2, Vec2f, Vec2i, Vec3, Vec3f, Vec3i, Vector};
use std::io::{self, Read, Write};
use std::thread;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor};

fn render<W: Write>(out: &mut W, pixel: &PixelBuffer) {
    match pixel.color {
        Color::Red => write!(out, "{}#", color::Fg(color::Red)),
        Color::Blue => write!(out, "{}#", color::Fg(color::Blue)),
        Color::Black => write!(out, "{}#", color::Fg(color::Black)),
    }
    .unwrap();
}

fn rotate_y(point: &Vec3f, rad: f32) -> Vec3f {
    Vec3(
        point.0 * rad.cos() + point.2 * rad.sin(),
        point.1,
        -point.0 * rad.sin() + point.2 * rad.cos(),
    )
}

fn inside(p0: &Vec2f, p1: &Vec2f, p2: &Vec2f, p: &Vec2f) -> bool {
    let d_p1_x = p1.0 - p0.0;
    let d_p1_y = p1.1 - p0.1;
    let d_p2_x = p2.0 - p0.0;
    let d_p2_y = p2.1 - p0.1;
    let d_x = p.0 as f32 + 0.5 - p0.0;
    let d_y = p.1 as f32 + 0.5 - p0.1;
    let c1 = d_x * d_p1_y - d_y * d_p1_x;
    let c2 = d_x * d_p2_y - d_y * d_p2_x;
    c1 * c2 <= 0.0
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
enum Color {
    Red,
    Blue,
    Black,
}

struct Vertex(Vec3f, Color);

struct PixelBuffer {
    color: Color,
    depth: f32,
}

fn rasterize(
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    screen_buffer: &mut [Option<PixelBuffer>],
    width: i32,
    height: i32,
) {
    let center = Vec2((width as f32) / 2.0, (height as f32) / 2.0);
    let p0_proj = Vec2((v0.0).0, (v0.0).1)
        .scal(100.0 / (100.0 + (v0.0).2))
        .add(&center);
    let p1_proj = Vec2((v1.0).0, (v1.0).1)
        .scal(100.0 / (100.0 + (v1.0).2))
        .add(&center);
    let p2_proj = Vec2((v2.0).0, (v2.0).1)
        .scal(100.0 / (100.0 + (v2.0).2))
        .add(&center);
    let min_x = p0_proj
        .0
        .min(p1_proj.0)
        .min(p2_proj.0)
        .max(0.0)
        .min(width as f32 - 1.0)
        .round() as i32;
    let max_x = p0_proj
        .0
        .max(p1_proj.0)
        .max(p2_proj.0)
        .max(0.0)
        .min(width as f32 - 1.0)
        .round() as i32;
    let min_y = p0_proj
        .1
        .min(p1_proj.1)
        .min(p2_proj.1)
        .max(0.0)
        .min(height as f32 - 1.0)
        .round() as i32;
    let max_y = p0_proj
        .1
        .max(p1_proj.1)
        .max(p2_proj.1)
        .max(0.0)
        .min(height as f32 - 1.0)
        .round() as i32;

    for y in min_y..max_y {
        for x in min_x..max_x {
            let p = Vec2(x as f32 + 0.5, y as f32 + 0.5);

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

pub fn test() {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = stdout.into_raw_mode().unwrap();
    let termsize = termion::terminal_size().ok();
    let width = termsize.map(|(w, _)| w - 2).unwrap_or(80);
    let height = termsize.map(|(_, h)| h - 2).unwrap_or(40);

    let mut buffer: Vec<Option<PixelBuffer>> = vec![];
    for _ in 0..width * height {
        buffer.push(None);
    }

    let mut vecs = [
        Vertex(Vec3(-20.0, -10.0, 20.0), Color::Blue),
        Vertex(Vec3(20.0, -10.0, 20.0), Color::Blue),
        Vertex(Vec3(-20.0, 10.0, 20.0), Color::Blue),
        Vertex(Vec3(-20.0, 10.0, 20.0), Color::Red),
        Vertex(Vec3(20.0, -10.0, 20.0), Color::Red),
        Vertex(Vec3(20.0, 10.0, 20.0), Color::Red),
        Vertex(Vec3(-20.0, -10.0, -20.0), Color::Blue),
        Vertex(Vec3(20.0, -10.0, -20.0), Color::Blue),
        Vertex(Vec3(20.0, 10.0, -20.0), Color::Blue),
        Vertex(Vec3(-20.0, 10.0, -20.0), Color::Red),
        Vertex(Vec3(-20.0, -10.0, -20.0), Color::Red),
        Vertex(Vec3(20.0, 10.0, -20.0), Color::Red),
        Vertex(Vec3(-20.0, -10.0, -20.0), Color::Blue),
        Vertex(Vec3(-20.0, -10.0, 20.0), Color::Blue),
        Vertex(Vec3(-20.0, 10.0, -20.0), Color::Blue),
        Vertex(Vec3(-20.0, -10.0, 20.0), Color::Red),
        Vertex(Vec3(-20.0, 10.0, -20.0), Color::Red),
        Vertex(Vec3(-20.0, 10.0, 20.0), Color::Red),
        Vertex(Vec3(20.0, -10.0, -20.0), Color::Blue),
        Vertex(Vec3(20.0, -10.0, 20.0), Color::Blue),
        Vertex(Vec3(20.0, 10.0, 20.0), Color::Blue),
        Vertex(Vec3(20.0, -10.0, -20.0), Color::Red),
        Vertex(Vec3(20.0, 10.0, -20.0), Color::Red),
        Vertex(Vec3(20.0, 10.0, 20.0), Color::Red),
    ];

    let default = PixelBuffer {
        color: Color::Black,
        depth: 0.0,
    };

    print!("{}", clear::All);
    for _ in 0..200 {
        for i in 0..width * height {
            buffer[i as usize] = None;
        }
        for i in 0..vecs.len() {
            vecs[i] = Vertex(rotate_y(&vecs[i].0, 0.2), vecs[i].1);
        }
        for i in 0..vecs.len() / 3 {
            rasterize(
                &vecs[3 * i],
                &vecs[3 * i + 1],
                &vecs[3 * i + 2],
                &mut buffer[..],
                width as i32,
                height as i32,
            );
        }

        for i in 0..width * height {
            if i % width == 0 {
                write!(stdout, "{}", cursor::Goto(1, (i / width) as u16 + 1,));
            }
            if let Some(pixel) = &buffer[i as usize] {
                render(&mut stdout, pixel);
            } else {
                render(&mut stdout, &default);
            }
        }
        thread::sleep_ms(100);
    }
}
