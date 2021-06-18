use boostvoronoi::builder::Builder;
use boostvoronoi::geometry::Point;
use boostvoronoi::BvError;

type I1 = i32;
type F1 = f32;

fn almost_equal(x1: F1, x2: F1, y1: F1, y2: F1) -> bool {
    let delta = 0.001;
    assert!(F1::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F1::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F1::abs(x1 - x2) < delta) && (F1::abs(y1 - y2) < delta)
}

#[test]
fn single_point_1() {
    let output = {
        let _v = vec![Point { x: 10, y: 11 }];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("single_point_1");
        vb.construct().expect("single_point_1")
    };

    assert_eq!(output.cells().len(), 1);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 0);
}

#[test]
fn two_points_1() -> Result<(), BvError> {
    let output = {
        let _v = vec![Point { x: 10, y: 11 }, Point { x: 1, y: 3 }];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("two_points_1");
        vb.construct().expect("two_points_1")
    };
    assert_eq!(output.cells().len(), 2);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 2);
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 0);
    let e = output.edges()[0].get();
    assert!(output.edge_get_vertex1(e.id())?.is_none());
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);

    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 1);
    let e = output.edges()[1].get();
    assert!(output.edge_get_vertex1(e.id())?.is_none());
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    Ok(())
}

#[test]
/// same as test two_points_2 but reversed order
fn two_points_2() -> Result<(), BvError> {
    let output = {
        let _v = vec![Point { x: 1, y: 3 }, Point { x: 10, y: 11 }];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("two_points_1");
        vb.construct().expect("two_points_1")
    };
    assert_eq!(output.cells().len(), 2);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 2);
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 1);
    Ok(())
}

#[test]
fn two_points_3() -> Result<(), BvError> {
    let output = {
        let _v = vec![Point { x: 45, y: 1 }, Point { x: 8, y: 23 }];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("two_points_2");
        vb.construct().expect("two_points_2")
    };
    assert_eq!(output.cells().len(), 2);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 2);
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 1);
    Ok(())
}

#[test]
fn three_points_1() -> Result<(), BvError> {
    let output = {
        let _v = vec![
            Point { x: 10, y: 11 },
            Point { x: 1, y: 3 },
            Point { x: 5, y: 4 },
        ];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("three_points_1");
        vb.construct().expect("three_points_1")
    };
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 1);
    assert_eq!(output.edges().len(), 6);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 0.804348, v.y(), 12.2826));
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 5);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 5);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 2);
    assert_eq!(output.edges().get(2).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(2).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(2).unwrap().get().twin()?.0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().prev()?.0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(3).unwrap().get().twin()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next()?.0, 4);
    assert_eq!(output.edges().get(3).unwrap().get().prev()?.0, 4);
    assert_eq!(output.edges().get(4).unwrap().get().cell()?.0, 2);
    assert!(output.edges().get(4).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(4).unwrap().get().twin()?.0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next()?.0, 3);
    assert_eq!(output.edges().get(4).unwrap().get().prev()?.0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().cell()?.0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().twin()?.0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().prev()?.0, 0);
    Ok(())
}

#[test]
/// 3 co-linear sites (on x)
fn three_points_2() -> Result<(), BvError> {
    let output = {
        let _v = vec![
            Point { x: 1, y: 8 },
            Point { x: 1, y: 3 },
            Point { x: 1, y: 2 },
        ];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("three_points_2");
        vb.construct().expect("three_points_2")
    };
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 4);
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 2);
    assert_eq!(output.edges().get(2).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(2).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(2).unwrap().get().twin()?.0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().prev()?.0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().cell()?.0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next()?.0, 3);
    assert_eq!(output.edges().get(3).unwrap().get().prev()?.0, 3);
    Ok(())
}

/// 3 co-linear sites
#[test]
fn three_points_3() -> Result<(), BvError> {
    let output = {
        let _v = vec![
            Point { x: 19, y: 8 },
            Point { x: 19, y: 1 },
            Point { x: 19, y: 20 },
        ];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("three_points_3");
        vb.construct().expect("three_points_3")
    };
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 4);
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 2);
    assert_eq!(output.edges().get(2).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(2).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(2).unwrap().get().twin()?.0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().prev()?.0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().cell()?.0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next()?.0, 3);
    assert_eq!(output.edges().get(3).unwrap().get().prev()?.0, 3);
    Ok(())
}

