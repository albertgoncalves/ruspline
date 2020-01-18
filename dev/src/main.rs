#![feature(test)]

mod bench;
mod spline;

use cairo::{Antialias, Context, Format, ImageSurface, LineCap};
use rand::prelude::{SeedableRng, StdRng};
use rand_distr::{Distribution, Normal};
use spline::{Point, Slice};
use std::env;
use std::f64;
use std::fs::File;
use std::process;

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

const LINE_WIDTH: f64 = 0.01;
const ARC_RADIUS: f64 = 0.025;
const PI_2: f64 = f64::consts::PI * 2.0;

const TILE_SCALE: f64 = 0.65;
const TILE_OFFSET: f64 = 0.5;

const N_SLICES: usize = 100;

const MEAN: f64 = 0.0;
const STD: f64 = 0.2;

fn random_points(
    distribution: &Normal<f64>,
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

struct Args {
    alpha: f64,
    tension: f64,
    n_points: u8,
    seed: u64,
    width: u16,
    height: u16,
    tile_res: u16,
    filename: String,
}

fn parse() -> Args {
    let args: Vec<String> = env::args().collect();
    if args.len() == 9 {
        if let (
            Ok(alpha),
            Ok(tension),
            Ok(n_points),
            Ok(seed),
            Ok(width),
            Ok(height),
            Ok(tile_res),
        ) = (
            args[1].parse::<f64>(),
            args[2].parse::<f64>(),
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
            {
                return Args {
                    alpha,
                    tension,
                    n_points,
                    seed,
                    width,
                    height,
                    tile_res,
                    filename: args[8].to_owned(),
                };
            }
        }
    }
    eprintln!(
        "usage: {} <alpha: f64> <tension: f64> <n_points: u8> <seed: u64> \
         <width: u16> <height: u16> <tile_res: u16> <filename: string>",
        &args[0]
    );
    process::exit(1);
}

fn main() {
    let args: Args = parse();
    let tile_res_f: f64 = f64::from(args.tile_res);
    let scale: f64 = tile_res_f * TILE_SCALE;
    let mut rng: StdRng = SeedableRng::seed_from_u64(args.seed);
    let distrbution: Normal<f64> = Normal::new(MEAN, STD).unwrap();
    let slices: Vec<Slice> = spline::make_slices(N_SLICES);
    let inverse_tension: f64 = 1.0 - args.tension;
    let surface: ImageSurface = ImageSurface::create(
        Format::ARgb32,
        (args.tile_res * args.width).into(),
        (args.tile_res * args.height).into(),
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
            let x = (f64::from(i) + TILE_OFFSET) * tile_res_f;
            let y = (f64::from(j) + TILE_OFFSET) * tile_res_f;
            context.save();
            context.translate(x, y);
            context.scale(scale, scale);
            let points: Vec<Point> =
                random_points(&distrbution, &mut rng, args.n_points.into());
            let spline: Vec<Point> = spline::make_spline(
                &points,
                &slices,
                args.alpha,
                inverse_tension,
            );
            for point in points {
                let x: f64 = point.x;
                let y: f64 = point.y;
                context.move_to(x, y);
                context.arc(x, y, ARC_RADIUS, 0.0, PI_2);
            }
            context.set_source_rgba(TEAL.r, TEAL.g, TEAL.b, TEAL.a);
            context.fill();
            context.move_to(spline[0].x, spline[0].y);
            for point in spline[1..].iter() {
                context.line_to(point.x, point.y);
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
