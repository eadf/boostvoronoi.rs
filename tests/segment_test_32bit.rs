use boostvoronoi::builder as VB;
use boostvoronoi::BvError;

type I = i32;
type F = f32;

#[allow(dead_code)]
fn almost_equal(x1: F, x2: F, y1: F, y2: F) -> bool {
    let delta = 0.00001;
    assert!(F::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F::abs(x1 - x2) < delta) && (F::abs(y1 - y2) < delta)
}

//#[ignore]
#[test]
// this test crashed once, but seem to be work now???
fn segment_32bit_1() -> Result<(), BvError> {
    let _output = {
        let points: [[I; 2]; 1] = [[428, 263]];
        let points = VB::to_points(&points);
        let segments: [[I; 4]; 6] = [
            [0, 0, 100, 0],
            [100, 0, 100, 100],
            [100, 100, 0, 100],
            [0, 100, 0, 0],
            [40, 50, 60, 50],
            [204, 40, 136, 294],
        ];
        let segments = VB::to_segments(&segments);

        let mut vb = VB::Builder::<I, F>::default();
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    Ok(())
}