/// 2+1 co-linear sites
#[test]
fn three_points_4() -> Result<(), BvError> {
    let output = {
        let _v = vec![
            Point { x: 10, y: 10 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 6 },
        ];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("three_points_4");
        vb.construct().expect("three_points_4")
    };
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 1);
    assert_eq!(output.edges().len(), 6);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 7.5, v.y(), 3.5));
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 4);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 4);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 2);
    assert_eq!(output.edges().get(2).unwrap().get().cell()?.0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().twin()?.0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(2).unwrap().get().prev()?.0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().cell()?.0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next()?.0, 5);
    assert_eq!(output.edges().get(3).unwrap().get().prev()?.0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(4).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(4).unwrap().get().twin()?.0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(4).unwrap().get().prev()?.0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().twin()?.0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next()?.0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().prev()?.0, 3);
    Ok(())
}

#[test]
fn four_points_1() -> Result<(), BvError> {
    let output = {
        let _v = vec![
            Point { x: 10, y: 8 },
            Point { x: 1, y: 3 },
            Point { x: 4, y: 2 },
            Point { x: 5, y: 6 },
        ];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("four_points_1");
        vb.construct().expect("four_points_1")
    };
    assert_eq!(output.cells().len(), 4);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    assert_eq!(cell.source_index(), 3);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(output.vertices().len(), 2);
    assert_eq!(output.edges().len(), 10);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 3.11538, v.y(), 4.34615));
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 9.16667, v.y(), 2.83333));
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 2);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 8);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 4);
    assert_eq!(output.edges().get(2).unwrap().get().cell()?.0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().twin()?.0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().prev()?.0, 0);
    assert_eq!(output.edges().get(3).unwrap().get().cell()?.0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next()?.0, 5);
    assert_eq!(output.edges().get(3).unwrap().get().prev()?.0, 6);
    assert_eq!(output.edges().get(4).unwrap().get().cell()?.0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().twin()?.0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().prev()?.0, 8);
    assert_eq!(output.edges().get(5).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().twin()?.0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next()?.0, 6);
    assert_eq!(output.edges().get(5).unwrap().get().prev()?.0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().twin()?.0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next()?.0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().prev()?.0, 5);
    assert_eq!(output.edges().get(7).unwrap().get().cell()?.0, 3);
    assert!(output.edges().get(7).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(7).unwrap().get().twin()?.0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next()?.0, 9);
    assert_eq!(output.edges().get(7).unwrap().get().prev()?.0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(8).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(8).unwrap().get().twin()?.0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next()?.0, 4);
    assert_eq!(output.edges().get(8).unwrap().get().prev()?.0, 1);
    assert_eq!(output.edges().get(9).unwrap().get().cell()?.0, 3);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(9).unwrap().get().twin()?.0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next()?.0, 7);
    assert_eq!(output.edges().get(9).unwrap().get().prev()?.0, 7);
    Ok(())
}

//#[ignore]
#[test]
fn four_points_2() -> Result<(), BvError> {
    let output = {
        let _v = vec![
            Point { x: 10, y: 18 },
            Point { x: 12, y: 3 },
            Point { x: 4, y: 21 },
            Point { x: 8, y: 62 },
        ];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("four_points_2");
        vb.construct().expect("four_points_2")
    };
    assert_eq!(output.cells().len(), 4);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 3);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(output.vertices().len(), 3);
    assert_eq!(output.edges().len(), 12);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 1.89286, v.y(), 9.28571));
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 17.4419, v.y(), 40.3837));
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 347.724, v.y(), 55.3966));
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 4);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 9);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 11);
    assert_eq!(output.edges().get(2).unwrap().get().cell()?.0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().twin()?.0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().prev()?.0, 4);
    assert_eq!(output.edges().get(3).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(3).unwrap().get().twin()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next()?.0, 7);
    assert_eq!(output.edges().get(3).unwrap().get().prev()?.0, 8);
    assert_eq!(output.edges().get(4).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(4).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(4).unwrap().get().twin()?.0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next()?.0, 2);
    assert_eq!(output.edges().get(4).unwrap().get().prev()?.0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().cell()?.0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().twin()?.0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next()?.0, 10);
    assert_eq!(output.edges().get(5).unwrap().get().prev()?.0, 6);
    assert_eq!(output.edges().get(6).unwrap().get().cell()?.0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(6).unwrap().get().twin()?.0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next()?.0, 5);
    assert_eq!(output.edges().get(6).unwrap().get().prev()?.0, 10);
    assert_eq!(output.edges().get(7).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(7).unwrap().get().twin()?.0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next()?.0, 8);
    assert_eq!(output.edges().get(7).unwrap().get().prev()?.0, 3);
    assert_eq!(output.edges().get(8).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(8).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(8).unwrap().get().twin()?.0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next()?.0, 3);
    assert_eq!(output.edges().get(8).unwrap().get().prev()?.0, 7);
    assert_eq!(output.edges().get(9).unwrap().get().cell()?.0, 1);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(9).unwrap().get().twin()?.0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next()?.0, 11);
    assert_eq!(output.edges().get(9).unwrap().get().prev()?.0, 1);
    assert_eq!(output.edges().get(10).unwrap().get().cell()?.0, 3);
    assert!(output.edges().get(10).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(10).unwrap().get().twin()?.0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next()?.0, 6);
    assert_eq!(output.edges().get(10).unwrap().get().prev()?.0, 5);
    assert_eq!(output.edges().get(11).unwrap().get().cell()?.0, 1);
    assert_eq!(
        output.edges().get(11).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(11).unwrap().get().twin()?.0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(11).unwrap().get().prev()?.0, 9);
    Ok(())
}

