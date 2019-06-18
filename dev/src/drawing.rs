use cairo::{Context, Format, ImageSurface};
use std::fs::File;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub fn init_surface(
    width: i32,
    height: i32,
    color: &Color,
) -> (cairo::ImageSurface, cairo::Context) {
    let surface: cairo::ImageSurface =
        ImageSurface::create(Format::ARgb32, width, height)
            .expect("Unable to create surface");
    let context: cairo::Context = Context::new(&surface);
    context.set_antialias(cairo::Antialias::Best);
    context.set_source_rgb(color.r, color.g, color.b);
    context.paint();
    (surface, context)
}

pub fn write_png(surface: &cairo::ImageSurface) {
    let mut file =
        File::create("out/main.png").expect("Unable to create file");
    surface
        .write_to_png(&mut file)
        .expect("Unable to write to png");
}
