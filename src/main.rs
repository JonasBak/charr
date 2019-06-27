#![allow(dead_code)]
pub mod export;
pub mod render;
pub mod vector;
extern crate termion;

use export::write_to_file;
use render::{rasterize, Color, PixelBuffer, Vertex};
use vector::{Vec2, Vec2f, Vec2i, Vec3, Vec3f, Vec3i, Vector};

fn main() {
    let width = 300;
    let height = 300;

    let mut buffer: Vec<Option<PixelBuffer>> = vec![];
    for _ in 0..width * height {
        buffer.push(None);
    }

    let blue = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };
    let red = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
    let vecs = [
        Vertex(Vec3(-0.75, -0.75, 0.75), blue),
        Vertex(Vec3(0.75, -0.75, 0.75), blue),
        Vertex(Vec3(-0.75, 0.75, 0.75), blue),
        Vertex(Vec3(-0.75, 0.75, 0.75), red),
        Vertex(Vec3(0.75, -0.75, 0.75), red),
        Vertex(Vec3(0.75, 0.75, 0.75), red),
        //Vertex(Vec3(-0.5, -0.1, -0.1), blue),
        //Vertex(Vec3(0.5, -0.1, -0.1), blue),
        //Vertex(Vec3(0.5, 0.1, -0.1), blue),
        //Vertex(Vec3(-0.5, 0.1, -0.1), red),
        //Vertex(Vec3(-0.5, -0.1, -0.1), red),
        //Vertex(Vec3(0.5, 0.1, -0.1), red),
        //Vertex(Vec3(-0.5, -0.1, -0.1), blue),
        //Vertex(Vec3(-0.5, -0.1, 0.1), blue),
        //Vertex(Vec3(-0.5, 0.1, -0.1), blue),
        //Vertex(Vec3(-0.5, -0.1, 0.1), red),
        //Vertex(Vec3(-0.5, 0.1, -0.1), red),
        //Vertex(Vec3(-0.5, 0.1, 0.1), red),
        //Vertex(Vec3(0.5, -0.1, -0.1), blue),
        //Vertex(Vec3(0.5, -0.1, 0.1), blue),
        //Vertex(Vec3(0.5, 0.1, 0.1), blue),
        //Vertex(Vec3(0.5, -0.1, -0.1), red),
        //Vertex(Vec3(0.5, 0.1, -0.1), red),
        //Vertex(Vec3(0.5, 0.1, 0.1), red),
    ];

    let default = Color {
        r: 0.1,
        g: 0.1,
        b: 0.1,
    };

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

    write_to_file(width, height, &buffer, default).unwrap();
}
