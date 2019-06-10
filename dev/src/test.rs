#[cfg(test)]
mod tests {
    extern crate test;
    use crate::init_ts;
    use crate::spline;
    use test::Bencher;
    const TS: [f32; 11] =
        [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    const POINTS: [f32; 8] = [
        -1.0, 0.0, // x, y,
        -0.5, 0.5, //
        0.5, -0.5, //
        1.0, 0.0,
    ];
    #[test]
    fn ts_valid() {
        assert_eq!(TS.to_vec(), init_ts(10));
    }
    #[test]
    fn spline_valid() {
        let curve: Vec<f32> = vec![
            -0.75, // x,
            0.25,  // y,
            -0.64,
            0.32,
            -0.50999993,
            0.32999998,
            -0.36000007,
            0.28000003,
            -0.19000004,
            0.17000003,
            0.0,
            0.0,
            0.19000004,
            -0.17000003,
            0.36000007,
            -0.28000003,
            0.50999993,
            -0.32999998,
            0.64,
            -0.32,
            0.75,
            -0.25,
        ];
        assert_eq!(
            spline(&POINTS.to_vec(), 4, 2, 2, &TS.to_vec()),
            Some(curve),
        );
    }
    #[test]
    fn spline_invalid() {
        let points: Vec<f32> = vec![
            -1.0, 0.0, // x, y,
            -0.5, 0.5, //
            0.5, -0.5, //
            1.0,
        ];
        assert_eq!(spline(&points, 4, 2, 2, &TS.to_vec()), None);
    }
    #[bench]
    fn bench_spline(b: &mut Bencher) {
        let points: Vec<f32> = POINTS.to_vec();
        let ts: Vec<f32> = init_ts(1000);
        b.iter(|| spline(&points, 4, 2, 2, &ts));
    }
}
