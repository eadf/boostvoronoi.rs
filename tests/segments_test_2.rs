#![allow(unused_imports)]

use boostvoronoi::voronoi_builder::VoronoiBuilder;
use boostvoronoi::voronoi_diagram::VoronoiDiagram;
use boostvoronoi::voronoi_error::BVError;
use boostvoronoi::{BoostInputType, BoostOutputType};
use geo::{Line, Point};
use std::ops::Neg;

type I1 = i32;
//type F1 = f32;
type F1 = f64;
type I2 = i64;
type F2 = f64;

fn almost_equal(x1: F1, x2: F1, y1: F1, y2: F1) -> bool {
    let delta = 0.0001;
    assert!(F1::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F1::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F1::abs(x1 - x2) < delta) && (F1::abs(y1 - y2) < delta)
}

fn to_points<T>(points: &[[T; 2]]) -> Vec<Point<T>>
where
    T: BoostInputType + Neg<Output = T>,
{
    let mut rv = Vec::with_capacity(points.len());
    for p in points.iter() {
        rv.push(Point::<T>::new(p[0], p[1]));
    }
    rv
}

fn to_segments<T>(points: &[[T; 4]]) -> Vec<Line<T>>
where
    T: BoostInputType + Neg<Output = T>,
{
    let mut rv = Vec::with_capacity(points.len());
    for p in points.iter() {
        rv.push(Line::<T>::new(
            Point::<T>::new(p[0], p[1]),
            Point::<T>::new(p[2], p[3]),
        ));
    }
    rv
}

//#[ignore]
#[test]
/// four segments in a loop + one more
fn two_segments_9() {
    let output = {
        let points: [[I1; 2]; 0] = [];
        let segments: [[I1; 4]; 5] = [
            [200, 200, 200, 400],
            [200, 400, 400, 400],
            [400, 400, 400, 200],
            [400, 200, 200, 200],
            [529, 242, 367, 107],
        ];

        let _v = to_points::<I1>(&points);
        let _s = to_segments::<I1>(&segments);

        let mut vb = VoronoiBuilder::<I1, F1, I2, F2>::new();
        vb.with_vertices(_v.iter()).expect("two_segments_9");
        vb.with_segments(_s.iter()).expect("two_segments_9");
        vb.construct().expect("two_segments_9")
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
}
