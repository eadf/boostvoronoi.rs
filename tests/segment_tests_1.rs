use boostvoronoi::voronoi_builder::VoronoiBuilder;
use boostvoronoi::InputType;
use geo::{Coordinate, Line};
use std::ops::Neg;

type I = i32;
type O = f32;
type DI = i64;
type DF = f64;

fn almost_equal(x1: O, x2: O, y1: O, y2: O) -> bool {
    let delta = 0.0001;
    assert!(O::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(O::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (O::abs(x1 - x2) < delta) && (O::abs(y1 - y2) < delta)
}

fn to_points<T>(points: &[[T; 2]]) -> Vec<Coordinate<T>>
where
    T: InputType + Neg<Output = T>,
{
    let mut rv = Vec::with_capacity(points.len());
    for p in points.iter() {
        rv.push(Coordinate { x: p[0], y: p[1] });
    }
    rv
}

fn to_segments<T>(points: &[[T; 4]]) -> Vec<Line<T>>
where
    T: InputType + Neg<Output = T>,
{
    let mut rv = Vec::with_capacity(points.len());
    for p in points.iter() {
        rv.push(Line::<T>::new(
            Coordinate::<T> { x: p[0], y: p[1] },
            Coordinate::<T> { x: p[2], y: p[3] },
        ));
    }
    rv
}

//#[ignore]
#[test]
fn single_segment_1() {
    let output = {
        let _s = vec![Line::new(
            Coordinate { x: 10, y: 10 },
            Coordinate { x: 50, y: 50 },
        )];
        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_segments(_s.iter()).expect("single_segment_1");
        vb.construct().expect("single_segment_1")
    };
    // results verified against c++ boost
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 4);
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 2);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(2).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 3);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 3);
}

//#[ignore]
#[test]
fn single_segment_2() {
    let output = {
        let _s = vec![Line::new(
            Coordinate { x: 10, y: 10 },
            Coordinate { x: 50, y: 50 },
        )];
        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_segments(_s.iter()).expect("single_segment_2");
        vb.construct().expect("single_segment_2")
    };
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 4);
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 2);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(2).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 3);
}

#[test]
fn single_segment_3() {
    let output = {
        let _s = vec![Line::new(
            Coordinate { x: 10, y: 10 },
            Coordinate { x: 50, y: 10 },
        )];
        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_segments(_s.iter()).expect("single_segment_3");
        vb.construct().expect("single_segment_3")
    };
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 4);
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 2);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(2).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 3);
}

#[test]
fn single_segment_4() {
    let output = {
        let _s = vec![Line::new(
            Coordinate { x: 50, y: 10 },
            Coordinate { x: 10, y: 10 },
        )];
        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_segments(_s.iter()).expect("single_segment_4");
        vb.construct().expect("single_segment_4")
    };
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 4);
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 2);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(2).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 3);
}

