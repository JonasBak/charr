use std::io::{Error, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

struct Token {
    strength: i16,
    color: Color,
}

fn render(stream: &mut StandardStream, token: Token) -> Result<(), Error> {
    let Token { strength, color } = token;
    stream.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(stream, "#")?;
    Ok(())
}

struct Point2<T>(T, T);
struct Point3<T>(T, T, T);

struct Vertex(Point3<f32>, Color);

struct PixelBuffer {}

fn rasterize(p0: Point2<f32>, p1: Point2<f32>, p2: Point2<f32>) -> Vec<i32> {
    return vec![1];
}

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    render(
        &mut stdout,
        Token {
            strength: 3,
            color: Color::Red,
        },
    );
    println!("Hello, world!");
}
