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
        -point.0 * rad.sin() + point.2 * rad.sin(),
    )
}

fn angle(p0: &Point2<f32>, p1: &Point2<f32>, p2: &Point2<f32>) -> f32 {
    (p2.1 - p0.1).atan2(p2.0 - p0.0) - (p1.1 - p0.1).atan2(p1.0 - p0.0)
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
    let base_angle0 = angle(&p0_proj, &p1_proj, &p2_proj);
    let base_angle1 = angle(&p1_proj, &p2_proj, &p0_proj);
    let min_x = p0_proj
        .0
        .min(p1_proj.0)
        .min(p2_proj.0)
        .max(0 as f32)
        .min((width - 1) as f32) as i32;
    let max_x = p0_proj
        .0
        .max(p1_proj.0)
        .max(p2_proj.0)
        .max(0 as f32)
        .min((width - 1) as f32) as i32;
    let min_y = p0_proj
        .1
        .min(p1_proj.1)
        .min(p2_proj.1)
        .max(0 as f32)
        .min((height - 1) as f32) as i32;
    let max_y = p0_proj
        .1
        .max(p1_proj.1)
        .max(p2_proj.1)
        .max(0 as f32)
        .min((height - 1) as f32) as i32;

    for y in min_y..max_y {
        for x in min_x..max_x {
            let alpha0 = angle(&p0_proj, &p1_proj, &Point2(x as f32, y as f32));
            let alpha1 = angle(&p1_proj, &p2_proj, &Point2(x as f32, y as f32));
            let epsilon = 0.001;
            if alpha0 + epsilon < base_angle0
                && alpha1 + epsilon < base_angle1
                && alpha0 > -epsilon
                && alpha1 > -epsilon
            {
                screen_buffer[(y * width + x) as usize] = Some(PixelBuffer {
                    color: Color::Blue,
                    depth: 0.0,
                });
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

    let mut v0 = Vertex(Point3(-10.0, -10.0, 0.0), Color::Red);
    let mut v1 = Vertex(Point3(10.0, -10.0, 0.0), Color::Red);
    let mut v2 = Vertex(Point3(-10.0, 10.0, 0.0), Color::Red);
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
        v0 = Vertex(rotate_y(v0.0, 0.2), Color::Red);
        v1 = Vertex(rotate_y(v1.0, 0.2), Color::Red);
        v2 = Vertex(rotate_y(v2.0, 0.2), Color::Red);
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
                println!("");
            }
        }
        thread::sleep_ms(200);
    }
}
