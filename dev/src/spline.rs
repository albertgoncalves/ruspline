#[allow(clippy::cast_precision_loss)]
pub fn init_ts(n_points: usize) -> Vec<f32> {
    (0..=n_points).map(|x| x as f32 / n_points as f32).collect()
}

pub fn init_vs(
    points: &[f32],
    n_points: usize,
    n_dimensions: usize,
) -> Vec<f32> {
    let mut vs: Vec<f32> = vec![1.0; n_points * (n_dimensions + 1)];
    for i in 0..n_points {
        for j in 0..n_dimensions {
            vs[(i * (n_dimensions + 1)) + j] = points[(i * n_dimensions) + j];
        }
    }
    vs
}

fn find_s(
    n_points: usize,
    degree: usize,
    t: f32,
    knots: &[f32],
) -> Option<usize> {
    for i in degree..n_points {
        if (t <= knots[i + 1]) && (knots[i] <= t) {
            return Some(i);
        }
    }
    None
}

#[inline]
fn deboor(
    n_dimensions: usize,
    degree: usize,
    t: f32,
    s: usize,
    mut xs: Vec<f32>,
    knots: &[f32],
) -> Vec<f32> {
    for l in 1..=degree {
        for i in (s - degree + l..=s).rev() {
            let alpha: f32 =
                (t - knots[i]) / (knots[i + degree + 1 - l] - knots[i]);
            for j in 0..=n_dimensions {
                let ij: usize = (i * (n_dimensions + 1)) + j;
                xs[ij] = ((1.0 - alpha)
                    * xs[((i - 1) * (n_dimensions + 1)) + j])
                    + (alpha * xs[ij]);
            }
        }
    }
    xs
}

#[inline]
#[allow(clippy::cast_precision_loss)]
pub fn spline(
    points: &[f32],
    n_points: usize,
    n_dimensions: usize,
    degree: usize,
    ts: &[f32],
) -> Option<Vec<f32>> {
    if (n_points * n_dimensions) != points.len() {
        return None;
    }
    let knots: Vec<f32> = (0..=n_points + degree).map(|x| x as f32).collect();
    let low: f32 = knots[degree];
    let high: f32 = knots[n_points];
    let vs: Vec<f32> = init_vs(points, n_points, n_dimensions);
    let mut ys: Vec<f32> = vec![0.0; ts.len() * n_dimensions];
    for k in 0..ts.len() {
        if (0.0 <= ts[k]) && (ts[k] <= 1.0) {
            /* NOTE: "Time" position along spline. */
            let t: f32 = (ts[k] * (high - low)) + low;
            let s: usize = find_s(n_points, degree, t, &knots)?;
            let xs: Vec<f32> =
                deboor(n_dimensions, degree, t, s, vs.clone(), &knots);
            for j in 0..n_dimensions {
                ys[(k * n_dimensions) + j] = xs[(s * (n_dimensions + 1)) + j]
                    / xs[(s * (n_dimensions + 1)) + n_dimensions];
            }
        }
    }
    Some(ys)
}