//#[ignore]
#[test]
///Two segments
fn two_segments_1() {
    let output = {
        let _s = vec![
            Line::new(Coordinate { x: 1, y: 2 }, Coordinate { x: 3, y: 4 }),
            Line::new(Coordinate { x: 2, y: 2 }, Coordinate { x: 5, y: 4 }),
        ];
        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_segments(_s.iter()).expect("two_segments_1");
        vb.construct().expect("two_segments_1")
    };
    assert_eq!(output.cells().len(), 6);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.source_category().0, 1);
    assert!(cell.contains_point());
    assert!(!cell.contains_segment());
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.source_category().0, 8);
    assert!(!cell.contains_point());
    assert!(cell.contains_segment());
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.source_category().0, 1);
    assert!(cell.contains_point());
    assert!(!cell.contains_segment());
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.source_category().0, 8);
    assert!(!cell.contains_point());
    assert!(cell.contains_segment());
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.source_category().0, 2);
    assert!(cell.contains_point());
    assert!(!cell.contains_segment());
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.source_category().0, 2);
    assert!(cell.contains_point());
    assert!(!cell.contains_segment());
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 1.80196, v.y(), 2.29706));
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 1.5, v.y(), 1.5));
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 3.39608, v.y(), 3.60392));
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 4.0, v.y(), 5.5));
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 8);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 8);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 10);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 7);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 9);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 4);
    assert_eq!(output.edges().get(4).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(4).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(4).unwrap().get().twin().unwrap().0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(4).unwrap().get().prev().unwrap().0, 9);
    assert_eq!(output.edges().get(5).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().twin().unwrap().0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next().unwrap().0, 14);
    assert_eq!(output.edges().get(5).unwrap().get().prev().unwrap().0, 6);
    assert_eq!(output.edges().get(6).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(6).unwrap().get().twin().unwrap().0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next().unwrap().0, 5);
    assert_eq!(output.edges().get(6).unwrap().get().prev().unwrap().0, 12);
    assert_eq!(output.edges().get(7).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(7).unwrap().get().twin().unwrap().0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next().unwrap().0, 10);
    assert_eq!(output.edges().get(7).unwrap().get().prev().unwrap().0, 2);
    assert_eq!(output.edges().get(8).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(8).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(8).unwrap().get().twin().unwrap().0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(8).unwrap().get().prev().unwrap().0, 0);
    assert_eq!(output.edges().get(9).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(9).unwrap().get().twin().unwrap().0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next().unwrap().0, 4);
    assert_eq!(output.edges().get(9).unwrap().get().prev().unwrap().0, 3);
    assert_eq!(output.edges().get(10).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(
        output.edges().get(10).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(10).unwrap().get().twin().unwrap().0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(10).unwrap().get().prev().unwrap().0, 7);
    assert_eq!(output.edges().get(11).unwrap().get().cell().unwrap().0, 4);
    assert!(output.edges().get(11).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(11).unwrap().get().twin().unwrap().0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next().unwrap().0, 13);
    assert_eq!(output.edges().get(11).unwrap().get().prev().unwrap().0, 17);
    assert_eq!(output.edges().get(12).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(
        output.edges().get(12).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(12).unwrap().get().twin().unwrap().0, 13);
    assert_eq!(output.edges().get(12).unwrap().get().next().unwrap().0, 6);
    assert_eq!(output.edges().get(12).unwrap().get().prev().unwrap().0, 14);
    assert_eq!(output.edges().get(13).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(13).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(13).unwrap().get().twin().unwrap().0, 12);
    assert_eq!(output.edges().get(13).unwrap().get().next().unwrap().0, 17);
    assert_eq!(output.edges().get(13).unwrap().get().prev().unwrap().0, 11);
    assert_eq!(output.edges().get(14).unwrap().get().cell().unwrap().0, 3);
    assert!(output.edges().get(14).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(14).unwrap().get().twin().unwrap().0, 15);
    assert_eq!(output.edges().get(14).unwrap().get().next().unwrap().0, 12);
    assert_eq!(output.edges().get(14).unwrap().get().prev().unwrap().0, 5);
    assert_eq!(output.edges().get(15).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(15).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(15).unwrap().get().twin().unwrap().0, 14);
    assert_eq!(output.edges().get(15).unwrap().get().next().unwrap().0, 16);
    assert_eq!(output.edges().get(15).unwrap().get().prev().unwrap().0, 16);
    assert_eq!(output.edges().get(16).unwrap().get().cell().unwrap().0, 5);
    assert!(output.edges().get(16).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(16).unwrap().get().twin().unwrap().0, 17);
    assert_eq!(output.edges().get(16).unwrap().get().next().unwrap().0, 15);
    assert_eq!(output.edges().get(16).unwrap().get().prev().unwrap().0, 15);
    assert_eq!(output.edges().get(17).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(17).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(17).unwrap().get().twin().unwrap().0, 16);
    assert_eq!(output.edges().get(17).unwrap().get().next().unwrap().0, 11);
    assert_eq!(output.edges().get(17).unwrap().get().prev().unwrap().0, 13);
}

//#[ignore]
#[test]
/// two segments and one point
fn two_segments_2() {
    let output = {
        let _v = vec![Coordinate { x: 10, y: 11 }];
        let _s = vec![
            Line::new(Coordinate { x: 1, y: 2 }, Coordinate { x: 3, y: 4 }),
            Line::new(Coordinate { x: 2, y: 2 }, Coordinate { x: 5, y: 4 }),
        ];
        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_vertices(_v.iter()).expect("two_segments_2");
        vb.with_segments(_s.iter()).expect("two_segments_2");
        vb.construct().expect("two_segments_2")
    };
    assert_eq!(output.cells().len(), 7);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 5);
    assert_eq!(output.edges().len(), 22);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 1.80196, v.y(), 2.29706));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 1.50000, v.y(), 1.50000));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 3.39608, v.y(), 3.60392));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 4.00000, v.y(), 5.50000));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 4.00000, v.y(), 10.0000));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 8);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 8);
    let e = output.edges()[0].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 9);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 10);
    let e = output.edges()[1].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 11);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 7);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 1);
    let e = output.edges()[2].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 9);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 4);
    let e = output.edges()[3].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(4).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(4).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(4).unwrap().get().twin().unwrap().0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(4).unwrap().get().prev().unwrap().0, 9);
    let e = output.edges()[4].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 8);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(5).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().twin().unwrap().0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next().unwrap().0, 14);
    assert_eq!(output.edges().get(5).unwrap().get().prev().unwrap().0, 6);
    let e = output.edges()[5].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 7);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(6).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(6).unwrap().get().twin().unwrap().0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next().unwrap().0, 5);
    assert_eq!(output.edges().get(6).unwrap().get().prev().unwrap().0, 12);
    let e = output.edges()[6].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 13);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(7).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(7).unwrap().get().twin().unwrap().0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next().unwrap().0, 10);
    assert_eq!(output.edges().get(7).unwrap().get().prev().unwrap().0, 2);
    let e = output.edges()[7].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(8).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(8).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(8).unwrap().get().twin().unwrap().0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(8).unwrap().get().prev().unwrap().0, 0);
    let e = output.edges()[8].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(9).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(9).unwrap().get().twin().unwrap().0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next().unwrap().0, 4);
    assert_eq!(output.edges().get(9).unwrap().get().prev().unwrap().0, 3);
    let e = output.edges()[9].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(10).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(
        output.edges().get(10).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(10).unwrap().get().twin().unwrap().0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(10).unwrap().get().prev().unwrap().0, 7);
    let e = output.edges()[10].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(11).unwrap().get().cell().unwrap().0, 4);
    assert!(output.edges().get(11).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(11).unwrap().get().twin().unwrap().0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next().unwrap().0, 13);
    assert_eq!(output.edges().get(11).unwrap().get().prev().unwrap().0, 18);
    let e = output.edges()[11].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 19);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(12).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(
        output.edges().get(12).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(12).unwrap().get().twin().unwrap().0, 13);
    assert_eq!(output.edges().get(12).unwrap().get().next().unwrap().0, 6);
    assert_eq!(output.edges().get(12).unwrap().get().prev().unwrap().0, 14);
    let e = output.edges()[12].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 15);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(13).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(13).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(13).unwrap().get().twin().unwrap().0, 12);
    assert_eq!(output.edges().get(13).unwrap().get().next().unwrap().0, 17);
    assert_eq!(output.edges().get(13).unwrap().get().prev().unwrap().0, 11);
    let e = output.edges()[13].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 10);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(14).unwrap().get().cell().unwrap().0, 3);
    assert!(output.edges().get(14).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(14).unwrap().get().twin().unwrap().0, 15);
    assert_eq!(output.edges().get(14).unwrap().get().next().unwrap().0, 12);
    assert_eq!(output.edges().get(14).unwrap().get().prev().unwrap().0, 5);
    let e = output.edges()[14].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(15).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(15).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(15).unwrap().get().twin().unwrap().0, 14);
    assert_eq!(output.edges().get(15).unwrap().get().next().unwrap().0, 20);
    assert_eq!(output.edges().get(15).unwrap().get().prev().unwrap().0, 16);
    let e = output.edges()[15].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 17);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(16).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(16).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(16).unwrap().get().twin().unwrap().0, 17);
    assert_eq!(output.edges().get(16).unwrap().get().next().unwrap().0, 15);
    assert_eq!(output.edges().get(16).unwrap().get().prev().unwrap().0, 20);
    let e = output.edges()[16].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 21);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(17).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(17).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(17).unwrap().get().twin().unwrap().0, 16);
    assert_eq!(output.edges().get(17).unwrap().get().next().unwrap().0, 18);
    assert_eq!(output.edges().get(17).unwrap().get().prev().unwrap().0, 13);
    let e = output.edges()[17].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 12);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(18).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(18).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(18).unwrap().get().twin().unwrap().0, 19);
    assert_eq!(output.edges().get(18).unwrap().get().next().unwrap().0, 11);
    assert_eq!(output.edges().get(18).unwrap().get().prev().unwrap().0, 17);
    let e = output.edges()[18].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 16);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(19).unwrap().get().cell().unwrap().0, 6);
    assert!(output.edges().get(19).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(19).unwrap().get().twin().unwrap().0, 18);
    assert_eq!(output.edges().get(19).unwrap().get().next().unwrap().0, 21);
    assert_eq!(output.edges().get(19).unwrap().get().prev().unwrap().0, 21);
    let e = output.edges()[19].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 20);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(20).unwrap().get().cell().unwrap().0, 5);
    assert!(output.edges().get(20).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(20).unwrap().get().twin().unwrap().0, 21);
    assert_eq!(output.edges().get(20).unwrap().get().next().unwrap().0, 16);
    assert_eq!(output.edges().get(20).unwrap().get().prev().unwrap().0, 15);
    let e = output.edges()[20].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 14);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(21).unwrap().get().cell().unwrap().0, 6);
    assert_eq!(
        output.edges().get(21).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(21).unwrap().get().twin().unwrap().0, 20);
    assert_eq!(output.edges().get(21).unwrap().get().next().unwrap().0, 19);
    assert_eq!(output.edges().get(21).unwrap().get().prev().unwrap().0, 19);
    let e = output.edges()[21].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 18);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
}