//#[ignore]
#[test]
fn four_points_3() -> Result<(), BvError> {
    let output = {
        let _v = vec![
            Point { x: 10, y: 18 },
            Point { x: 12, y: 3 },
            Point { x: 4, y: 21 },
            Point { x: 8, y: 3 },
        ];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("four_points_3");
        vb.construct().expect("four_points_3")
    };
    assert_eq!(output.cells().len(), 4);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 3);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(output.vertices().len(), 2);
    assert_eq!(output.edges().len(), 10);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 2.90625, v.y(), 11.3125));
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 10.0, v.y(), 10.3667));
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 2);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 4);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 6);
    assert_eq!(output.edges().get(2).unwrap().get().cell()?.0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().twin()?.0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().prev()?.0, 0);
    assert_eq!(output.edges().get(3).unwrap().get().cell()?.0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next()?.0, 7);
    assert_eq!(output.edges().get(3).unwrap().get().prev()?.0, 9);
    assert_eq!(output.edges().get(4).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(4).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(4).unwrap().get().twin()?.0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next()?.0, 6);
    assert_eq!(output.edges().get(4).unwrap().get().prev()?.0, 1);
    assert_eq!(output.edges().get(5).unwrap().get().cell()?.0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(5).unwrap().get().twin()?.0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next()?.0, 8);
    assert_eq!(output.edges().get(5).unwrap().get().prev()?.0, 8);
    assert_eq!(output.edges().get(6).unwrap().get().cell()?.0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().twin()?.0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().prev()?.0, 4);
    assert_eq!(output.edges().get(7).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(7).unwrap().get().twin()?.0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next()?.0, 9);
    assert_eq!(output.edges().get(7).unwrap().get().prev()?.0, 3);
    assert_eq!(output.edges().get(8).unwrap().get().cell()?.0, 3);
    assert!(output.edges().get(8).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(8).unwrap().get().twin()?.0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next()?.0, 5);
    assert_eq!(output.edges().get(8).unwrap().get().prev()?.0, 5);
    assert_eq!(output.edges().get(9).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(9).unwrap().get().twin()?.0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next()?.0, 3);
    assert_eq!(output.edges().get(9).unwrap().get().prev()?.0, 7);
    Ok(())
}

