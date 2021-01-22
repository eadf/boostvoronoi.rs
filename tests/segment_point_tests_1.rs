#![allow(unused_imports)]

use boostvoronoi::voronoi_builder::VoronoiBuilder;
use boostvoronoi::voronoi_diagram::VoronoiDiagram;
use boostvoronoi::voronoi_error::BVError;
use geo::{Coordinate, Line};

type I1 = i32;
type F1 = f32;
type I2 = i64;
type F2 = f64;

fn almost_equal(x1: F1, x2: F1, y1: F1, y2: F1) -> bool {
    let delta = 0.001;
    assert!(F1::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F1::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F1::abs(x1 - x2) < delta) && (F1::abs(y1 - y2) < delta)
}

//#[ignore]
#[test]
fn single_segment_point_1() {
    let output = {
        let _p = vec![Coordinate { x: 9, y: 10 }];
        let _s = vec![Line::new(
            Coordinate { x: 10, y: 11 },
            Coordinate { x: 12, y: 13 },
        )];
        let mut vb = VoronoiBuilder::<I1, F1, I2, F2>::new();
        vb.with_vertices(_p.iter()).expect("single_segment_point_1");
        vb.with_segments(_s.iter()).expect("single_segment_point_1");
        vb.construct().expect("single_segment_point_1")
    };
    assert_eq!(output.cells().len(), 4);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 6);
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
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 4);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 4);
    assert_eq!(output.edges().get(4).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(4).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(4).unwrap().get().twin().unwrap().0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(4).unwrap().get().prev().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().cell().unwrap().0, 3);
    assert!(output.edges().get(5).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(5).unwrap().get().twin().unwrap().0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next().unwrap().0, 5);
    assert_eq!(output.edges().get(5).unwrap().get().prev().unwrap().0, 5);
}

