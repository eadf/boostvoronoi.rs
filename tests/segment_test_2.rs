use boostvoronoi::builder as VB;
use boostvoronoi::BvError;

type I = i32;
type F = f64;

fn almost_equal(x1: F, x2: F, y1: F, y2: F) -> bool {
    let delta = 0.0001;
    assert!(F::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F::abs(x1 - x2) < delta) && (F::abs(y1 - y2) < delta)
}

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
        //let s = segments.iter().map(|x|x.into()).collect();

        let _v = VB::to_points::<I, I>(&points);
        let _s = VB::to_segments::<I, I>(&segments);

        let mut vb = VB::Builder::<I, F>::default();
        vb.with_vertices(_v.iter()).expect("two_segments_9");
        vb.with_segments(_s.iter()).expect("two_segments_9");
        vb.build().expect("two_segments_9")
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