//#[ignore]
#[test]
/// two segments and two points
fn two_segments_3() {
    let output = {
        let _v = vec![Coordinate { x: 4, y: 3 }, Coordinate { x: 1, y: 1 }];
        let _s = vec![
            Line::new(Coordinate { x: 1, y: 2 }, Coordinate { x: 3, y: 4 }),
            Line::new(Coordinate { x: 2, y: 2 }, Coordinate { x: 5, y: 4 }),
        ];
        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_vertices(_v.iter()).expect("two_segments_3");
        vb.with_segments(_s.iter()).expect("two_segments_3");
        vb.construct().expect("two_segments_3")
    };
    assert_eq!(output.cells().len(), 8);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 7);
    assert_eq!(output.edges().len(), 28);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 1.80196, v.y(), 2.29706));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 1.50000, v.y(), 1.50000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 3.39608, v.y(), 3.60392));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 4.00000, v.y(), 5.50000));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 4.00000, v.y(), -1.00000));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 7.00000, v.y(), 1.00000));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 5.67308, v.y(), -2.75962));
    assert_eq!(v.get_incident_edge().unwrap().0, 27);
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 26);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 10);
    let e = output.edges()[0].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 11);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 2);
    let e = output.edges()[1].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 26);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 1);
    let e = output.edges()[2].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 4);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 12);
    let e = output.edges()[3].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 13);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(4).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(4).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().twin().unwrap().0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next().unwrap().0, 9);
    assert_eq!(output.edges().get(4).unwrap().get().prev().unwrap().0, 3);
    let e = output.edges()[4].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 11);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(5).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().twin().unwrap().0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next().unwrap().0, 11);
    assert_eq!(output.edges().get(5).unwrap().get().prev().unwrap().0, 6);
    let e = output.edges()[5].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 7);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 9);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(6).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 4);
    assert_eq!(output.edges().get(6).unwrap().get().twin().unwrap().0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next().unwrap().0, 5);
    assert_eq!(output.edges().get(6).unwrap().get().prev().unwrap().0, 11);
    let e = output.edges()[6].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 10);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 23);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(7).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(7).unwrap().get().twin().unwrap().0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next().unwrap().0, 23);
    assert_eq!(output.edges().get(7).unwrap().get().prev().unwrap().0, 8);
    let e = output.edges()[7].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 9);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(8).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(output.edges().get(8).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(8).unwrap().get().twin().unwrap().0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next().unwrap().0, 7);
    assert_eq!(output.edges().get(8).unwrap().get().prev().unwrap().0, 14);
    let e = output.edges()[8].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 15);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 12);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(9).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(9).unwrap().get().twin().unwrap().0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next().unwrap().0, 12);
    assert_eq!(output.edges().get(9).unwrap().get().prev().unwrap().0, 4);
    let e = output.edges()[9].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 7);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(10).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(
        output.edges().get(10).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(10).unwrap().get().twin().unwrap().0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(10).unwrap().get().prev().unwrap().0, 22);
    let e = output.edges()[10].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 23);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(11).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(
        output.edges().get(11).unwrap().get().vertex0().unwrap().0,
        1
    );
    assert_eq!(output.edges().get(11).unwrap().get().twin().unwrap().0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next().unwrap().0, 6);
    assert_eq!(output.edges().get(11).unwrap().get().prev().unwrap().0, 5);
    let e = output.edges()[11].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(12).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(
        output.edges().get(12).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(12).unwrap().get().twin().unwrap().0, 13);
    assert_eq!(output.edges().get(12).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(12).unwrap().get().prev().unwrap().0, 9);
    let e = output.edges()[12].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 8);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 15);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(13).unwrap().get().cell().unwrap().0, 5);
    assert!(output.edges().get(13).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(13).unwrap().get().twin().unwrap().0, 12);
    assert_eq!(output.edges().get(13).unwrap().get().next().unwrap().0, 15);
    assert_eq!(output.edges().get(13).unwrap().get().prev().unwrap().0, 21);
    let e = output.edges()[13].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 20);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(14).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(14).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(14).unwrap().get().twin().unwrap().0, 15);
    assert_eq!(output.edges().get(14).unwrap().get().next().unwrap().0, 8);
    assert_eq!(output.edges().get(14).unwrap().get().prev().unwrap().0, 18);
    let e = output.edges()[14].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 19);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 21);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(15).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(15).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(15).unwrap().get().twin().unwrap().0, 14);
    assert_eq!(output.edges().get(15).unwrap().get().next().unwrap().0, 21);
    assert_eq!(output.edges().get(15).unwrap().get().prev().unwrap().0, 13);
    let e = output.edges()[15].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 12);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 8);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(16).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(16).unwrap().get().vertex0().unwrap().0,
        6
    );
    assert_eq!(output.edges().get(16).unwrap().get().twin().unwrap().0, 17);
    assert_eq!(output.edges().get(16).unwrap().get().next().unwrap().0, 18);
    assert_eq!(output.edges().get(16).unwrap().get().prev().unwrap().0, 23);
    let e = output.edges()[16].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 22);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 27);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(17).unwrap().get().cell().unwrap().0, 6);
    assert_eq!(
        output.edges().get(17).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(17).unwrap().get().twin().unwrap().0, 16);
    assert_eq!(output.edges().get(17).unwrap().get().next().unwrap().0, 27);
    assert_eq!(output.edges().get(17).unwrap().get().prev().unwrap().0, 24);
    let e = output.edges()[17].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 25);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 18);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(18).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(18).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(18).unwrap().get().twin().unwrap().0, 19);
    assert_eq!(output.edges().get(18).unwrap().get().next().unwrap().0, 14);
    assert_eq!(output.edges().get(18).unwrap().get().prev().unwrap().0, 16);
    let e = output.edges()[18].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 17);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 25);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(19).unwrap().get().cell().unwrap().0, 7);
    assert_eq!(
        output.edges().get(19).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(19).unwrap().get().twin().unwrap().0, 18);
    assert_eq!(output.edges().get(19).unwrap().get().next().unwrap().0, 25);
    assert_eq!(output.edges().get(19).unwrap().get().prev().unwrap().0, 20);
    let e = output.edges()[19].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 21);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 14);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(20).unwrap().get().cell().unwrap().0, 7);
    assert!(output.edges().get(20).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(20).unwrap().get().twin().unwrap().0, 21);
    assert_eq!(output.edges().get(20).unwrap().get().next().unwrap().0, 19);
    assert_eq!(output.edges().get(20).unwrap().get().prev().unwrap().0, 25);
    let e = output.edges()[20].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 24);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 13);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(21).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(21).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(21).unwrap().get().twin().unwrap().0, 20);
    assert_eq!(output.edges().get(21).unwrap().get().next().unwrap().0, 13);
    assert_eq!(output.edges().get(21).unwrap().get().prev().unwrap().0, 15);
    let e = output.edges()[21].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 14);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 19);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(22).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(
        output.edges().get(22).unwrap().get().vertex0().unwrap().0,
        6
    );
    assert_eq!(output.edges().get(22).unwrap().get().twin().unwrap().0, 23);
    assert_eq!(output.edges().get(22).unwrap().get().next().unwrap().0, 10);
    assert_eq!(output.edges().get(22).unwrap().get().prev().unwrap().0, 26);
    let e = output.edges()[22].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 27);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 16);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(23).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(23).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(23).unwrap().get().twin().unwrap().0, 22);
    assert_eq!(output.edges().get(23).unwrap().get().next().unwrap().0, 16);
    assert_eq!(output.edges().get(23).unwrap().get().prev().unwrap().0, 7);
    let e = output.edges()[23].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 10);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(24).unwrap().get().cell().unwrap().0, 6);
    assert!(output.edges().get(24).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(24).unwrap().get().twin().unwrap().0, 25);
    assert_eq!(output.edges().get(24).unwrap().get().next().unwrap().0, 17);
    assert_eq!(output.edges().get(24).unwrap().get().prev().unwrap().0, 27);
    let e = output.edges()[24].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 26);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 20);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(25).unwrap().get().cell().unwrap().0, 7);
    assert_eq!(
        output.edges().get(25).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(25).unwrap().get().twin().unwrap().0, 24);
    assert_eq!(output.edges().get(25).unwrap().get().next().unwrap().0, 20);
    assert_eq!(output.edges().get(25).unwrap().get().prev().unwrap().0, 19);
    let e = output.edges()[25].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 18);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 17);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(26).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(26).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(26).unwrap().get().twin().unwrap().0, 27);
    assert_eq!(output.edges().get(26).unwrap().get().next().unwrap().0, 22);
    assert_eq!(output.edges().get(26).unwrap().get().prev().unwrap().0, 0);
    let e = output.edges()[26].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 24);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(27).unwrap().get().cell().unwrap().0, 6);
    assert_eq!(
        output.edges().get(27).unwrap().get().vertex0().unwrap().0,
        6
    );
    assert_eq!(output.edges().get(27).unwrap().get().twin().unwrap().0, 26);
    assert_eq!(output.edges().get(27).unwrap().get().next().unwrap().0, 24);
    assert_eq!(output.edges().get(27).unwrap().get().prev().unwrap().0, 17);
    let e = output.edges()[27].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 16);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 22);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
}

