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

fn pt_add_pt(a: &Point, b: &Point) -> Point {
    Point {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

fn pt_sub_pt(a: &Point, b: &Point) -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

fn pt_mul_fl(a: &Point, fl: f64) -> Point {
    Point {
        x: a.x * fl,
        y: a.y * fl,
    }
}

fn pt_div_fl(a: &Point, fl: f64) -> Point {
    Point {
        x: a.x / fl,
        y: a.y / fl,
    }
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
        let p2_sub_p1 = pt_sub_pt(p2, p1);
        let m1: Point = pt_mul_fl(
            &pt_add_pt(
                &p2_sub_p1,
                &pt_mul_fl(
                    &pt_sub_pt(
                        &pt_div_fl(&pt_sub_pt(p1, p0), d01),
                        &pt_div_fl(&pt_sub_pt(p2, p0), d01 + d12),
                    ),
                    d12,
                ),
            ),
            inverse_tension,
        );
        let m2: Point = pt_mul_fl(
            &pt_add_pt(
                &p2_sub_p1,
                &pt_mul_fl(
                    &pt_sub_pt(
                        &pt_div_fl(&pt_sub_pt(p3, p2), d23),
                        &pt_div_fl(&pt_sub_pt(p3, p1), d12 + d23),
                    ),
                    d12,
                ),
            ),
            inverse_tension,
        );
        let p1_sub_p2: Point = pt_sub_pt(p1, p2);
        let s_a: Point =
            pt_add_pt(&pt_add_pt(&pt_mul_fl(&p1_sub_p2, 2.0), &m1), &m2);
        let s_b: Point = pt_sub_pt(
            &pt_sub_pt(&pt_sub_pt(&pt_mul_fl(&p1_sub_p2, -3.0), &m1), &m1),
            &m2,
        );
        for slice in slices {
            spline.push(pt_add_pt(
                &pt_add_pt(
                    &pt_add_pt(
                        &pt_mul_fl(&s_a, slice.t_cubed),
                        &pt_mul_fl(&s_b, slice.t_squared),
                    ),
                    &pt_mul_fl(&m1, slice.t),
                ),
                p1,
            ));
        }
    }
    spline
}
