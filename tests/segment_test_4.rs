use boostvoronoi::builder::Builder;
use boostvoronoi::{BvError, InputType};
use boostvoronoi::Line;

type I1 = i32;
type F1 = f64;
type I2 = i64;
type F2 = f64;

#[allow(dead_code)]
fn almost_equal(x1: F1, x2: F1, y1: F1, y2: F1) -> bool {
    let delta = 0.00001;
    assert!(F1::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F1::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F1::abs(x1 - x2) < delta) && (F1::abs(y1 - y2) < delta)
}

fn to_segments<T: InputType>(points: &[[T; 4]]) -> Vec<Line<T>> {
    points.iter().map(|x| x.into()).collect()
}

#[test]
fn segment_4_1_intersecting()  {
    let _output = {
        let segments: [[I1; 4]; 9] = [
            [207, 208, 405, 400],
            [409, 401, 200, 201],
            [400, 402, 403, 230],
            [410, 203, 204, 220],
            [529, 244, 367, 107],
            [94, 628, 512, 632],
            [680, 608, 669, 291],
            [572, 96, 363, 51],
            [147, 103, 96, 170],
        ];
        let segments = to_segments(&segments);

        let mut vb = Builder::<I1, F1, I2, F2>::new();
        vb.with_segments(segments.iter()).expect("should not fail");
        vb.construct()
    };
    _output.expect_err("should fail");
}


#[ignore]
#[test]
fn segment_4_1() -> Result<(), BvError> {
    let _output = {
        let segments: [[I1; 4]; 9] = [
            [200, 200, 200, 400],
            [200, 400, 400, 400],
            [400, 400, 400, 200],
            [400, 200, 200, 200],
            [529, 242, 367, 107],
            [94, 628, 512, 632],
            [680, 608, 669, 291],
            [572, 96, 363, 51],
            [147, 103, 96, 170],
        ];
        let segments = to_segments(&segments);

        let mut vb = Builder::<I1, F1, I2, F2>::new();
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    Ok(())
}