//#[ignore]
#[test]
/// three segments and one point
fn two_segments_4() {
    let output = {
        let _v = vec![Coordinate { x: 4, y: 3 }];
        let _s = vec![
            Line::new(Coordinate { x: 1, y: 2 }, Coordinate { x: 3, y: 4 }),
            Line::new(Coordinate { x: 2, y: 2 }, Coordinate { x: 5, y: 4 }),
            Line::new(Coordinate { x: 5, y: 6 }, Coordinate { x: 3, y: 1 }),
        ];
        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_vertices(_v.iter()).expect("two_segments_4");
        vb.with_segments(_s.iter()).expect("two_segments_4");
        vb.construct().expect("two_segments_4")
    };
    assert_eq!(output.cells().len(), 10);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 42);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 1.80196, v.y(), 2.29706));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 1.50000, v.y(), 1.50000));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 2.28571, v.y(), 1.28571));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 1.50000, v.y(), 0.500000));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 2.39530, v.y(), 1.40706));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 3.39608, v.y(), 3.60392));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 3.97999, v.y(), 5.02001));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 4.75205, v.y(), 3.24795));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -3.78095, v.y(), 9.51238));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 7.50000, v.y(), 5.00000));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 8.13684, v.y(), -0.136838));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 11.1250, v.y(), -2.25000));
    assert_eq!(v.get_incident_edge().unwrap().0, 41);
}

