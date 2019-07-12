use super::render::{rasterize, Color, PixelBuffer, Vertex};
use super::vector::{Vec2, Vec2f, Vec2i, Vec3, Vec3f, Vec3i, Vector};
use std::fs::File;
use std::io::prelude::*;

pub fn write_to_file(
    width: usize,
    height: usize,
    buffer: &[Option<PixelBuffer>],
    default_color: Color,
) -> std::io::Result<()> {
    let mut file = File::create("test.ppm")?;
    file.write(format!("P6\n{} {}\n255\n", width, height).as_bytes())?;

    for i in 0..buffer.len() {
        let color = match buffer[i] {
            Some(PixelBuffer { color, .. }) => color,
            _ => default_color,
        };
        file.write(&[
            (color.0 * 255.0) as u8,
            (color.1 * 255.0) as u8,
            (color.2 * 255.0) as u8,
        ])?;
    }
    Ok(())
}
