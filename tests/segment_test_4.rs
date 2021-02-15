use boostvoronoi::builder::Builder;
//use boostvoronoi::diagram as VD;
use boostvoronoi::InputType;
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


fn to_segments<T:InputType>(points: &[[T; 4]]) -> Vec<Line<T>>
{
    points.iter().map(|x|x.into()).collect()
}

#[ignore]
#[test]
fn segment_1() {
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
        vb.with_segments(segments.iter()).expect("segment_1");
        vb.construct().expect("segment_1")
    };
}