//#[ignore]
#[test]
/// two segments and four points
fn two_segments_5() {
    let output = {
        let points: [[I; 2]; 4] = [[582, 779], [683, 1329], [741, 1155], [1239, 1102]];
        let segments: [[I; 4]; 2] = [[1394, 1470, 982, 1594], [1047, 1427, 1155, 1228]];

        let _v = to_points::<I>(&points);
        let _s = to_segments::<I>(&segments);

        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_vertices(_v.iter()).expect("two_segments_5");
        vb.with_segments(_s.iter()).expect("two_segments_5");
        vb.construct().expect("two_segments_5")
    };
    assert_eq!(output.cells().len(), 10);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 4);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 4);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    assert_eq!(cell.source_index(), 5);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    assert_eq!(cell.source_index(), 5);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    assert_eq!(cell.source_index(), 5);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    assert_eq!(cell.source_index(), 4);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 42);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 320.1025791, v.y(), 1111.3675264));
    assert_eq!(v.get_incident_edge().unwrap().0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 846.8907651, v.y(), 1445.2628726));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 949.2219808, v.y(), 1485.0923877));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 876.6922428, v.y(), 1334.5716695));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 884.4954950, v.y(), 1299.4984983));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 960.2050792, v.y(), 1122.2821535));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 1119.5619154, v.y(), 1466.3803360));
    assert_eq!(v.get_incident_edge().unwrap().0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 978.3728386, v.y(), 1019.2485591));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 959.4051795, v.y(), 841.0241395));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 1309.5034594, v.y(), 1311.8511237));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 1338.5456162, v.y(), 1285.7483377));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 1354.4950042, v.y(), 1269.9966694));
    assert_eq!(v.get_incident_edge().unwrap().0, 41);
}