//#[ignore]
#[test]
fn single_segment_point_2() {
    let point_new = |x, y| Coordinate::<I1> { x, y };

    let output = {
        let _p = vec![point_new(12, 14)];
        let _s = vec![Line::new(point_new(10, 11), point_new(12, 13))];
        let mut vb = VoronoiBuilder::<I1, F1, I2, F2>::new();
        vb.with_vertices(_p.iter()).expect("single_segment_point_2");
        vb.with_segments(_s.iter()).expect("single_segment_point_2");
        vb.construct().expect("single_segment_point_2")
    };
    assert_eq!(output.cells().len(), 4);
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
    assert_eq!(output.vertices().len(), 2);
    assert_eq!(output.edges().len(), 10);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 11.5000000, v.y(), 13.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 3.5000000, v.y(), 17.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 9);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 9);
    let e = output.edges()[0].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 8);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 4);
    let e = output.edges()[1].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 5);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 9);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(2).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 4);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 1);
    let e = output.edges()[2].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), false);
    assert_eq!(e.is_secondary(), true);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 6);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 6);
    let e = output.edges()[3].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 7);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 4);
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
    assert_eq!(output.edges().get(4).unwrap().get().prev().unwrap().0, 2);
    let e = output.edges()[4].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 7);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(5).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(5).unwrap().get().twin().unwrap().0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next().unwrap().0, 7);
    assert_eq!(output.edges().get(5).unwrap().get().prev().unwrap().0, 8);
    let e = output.edges()[5].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 9);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), true);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), false);
    assert_eq!(e.is_linear(), false);
    assert_eq!(e.is_curved(), true);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(6).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(6).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(6).unwrap().get().twin().unwrap().0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().prev().unwrap().0, 3);
    let e = output.edges()[6].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 2);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 8);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(7).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(7).unwrap().get().twin().unwrap().0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next().unwrap().0, 8);
    assert_eq!(output.edges().get(7).unwrap().get().prev().unwrap().0, 5);
    let e = output.edges()[7].get();
    assert!(output.edge_get_vertex1(Some(e.get_id())).is_none());
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 4);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 3);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(8).unwrap().get().cell().unwrap().0, 3);
    assert!(output.edges().get(8).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(8).unwrap().get().twin().unwrap().0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next().unwrap().0, 5);
    assert_eq!(output.edges().get(8).unwrap().get().prev().unwrap().0, 7);
    let e = output.edges()[8].get();
    assert_eq!(output.edge_get_vertex1(Some(e.get_id())).unwrap().0, 1);
    assert_eq!(output.edge_rot_next(Some(e.get_id())).unwrap().0, 6);
    assert_eq!(output.edge_rot_prev(Some(e.get_id())).unwrap().0, 0);
    assert_eq!(output.edge_is_finite(Some(e.get_id())).unwrap(), false);
    assert_eq!(output.edge_is_infinite(Some(e.get_id())).unwrap(), true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(9).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(9).unwrap().get().twin().unwrap().0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(9).unwrap().get().prev().unwrap().0, 0);
    let e = output.edges()[9].get();
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
fn single_segment_point_3() {
    let point_new = |x, y| Coordinate::<I1> { x, y };
    let output = {
        let _p = vec![point_new(12, 14), point_new(4, 5)];
        let _s = vec![Line::new(point_new(10, 11), point_new(12, 13))];
        let mut vb = VoronoiBuilder::<I1, F1, I2, F2>::new();
        vb.with_vertices(_p.iter()).expect("single_segment_point_3");
        vb.with_segments(_s.iter()).expect("single_segment_point_3");
        vb.construct().expect("single_segment_point_3")
    };
    assert_eq!(output.cells().len(), 5);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 3);
    assert_eq!(output.edges().len(), 14);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 11.5, v.y(), 13.5));
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 3.5, v.y(), 17.5));
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -14.5, v.y(), 29.5));
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 13);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 13);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 11);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 1);
    assert!(output.edges().get(2).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 11);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 4);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 6);
    assert_eq!(output.edges().get(4).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(4).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(4).unwrap().get().twin().unwrap().0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next().unwrap().0, 6);
    assert_eq!(output.edges().get(4).unwrap().get().prev().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().twin().unwrap().0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next().unwrap().0, 8);
    assert_eq!(output.edges().get(5).unwrap().get().prev().unwrap().0, 8);
    assert_eq!(output.edges().get(6).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(6).unwrap().get().twin().unwrap().0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().prev().unwrap().0, 4);
    assert_eq!(output.edges().get(7).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(7).unwrap().get().twin().unwrap().0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next().unwrap().0, 9);
    assert_eq!(output.edges().get(7).unwrap().get().prev().unwrap().0, 10);
    assert_eq!(output.edges().get(8).unwrap().get().cell().unwrap().0, 3);
    assert!(output.edges().get(8).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(8).unwrap().get().twin().unwrap().0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next().unwrap().0, 5);
    assert_eq!(output.edges().get(8).unwrap().get().prev().unwrap().0, 5);
    assert_eq!(output.edges().get(9).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(9).unwrap().get().twin().unwrap().0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next().unwrap().0, 12);
    assert_eq!(output.edges().get(9).unwrap().get().prev().unwrap().0, 7);
    assert_eq!(output.edges().get(10).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(10).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(10).unwrap().get().twin().unwrap().0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next().unwrap().0, 7);
    assert_eq!(output.edges().get(10).unwrap().get().prev().unwrap().0, 12);
    assert_eq!(output.edges().get(11).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(
        output.edges().get(11).unwrap().get().vertex0().unwrap().0,
        1
    );
    assert_eq!(output.edges().get(11).unwrap().get().twin().unwrap().0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(11).unwrap().get().prev().unwrap().0, 2);
    assert_eq!(output.edges().get(12).unwrap().get().cell().unwrap().0, 4);
    assert!(output.edges().get(12).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(12).unwrap().get().twin().unwrap().0, 13);
    assert_eq!(output.edges().get(12).unwrap().get().next().unwrap().0, 10);
    assert_eq!(output.edges().get(12).unwrap().get().prev().unwrap().0, 9);
    assert_eq!(output.edges().get(13).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(
        output.edges().get(13).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(13).unwrap().get().twin().unwrap().0, 12);
    assert_eq!(output.edges().get(13).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(13).unwrap().get().prev().unwrap().0, 0);
}

//#[ignore]
#[test]
fn single_segment_point_4() {
    let point_new = |x, y| Coordinate::<I1> { x, y };
    let output = {
        let _p = vec![point_new(10, 14), point_new(8, 7), point_new(11, 11)];
        let _s = vec![Line::new(point_new(10, 11), point_new(12, 13))];
        let mut vb = VoronoiBuilder::<I1, F1, I2, F2>::new();
        vb.with_vertices(_p.iter()).expect("single_segment_point_3");
        vb.with_segments(_s.iter()).expect("single_segment_point_3");
        vb.construct().expect("single_segment_point_3")
    };
    assert_eq!(output.cells().len(), 6);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 3);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 3);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(output.vertices().len(), 6);
    assert_eq!(output.edges().len(), 22);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 2.0, v.y(), 12.5));
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 8.5, v.y(), 12.5));
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 10.5, v.y(), 10.5));
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 11.1667, v.y(), 13.8333));
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 10.5, v.y(), 8.25));
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 14.5, v.y(), 10.5));
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().vertex0().unwrap().0, 4);
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 18);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 12);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 6);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 18);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 0);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 7);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 17);
    assert_eq!(output.edges().get(4).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(4).unwrap().get().twin().unwrap().0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next().unwrap().0, 6);
    assert_eq!(output.edges().get(4).unwrap().get().prev().unwrap().0, 12);
    assert_eq!(output.edges().get(5).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(5).unwrap().get().twin().unwrap().0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next().unwrap().0, 10);
    assert_eq!(output.edges().get(5).unwrap().get().prev().unwrap().0, 8);
    assert_eq!(output.edges().get(6).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().twin().unwrap().0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().prev().unwrap().0, 4);
    assert_eq!(output.edges().get(7).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(7).unwrap().get().twin().unwrap().0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next().unwrap().0, 9);
    assert_eq!(output.edges().get(7).unwrap().get().prev().unwrap().0, 3);
    assert_eq!(output.edges().get(8).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(8).unwrap().get().vertex0().unwrap().0, 3);
    assert_eq!(output.edges().get(8).unwrap().get().twin().unwrap().0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next().unwrap().0, 5);
    assert_eq!(output.edges().get(8).unwrap().get().prev().unwrap().0, 14);
    assert_eq!(output.edges().get(9).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(9).unwrap().get().twin().unwrap().0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next().unwrap().0, 17);
    assert_eq!(output.edges().get(9).unwrap().get().prev().unwrap().0, 7);
    assert_eq!(output.edges().get(10).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(
        output.edges().get(10).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(10).unwrap().get().twin().unwrap().0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next().unwrap().0, 14);
    assert_eq!(output.edges().get(10).unwrap().get().prev().unwrap().0, 5);
    assert_eq!(output.edges().get(11).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(11).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(11).unwrap().get().twin().unwrap().0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next().unwrap().0, 13);
    assert_eq!(output.edges().get(11).unwrap().get().prev().unwrap().0, 20);
    assert_eq!(output.edges().get(12).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(
        output.edges().get(12).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(12).unwrap().get().twin().unwrap().0, 13);
    assert_eq!(output.edges().get(12).unwrap().get().next().unwrap().0, 4);
    assert_eq!(output.edges().get(12).unwrap().get().prev().unwrap().0, 1);
    assert_eq!(output.edges().get(13).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(13).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(13).unwrap().get().twin().unwrap().0, 12);
    assert_eq!(output.edges().get(13).unwrap().get().next().unwrap().0, 19);
    assert_eq!(output.edges().get(13).unwrap().get().prev().unwrap().0, 11);
    assert_eq!(output.edges().get(14).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(
        output.edges().get(14).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(14).unwrap().get().twin().unwrap().0, 15);
    assert_eq!(output.edges().get(14).unwrap().get().next().unwrap().0, 8);
    assert_eq!(output.edges().get(14).unwrap().get().prev().unwrap().0, 10);
    assert_eq!(output.edges().get(15).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(15).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(15).unwrap().get().twin().unwrap().0, 14);
    assert_eq!(output.edges().get(15).unwrap().get().next().unwrap().0, 21);
    assert_eq!(output.edges().get(15).unwrap().get().prev().unwrap().0, 16);
    assert_eq!(output.edges().get(16).unwrap().get().cell().unwrap().0, 5);
    assert!(output.edges().get(16).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(16).unwrap().get().twin().unwrap().0, 17);
    assert_eq!(output.edges().get(16).unwrap().get().next().unwrap().0, 15);
    assert_eq!(output.edges().get(16).unwrap().get().prev().unwrap().0, 21);
    assert_eq!(output.edges().get(17).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(
        output.edges().get(17).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(17).unwrap().get().twin().unwrap().0, 16);
    assert_eq!(output.edges().get(17).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(17).unwrap().get().prev().unwrap().0, 9);
    assert_eq!(output.edges().get(18).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(18).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(18).unwrap().get().twin().unwrap().0, 19);
    assert_eq!(output.edges().get(18).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(18).unwrap().get().prev().unwrap().0, 2);
    assert_eq!(output.edges().get(19).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(19).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(19).unwrap().get().twin().unwrap().0, 18);
    assert_eq!(output.edges().get(19).unwrap().get().next().unwrap().0, 20);
    assert_eq!(output.edges().get(19).unwrap().get().prev().unwrap().0, 13);
    assert_eq!(output.edges().get(20).unwrap().get().cell().unwrap().0, 4);
    assert!(output.edges().get(20).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(20).unwrap().get().twin().unwrap().0, 21);
    assert_eq!(output.edges().get(20).unwrap().get().next().unwrap().0, 11);
    assert_eq!(output.edges().get(20).unwrap().get().prev().unwrap().0, 19);
    assert_eq!(output.edges().get(21).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(21).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(21).unwrap().get().twin().unwrap().0, 20);
    assert_eq!(output.edges().get(21).unwrap().get().next().unwrap().0, 16);
    assert_eq!(output.edges().get(21).unwrap().get().prev().unwrap().0, 15);
}

//#[ignore]
#[test]
fn single_segment_point_5() {
    let point_new = |x, y| Coordinate::<I1> { x, y };
    let output = {
        let _p = vec![point_new(10, 14), point_new(8, 7), point_new(11, 11)];
        let _s = vec![Line::new(point_new(12, 13), point_new(10, 11))];
        let mut vb = VoronoiBuilder::<I1, F1, I2, F2>::new();
        vb.with_vertices(_p.iter()).expect("single_segment_point_3");
        vb.with_segments(_s.iter()).expect("single_segment_point_3");
        vb.construct().expect("single_segment_point_3")
    };
    assert_eq!(output.cells().len(), 6);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    assert_eq!(cell.source_index(), 3);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    assert_eq!(cell.source_index(), 3);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(output.vertices().len(), 6);
    assert_eq!(output.edges().len(), 22);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 2.0, v.y(), 12.5));
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 8.5, v.y(), 12.5));
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 10.5, v.y(), 10.5));
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 11.1667, v.y(), 13.8333));
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 10.5, v.y(), 8.25));
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 14.5, v.y(), 10.5));
    assert_eq!(output.edges().get(0).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().vertex0().unwrap().0, 4);
    assert_eq!(output.edges().get(0).unwrap().get().twin().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next().unwrap().0, 2);
    assert_eq!(output.edges().get(0).unwrap().get().prev().unwrap().0, 18);
    assert_eq!(output.edges().get(1).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().twin().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next().unwrap().0, 12);
    assert_eq!(output.edges().get(1).unwrap().get().prev().unwrap().0, 6);
    assert_eq!(output.edges().get(2).unwrap().get().cell().unwrap().0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().twin().unwrap().0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next().unwrap().0, 18);
    assert_eq!(output.edges().get(2).unwrap().get().prev().unwrap().0, 0);
    assert_eq!(output.edges().get(3).unwrap().get().cell().unwrap().0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin().unwrap().0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next().unwrap().0, 7);
    assert_eq!(output.edges().get(3).unwrap().get().prev().unwrap().0, 17);
    assert_eq!(output.edges().get(4).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(4).unwrap().get().twin().unwrap().0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next().unwrap().0, 6);
    assert_eq!(output.edges().get(4).unwrap().get().prev().unwrap().0, 12);
    assert_eq!(output.edges().get(5).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(5).unwrap().get().twin().unwrap().0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next().unwrap().0, 10);
    assert_eq!(output.edges().get(5).unwrap().get().prev().unwrap().0, 8);
    assert_eq!(output.edges().get(6).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().twin().unwrap().0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next().unwrap().0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().prev().unwrap().0, 4);
    assert_eq!(output.edges().get(7).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(7).unwrap().get().twin().unwrap().0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next().unwrap().0, 9);
    assert_eq!(output.edges().get(7).unwrap().get().prev().unwrap().0, 3);
    assert_eq!(output.edges().get(8).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(output.edges().get(8).unwrap().get().vertex0().unwrap().0, 3);
    assert_eq!(output.edges().get(8).unwrap().get().twin().unwrap().0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next().unwrap().0, 5);
    assert_eq!(output.edges().get(8).unwrap().get().prev().unwrap().0, 14);
    assert_eq!(output.edges().get(9).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(9).unwrap().get().twin().unwrap().0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next().unwrap().0, 17);
    assert_eq!(output.edges().get(9).unwrap().get().prev().unwrap().0, 7);
    assert_eq!(output.edges().get(10).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(
        output.edges().get(10).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(10).unwrap().get().twin().unwrap().0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next().unwrap().0, 14);
    assert_eq!(output.edges().get(10).unwrap().get().prev().unwrap().0, 5);
    assert_eq!(output.edges().get(11).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(11).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(11).unwrap().get().twin().unwrap().0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next().unwrap().0, 13);
    assert_eq!(output.edges().get(11).unwrap().get().prev().unwrap().0, 20);
    assert_eq!(output.edges().get(12).unwrap().get().cell().unwrap().0, 1);
    assert_eq!(
        output.edges().get(12).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(12).unwrap().get().twin().unwrap().0, 13);
    assert_eq!(output.edges().get(12).unwrap().get().next().unwrap().0, 4);
    assert_eq!(output.edges().get(12).unwrap().get().prev().unwrap().0, 1);
    assert_eq!(output.edges().get(13).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(13).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(13).unwrap().get().twin().unwrap().0, 12);
    assert_eq!(output.edges().get(13).unwrap().get().next().unwrap().0, 19);
    assert_eq!(output.edges().get(13).unwrap().get().prev().unwrap().0, 11);
    assert_eq!(output.edges().get(14).unwrap().get().cell().unwrap().0, 3);
    assert_eq!(
        output.edges().get(14).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(14).unwrap().get().twin().unwrap().0, 15);
    assert_eq!(output.edges().get(14).unwrap().get().next().unwrap().0, 8);
    assert_eq!(output.edges().get(14).unwrap().get().prev().unwrap().0, 10);
    assert_eq!(output.edges().get(15).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(15).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(15).unwrap().get().twin().unwrap().0, 14);
    assert_eq!(output.edges().get(15).unwrap().get().next().unwrap().0, 21);
    assert_eq!(output.edges().get(15).unwrap().get().prev().unwrap().0, 16);
    assert_eq!(output.edges().get(16).unwrap().get().cell().unwrap().0, 5);
    assert!(output.edges().get(16).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(16).unwrap().get().twin().unwrap().0, 17);
    assert_eq!(output.edges().get(16).unwrap().get().next().unwrap().0, 15);
    assert_eq!(output.edges().get(16).unwrap().get().prev().unwrap().0, 21);
    assert_eq!(output.edges().get(17).unwrap().get().cell().unwrap().0, 2);
    assert_eq!(
        output.edges().get(17).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(17).unwrap().get().twin().unwrap().0, 16);
    assert_eq!(output.edges().get(17).unwrap().get().next().unwrap().0, 3);
    assert_eq!(output.edges().get(17).unwrap().get().prev().unwrap().0, 9);
    assert_eq!(output.edges().get(18).unwrap().get().cell().unwrap().0, 0);
    assert!(output.edges().get(18).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(18).unwrap().get().twin().unwrap().0, 19);
    assert_eq!(output.edges().get(18).unwrap().get().next().unwrap().0, 0);
    assert_eq!(output.edges().get(18).unwrap().get().prev().unwrap().0, 2);
    assert_eq!(output.edges().get(19).unwrap().get().cell().unwrap().0, 4);
    assert_eq!(
        output.edges().get(19).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(19).unwrap().get().twin().unwrap().0, 18);
    assert_eq!(output.edges().get(19).unwrap().get().next().unwrap().0, 20);
    assert_eq!(output.edges().get(19).unwrap().get().prev().unwrap().0, 13);
    assert_eq!(output.edges().get(20).unwrap().get().cell().unwrap().0, 4);
    assert!(output.edges().get(20).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(20).unwrap().get().twin().unwrap().0, 21);
    assert_eq!(output.edges().get(20).unwrap().get().next().unwrap().0, 11);
    assert_eq!(output.edges().get(20).unwrap().get().prev().unwrap().0, 19);
    assert_eq!(output.edges().get(21).unwrap().get().cell().unwrap().0, 5);
    assert_eq!(
        output.edges().get(21).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(21).unwrap().get().twin().unwrap().0, 20);
    assert_eq!(output.edges().get(21).unwrap().get().next().unwrap().0, 16);
    assert_eq!(output.edges().get(21).unwrap().get().prev().unwrap().0, 15);
}
