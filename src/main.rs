#![allow(dead_code)]
use std::f32::consts;
use std::io::{Error, Write};
use std::thread;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn render(stream: &mut StandardStream, pixel: &PixelBuffer) -> Result<(), Error> {
    stream.set_color(ColorSpec::new().set_fg(Some(pixel.color)))?;
    write!(stream, "#")?;
    Ok(())
}

struct Point2<T>(T, T);
struct Point3<T>(T, T, T);

fn rotate_y(point: Point3<f32>, rad: f32) -> Point3<f32> {
    Point3(
        point.0 * rad.cos() + point.2 * rad.sin(),
        point.1,
        -point.0 * rad.sin() + point.2 * rad.cos(),
    )
}

fn angle(p0: &Point2<f32>, p1: &Point2<f32>, p2: &Point2<f32>) -> f32 {
    (p2.1 - p0.1).atan2(p2.0 - p0.0) - (p1.1 - p0.1).atan2(p1.0 - p0.0)
}

fn inside(p0: &Point2<f32>, p1: &Point2<f32>, p2: &Point2<f32>, p: &Point2<f32>) -> bool {
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

struct Vertex(Point3<f32>, Color);

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
    let p0_proj = Point2(
        (v0.0).0 + (width as f32) / 2.0,
        (v0.0).1 + (height as f32) / 2.0,
    );
    let p1_proj = Point2(
        (v1.0).0 + (width as f32) / 2.0,
        (v1.0).1 + (height as f32) / 2.0,
    );
    let p2_proj = Point2(
        (v2.0).0 + (width as f32) / 2.0,
        (v2.0).1 + (height as f32) / 2.0,
    );
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
            let p = Point2(x as f32 + 0.5, y as f32 + 0.5);

            if inside(&p0_proj, &p1_proj, &p2_proj, &p) && inside(&p1_proj, &p2_proj, &p0_proj, &p)
            {
                let depth = (v0.0).2;
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

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let width = 50;
    let height = 20;

    let mut buffer: Vec<Option<PixelBuffer>> = vec![];
    for _ in 0..width * height {
        buffer.push(None);
    }

    let mut v0 = Vertex(Point3(-10.0, -10.0, 0.0), Color::Blue);
    let mut v1 = Vertex(Point3(10.0, -10.0, 0.0), Color::Blue);
    let mut v2 = Vertex(Point3(-10.0, 10.0, 0.0), Color::Blue);
    let mut v3 = Vertex(Point3(-10.0, 10.0, 0.0), Color::Red);
    let mut v4 = Vertex(Point3(10.0, -10.0, 0.0), Color::Red);
    let mut v5 = Vertex(Point3(10.0, 10.0, 0.0), Color::Red);

    let default = PixelBuffer {
        color: Color::Black,
        depth: 0.0,
    };

    for _ in 0..100 {
        for i in 0..width * height {
            buffer[i] = None;
        }
        v0 = Vertex(rotate_y(v0.0, 0.2), Color::Blue);
        v1 = Vertex(rotate_y(v1.0, 0.2), Color::Blue);
        v2 = Vertex(rotate_y(v2.0, 0.2), Color::Blue);
        v3 = Vertex(rotate_y(v3.0, 0.2), Color::Red);
        v4 = Vertex(rotate_y(v4.0, 0.2), Color::Red);
        v5 = Vertex(rotate_y(v5.0, 0.2), Color::Red);
        rasterize(&v0, &v1, &v2, &mut buffer[..], width as i32, height as i32);
        rasterize(&v3, &v4, &v5, &mut buffer[..], width as i32, height as i32);

        for i in 0..width * height {
            if let Some(pixel) = &buffer[i as usize] {
                render(&mut stdout, pixel);
            } else {
                render(&mut stdout, &default);
            }
            if i % width == 0 {
                println!("\r\r\r\r\r");
            }
        }
        thread::sleep_ms(200);
    }
}
