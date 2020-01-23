#![feature(test)]

use arrayvec::ArrayVec;
use cairo::{Antialias, Context, Format, ImageSurface, LineCap};
use rand::prelude::{SeedableRng, StdRng};
use rand_distr::{Distribution, Normal};
use std::env;
use std::f64;
use std::fs::File;
use std::process;

const CAPACITY: usize = 100;

const LINE_WIDTH: f64 = 0.01;
const ARC_RADIUS: f64 = 0.025;
const PI_2: f64 = f64::consts::PI * 2.0;

const TILE_SCALE: f64 = 0.65;
const TILE_OFFSET: f64 = 0.5;

const MEAN: f32 = 0.0;
const STD: f32 = 0.2;

struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

const LIGHT_GRAY: Color = Color {
    r: 0.95,
    g: 0.95,
    b: 0.95,
    a: 1.0,
};

const DARK_GRAY: Color = Color {
    r: 0.15,
    g: 0.15,
    b: 0.15,
    a: 1.0,
};

const TEAL: Color = Color {
    r: 0.17,
    g: 0.82,
    b: 0.76,
    a: 0.75,
};

struct Args {
    alpha: f32,
    tension: f32,
    n_points: u8,
    seed: u64,
    width: u16,
    height: u16,
    tile_size: u16,
    filename: String,
}

fn get_args() -> Args {
    let args: Vec<String> = env::args().collect();
    if args.len() == 9 {
        if let (
            Ok(alpha),
            Ok(tension),
            Ok(n_points),
            Ok(seed),
            Ok(width),
            Ok(height),
            Ok(tile_size),
        ) = (
            args[1].parse::<f32>(),
            args[2].parse::<f32>(),
            args[3].parse::<u8>(),
            args[4].parse::<u64>(),
            args[5].parse::<u16>(),
            args[6].parse::<u16>(),
            args[7].parse::<u16>(),
        ) {
            if (0.0 <= alpha)
                && (alpha <= 1.0)
                && (0.0 <= tension)
                && (tension <= 1.0)
                && (3 < n_points)
                && (0 < width)
                && (0 < height)
                && (0 < tile_size)
            {
                return Args {
                    alpha,
                    tension,
                    n_points,
                    seed,
                    width,
                    height,
                    tile_size,
                    filename: args[8].to_owned(),
                };
            }
        }
    }
    eprintln!(
        "{} ALPHA TENSION N_POINTS SEED WIDTH HEIGHT TILE_SIZE FILENAME\
         \n  ALPHA     : float  [0.0,1.0]\
         \n  TENSION   : float  [0.0,1.0]\
         \n  N_POINTS  : int    [4,2^8 - 1]\
         \n  SEED      : int    [0,2^64 - 1]\
         \n  WIDTH     : int    [1,2^16 - 1]\
         \n  HEIGHT    : int    [1,2^16 - 1]\
         \n  TILE_SIZE : int    [1,2^16 - 1]\
         \n  FILENAME  : string",
        &args[0]
    );
    process::exit(1);
}

struct Point {
    x: f32,
    y: f32,
}

fn random_points(
    distribution: Normal<f32>,
    rng: &mut StdRng,
    n: usize,
) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::with_capacity(n);
    for _ in 0..n {
        points.push(Point {
            x: distribution.sample(rng),
            y: distribution.sample(rng),
        });
    }
    points
}

fn distance(a: &Point, b: &Point) -> f32 {
    let x: f32 = a.x - b.x;
    let y: f32 = a.y - b.y;
    ((x * x) + (y * y)).sqrt()
}

struct Slice {
    t: f32,
    t_squared: f32,
    t_cubed: f32,
}

#[allow(clippy::cast_precision_loss)]
fn make_slices() -> ArrayVec<[Slice; CAPACITY]> {
    let mut slices: ArrayVec<[Slice; CAPACITY]> = ArrayVec::new();
    for i in 0..CAPACITY {
        let t: f32 = (i as f32) / (CAPACITY as f32);
        let t_squared: f32 = t * t;
        slices.push(Slice {
            t,
            t_squared,
            t_cubed: t_squared * t,
        });
    }
    slices
}

