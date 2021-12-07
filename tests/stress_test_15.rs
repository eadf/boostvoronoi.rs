use boostvoronoi as BV;
use boostvoronoi::prelude::*;
mod common;
use common::almost_equal;
use std::io::{BufReader, Cursor};

type I = i64;
type F = f64;

//#[ignore]
#[test]
fn stress_test_15() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
-649 -607 956 199
153 3 13 -252
89 -186 293 -40
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
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
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
    assert!(almost_equal(v.x(), -87.0876496, v.y(), -197.0499179));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 22.6901396, v.y(), -257.3200766));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 78.3314577, v.y(), -171.0932697));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 70.7833012, v.y(), -210.9603004));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 102.1837945, v.y(), -204.4211923));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 206.6804003, v.y(), -26.4715923));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -766.6186943, v.y(), 507.8886949));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 239.3995276, v.y(), 34.8938108));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 317.5792016, v.y(), -74.3435419));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 359.3196367, v.y(), 425.3313754));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -1879.0684588, v.y(), 1842.4539409));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 128.2542975, v.y(), 1847.3025466));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}
