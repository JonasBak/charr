#![allow(dead_code)]
pub mod export;
pub mod render;
pub mod vector;
extern crate termion;

use export::write_to_file;
use render::{rasterize, Color, Fragment, PixelBuffer, Vertex};
use vector::{Vec2, Vec2f, Vec2i, Vec3, Vec3f, Vec3i, Vector};

fn main() {
    let width = 300;
    let height = 300;

    let mut buffer: Vec<Option<PixelBuffer>> = vec![];
    for _ in 0..width * height {
        buffer.push(None);
    }

    let red = Vec3(1.0, 0.0, 0.0);
    let green = Vec3(0.0, 1.0, 0.0);
    let blue = Vec3(0.0, 0.0, 1.0);
    let vecs = [
        Vertex(Vec3(-0.75, -0.75, 0.75), blue),
        Vertex(Vec3(0.75, -0.75, 0.75), red),
        Vertex(Vec3(-0.75, 0.75, 0.75), green),
        Vertex(Vec3(-0.75, 0.75, 0.75), red),
        Vertex(Vec3(0.75, -0.75, 0.75), blue),
        Vertex(Vec3(0.75, 0.75, 0.75), blue),
    ];

    let default = Vec3(0.1, 0.1, 0.1);

    let vertex_shader =
        |v: &Vertex| Fragment(Vec2((v.0).0, (v.0).1).scal(100.0 / (100.0 + (v.0).2)), v.1);
    let fragment_shader = |v: &Fragment, depth: f32| PixelBuffer {
        color: v.1,
        depth: depth,
    };
    for i in 0..vecs.len() / 3 {
        rasterize(
            &vertex_shader,
            &fragment_shader,
            &vecs[3 * i],
            &vecs[3 * i + 1],
            &vecs[3 * i + 2],
            &mut buffer[..],
            width as i32,
            height as i32,
        );
    }

    write_to_file(width, height, &buffer, default).unwrap();
}
