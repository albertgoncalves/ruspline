#![feature(test)]

mod drawing;
mod spline;
mod test;

use drawing::Color;
use rand::prelude::{Rng, SeedableRng, StdRng};
use std::env;
use std::process;

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

fn parse_args() -> (i32, i32, u64, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() == 5 {
        if let (Ok(width), Ok(height), Ok(seed)) = (
            args[1].parse::<i32>(),
            args[2].parse::<i32>(),
            args[3].parse::<u64>(),
        ) {
            return (width, height, seed, args[4].to_owned());
        }
    }
    eprintln!(
        "usage: {} <width: int> <height: int> <seed: int> <filename: string>",
        &args[0]
    );
    process::exit(1);
}

fn random_points(rng: &mut StdRng, n: usize) -> Vec<f32> {
    (0..n * 2).map(|_| (rng.gen::<f32>() * 2.0) - 1.0).collect()
}

fn main() {
    let (width, height, seed, filename): (i32, i32, u64, String) =
        parse_args();
    let res_int: i32 = 256;
    let res_float: f64 = f64::from(res_int);
    let res_half: f64 = res_float / 2.0;
    let n_points: usize = 15;
    let n_slices: usize = 1000; /* NOTE: curve.len() == n_slices + 1 */
    let (surface, context): (cairo::ImageSurface, cairo::Context) =
        drawing::init_surface(res_int * width, res_int * height, &WHITE)
            .unwrap();
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
    for i in 0..width {
        for j in 0..height {
            let x = (f64::from(i) * res_float) + res_half;
            let y = (f64::from(j) * res_float) + res_half;
            context.save();
            context.translate(x, y);
            context.scale(res_half, res_half);
            let points: Vec<f32> = random_points(&mut rng, n_points);
            let curve: Vec<f32> = spline::spline(
                &points,
                n_points,
                2,
                5,
                &spline::init_ts(n_slices),
            )
            .unwrap();
            drawing::draw_lines(
                &context,
                &curve,
                n_slices + 1,
                6.5 / res_float,
                1.0,
                true,
                0.075,
                &BLACK,
            )
            .unwrap();
            drawing::draw_lines(
                &context,
                &points,
                n_points,
                4.0 / res_float,
                0.125,
                false,
                0.0,
                &BLACK,
            )
            .unwrap();
            context.restore();
        }
    }
    drawing::write_png(&surface, &filename).unwrap();
}
