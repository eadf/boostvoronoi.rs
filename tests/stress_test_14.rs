use boostvoronoi as BV;
use boostvoronoi::prelude::*;
mod common;
use common::almost_equal;
use std::io::{BufReader, Cursor};

type I = i64;
type F = f64;

//#[ignore]
#[test]
fn stress_test_14() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
180 217 436 -48
561 725 -777 -922
-120 -155 443 558
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
    assert!(almost_equal(v.x(), -130.3255115, v.y(), -146.8467560));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 176.8832875, v.y(), 213.9891381));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 184.0803011, v.y(), 220.9417248));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 437.6183729, v.y(), 562.2494474));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 201.2781778, v.y(), -274.7501377));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 219.9553002, v.y(), -423.4359523));
    assert_eq!(v.get_incident_edge().unwrap().0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 277.2534081, v.y(), -721.1719151));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 743.2903644, v.y(), 248.8540879));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 832.4769949, v.y(), 250.4606618));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 1038.6074092, v.y(), 248.0796174));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 1170.5279195, v.y(), 229.8279561));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 1445.0261704, v.y(), -2727.1433005));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}
