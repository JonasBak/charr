#![allow(dead_code)]
pub mod brot;
pub mod export;
pub mod render;
pub mod vector;
extern crate termion;

use brot::render_mandelbrot;
use export::write_to_file;
use render::{rasterize, rasterize_conc, Color, Fragment, PixelBuffer, Vertex};
use vector::{Vec2, Vec2f, Vec2i, Vec3, Vec3f, Vec3i, Vector};

fn main() {
    let width = 5000;
    let height = 5000;

    let mut buffer: Vec<Option<PixelBuffer>> = vec![];
    for _ in 0..width * height {
        buffer.push(None);
    }

    let red = Vec3(1.0, 0.0, 0.0);
    let vecs = [
        Vertex(Vec3(-1.0, -1.0, 1.0), red),
        Vertex(Vec3(1.0, -1.0, 1.0), red),
        Vertex(Vec3(-1.0, 1.0, 1.0), red),
        Vertex(Vec3(-1.0, 1.0, 1.0), red),
        Vertex(Vec3(1.0, -1.0, 1.0), red),
        Vertex(Vec3(1.0, 1.0, 1.0), red),
    ];

    let default = Vec3(0.1, 0.1, 0.1);

    let vertex_shader = |v: &Vertex| Fragment(v.0, v.1);
    for i in 0..vecs.len() / 3 {
        rasterize_conc(
            &vertex_shader,
            &render_mandelbrot,
            &vecs[3 * i],
            &vecs[3 * i + 1],
            &vecs[3 * i + 2],
            &mut buffer[..],
            width as i32,
            height as i32,
            10,
        );
        //rasterize(
        //    &vertex_shader,
        //    &render_mandelbrot,
        //    &vecs[3 * i],
        //    &vecs[3 * i + 1],
        //    &vecs[3 * i + 2],
        //    &mut buffer[..],
        //    width as i32,
        //    height as i32,
        //);
    } //

    write_to_file(width, height, &buffer, default).unwrap();
}
