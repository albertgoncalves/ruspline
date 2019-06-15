#![feature(test)]

mod drawing;
mod spline;
mod test;

use drawing::{init_surface, write_png, Color};
use rand::prelude::{Rng, SeedableRng, StdRng};
use spline::{init_ts, spline};
use std::env;
use std::process::exit;

struct CurveParams {
    lw_curve: f64,
    lw_skeleton: f64,
    n_control: usize,
    n_slices: usize,
    degree: usize,
    color: Color,
}

struct LineParams<'a> {
    width: f64,
    line_alpha: f64,
    fill: bool,
    fill_alpha: f64,
    color: &'a Color,
}

const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};
const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

fn parse_args() -> (i32, i32, u64) {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 {
        if let (Ok(width), Ok(height), Ok(seed)) = (
            args[1].parse::<i32>(),
            args[2].parse::<i32>(),
            args[3].parse::<u64>(),
        ) {
            return (width, height, seed);
        }
    }
    eprintln!("usage: {} <width: int> <height: int> <seed: int>", &args[0]);
    exit(1);
}

fn random_points(rng: &mut StdRng, n: usize) -> Vec<f32> {
    (0..n * 2).map(|_| (rng.gen::<f32>() * 2.0) - 1.0).collect()
}

fn draw_lines<'a>(
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

fn iter_curve(
    context: &cairo::Context,
    rng: &mut StdRng,
    resolution: f64,
    width: i32,
    height: i32,
    params: &CurveParams,
) {
    let res_half: f64 = resolution / 2.0;
    for i in 0..width {
        for j in 0..height {
            let x = (f64::from(i) * resolution) + res_half;
            let y = (f64::from(j) * resolution) + res_half;
            context.save();
            context.translate(x, y);
            context.scale(res_half, res_half);
            let points: Vec<f32> = random_points(rng, params.n_control);
            let curve: Vec<f32> = spline(
                &points,
                params.n_control,
                2,
                params.degree,
                &init_ts(params.n_slices),
            )
            .expect("Unable to generate curve");
            draw_lines(
                context,
                &curve,
                params.n_slices + 1,
                &LineParams {
                    width: params.lw_curve,
                    line_alpha: 1.0,
                    fill: true,
                    fill_alpha: 0.075,
                    color: &params.color,
                },
            )
            .expect("Unable to draw curve");
            draw_lines(
                context,
                &points,
                params.n_control,
                &LineParams {
                    width: params.lw_skeleton,
                    line_alpha: 0.125,
                    fill: false,
                    fill_alpha: 0.0,
                    color: &params.color,
                },
            )
            .expect("Unable to draw skeleton");
            context.restore();
        }
    }
}

fn main() {
    let (width, height, seed): (i32, i32, u64) = parse_args();
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    let resolution_int: i32 = 256;
    let resolution_float: f64 = f64::from(resolution_int);
    let n_control: usize = 15;
    let degree: usize = 5;
    let n_slices: usize = 1000; // curve.len() == n_slices + 1
    let lw_curve: f64 = 6.5 / resolution_float;
    let lw_skeleton: f64 = 4.0 / resolution_float;
    let (surface, context): (cairo::ImageSurface, cairo::Context) =
        init_surface(resolution_int * width, resolution_int * height, &WHITE);
    iter_curve(
        &context,
        &mut rng,
        resolution_float,
        width,
        height,
        &CurveParams {
            lw_curve,
            lw_skeleton,
            n_control,
            n_slices,
            degree,
            color: BLACK,
        },
    );
    write_png(surface);
}
