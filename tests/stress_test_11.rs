use boostvoronoi as BV;
use boostvoronoi::prelude::*;
mod common;
use common::almost_equal;
use std::io::{BufReader, Cursor};

type I = i64;
type F = f64;

//#[ignore]
#[test]
fn stress_test_11() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
570 8 245 194
838 785 8 157
-965 -572 934 858
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
    assert!(almost_equal(v.x(), 7.1092255, v.y(), 158.1772975));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 152.2153901, v.y(), 31.8763537));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 285.4865785, v.y(), 264.7426775));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 156.7970070, v.y(), -39.6584646));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 158.8011680, v.y(), -114.2576293));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 837.8290535, v.y(), 785.2259324));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 192.5889990, v.y(), -651.4547061));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 733.3522361, v.y(), 293.4272943));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 1283.0712790, v.y(), 196.7688510));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 1330.4718373, v.y(), 180.4196237));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 1492.9844280, v.y(), 115.6843155));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 795.4005697, v.y(), -2909.7627145));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}
