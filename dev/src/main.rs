#![feature(test)]

use arrayvec::ArrayVec;
use cairo::{Antialias, Context, Format, ImageSurface, LineCap};
use rand::prelude::{SeedableRng, StdRng};
use rand_distr::{Distribution, Normal};
use std::env;
use std::f64;
use std::fs::File;
use std::ops::{Add, Div, Mul, Sub};
use std::process;

const CAPACITY: usize = 100;

const LINE_WIDTH: f64 = 0.01;
const ARC_RADIUS: f64 = 0.025;
const PI_2: f64 = f64::consts::PI * 2.0;

const TILE_SCALE: f64 = 0.65;
const TILE_OFFSET: f64 = 0.5;

const MEAN: f64 = 0.0;
const STD: f64 = 0.2;

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
    alpha: f64,
    tension: f64,
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
                && (3 < n_points)
                && (width != 0)
                && (height != 0)
                && (tile_size != 0)
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

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f64> for Point {
    type Output = Point;
    fn add(self, other: f64) -> Point {
        Point {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f64> for Point {
    type Output = Point;
    fn sub(self, other: f64) -> Point {
        Point {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl Mul<Point> for Point {
    type Output = Point;
    fn mul(self, other: Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<Point> for Point {
    type Output = Point;
    fn div(self, other: Point) -> Point {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;
    fn div(self, other: f64) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

fn get_random_points(
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

fn get_distance(a: &Point, b: &Point) -> f64 {
    let x: f64 = a.x - b.x;
    let y: f64 = a.y - b.y;
    ((x * x) + (y * y)).sqrt()
}

struct Slice {
    t: f64,
    t_squared: f64,
    t_cubed: f64,
}

#[allow(clippy::cast_precision_loss)]
fn get_slices() -> ArrayVec<[Slice; CAPACITY]> {
    let mut slices: ArrayVec<[Slice; CAPACITY]> = ArrayVec::new();
    for i in 0..CAPACITY {
        let t: f64 = (i as f64) / (CAPACITY as f64);
        let t_squared: f64 = t * t;
        slices.push(Slice {
            t,
            t_squared,
            t_cubed: t_squared * t,
        });
    }
    slices
}

fn get_spline(
    points: &[Point],
    slices: &[Slice],
    alpha: f64,
    inverse_tension: f64,
) -> Vec<Point> {
    let n_points: usize = points.len();
    let n_splines: usize = n_points - 3;
    let n_distances: usize = n_points - 1;
    let mut distances: Vec<f64> = Vec::with_capacity(n_distances);
    for i in 0..n_distances {
        distances.push(get_distance(&points[i], &points[i + 1]).powf(alpha));
    }
    let mut spline: Vec<Point> = Vec::with_capacity(n_points * CAPACITY);
    for i in 0..n_splines {
        let p0: Point = points[i];
        let p1: Point = points[i + 1];
        let p2: Point = points[i + 2];
        let p3: Point = points[i + 3];
        let d01: f64 = distances[i];
        let d12: f64 = distances[i + 1];
        let d23: f64 = distances[i + 2];
        let p2_sub_p1: Point = p2 - p1;
        let d01_d12: f64 = d01 + d12;
        let d12_d23: f64 = d12 + d23;
        let m1: Point = (p2_sub_p1
            + ((((p1 - p0) / d01) - ((p2 - p0) / d01_d12)) * d12))
            * inverse_tension;
        let m2: Point = (p2_sub_p1
            + ((((p3 - p2) / d23) - ((p3 - p1) / d12_d23)) * d12))
            * inverse_tension;
        let p1_sub_p2: Point = p1 - p2;
        let a: Point = (p1_sub_p2 * 2.0) + m1 + m2;
        let b: Point = (p1_sub_p2 * -3.0) - (m1 * 2.0) - m2;
        for slice in slices {
            spline.push(
                (a * slice.t_cubed)
                    + (b * slice.t_squared)
                    + (m1 * slice.t)
                    + p1,
            );
        }
    }
    spline
}

#[allow(clippy::cast_lossless)]
fn main() {
    let args: Args = get_args();
    let mut rng: StdRng = SeedableRng::seed_from_u64(args.seed);
    let distrbution: Normal<f64> = Normal::new(MEAN, STD).unwrap();
    let inverse_tension: f64 = 1.0 - args.tension;
    let slices: ArrayVec<[Slice; CAPACITY]> = get_slices();
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
            let points: Vec<Point> = get_random_points(
                &distrbution,
                &mut rng,
                args.n_points as usize,
            );
            let spline: Vec<Point> =
                get_spline(&points, &slices, args.alpha, inverse_tension);
            context.save();
            context.translate(
                ((i as f64) + TILE_OFFSET) * tile_size,
                ((j as f64) + TILE_OFFSET) * tile_size,
            );
            context.scale(scale, scale);
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

#[cfg(test)]
mod benches {
    extern crate test;

    use super::{
        f64, get_random_points, get_slices, get_spline, Add, ArrayVec,
        Distribution, Div, Mul, Normal, Point, SeedableRng, Slice, StdRng,
        Sub, CAPACITY, MEAN, STD,
    };
    use test::Bencher;

    const ALPHA: f64 = 0.5;
    const INVERSE_TENSION: f64 = 0.5;
    const N_POINTS: usize = 20;

    #[bench]
    fn bench_get_spline_f64(b: &mut Bencher) {
        let mut rng: StdRng = SeedableRng::seed_from_u64(0);
        let distrbution: Normal<f64> = Normal::new(MEAN, STD).unwrap();
        let points: Vec<Point> =
            get_random_points(&distrbution, &mut rng, N_POINTS);
        let slices: ArrayVec<[Slice; CAPACITY]> = get_slices();
        b.iter(|| get_spline(&points, &slices, ALPHA, INVERSE_TENSION))
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy)]
    struct Point_f32 {
        x: f32,
        y: f32,
    }

    impl Add<Point_f32> for Point_f32 {
        type Output = Point_f32;
        fn add(self, other: Point_f32) -> Point_f32 {
            Point_f32 {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Add<f32> for Point_f32 {
        type Output = Point_f32;
        fn add(self, other: f32) -> Point_f32 {
            Point_f32 {
                x: self.x + other,
                y: self.y + other,
            }
        }
    }

    impl Sub<Point_f32> for Point_f32 {
        type Output = Point_f32;
        fn sub(self, other: Point_f32) -> Point_f32 {
            Point_f32 {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Sub<f32> for Point_f32 {
        type Output = Point_f32;
        fn sub(self, other: f32) -> Point_f32 {
            Point_f32 {
                x: self.x - other,
                y: self.y - other,
            }
        }
    }

    impl Mul<Point_f32> for Point_f32 {
        type Output = Point_f32;
        fn mul(self, other: Point_f32) -> Point_f32 {
            Point_f32 {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }

    impl Mul<f32> for Point_f32 {
        type Output = Point_f32;
        fn mul(self, other: f32) -> Point_f32 {
            Point_f32 {
                x: self.x * other,
                y: self.y * other,
            }
        }
    }

    impl Div<Point_f32> for Point_f32 {
        type Output = Point_f32;
        fn div(self, other: Point_f32) -> Point_f32 {
            Point_f32 {
                x: self.x / other.x,
                y: self.y / other.y,
            }
        }
    }

    impl Div<f32> for Point_f32 {
        type Output = Point_f32;
        fn div(self, other: f32) -> Point_f32 {
            Point_f32 {
                x: self.x / other,
                y: self.y / other,
            }
        }
    }

    fn get_random_points_f32(
        distribution: Normal<f32>,
        rng: &mut StdRng,
        n: usize,
    ) -> Vec<Point_f32> {
        let mut points: Vec<Point_f32> = Vec::with_capacity(n);
        for _ in 0..n {
            points.push(Point_f32 {
                x: distribution.sample(rng),
                y: distribution.sample(rng),
            });
        }
        points
    }

    fn get_distance_f32(a: Point_f32, b: Point_f32) -> f32 {
        let x: f32 = a.x - b.x;
        let y: f32 = a.y - b.y;
        ((x * x) + (y * y)).sqrt()
    }

    #[allow(non_camel_case_types)]
    struct Slice_f32 {
        t: f32,
        t_squared: f32,
        t_cubed: f32,
    }

    #[allow(clippy::cast_precision_loss)]
    fn get_slices_f32() -> ArrayVec<[Slice_f32; CAPACITY]> {
        let mut slices: ArrayVec<[Slice_f32; CAPACITY]> = ArrayVec::new();
        for i in 0..CAPACITY {
            let t: f32 = (i as f32) / (CAPACITY as f32);
            let t_squared: f32 = t * t;
            slices.push(Slice_f32 {
                t,
                t_squared,
                t_cubed: t_squared * t,
            });
        }
        slices
    }

    fn get_spline_f32(
        points: &[Point_f32],
        slices: &[Slice_f32],
        alpha: f32,
        inverse_tension: f32,
    ) -> Vec<Point_f32> {
        let n_points: usize = points.len();
        let n_splines: usize = n_points - 3;
        let n_distances: usize = n_points - 1;
        let mut distances: Vec<f32> = Vec::with_capacity(n_distances);
        for i in 0..n_distances {
            distances
                .push(get_distance_f32(points[i], points[i + 1]).powf(alpha));
        }
        let mut spline: Vec<Point_f32> =
            Vec::with_capacity(n_points * CAPACITY);
        for i in 0..n_splines {
            let p0: Point_f32 = points[i];
            let p1: Point_f32 = points[i + 1];
            let p2: Point_f32 = points[i + 2];
            let p3: Point_f32 = points[i + 3];
            let d01: f32 = distances[i];
            let d12: f32 = distances[i + 1];
            let d23: f32 = distances[i + 2];
            let p2_sub_p1: Point_f32 = p2 - p1;
            let d01_d12: f32 = d01 + d12;
            let d12_d23: f32 = d12 + d23;
            let m1: Point_f32 = (p2_sub_p1
                + ((((p1 - p0) / d01) - ((p2 - p0) / d01_d12)) * d12))
                * inverse_tension;
            let m2: Point_f32 = (p2_sub_p1
                + ((((p3 - p2) / d23) - ((p3 - p1) / d12_d23)) * d12))
                * inverse_tension;
            let p1_sub_p2: Point_f32 = p1 - p2;
            let a: Point_f32 = (p1_sub_p2 * 2.0) + m1 + m2;
            let b: Point_f32 = (p1_sub_p2 * -3.0) - (m1 * 2.0) - m2;
            for slice in slices {
                spline.push(
                    (a * slice.t_cubed)
                        + (b * slice.t_squared)
                        + (m1 * slice.t)
                        + p1,
                );
            }
        }
        spline
    }

    #[bench]
    #[allow(clippy::cast_possible_truncation)]
    fn bench_get_spline_f32(b: &mut Bencher) {
        let mut rng: StdRng = SeedableRng::seed_from_u64(0);
        let distrbution: Normal<f32> =
            Normal::new(MEAN as f32, STD as f32).unwrap();
        let points: Vec<Point_f32> =
            get_random_points_f32(distrbution, &mut rng, N_POINTS);
        let slices: ArrayVec<[Slice_f32; CAPACITY]> = get_slices_f32();
        b.iter(|| {
            get_spline_f32(
                &points,
                &slices,
                ALPHA as f32,
                INVERSE_TENSION as f32,
            )
        })
    }
}
