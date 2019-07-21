use cairo::{Context, Format, ImageSurface};
use std::fs::File;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub struct LineParams<'a> {
    pub width: f64,
    pub line_alpha: f64,
    pub fill: bool,
    pub fill_alpha: f64,
    pub color: &'a Color,
}

pub fn init_surface(
    width: i32,
    height: i32,
    color: &Color,
) -> Option<(cairo::ImageSurface, cairo::Context)> {
    ImageSurface::create(Format::ARgb32, width, height)
        .ok()
        .map(|surface| {
            let context: cairo::Context = Context::new(&surface);
            context.set_antialias(cairo::Antialias::Best);
            context.set_source_rgb(color.r, color.g, color.b);
            context.paint();
            (surface, context)
        })
}

#[allow(clippy::integer_division)]
pub fn draw_lines<'a>(
    context: &cairo::Context,
    xs: &'a [f32],
    n: usize,
    params: &LineParams,
) -> Result<(), &'a str> {
    if xs.len() == n * 2 {
        for i in 0..n / 2 {
            context.line_to(xs[i * 2].into(), xs[(i * 2) + 1].into());
        }
        context.set_line_width(params.width);
        if params.fill {
            context.set_source_rgba(
                params.color.r,
                params.color.g,
                params.color.b,
                params.fill_alpha,
            );
            context.fill_preserve();
        }
        context.set_source_rgba(
            params.color.r,
            params.color.g,
            params.color.b,
            params.line_alpha,
        );
        context.stroke();
        Ok(())
    } else {
        Err("xs.len() != t * 2")
    }
}

pub fn write_png(surface: &cairo::ImageSurface, filename: &str) -> Option<()> {
    File::create(filename)
        .ok()
        .as_mut()
        .and_then(|mut file| surface.write_to_png(&mut file).ok())
}
