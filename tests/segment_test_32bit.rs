use boostvoronoi::builder as VB;
use boostvoronoi::BvError;

type I1 = i32;
type F1 = f32;

#[allow(dead_code)]
fn almost_equal(x1: F1, x2: F1, y1: F1, y2: F1) -> bool {
    let delta = 0.00001;
    assert!(F1::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F1::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F1::abs(x1 - x2) < delta) && (F1::abs(y1 - y2) < delta)
}

//#[ignore]
#[test]
// this test crashed once, but seem to be work now???
fn segment_32bit_1() -> Result<(), BvError> {
    let _output = {
        let points: [[I1; 2]; 1] = [[428, 263]];
        let points = VB::to_points(&points);
        let segments: [[I1; 4]; 6] = [
            [0, 0, 100, 0],
            [100, 0, 100, 100],
            [100, 100, 0, 100],
            [0, 100, 0, 0],
            [40, 50, 60, 50],
            [204, 40, 136, 294],
        ];
        let segments = VB::to_segments(&segments);

        let mut vb = VB::Builder::<I1, F1>::default();
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    Ok(())
}