//#[ignore]
#[test]
fn five_points_1() -> Result<(), BvError> {
    let output = {
        let _v = vec![
            Point { x: 8, y: 9 },
            Point { x: 2, y: 14 },
            Point { x: 1, y: 15 },
            Point { x: 4, y: 16 },
            Point { x: 9, y: 8 },
        ];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("five_points_1");
        vb.construct().expect("five_points_1")
    };
    assert_eq!(output.cells().len(), 5);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 2);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 1);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    assert_eq!(cell.source_index(), 3);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    assert_eq!(cell.source_index(), 0);
    let cell = output.cells()[4].get();
    assert_eq!(cell.id().0, 4);
    assert_eq!(cell.source_index(), 4);
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 16);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 2.5, v.y(), 15.5));
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 5.68182, v.y(), 12.3182));
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -27.5, v.y(), -27.5));
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 21.1667, v.y(), 21.1667));
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(0).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 2);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 2);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert_eq!(output.edges().get(1).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 12);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 4);
    assert_eq!(output.edges().get(2).unwrap().get().cell()?.0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().twin()?.0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().prev()?.0, 0);
    assert_eq!(output.edges().get(3).unwrap().get().cell()?.0, 2);
    assert!(output.edges().get(3).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(3).unwrap().get().twin()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next()?.0, 5);
    assert_eq!(output.edges().get(3).unwrap().get().prev()?.0, 15);
    assert_eq!(output.edges().get(4).unwrap().get().cell()?.0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().twin()?.0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(4).unwrap().get().prev()?.0, 6);
    assert_eq!(output.edges().get(5).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(5).unwrap().get().twin()?.0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next()?.0, 11);
    assert_eq!(output.edges().get(5).unwrap().get().prev()?.0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().cell()?.0, 1);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(6).unwrap().get().twin()?.0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next()?.0, 4);
    assert_eq!(output.edges().get(6).unwrap().get().prev()?.0, 12);
    assert_eq!(output.edges().get(7).unwrap().get().cell()?.0, 3);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(7).unwrap().get().twin()?.0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next()?.0, 8);
    assert_eq!(output.edges().get(7).unwrap().get().prev()?.0, 10);
    assert_eq!(output.edges().get(8).unwrap().get().cell()?.0, 3);
    assert_eq!(output.edges().get(8).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(8).unwrap().get().twin()?.0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next()?.0, 10);
    assert_eq!(output.edges().get(8).unwrap().get().prev()?.0, 7);
    assert_eq!(output.edges().get(9).unwrap().get().cell()?.0, 4);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 3);
    assert_eq!(output.edges().get(9).unwrap().get().twin()?.0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next()?.0, 13);
    assert_eq!(output.edges().get(9).unwrap().get().prev()?.0, 14);
    assert_eq!(output.edges().get(10).unwrap().get().cell()?.0, 3);
    assert_eq!(
        output.edges().get(10).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(10).unwrap().get().twin()?.0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next()?.0, 7);
    assert_eq!(output.edges().get(10).unwrap().get().prev()?.0, 8);
    assert_eq!(output.edges().get(11).unwrap().get().cell()?.0, 2);
    assert_eq!(
        output.edges().get(11).unwrap().get().vertex0().unwrap().0,
        1
    );
    assert_eq!(output.edges().get(11).unwrap().get().twin()?.0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next()?.0, 15);
    assert_eq!(output.edges().get(11).unwrap().get().prev()?.0, 5);
    assert_eq!(output.edges().get(12).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(12).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(12).unwrap().get().twin()?.0, 13);
    assert_eq!(output.edges().get(12).unwrap().get().next()?.0, 6);
    assert_eq!(output.edges().get(12).unwrap().get().prev()?.0, 1);
    assert_eq!(output.edges().get(13).unwrap().get().cell()?.0, 4);
    assert_eq!(
        output.edges().get(13).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(13).unwrap().get().twin()?.0, 12);
    assert_eq!(output.edges().get(13).unwrap().get().next()?.0, 14);
    assert_eq!(output.edges().get(13).unwrap().get().prev()?.0, 9);
    assert_eq!(output.edges().get(14).unwrap().get().cell()?.0, 4);
    assert!(output.edges().get(14).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(14).unwrap().get().twin()?.0, 15);
    assert_eq!(output.edges().get(14).unwrap().get().next()?.0, 9);
    assert_eq!(output.edges().get(14).unwrap().get().prev()?.0, 13);
    assert_eq!(output.edges().get(15).unwrap().get().cell()?.0, 2);
    assert_eq!(
        output.edges().get(15).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(15).unwrap().get().twin()?.0, 14);
    assert_eq!(output.edges().get(15).unwrap().get().next()?.0, 3);
    assert_eq!(output.edges().get(15).unwrap().get().prev()?.0, 11);
    Ok(())
}

