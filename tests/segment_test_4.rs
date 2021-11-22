use boostvoronoi as BV;
use boostvoronoi::prelude::*;

mod common;
use common::to_segments;

type I = i32;
type F = f64;

fn retrieve_point<T: InputType>(
    point_data_: &Vec<Point<T>>,
    segment_data_: &Vec<Line<T>>,
    source: (BV::SourceIndex, BV::SourceCategory),
) -> Point<T> {
    match source.1 {
        BV::SourceCategory::SinglePoint => point_data_[source.0],
        BV::SourceCategory::SegmentStart => segment_data_[source.0 - point_data_.len()].start,
        BV::SourceCategory::Segment | BV::SourceCategory::SegmentEnd => {
            segment_data_[source.0 - point_data_.len()].end
        }
    }
}

//#[ignore]
#[test]
fn segment_4_1() -> Result<(), BvError> {
    let _output = {
        let segments = to_segments(&[
            [200, 200, 200, 400],
            [200, 400, 400, 400],
            [400, 400, 400, 200],
            [400, 200, 200, 200],
            [529, 242, 367, 107],
            [94, 628, 512, 632],
            [680, 608, 669, 291],
            [572, 96, 363, 51],
            [147, 103, 96, 170],
        ]);

        Builder::<I, F>::default()
            .with_segments(segments.iter())?
            .build()?
    };
    for v in _output.vertices().iter() {
        let v = v.get();
        assert!(!v.x().is_nan());
        assert!(!v.y().is_nan());
    }
    Ok(())
}

//#[ignore]
#[test]
fn vertical_segment_1() -> Result<(), BvError> {
    let v = Vec::<Point<I>>::default();
    let s = to_segments::<I>(&[
        [200, 200, 200, 400],
        [94, 628, 512, 632],
        [147, 103, 96, 170],
    ]);

    let output = Builder::<I, F>::default()
        .with_vertices(v.iter())?
        .with_segments(s.iter())?
        .build()?;

    for v in output.vertices().iter() {
        let v = v.get();
        assert!(!v.x().is_nan());
        assert!(!v.y().is_nan());
    }

    assert_eq!(output.cells().len(), 9);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&v, &s, (source_index, cat));
    assert_eq!(p, Point { x: 94, y: 628 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    let (source_index, cat) = cell.source_index_2();
    assert_eq!(cat, BV::SourceCategory::Segment);
    let p = retrieve_point(&v, &s, (source_index, cat));
    assert_eq!(p, Point { x: 512, y: 632 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&v, &s, (source_index, cat));
    assert_eq!(p, Point { x: 96, y: 170 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    let (source_index, cat) = cell.source_index_2();
    assert_eq!(cat, BV::SourceCategory::Segment);
    let p = retrieve_point(&v, &s, (source_index, cat));
    assert_eq!(p, Point { x: 96, y: 170 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.id().0, 4);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&v, &s, (source_index, cat));
    assert_eq!(p, Point { x: 147, y: 103 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.id().0, 5);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&v, &s, (source_index, cat));
    assert_eq!(p, Point { x: 200, y: 200 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    let (source_index, cat) = cell.source_index_2();
    assert_eq!(cat, BV::SourceCategory::Segment);
    let p = retrieve_point(&v, &s, (source_index, cat));
    assert_eq!(p, Point { x: 200, y: 400 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[7].get();
    assert_eq!(cell.id().0, 7);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&v, &s, (source_index, cat));
    assert_eq!(p, Point { x: 200, y: 400 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.id().0, 8);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&v, &s, (source_index, cat));
    assert_eq!(p, Point { x: 512, y: 632 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 10);
    assert_eq!(output.edges().len(), 36);
    /*let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 143.7970909, v.y(), 200.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0,13);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 142.0840045, v.y(), 205.0788691));
    assert_eq!(v.get_incident_edge().unwrap().0,15);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -102.2273417, v.y(), 398.1387452));
    assert_eq!(v.get_incident_edge().unwrap().0,17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -98.2075472, v.y(), 400.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0,21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 95.3208257, v.y(), 489.9737172));
    assert_eq!(v.get_incident_edge().unwrap().0,23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 195.1646270, v.y(), 139.6626265));
    assert_eq!(v.get_incident_edge().unwrap().0,25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 431.2163645, v.y(), 400.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0,29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 515.0143534, v.y(), 317.0000690));
    assert_eq!(v.get_incident_edge().unwrap().0,31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 655.0769231, v.y(), 200.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0,33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 2427.4071661, v.y(), -1080.0162866));
    assert_eq!(v.get_incident_edge().unwrap().0,35);*/
    Ok(())
}

//#[ignore]
#[test]
/// This used to give NaN coordinates in some vertexes
fn segment_4_2() -> Result<(), BvError> {
    let _output = {
        let segments: [[I; 4]; 7] = [
            [-19546, 47259, -45936, 36666],
            [-45936, 36666, -59968, -21417],
            [-59968, -21417, -125257, -19781],
            [-125257, -19781, -148480, -47150],
            [-148480, -47150, 148480, -57522],
            [148480, -57522, 105345, 58720],
            [105345, 58720, -19546, 47259],
        ];

        Builder::<I, F>::default()
            .with_segments(segments.iter())?
            .build()?
    };
    for v in _output.vertices().iter() {
        let v = v.get();
        assert!(!v.x().is_nan());
        assert!(!v.y().is_nan());
    }
    Ok(())
}