fn make_spline(
    points: &[Point],
    slices: &[Slice],
    alpha: f32,
    inverse_tension: f32,
) -> Vec<Point> {
    let n_points: usize = points.len();
    let n_splines: usize = n_points - 3;
    let n_distances: usize = n_points - 1;
    let mut distances: Vec<f32> = Vec::with_capacity(n_distances);
    for i in 0..n_distances {
        distances.push(distance(&points[i], &points[i + 1]).powf(alpha));
    }
    let mut spline: Vec<Point> = Vec::with_capacity(n_points * CAPACITY);
    for i in 0..n_splines {
        let p0: &Point = &points[i];
        let p1: &Point = &points[i + 1];
        let p2: &Point = &points[i + 2];
        let p3: &Point = &points[i + 3];
        let d01: f32 = distances[i];
        let d12: f32 = distances[i + 1];
        let d23: f32 = distances[i + 2];
        let x_p2_sub_p1: f32 = p2.x - p1.x;
        let y_p2_sub_p1: f32 = p2.y - p1.y;
        let d01_d12: f32 = d01 + d12;
        let d12_d23: f32 = d12 + d23;
        let x_m1: f32 = inverse_tension
            * (x_p2_sub_p1
                + (d12 * (((p1.x - p0.x) / d01) - ((p2.x - p0.x) / d01_d12))));
        let y_m1: f32 = inverse_tension
            * (y_p2_sub_p1
                + (d12 * (((p1.y - p0.y) / d01) - ((p2.y - p0.y) / d01_d12))));
        let x_m2: f32 = inverse_tension
            * (x_p2_sub_p1
                + (d12 * (((p3.x - p2.x) / d23) - ((p3.x - p1.x) / d12_d23))));
        let y_m2: f32 = inverse_tension
            * (y_p2_sub_p1
                + (d12 * (((p3.y - p2.y) / d23) - ((p3.y - p1.y) / d12_d23))));
        let x_p1_sub_p2: f32 = p1.x - p2.x;
        let y_p1_sub_p2: f32 = p1.y - p2.y;
        let x_a: f32 = 2.0 * x_p1_sub_p2 + x_m1 + x_m2;
        let y_a: f32 = 2.0 * y_p1_sub_p2 + y_m1 + y_m2;
        let x_b: f32 = -3.0 * x_p1_sub_p2 - x_m1 - x_m1 - x_m2;
        let y_b: f32 = -3.0 * y_p1_sub_p2 - y_m1 - y_m1 - y_m2;
        for slice in slices {
            spline.push(Point {
                x: (x_a * slice.t_cubed)
                    + (x_b * slice.t_squared)
                    + (x_m1 * slice.t)
                    + p1.x,
                y: (y_a * slice.t_cubed)
                    + (y_b * slice.t_squared)
                    + (y_m1 * slice.t)
                    + p1.y,
            });
        }
    }
    spline
}

#[allow(clippy::cast_lossless)]
fn main() {
    let args: Args = get_args();
    let mut rng: StdRng = SeedableRng::seed_from_u64(args.seed);
    let distrbution: Normal<f32> = Normal::new(MEAN, STD).unwrap();
    let inverse_tension: f32 = 1.0 - args.tension;
    let slices: ArrayVec<[Slice; CAPACITY]> = make_slices();
    let tile_size: f64 = args.tile_size as f64;
    let scale: f64 = tile_size * TILE_SCALE;
    let surface: ImageSurface = ImageSurface::create(
        Format::ARgb32,
        (args.tile_size * args.width) as i32,
        (args.tile_size * args.height) as i32,
    )
    .unwrap();
    let context: Context = Context::new(&surface);
    context.set_antialias(Antialias::Best);
    context.set_line_width(LINE_WIDTH);
    context.set_line_cap(LineCap::Round);
    context.set_source_rgba(
        DARK_GRAY.r,
        DARK_GRAY.g,
        DARK_GRAY.b,
        DARK_GRAY.a,
    );
    context.paint();
    for i in 0..args.width {
        for j in 0..args.height {
            context.save();
            context.translate(
                ((i as f64) + TILE_OFFSET) * tile_size,
                ((j as f64) + TILE_OFFSET) * tile_size,
            );
            context.scale(scale, scale);
            let points: Vec<Point> =
                random_points(distrbution, &mut rng, args.n_points.into());
            let spline: Vec<Point> =
                make_spline(&points, &slices, args.alpha, inverse_tension);
            for point in points {
                let x: f64 = point.x as f64;
                let y: f64 = point.y as f64;
                context.move_to(x, y);
                context.arc(x, y, ARC_RADIUS, 0.0, PI_2);
            }
            context.set_source_rgba(TEAL.r, TEAL.g, TEAL.b, TEAL.a);
            context.fill();
            context.move_to(spline[0].x as f64, spline[0].y as f64);
            for point in spline[1..].iter() {
                context.line_to(point.x as f64, point.y as f64);
            }
            context.set_source_rgba(
                LIGHT_GRAY.r,
                LIGHT_GRAY.g,
                LIGHT_GRAY.b,
                LIGHT_GRAY.a,
            );
            context.stroke();
            context.restore();
        }
    }
    if let Ok(mut file) = File::create(args.filename) {
        surface.write_to_png(&mut file).unwrap();
    }
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    const ALPHA: f32 = 0.5;
    const INVERSE_TENSION: f32 = 0.5;
    const N_POINTS: usize = 20;

    #[bench]
    fn bench_spline(b: &mut Bencher) {
        let mut rng: StdRng = SeedableRng::seed_from_u64(0);
        let distrbution: Normal<f32> = Normal::new(MEAN, STD).unwrap();
        let points: Vec<Point> =
            random_points(distrbution, &mut rng, N_POINTS);
        let slices: ArrayVec<[Slice; CAPACITY]> = make_slices();
        b.iter(|| make_spline(&points, &slices, ALPHA, INVERSE_TENSION))
    }
}