//#[ignore]
#[test]
/// two problematic segments
fn two_segments_6() {
    let output = {
        let points: [[I; 2]; 0] = [];
        let segments: [[I; 4]; 2] = [[442, 215, 438, 355], [129, 559, 141, 60]];

        let _v = to_points::<I>(&points);
        let _s = to_segments::<I>(&segments);

        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_vertices(_v.iter()).expect("two_segments_6");
        vb.with_segments(_s.iter()).expect("two_segments_6");
        vb.construct().expect("two_segments_6")
    };
    assert_eq!(output.cells().len(), 6);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 286.0580800, v.y(), 350.6588023));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 289.7416658, v.y(), 210.6497619));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 329.0795415, v.y(), 64.5229549));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 354.4186453, v.y(), 564.4208893));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 17);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 17);
    let e = output.edges()[0].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 16);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().vertex0().unwrap().0, 3);
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 4);
    let e = output.edges()[1].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 17);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(2).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 12);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 1);
    let e = output.edges()[2].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 14);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 14);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 14);
    let e = output.edges()[3].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 15);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 12);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(4).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(4).unwrap().get().twin().unwrap().0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().prev().unwrap().0, 8);
    let e = output.edges()[4].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 9);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(5).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().twin().unwrap().0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next().unwrap().0, 6);
    assert_eq!(output.edges().get(5).unwrap().get().prev().unwrap().0, 16);
    let e = output.edges()[5].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 17);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(6).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(6).unwrap().get().twin().unwrap().0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next().unwrap().0, 16);
    assert_eq!(output.edges().get(6).unwrap().get().prev().unwrap().0, 5);
    let e = output.edges()[6].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 9);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(7).unwrap().get().cell().unwrap().0, 4);
    assert!(output.edges().get(7).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(7).unwrap().get().twin().unwrap().0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next().unwrap().0, 9);
    assert_eq!(output.edges().get(7).unwrap().get().prev().unwrap().0, 10);
    let e = output.edges()[7].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 11);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 16);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(8).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(8).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(8).unwrap().get().twin().unwrap().0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next().unwrap().0, 4);
    assert_eq!(output.edges().get(8).unwrap().get().prev().unwrap().0, 12);
    let e = output.edges()[8].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 13);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 10);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(9).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(9).unwrap().get().twin().unwrap().0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next().unwrap().0, 10);
    assert_eq!(output.edges().get(9).unwrap().get().prev().unwrap().0, 7);
    let e = output.edges()[9].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(10).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(10).unwrap().get().vertex0().unwrap().0,
        1
    );
    assert_eq!(output.edges().get(10).unwrap().get().twin().unwrap().0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next().unwrap().0, 7);
    assert_eq!(output.edges().get(10).unwrap().get().prev().unwrap().0, 9);
    let e = output.edges()[10].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 8);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 13);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(11).unwrap().get().cell().unwrap().0, 5);
    assert!(output.edges().get(11).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(11).unwrap().get().twin().unwrap().0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next().unwrap().0, 13);
    assert_eq!(output.edges().get(11).unwrap().get().prev().unwrap().0, 15);
    let e = output.edges()[11].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 14);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 7);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(12).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(
        output.edges().get(12).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(12).unwrap().get().twin().unwrap().0, 13);
    assert_eq!(output.edges().get(12).unwrap().get().next().unwrap().0, 8);
    assert_eq!(output.edges().get(12).unwrap().get().prev().unwrap().0, 2);
    let e = output.edges()[12].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 15);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(13).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(13).unwrap().get().vertex0().unwrap().0,
        1
    );
    assert_eq!(output.edges().get(13).unwrap().get().twin().unwrap().0, 12);
    assert_eq!(output.edges().get(13).unwrap().get().next().unwrap().0, 15);
    assert_eq!(output.edges().get(13).unwrap().get().prev().unwrap().0, 11);
    let e = output.edges()[13].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 10);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 8);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(14).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(14).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(14).unwrap().get().twin().unwrap().0, 15);
    assert_eq!(output.edges().get(14).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(14).unwrap().get().prev().unwrap().0, 3);
    let e = output.edges()[14].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 11);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(15).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(15).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(15).unwrap().get().twin().unwrap().0, 14);
    assert_eq!(output.edges().get(15).unwrap().get().next().unwrap().0, 11);
    assert_eq!(output.edges().get(15).unwrap().get().prev().unwrap().0, 13);
    let e = output.edges()[15].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 12);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(16).unwrap().get().cell().unwrap().0, 3);
    assert!(output.edges().get(16).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(16).unwrap().get().twin().unwrap().0, 17);
    assert_eq!(output.edges().get(16).unwrap().get().next().unwrap().0, 5);
    assert_eq!(output.edges().get(16).unwrap().get().prev().unwrap().0, 6);
    let e = output.edges()[16].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 7);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(17).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(
        output.edges().get(17).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(17).unwrap().get().twin().unwrap().0, 16);
    assert_eq!(output.edges().get(17).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(17).unwrap().get().prev().unwrap().0, 0);
    let e = output.edges()[17].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
}

