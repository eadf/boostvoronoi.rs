use boostvoronoi as BV;
use boostvoronoi::prelude::*;
mod common;
use common::almost_equal;
use std::io::{BufReader, Cursor};

type I = i64;
type F = f64;

//#[ignore]
#[test]
fn stress_test_13() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
503 -633 600 -981
664 -702 688 -800
-658 584 880 -850
"#;
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = BV::read_boost_input_buffer::<I, _>(br)?;
        Builder::<I, F>::default()
            .with_vertices(points.iter())?
            .with_segments(segments.iter())?
            .build()?
    };
    assert_eq!(output.cells().len(), 9);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.id().0, 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.id().0, 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[7].get();
    assert_eq!(cell.id().0, 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.id().0, 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 40);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -142.9265766, v.y(), -813.0427527));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 554.1342703, v.y(), -618.7470568));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 597.6624688, v.y(), -718.2459260));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 593.1177536, v.y(), -682.8898519));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -1282.7701855, v.y(), -1505.7951379));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 623.2141768, v.y(), -815.8659159));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 684.6559989, v.y(), -696.9413880));
    assert_eq!(v.get_incident_edge().unwrap().0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 737.9114534, v.y(), -787.7767869));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 746.2790312, v.y(), -940.2268218));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 750.8438609, v.y(), -942.4461865));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 766.3723644, v.y(), -971.8684125));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), -5452.0336668, v.y(), -4557.7181168));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}
