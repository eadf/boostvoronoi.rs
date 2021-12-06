use boostvoronoi::prelude::*;
use boostvoronoi as BV;
mod common;
use common::almost_equal;
use std::io::{BufReader, Cursor};

type I = i64;
type F = f64;

//#[ignore]
#[test]
fn stress_test_12() -> Result<(), BvError> {
    let output  = {
        let input = r#"0
3
310 407 365 177
754 177 -893 79
300 558 109 347
"#;
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = BV::read_boost_input_buffer::<I, _>(br)?;
        Builder::<I, F>::default()
            .with_vertices(points.iter())?
            .with_segments(segments.iter())?
            .build()?
    };
    assert_eq!(output.cells().len(),9);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0,0);
    let (_source_index,_cat)=cell.source_index_2();
    assert_eq!(cell.is_degenerate(),false);
    assert_eq!(cell.contains_point(),true);
    assert_eq!(cell.contains_segment(),false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0,1);
    let (_source_index,_cat)=cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(),false);
    assert_eq!(cell.contains_point(),false);
    assert_eq!(cell.contains_segment(),true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0,2);
    let (_source_index,_cat)=cell.source_index_2();
    assert_eq!(cell.is_degenerate(),false);
    assert_eq!(cell.contains_point(),true);
    assert_eq!(cell.contains_segment(),false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0,3);
    let (_source_index,_cat)=cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(),false);
    assert_eq!(cell.contains_point(),false);
    assert_eq!(cell.contains_segment(),true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.id().0,4);
    let (_source_index,_cat)=cell.source_index_2();
    assert_eq!(cell.is_degenerate(),false);
    assert_eq!(cell.contains_point(),true);
    assert_eq!(cell.contains_segment(),false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.id().0,5);
    let (_source_index,_cat)=cell.source_index_2();
    assert_eq!(cell.is_degenerate(),false);
    assert_eq!(cell.contains_point(),true);
    assert_eq!(cell.contains_segment(),false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0,6);
    let (_source_index,_cat)=cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(),false);
    assert_eq!(cell.contains_point(),false);
    assert_eq!(cell.contains_segment(),true);
    let cell = output.cells()[7].get();
    assert_eq!(cell.id().0,7);
    let (_source_index,_cat)=cell.source_index_2();
    assert_eq!(cell.is_degenerate(),false);
    assert_eq!(cell.contains_point(),true);
    assert_eq!(cell.contains_segment(),false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.id().0,8);
    let (_source_index,_cat)=cell.source_index_2();
    assert_eq!(cell.is_degenerate(),false);
    assert_eq!(cell.contains_point(),true);
    assert_eq!(cell.contains_segment(),false);
    assert_eq!(output.vertices().len(),12);
    assert_eq!(output.edges().len(),40);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -430.1073125, v.y(), 835.0070933));
    assert_eq!(v.get_incident_edge().unwrap().0,7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 242.4153334, v.y(), 390.8384493));
    assert_eq!(v.get_incident_edge().unwrap().0,15);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 198.9752931, v.y(), 265.5531707));
    assert_eq!(v.get_incident_edge().unwrap().0,17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 211.9531307, v.y(), 271.7785454));
    assert_eq!(v.get_incident_edge().unwrap().0,19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 345.8651286, v.y(), 172.4242699));
    assert_eq!(v.get_incident_edge().unwrap().0,23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 392.2186611, v.y(), 183.5088103));
    assert_eq!(v.get_incident_edge().unwrap().0,25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 378.0606604, v.y(), 487.3384543));
    assert_eq!(v.get_incident_edge().unwrap().0,27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -755.5931670, v.y(), 1513.5369426));
    assert_eq!(v.get_incident_edge().unwrap().0,29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 611.5329568, v.y(), 479.1057071));
    assert_eq!(v.get_incident_edge().unwrap().0,33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 629.4076802, v.y(), 503.9839523));
    assert_eq!(v.get_incident_edge().unwrap().0,35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -1013.5088252, v.y(), 2104.2860719));
    assert_eq!(v.get_incident_edge().unwrap().0,37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 728.3860140, v.y(), 607.4717857));
    assert_eq!(v.get_incident_edge().unwrap().0,39);
    Ok(())
}
