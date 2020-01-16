#[cfg(test)]
mod tests {
    extern crate test;

    use crate::spline;
    use test::Bencher;

    const TS: [f32; 11] =
        [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

    #[test]
    fn ts_valid() {
        assert_eq!(TS.to_vec(), spline::init_ts(10));
    }

    const POINTS: [f32; 8] = [
        -1.0, 0.0, // x, y,
        -0.5, 0.5, //
        0.5, -0.5, //
        1.0, 0.0,
    ];

    #[test]
    fn spline_valid() {
        assert_eq!(
            spline::spline(&POINTS, 4, 2, 2, &TS),
            Some(vec![
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
            ]),
        );
    }

    #[test]
    fn spline_invalid() {
        assert_eq!(
            spline::spline(
                &[
                    -1.0, 0.0, // x, y,
                    -0.5, 0.5, //
                    0.5, -0.5, //
                    1.0,
                ],
                4,
                2,
                2,
                &TS
            ),
            None
        );
    }

    #[bench]
    fn bench_spline(b: &mut Bencher) {
        let ts: Vec<f32> = spline::init_ts(1000);
        b.iter(|| spline::spline(&POINTS, 4, 2, 2, &ts));
    }
}
