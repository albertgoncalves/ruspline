pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Slice {
    t: f64,
    t_squared: f64,
    t_cubed: f64,
}

fn distance(a: &Point, b: &Point) -> f64 {
    let x: f64 = a.x - b.x;
    let y: f64 = a.y - b.y;
    ((x * x) + (y * y)).sqrt()
}

#[allow(clippy::cast_precision_loss)]
pub fn make_slices(resolution: usize) -> Vec<Slice> {
    let mut slices: Vec<Slice> = Vec::with_capacity(resolution);
    for i in 0..resolution {
        let t: f64 = (i as f64) / (resolution as f64);
        let t_squared: f64 = t * t;
        let t_cubed: f64 = t_squared * t;
        slices.push(Slice {
            t,
            t_squared,
            t_cubed,
        });
    }
    slices
}

#[allow(clippy::module_name_repetitions)]
pub fn make_spline(
    points: &[Point],
    slices: &[Slice],
    alpha: f64,
    inverse_tension: f64,
) -> Vec<Point> {
    let n_points: usize = points.len();
    let resolution: usize = slices.len();
    let n_slices: usize = n_points * resolution;
    let n_splines: usize = n_points - 3;
    let n_distances: usize = n_points - 1;
    let mut distances: Vec<f64> = Vec::with_capacity(n_distances);
    for i in 0..n_distances {
        distances.push(distance(&points[i], &points[i + 1]).powf(alpha));
    }
    let mut spline: Vec<Point> = Vec::with_capacity(n_slices);
    for i in 0..n_splines {
        let p0: &Point = &points[i];
        let p1: &Point = &points[i + 1];
        let p2: &Point = &points[i + 2];
        let p3: &Point = &points[i + 3];
        let d01: f64 = distances[i];
        let d12: f64 = distances[i + 1];
        let d23: f64 = distances[i + 2];
        let x_p2_sub_p1: f64 = p2.x - p1.x;
        let y_p2_sub_p1: f64 = p2.y - p1.y;
        let d01_d12: f64 = d01 + d12;
        let d12_d23: f64 = d12 + d23;
        let x_m1: f64 = inverse_tension
            * (x_p2_sub_p1
                + (d12 * (((p1.x - p0.x) / d01) - ((p2.x - p0.x) / d01_d12))));
        let y_m1: f64 = inverse_tension
            * (y_p2_sub_p1
                + (d12 * (((p1.y - p0.y) / d01) - ((p2.y - p0.y) / d01_d12))));
        let x_m2: f64 = inverse_tension
            * (x_p2_sub_p1
                + (d12 * (((p3.x - p2.x) / d23) - ((p3.x - p1.x) / d12_d23))));
        let y_m2: f64 = inverse_tension
            * (y_p2_sub_p1
                + (d12 * (((p3.y - p2.y) / d23) - ((p3.y - p1.y) / d12_d23))));
        let x_p1_sub_p2: f64 = p1.x - p2.x;
        let y_p1_sub_p2: f64 = p1.y - p2.y;
        let x_a: f64 = 2.0 * x_p1_sub_p2 + x_m1 + x_m2;
        let y_a: f64 = 2.0 * y_p1_sub_p2 + y_m1 + y_m2;
        let x_b: f64 = -3.0 * x_p1_sub_p2 - x_m1 - x_m1 - x_m2;
        let y_b: f64 = -3.0 * y_p1_sub_p2 - y_m1 - y_m1 - y_m2;
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
