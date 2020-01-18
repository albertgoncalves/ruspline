#[cfg(test)]
mod benches {
    extern crate test;

    use crate::spline;
    use rand::prelude::{SeedableRng, StdRng};
    use rand_distr::Normal;
    use spline::{Point, Slice};
    use test::Bencher;

    const ALPHA: f64 = 0.5;
    const INVERSE_TENSION: f64 = 0.5;
    const N_POINTS: usize = 20;

    #[bench]
    fn bench_spline(b: &mut Bencher) {
        let mut rng: StdRng = SeedableRng::seed_from_u64(0);
        let distrbution: Normal<f64> =
            Normal::new(crate::MEAN, crate::STD).unwrap();
        let points: Vec<Point> =
            crate::random_points(&distrbution, &mut rng, N_POINTS);
        let slices: Vec<Slice> = spline::make_slices(crate::N_SLICES);
        b.iter(|| {
            spline::make_spline(&points, &slices, ALPHA, INVERSE_TENSION)
        })
    }
}
