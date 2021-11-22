use boostvoronoi::prelude::*;
use boostvoronoi::BvError;

mod common;
use common::almost_equal;

type I = i32;
type F = f64;

//#[ignore]
#[test]
/// four segments in a loop + one more
fn two_segments_9() -> Result<(), BvError> {
    let output = {
        let points: [[I; 2]; 0] = [];
        let segments: [[I; 4]; 5] = [
            [200, 200, 200, 400],
            [200, 400, 400, 400],
            [400, 400, 400, 200],
            [400, 200, 200, 200],
            [529, 242, 367, 107],
        ];

        Builder::<I, F>::default()
            .with_vertices(points.iter())
            .expect("two_segments_9")
            .with_segments(segments.iter())
            .expect("two_segments_9")
            .build()
            .expect("two_segments_9")
    };
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 200.0000000, v.y(), 200.0000000));
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 200.0000000, v.y(), 400.0000000));
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 333.3293560, v.y(), 147.4047728));
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 200.0000000, v.y(), 3.5591398));
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 400.0000000, v.y(), 200.0000000));
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 300.0000000, v.y(), 300.0000000));
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 400.0000000, v.y(), 400.0000000));
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 400.0000000, v.y(), 171.5428751));
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 430.6785590, v.y(), 200.0000000));
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 478.6496933, v.y(), 302.4203680));
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 561.2596899, v.y(), 400.0000000));
    Ok(())
}