//#[ignore]
#[test]
/// two problematic segments
fn two_segments_7() {
    let output = {
        let points: [[I; 2]; 0] = [];
        let segments: [[I; 4]; 2] = [[498, 224, 475, 335], [250, 507, 60, 77]];

        let _v = to_points::<I>(&points);
        let _s = to_segments::<I>(&segments);

        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_vertices(_v.iter()).expect("two_segments_7");
        vb.with_segments(_s.iter()).expect("two_segments_7");
        vb.construct().expect("two_segments_7")
    };
    assert_eq!(output.cells().len(), 6);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 326.3534001, v.y(), 304.1993532));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 313.7737689, v.y(), 185.8269972));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 383.2375415, v.y(), 448.1275979));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 346.0943520, v.y(), -49.4137834));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().vertex0().unwrap().0, 3);
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 16);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 16);
    let e = output.edges()[0].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 17);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 12);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 12);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 2);
    let e = output.edges()[1].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 16);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 4);
    let e = output.edges()[2].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 15);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 15);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 15);
    let e = output.edges()[3].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 14);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(4).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(4).unwrap().get().twin().unwrap().0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(4).unwrap().get().prev().unwrap().0, 8);
    let e = output.edges()[4].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 9);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(5).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(5).unwrap().get().twin().unwrap().0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next().unwrap().0, 6);
    assert_eq!(output.edges().get(5).unwrap().get().prev().unwrap().0, 14);
    let e = output.edges()[5].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 15);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(6).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(6).unwrap().get().twin().unwrap().0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next().unwrap().0, 14);
    assert_eq!(output.edges().get(6).unwrap().get().prev().unwrap().0, 5);
    let e = output.edges()[6].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 9);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(7).unwrap().get().cell().unwrap().0, 4);
    assert!(output.edges().get(7).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(7).unwrap().get().twin().unwrap().0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next().unwrap().0, 9);
    assert_eq!(output.edges().get(7).unwrap().get().prev().unwrap().0, 10);
    let e = output.edges()[7].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 11);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 14);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(8).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(8).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(8).unwrap().get().twin().unwrap().0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next().unwrap().0, 4);
    assert_eq!(output.edges().get(8).unwrap().get().prev().unwrap().0, 12);
    let e = output.edges()[8].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 13);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 10);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(9).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(9).unwrap().get().twin().unwrap().0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next().unwrap().0, 10);
    assert_eq!(output.edges().get(9).unwrap().get().prev().unwrap().0, 7);
    let e = output.edges()[9].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(10).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(10).unwrap().get().vertex0().unwrap().0,
        1
    );
    assert_eq!(output.edges().get(10).unwrap().get().twin().unwrap().0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next().unwrap().0, 7);
    assert_eq!(output.edges().get(10).unwrap().get().prev().unwrap().0, 9);
    let e = output.edges()[10].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 8);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 13);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(11).unwrap().get().cell().unwrap().0, 5);
    assert!(output.edges().get(11).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(11).unwrap().get().twin().unwrap().0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next().unwrap().0, 13);
    assert_eq!(output.edges().get(11).unwrap().get().prev().unwrap().0, 17);
    let e = output.edges()[11].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 16);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 7);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(12).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(
        output.edges().get(12).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(12).unwrap().get().twin().unwrap().0, 13);
    assert_eq!(output.edges().get(12).unwrap().get().next().unwrap().0, 8);
    assert_eq!(output.edges().get(12).unwrap().get().prev().unwrap().0, 1);
    let e = output.edges()[12].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 17);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(13).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(13).unwrap().get().vertex0().unwrap().0,
        1
    );
    assert_eq!(output.edges().get(13).unwrap().get().twin().unwrap().0, 12);
    assert_eq!(output.edges().get(13).unwrap().get().next().unwrap().0, 17);
    assert_eq!(output.edges().get(13).unwrap().get().prev().unwrap().0, 11);
    let e = output.edges()[13].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 10);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 8);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(14).unwrap().get().cell().unwrap().0, 3);
    assert!(output.edges().get(14).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(14).unwrap().get().twin().unwrap().0, 15);
    assert_eq!(output.edges().get(14).unwrap().get().next().unwrap().0, 5);
    assert_eq!(output.edges().get(14).unwrap().get().prev().unwrap().0, 6);
    let e = output.edges()[14].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 7);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(15).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(
        output.edges().get(15).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(15).unwrap().get().twin().unwrap().0, 14);
    assert_eq!(output.edges().get(15).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(15).unwrap().get().prev().unwrap().0, 3);
    let e = output.edges()[15].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(16).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(16).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(16).unwrap().get().twin().unwrap().0, 17);
    assert_eq!(output.edges().get(16).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(16).unwrap().get().prev().unwrap().0, 0);
    let e = output.edges()[16].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 11);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(17).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(17).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(17).unwrap().get().twin().unwrap().0, 16);
    assert_eq!(output.edges().get(17).unwrap().get().next().unwrap().0, 11);
    assert_eq!(output.edges().get(17).unwrap().get().prev().unwrap().0, 13);
    let e = output.edges()[17].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 12);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
}

//#[ignore]
#[test]
/// four segments in a loop
fn two_segments_8() {
    let output = {
        let c: I = 300;
        let points: [[I; 2]; 0] = [];
        let segments: [[I; 4]; 4] = [
            [c, c, c, 200 + c],
            [c, 200 + c, 200 + c, 200 + c],
            [200 + c, 200 + c, 200 + c, c],
            [200 + c, c, c, c],
        ];

        let _v = to_points::<I>(&points);
        let _s = to_segments::<I>(&segments);

        let mut vb = VoronoiBuilder::<I, O, DI, DF>::new();
        vb.with_vertices(_v.iter()).expect("two_segments_8");
        vb.with_segments(_s.iter()).expect("two_segments_8");
        vb.construct().expect("two_segments_8")
    };
    assert_eq!(output.cells().len(), 8);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 5);
    assert_eq!(output.edges().len(), 24);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 300.0000000, v.y(), 300.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 300.0000000, v.y(), 500.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 500.0000000, v.y(), 300.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 400.0000000, v.y(), 400.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 500.0000000, v.y(), 500.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
}
