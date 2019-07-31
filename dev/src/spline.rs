#[allow(clippy::cast_precision_loss)]
pub fn init_ts(n: usize) -> Vec<f32> {
    (0..=n).map(|x| x as f32 / n as f32).collect()
}

pub fn init_vs(points: &[f32], n: usize, m: usize) -> Vec<f32> {
    let mut vs: Vec<f32> = vec![1.0; n * (m + 1)];
    for i in 0..n {
        for j in 0..m {
            vs[(i * (m + 1)) + j] = points[(i * m) + j];
        }
    }
    vs
}

fn find_s(n: usize, degree: usize, t: f32, knots: &[f32]) -> Option<usize> {
    for i in degree..n {
        if (t <= knots[i + 1]) && (t >= knots[i]) {
            return Some(i);
        }
    }
    None
}

#[inline]
fn deboor(
    m: usize,
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
            for j in 0..=m {
                let imj: usize = (i * (m + 1)) + j;
                xs[imj] = ((1.0 - alpha) * xs[((i - 1) * (m + 1)) + j])
                    + (alpha * xs[imj]);
            }
        }
    }
    xs
}

#[allow(clippy::cast_precision_loss)]
#[inline]
pub fn spline(
    points: &[f32],
    n: usize,
    m: usize,
    degree: usize,
    ts: &[f32],
) -> Option<Vec<f32>> {
    if (n * m) != points.len() {
        return None;
    }
    let knots: Vec<f32> = (0..=n + degree).map(|x| x as f32).collect();
    let low: f32 = knots[degree];
    let high: f32 = knots[n];
    let vs: Vec<f32> = init_vs(points, n, m);
    let mut ys: Vec<f32> = vec![0.0; ts.len() * m];
    for k in 0..ts.len() {
        if (ts[k] >= 0.0) && (ts[k] <= 1.0) {
            let t: f32 = (ts[k] * (high - low)) + low;
            let s: usize = find_s(n, degree, t, &knots)?;
            let xs: Vec<f32> = deboor(m, degree, t, s, vs.clone(), &knots);
            for j in 0..m {
                ys[(k * m) + j] =
                    xs[(s * (m + 1)) + j] / xs[(s * (m + 1)) + m];
            }
        }
    }
    Some(ys)
}
