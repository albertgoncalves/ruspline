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

#[allow(clippy::integer_division, clippy::too_many_arguments)]
pub fn draw_lines<'a>(
    context: &cairo::Context,
    xs: &'a [f32],
    n: usize,
    width: f64,
    line_alpha: f64,
    fill: bool,
    fill_alpha: f64,
    color: &Color,
) -> Result<(), &'static str> {
    if xs.len() == n * 2 {
        context.move_to(xs[0].into(), xs[1].into());
        for i in 1..n / 2 {
            context.line_to(xs[i * 2].into(), xs[(i * 2) + 1].into());
        }
        context.set_line_width(width);
        if fill {
            context.set_source_rgba(color.r, color.g, color.b, fill_alpha);
            context.fill_preserve();
        }
        context.set_source_rgba(color.r, color.g, color.b, line_alpha);
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