//#[ignore]
#[test]
fn eighth_points_1() -> Result<(), BvError> {
    let output = {
        let _v = vec![
            Point { x: 10, y: 16 },
            Point { x: 12, y: 3 },
            Point { x: 4, y: 12 },
            Point { x: 8, y: 10 },
            Point { x: 7, y: 18 },
            Point { x: 8, y: 9 },
            Point { x: 9, y: 8 },
            Point { x: 11, y: 11 },
        ];
        let mut vb = Builder::<I1, F1>::default();
        vb.with_vertices(_v.iter()).expect("eighth_points_1");
        vb.construct().expect("eighth_points_1")
    };
    assert_eq!(output.cells().len(), 8);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    assert_eq!(cell.source_index(), 2);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    assert_eq!(cell.source_index(), 4);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    assert_eq!(cell.source_index(), 5);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    assert_eq!(cell.source_index(), 3);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.id().0, 4);
    assert_eq!(cell.source_index(), 6);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.id().0, 5);
    assert_eq!(cell.source_index(), 0);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    assert_eq!(cell.source_index(), 7);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.id().0, 7);
    assert_eq!(cell.source_index(), 1);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 9);
    assert_eq!(output.edges().len(), 32);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 5.25, v.y(), 9.5));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 6.75, v.y(), 14.375));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 7.28571, v.y(), 13.5714));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 9.5, v.y(), 9.5));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 9.78571, v.y(), 9.64286));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 8.625, v.y(), 13.125));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -2.0, v.y(), -2.0));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -4.75, v.y(), -3.83333));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 13.3947, v.y(), 7.23684));
    assert_eq!(v.get_incident_edge()?.0, 31);
    assert_eq!(output.edges().get(0).unwrap().get().cell()?.0, 0);
    assert_eq!(output.edges().get(0).unwrap().get().vertex0().unwrap().0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().twin()?.0, 1);
    assert_eq!(output.edges().get(0).unwrap().get().next()?.0, 28);
    assert_eq!(output.edges().get(0).unwrap().get().prev()?.0, 12);
    let e = output.edges()[0].get();
    assert!(output.edge_get_vertex1(e.id())?.is_none());
    assert_eq!(output.edge_rot_next(e.id())?.0, 13);
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(1).unwrap().get().cell()?.0, 1);
    assert!(output.edges().get(1).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(1).unwrap().get().twin()?.0, 0);
    assert_eq!(output.edges().get(1).unwrap().get().next()?.0, 10);
    assert_eq!(output.edges().get(1).unwrap().get().prev()?.0, 10);
    let e = output.edges()[1].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 1);
    assert_eq!(output.edge_rot_next(e.id())?.0, 11);
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(2).unwrap().get().cell()?.0, 0);
    assert_eq!(output.edges().get(2).unwrap().get().vertex0().unwrap().0, 7);
    assert_eq!(output.edges().get(2).unwrap().get().twin()?.0, 3);
    assert_eq!(output.edges().get(2).unwrap().get().next()?.0, 4);
    assert_eq!(output.edges().get(2).unwrap().get().prev()?.0, 28);
    let e = output.edges()[2].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 0);
    assert_eq!(output.edge_rot_next(e.id())?.0, 29);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(3).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(3).unwrap().get().twin()?.0, 2);
    assert_eq!(output.edges().get(3).unwrap().get().next()?.0, 26);
    assert_eq!(output.edges().get(3).unwrap().get().prev()?.0, 6);
    let e = output.edges()[3].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 7);
    assert_eq!(output.edge_rot_next(e.id())?.0, 7);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(4).unwrap().get().cell()?.0, 0);
    assert_eq!(output.edges().get(4).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(4).unwrap().get().twin()?.0, 5);
    assert_eq!(output.edges().get(4).unwrap().get().next()?.0, 12);
    assert_eq!(output.edges().get(4).unwrap().get().prev()?.0, 2);
    let e = output.edges()[4].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 2);
    assert_eq!(output.edge_rot_next(e.id())?.0, 3);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(5).unwrap().get().cell()?.0, 3);
    assert_eq!(output.edges().get(5).unwrap().get().vertex0().unwrap().0, 2);
    assert_eq!(output.edges().get(5).unwrap().get().twin()?.0, 4);
    assert_eq!(output.edges().get(5).unwrap().get().next()?.0, 7);
    assert_eq!(output.edges().get(5).unwrap().get().prev()?.0, 14);
    let e = output.edges()[5].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 0);
    assert_eq!(output.edge_rot_next(e.id())?.0, 15);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(6).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(6).unwrap().get().vertex0().unwrap().0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().twin()?.0, 7);
    assert_eq!(output.edges().get(6).unwrap().get().next()?.0, 3);
    assert_eq!(output.edges().get(6).unwrap().get().prev()?.0, 8);
    let e = output.edges()[6].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 0);
    assert_eq!(output.edge_rot_next(e.id())?.0, 9);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(7).unwrap().get().cell()?.0, 3);
    assert_eq!(output.edges().get(7).unwrap().get().vertex0().unwrap().0, 0);
    assert_eq!(output.edges().get(7).unwrap().get().twin()?.0, 6);
    assert_eq!(output.edges().get(7).unwrap().get().next()?.0, 19);
    assert_eq!(output.edges().get(7).unwrap().get().prev()?.0, 5);
    let e = output.edges()[7].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 3);
    assert_eq!(output.edge_rot_next(e.id())?.0, 4);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(8).unwrap().get().cell()?.0, 2);
    assert_eq!(output.edges().get(8).unwrap().get().vertex0().unwrap().0, 6);
    assert_eq!(output.edges().get(8).unwrap().get().twin()?.0, 9);
    assert_eq!(output.edges().get(8).unwrap().get().next()?.0, 6);
    assert_eq!(output.edges().get(8).unwrap().get().prev()?.0, 26);
    let e = output.edges()[8].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 3);
    assert_eq!(output.edge_rot_next(e.id())?.0, 27);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(9).unwrap().get().cell()?.0, 4);
    assert_eq!(output.edges().get(9).unwrap().get().vertex0().unwrap().0, 3);
    assert_eq!(output.edges().get(9).unwrap().get().twin()?.0, 8);
    assert_eq!(output.edges().get(9).unwrap().get().next()?.0, 24);
    assert_eq!(output.edges().get(9).unwrap().get().prev()?.0, 18);
    let e = output.edges()[9].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 6);
    assert_eq!(output.edge_rot_next(e.id())?.0, 19);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(10).unwrap().get().cell()?.0, 1);
    assert_eq!(
        output.edges().get(10).unwrap().get().vertex0().unwrap().0,
        1
    );
    assert_eq!(output.edges().get(10).unwrap().get().twin()?.0, 11);
    assert_eq!(output.edges().get(10).unwrap().get().next()?.0, 1);
    assert_eq!(output.edges().get(10).unwrap().get().prev()?.0, 1);
    let e = output.edges()[10].get();
    assert!(output.edge_get_vertex1(e.id())?.is_none());
    assert_eq!(output.edge_rot_next(e.id())?.0, 0);
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(11).unwrap().get().cell()?.0, 5);
    assert!(output.edges().get(11).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(11).unwrap().get().twin()?.0, 10);
    assert_eq!(output.edges().get(11).unwrap().get().next()?.0, 13);
    assert_eq!(output.edges().get(11).unwrap().get().prev()?.0, 23);
    let e = output.edges()[11].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 1);
    assert_eq!(output.edge_rot_next(e.id())?.0, 22);
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(12).unwrap().get().cell()?.0, 0);
    assert_eq!(
        output.edges().get(12).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(12).unwrap().get().twin()?.0, 13);
    assert_eq!(output.edges().get(12).unwrap().get().next()?.0, 0);
    assert_eq!(output.edges().get(12).unwrap().get().prev()?.0, 4);
    let e = output.edges()[12].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 1);
    assert_eq!(output.edge_rot_next(e.id())?.0, 5);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(13).unwrap().get().cell()?.0, 5);
    assert_eq!(
        output.edges().get(13).unwrap().get().vertex0().unwrap().0,
        1
    );
    assert_eq!(output.edges().get(13).unwrap().get().twin()?.0, 12);
    assert_eq!(output.edges().get(13).unwrap().get().next()?.0, 15);
    assert_eq!(output.edges().get(13).unwrap().get().prev()?.0, 11);
    let e = output.edges()[13].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 2);
    assert_eq!(output.edge_rot_next(e.id())?.0, 10);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(14).unwrap().get().cell()?.0, 3);
    assert_eq!(
        output.edges().get(14).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(14).unwrap().get().twin()?.0, 15);
    assert_eq!(output.edges().get(14).unwrap().get().next()?.0, 5);
    assert_eq!(output.edges().get(14).unwrap().get().prev()?.0, 16);
    let e = output.edges()[14].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 2);
    assert_eq!(output.edge_rot_next(e.id())?.0, 17);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(15).unwrap().get().cell()?.0, 5);
    assert_eq!(
        output.edges().get(15).unwrap().get().vertex0().unwrap().0,
        2
    );
    assert_eq!(output.edges().get(15).unwrap().get().twin()?.0, 14);
    assert_eq!(output.edges().get(15).unwrap().get().next()?.0, 23);
    assert_eq!(output.edges().get(15).unwrap().get().prev()?.0, 13);
    let e = output.edges()[15].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 5);
    assert_eq!(output.edge_rot_next(e.id())?.0, 12);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(16).unwrap().get().cell()?.0, 3);
    assert_eq!(
        output.edges().get(16).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(16).unwrap().get().twin()?.0, 17);
    assert_eq!(output.edges().get(16).unwrap().get().next()?.0, 14);
    assert_eq!(output.edges().get(16).unwrap().get().prev()?.0, 19);
    let e = output.edges()[16].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 5);
    assert_eq!(output.edge_rot_next(e.id())?.0, 18);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(17).unwrap().get().cell()?.0, 6);
    assert_eq!(
        output.edges().get(17).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(17).unwrap().get().twin()?.0, 16);
    assert_eq!(output.edges().get(17).unwrap().get().next()?.0, 21);
    assert_eq!(output.edges().get(17).unwrap().get().prev()?.0, 22);
    let e = output.edges()[17].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 4);
    assert_eq!(output.edge_rot_next(e.id())?.0, 23);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(18).unwrap().get().cell()?.0, 4);
    assert_eq!(
        output.edges().get(18).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(18).unwrap().get().twin()?.0, 19);
    assert_eq!(output.edges().get(18).unwrap().get().next()?.0, 9);
    assert_eq!(output.edges().get(18).unwrap().get().prev()?.0, 20);
    let e = output.edges()[18].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 3);
    assert_eq!(output.edge_rot_next(e.id())?.0, 21);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(19).unwrap().get().cell()?.0, 3);
    assert_eq!(
        output.edges().get(19).unwrap().get().vertex0().unwrap().0,
        3
    );
    assert_eq!(output.edges().get(19).unwrap().get().twin()?.0, 18);
    assert_eq!(output.edges().get(19).unwrap().get().next()?.0, 16);
    assert_eq!(output.edges().get(19).unwrap().get().prev()?.0, 7);
    let e = output.edges()[19].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 4);
    assert_eq!(output.edge_rot_next(e.id())?.0, 6);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(20).unwrap().get().cell()?.0, 4);
    assert_eq!(
        output.edges().get(20).unwrap().get().vertex0().unwrap().0,
        8
    );
    assert_eq!(output.edges().get(20).unwrap().get().twin()?.0, 21);
    assert_eq!(output.edges().get(20).unwrap().get().next()?.0, 18);
    assert_eq!(output.edges().get(20).unwrap().get().prev()?.0, 24);
    let e = output.edges()[20].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 4);
    assert_eq!(output.edge_rot_next(e.id())?.0, 25);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(21).unwrap().get().cell()?.0, 6);
    assert_eq!(
        output.edges().get(21).unwrap().get().vertex0().unwrap().0,
        4
    );
    assert_eq!(output.edges().get(21).unwrap().get().twin()?.0, 20);
    assert_eq!(output.edges().get(21).unwrap().get().next()?.0, 31);
    assert_eq!(output.edges().get(21).unwrap().get().prev()?.0, 17);
    let e = output.edges()[21].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 8);
    assert_eq!(output.edge_rot_next(e.id())?.0, 16);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(22).unwrap().get().cell()?.0, 6);
    assert!(output.edges().get(22).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(22).unwrap().get().twin()?.0, 23);
    assert_eq!(output.edges().get(22).unwrap().get().next()?.0, 17);
    assert_eq!(output.edges().get(22).unwrap().get().prev()?.0, 31);
    let e = output.edges()[22].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 5);
    assert_eq!(output.edge_rot_next(e.id())?.0, 30);
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(23).unwrap().get().cell()?.0, 5);
    assert_eq!(
        output.edges().get(23).unwrap().get().vertex0().unwrap().0,
        5
    );
    assert_eq!(output.edges().get(23).unwrap().get().twin()?.0, 22);
    assert_eq!(output.edges().get(23).unwrap().get().next()?.0, 11);
    assert_eq!(output.edges().get(23).unwrap().get().prev()?.0, 15);
    let e = output.edges()[23].get();
    assert!(output.edge_get_vertex1(e.id())?.is_none());
    assert_eq!(output.edge_rot_next(e.id())?.0, 14);
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(24).unwrap().get().cell()?.0, 4);
    assert_eq!(
        output.edges().get(24).unwrap().get().vertex0().unwrap().0,
        6
    );
    assert_eq!(output.edges().get(24).unwrap().get().twin()?.0, 25);
    assert_eq!(output.edges().get(24).unwrap().get().next()?.0, 20);
    assert_eq!(output.edges().get(24).unwrap().get().prev()?.0, 9);
    let e = output.edges()[24].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 8);
    assert_eq!(output.edge_rot_next(e.id())?.0, 8);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(25).unwrap().get().cell()?.0, 7);
    assert_eq!(
        output.edges().get(25).unwrap().get().vertex0().unwrap().0,
        8
    );
    assert_eq!(output.edges().get(25).unwrap().get().twin()?.0, 24);
    assert_eq!(output.edges().get(25).unwrap().get().next()?.0, 27);
    assert_eq!(output.edges().get(25).unwrap().get().prev()?.0, 30);
    let e = output.edges()[25].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 6);
    assert_eq!(output.edge_rot_next(e.id())?.0, 31);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(26).unwrap().get().cell()?.0, 2);
    assert_eq!(
        output.edges().get(26).unwrap().get().vertex0().unwrap().0,
        7
    );
    assert_eq!(output.edges().get(26).unwrap().get().twin()?.0, 27);
    assert_eq!(output.edges().get(26).unwrap().get().next()?.0, 8);
    assert_eq!(output.edges().get(26).unwrap().get().prev()?.0, 3);
    let e = output.edges()[26].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 6);
    assert_eq!(output.edge_rot_next(e.id())?.0, 2);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(27).unwrap().get().cell()?.0, 7);
    assert_eq!(
        output.edges().get(27).unwrap().get().vertex0().unwrap().0,
        6
    );
    assert_eq!(output.edges().get(27).unwrap().get().twin()?.0, 26);
    assert_eq!(output.edges().get(27).unwrap().get().next()?.0, 29);
    assert_eq!(output.edges().get(27).unwrap().get().prev()?.0, 25);
    let e = output.edges()[27].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 7);
    assert_eq!(output.edge_rot_next(e.id())?.0, 24);
    assert_eq!(output.edge_is_finite(e.id())?, true);
    assert_eq!(output.edge_is_infinite(e.id())?, false);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(28).unwrap().get().cell()?.0, 0);
    assert!(output.edges().get(28).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(28).unwrap().get().twin()?.0, 29);
    assert_eq!(output.edges().get(28).unwrap().get().next()?.0, 2);
    assert_eq!(output.edges().get(28).unwrap().get().prev()?.0, 0);
    let e = output.edges()[28].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 7);
    assert_eq!(output.edge_rot_next(e.id())?.0, 1);
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(29).unwrap().get().cell()?.0, 7);
    assert_eq!(
        output.edges().get(29).unwrap().get().vertex0().unwrap().0,
        7
    );
    assert_eq!(output.edges().get(29).unwrap().get().twin()?.0, 28);
    assert_eq!(output.edges().get(29).unwrap().get().next()?.0, 30);
    assert_eq!(output.edges().get(29).unwrap().get().prev()?.0, 27);
    let e = output.edges()[29].get();
    assert!(output.edge_get_vertex1(e.id())?.is_none());
    assert_eq!(output.edge_rot_next(e.id())?.0, 26);
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(30).unwrap().get().cell()?.0, 7);
    assert!(output.edges().get(30).unwrap().get().vertex0().is_none());
    assert_eq!(output.edges().get(30).unwrap().get().twin()?.0, 31);
    assert_eq!(output.edges().get(30).unwrap().get().next()?.0, 25);
    assert_eq!(output.edges().get(30).unwrap().get().prev()?.0, 29);
    let e = output.edges()[30].get();
    assert_eq!(output.edge_get_vertex1(e.id())?.unwrap().0, 8);
    assert_eq!(output.edge_rot_next(e.id())?.0, 28);
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    assert_eq!(output.edges().get(31).unwrap().get().cell()?.0, 6);
    assert_eq!(
        output.edges().get(31).unwrap().get().vertex0().unwrap().0,
        8
    );
    assert_eq!(output.edges().get(31).unwrap().get().twin()?.0, 30);
    assert_eq!(output.edges().get(31).unwrap().get().next()?.0, 22);
    assert_eq!(output.edges().get(31).unwrap().get().prev()?.0, 21);
    let e = output.edges()[31].get();
    assert!(output.edge_get_vertex1(e.id())?.is_none());
    assert_eq!(output.edge_rot_next(e.id())?.0, 20);
    assert_eq!(output.edge_is_finite(e.id())?, false);
    assert_eq!(output.edge_is_infinite(e.id())?, true);
    assert_eq!(e.is_linear(), true);
    assert_eq!(e.is_curved(), false);
    assert_eq!(e.is_primary(), true);
    assert_eq!(e.is_secondary(), false);
    Ok(())
}
