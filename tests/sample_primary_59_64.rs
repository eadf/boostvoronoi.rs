use boostvoronoi as BV;
use boostvoronoi::prelude::*;
use std::io::{BufReader, Cursor};

mod common;
use common::almost_equal;

type I = i32;
type F = f64;

#[test]
fn sample_primary_059() -> Result<(), BvError> {
    let output = {
        let input = r#"0
11
214748364 -214748364 1073741820 -1073741820
-858993456 -214748364 -858993456 0
-214748364 -644245092 -858993456 -214748364
0 -644245092 -214748364 -644245092
214748364 -644245092 0 -644245092
-858993456 0 -858993456 -214748364
-644245092 214748364 -858993456 0
-214748364 214748364 -214748364 0
-214748364 0 429496728 0
-214748364 0 -214748364 -644245092
-214748364 -644245092 0 -644245092
"#;
        let mut vb = Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = BV::read_boost_input_buffer::<I, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 20);
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
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.id().0, 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.id().0, 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[8].get();
    assert_eq!(cell.id().0, 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.id().0, 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[10].get();
    assert_eq!(cell.id().0, 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[11].get();
    assert_eq!(cell.id().0, 11);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[12].get();
    assert_eq!(cell.id().0, 12);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[13].get();
    assert_eq!(cell.id().0, 13);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[14].get();
    assert_eq!(cell.id().0, 14);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[15].get();
    assert_eq!(cell.id().0, 15);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[16].get();
    assert_eq!(cell.id().0, 16);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[17].get();
    assert_eq!(cell.id().0, 17);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[18].get();
    assert_eq!(cell.id().0, 18);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[19].get();
    assert_eq!(cell.id().0, 19);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 22);
    assert_eq!(output.edges().len(), 82);
    let v = output.vertices()[0].get();
    assert!(almost_equal(
        v.x(),
        -858993456.0000000,
        v.y(),
        -214748364.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -858993456.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(
        v.x(),
        -632799048.9995401,
        v.y(),
        -93692791.1125303
    ));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(
        v.x(),
        -515605524.5450082,
        v.y(),
        -82088345.3248184
    ));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -481603418.5987024, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[5].get();
    assert!(almost_equal(
        v.x(),
        -466341722.2675318,
        v.y(),
        36844994.2675318
    ));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[6].get();
    assert!(almost_equal(
        v.x(),
        -429496728.0000000,
        v.y(),
        214748364.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 33);
    let v = output.vertices()[7].get();
    assert!(almost_equal(
        v.x(),
        -214748364.0000000,
        v.y(),
        -644245092.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 37);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -214748364.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 41);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), -429496727.9999999));
    assert_eq!(v.get_incident_edge()?.0, 47);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), 214748364.0000000));
    assert_eq!(v.get_incident_edge()?.0, 49);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), -214748364.0000000));
    assert_eq!(v.get_incident_edge()?.0, 55);
    let v = output.vertices()[12].get();
    assert!(almost_equal(
        v.x(),
        36844994.2675318,
        v.y(),
        -392651733.7324682
    ));
    assert_eq!(v.get_incident_edge()?.0, 59);
    let v = output.vertices()[13].get();
    assert!(almost_equal(
        v.x(),
        214748364.0000000,
        v.y(),
        -466341722.2675318
    ));
    assert_eq!(v.get_incident_edge()?.0, 61);
    let v = output.vertices()[14].get();
    assert!(almost_equal(
        v.x(),
        303700048.8662341,
        v.y(),
        -125796679.1337659
    ));
    assert_eq!(v.get_incident_edge()?.0, 63);
    let v = output.vertices()[15].get();
    assert!(almost_equal(
        v.x(),
        429496728.0000000,
        v.y(),
        -177903369.7324682
    ));
    assert_eq!(v.get_incident_edge()?.0, 67);
    let v = output.vertices()[16].get();
    assert!(almost_equal(
        v.x(),
        214748364.0000000,
        v.y(),
        -1681141917.7324686
    ));
    assert_eq!(v.get_incident_edge()?.0, 71);
    let v = output.vertices()[17].get();
    assert!(almost_equal(
        v.x(),
        429496727.9999999,
        v.y(),
        1073741819.9999999
    ));
    assert_eq!(v.get_incident_edge()?.0, 73);
    let v = output.vertices()[18].get();
    assert!(almost_equal(
        v.x(),
        36844994.2675318,
        v.y(),
        -2110638645.7324684
    ));
    assert_eq!(v.get_incident_edge()?.0, 75);
    let v = output.vertices()[19].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), -2201170731.0000000));
    assert_eq!(v.get_incident_edge()?.0, 77);
    let v = output.vertices()[20].get();
    assert!(almost_equal(
        v.x(),
        -214748364.0000001,
        v.y(),
        -2791728732.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 79);
    let v = output.vertices()[21].get();
    assert!(almost_equal(
        v.x(),
        2899102914.0000000,
        v.y(),
        751619274.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 81);
    Ok(())
}

#[test]
fn sample_primary_060() -> Result<(), BvError> {
    let output = {
        let input = r#"0
11
1 -1 5 -5
-4 -1 -4 0
-1 -3 -4 -1
0 -3 -1 -3
1 -3 0 -3
-4 0 -4 -1
-3 1 -4 0
-1 1 -1 0
-1 0 2 0
-1 0 -1 -3
-1 -3 0 -3
"#;
        let mut vb = Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = BV::read_boost_input_buffer::<I, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 20);
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
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.id().0, 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.id().0, 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[8].get();
    assert_eq!(cell.id().0, 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.id().0, 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[10].get();
    assert_eq!(cell.id().0, 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[11].get();
    assert_eq!(cell.id().0, 11);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[12].get();
    assert_eq!(cell.id().0, 12);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[13].get();
    assert_eq!(cell.id().0, 13);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[14].get();
    assert_eq!(cell.id().0, 14);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[15].get();
    assert_eq!(cell.id().0, 15);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[16].get();
    assert_eq!(cell.id().0, 16);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[17].get();
    assert_eq!(cell.id().0, 17);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[18].get();
    assert_eq!(cell.id().0, 18);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[19].get();
    assert_eq!(cell.id().0, 19);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 22);
    assert_eq!(output.edges().len(), 82);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -4.0000000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -4.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -2.9467002, v.y(), -0.4362911));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -2.4009753, v.y(), -0.3822536));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -2.2426407, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -2.1715729, v.y(), 0.1715729));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -2.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 33);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -3.0000000));
    assert_eq!(v.get_incident_edge()?.0, 37);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 41);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), -2.0000000));
    assert_eq!(v.get_incident_edge()?.0, 47);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 49);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 55);
    let v = output.vertices()[12].get();
    assert!(almost_equal(v.x(), 0.1715729, v.y(), -1.8284271));
    assert_eq!(v.get_incident_edge()?.0, 59);
    let v = output.vertices()[13].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), -2.1715729));
    assert_eq!(v.get_incident_edge()?.0, 61);
    let v = output.vertices()[14].get();
    assert!(almost_equal(v.x(), 1.4142136, v.y(), -0.5857864));
    assert_eq!(v.get_incident_edge()?.0, 63);
    let v = output.vertices()[15].get();
    assert!(almost_equal(v.x(), 2.0000000, v.y(), -0.8284271));
    assert_eq!(v.get_incident_edge()?.0, 67);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), -7.8284271));
    assert_eq!(v.get_incident_edge()?.0, 71);
    let v = output.vertices()[17].get();
    assert!(almost_equal(v.x(), 0.1715729, v.y(), -9.8284271));
    assert_eq!(v.get_incident_edge()?.0, 73);
    let v = output.vertices()[18].get();
    assert!(almost_equal(v.x(), 2.0000000, v.y(), 5.0000000));
    assert_eq!(v.get_incident_edge()?.0, 75);
    let v = output.vertices()[19].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), -10.2500000));
    assert_eq!(v.get_incident_edge()?.0, 77);
    let v = output.vertices()[20].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -13.0000000));
    assert_eq!(v.get_incident_edge()?.0, 79);
    let v = output.vertices()[21].get();
    assert!(almost_equal(v.x(), 13.5000000, v.y(), 3.5000000));
    assert_eq!(v.get_incident_edge()?.0, 81);
    Ok(())
}

#[test]
fn sample_primary_061() -> Result<(), BvError> {
    let output = {
        let input = r#"0
8
-858993456 644245092 0 -429496728
644245092 214748364 -214748364 214748364
0 429496728 214748364 644245092
214748364 644245092 429496728 644245092
429496728 644245092 644245092 858993456
-858993456 644245092 -429496728 214748364
-429496728 644245092 214748364 644245092
214748364 644245092 429496728 644245092
"#;
        let mut vb = Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = BV::read_boost_input_buffer::<I, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 17);
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
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
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
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[8].get();
    assert_eq!(cell.id().0, 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.id().0, 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[10].get();
    assert_eq!(cell.id().0, 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[11].get();
    assert_eq!(cell.id().0, 11);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[12].get();
    assert_eq!(cell.id().0, 12);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[13].get();
    assert_eq!(cell.id().0, 13);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[14].get();
    assert_eq!(cell.id().0, 14);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[15].get();
    assert_eq!(cell.id().0, 15);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[16].get();
    assert_eq!(cell.id().0, 16);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 24);
    assert_eq!(output.edges().len(), 80);
    let v = output.vertices()[0].get();
    assert!(almost_equal(
        v.x(),
        -858993456.0000000,
        v.y(),
        644245092.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(
        v.x(),
        -453284463.6098484,
        v.y(),
        190960628.3901515
    ));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(
        v.x(),
        -644245092.0000000,
        v.y(),
        858993456.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(
        v.x(),
        -429496728.0000000,
        v.y(),
        466341722.2675318
    ));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(
        v.x(),
        -322122546.0000000,
        v.y(),
        163566981.8097484
    ));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(
        v.x(),
        -322122546.0000000,
        v.y(),
        322122546.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(
        v.x(),
        -313266620.0500839,
        v.y(),
        418197635.1985877
    ));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(
        v.x(),
        -214748364.0000000,
        v.y(),
        70249604.4263135
    ));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(
        v.x(),
        -214748364.0000000,
        v.y(),
        429496728.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 35);
    let v = output.vertices()[9].get();
    assert!(almost_equal(
        v.x(),
        -88951684.8662341,
        v.y(),
        518448412.8662342
    ));
    assert_eq!(v.get_incident_edge()?.0, 39);
    let v = output.vertices()[10].get();
    assert!(almost_equal(
        v.x(),
        88951684.8662341,
        v.y(),
        340545043.1337659
    ));
    assert_eq!(v.get_incident_edge()?.0, 41);
    let v = output.vertices()[11].get();
    assert!(almost_equal(
        v.x(),
        214748364.0000000,
        v.y(),
        644245092.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 47);
    let v = output.vertices()[12].get();
    assert!(almost_equal(
        v.x(),
        429496728.0000000,
        v.y(),
        644245092.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 53);
    let v = output.vertices()[13].get();
    assert!(almost_equal(
        v.x(),
        303700048.8662341,
        v.y(),
        429496728.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 55);
    let v = output.vertices()[14].get();
    assert!(almost_equal(
        v.x(),
        429496728.0000000,
        v.y(),
        429496728.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 57);
    let v = output.vertices()[15].get();
    assert!(almost_equal(
        v.x(),
        309640199.0864711,
        v.y(),
        -181784568.7308232
    ));
    assert_eq!(v.get_incident_edge()?.0, 63);
    let v = output.vertices()[16].get();
    assert!(almost_equal(
        v.x(),
        214748364.0000000,
        v.y(),
        1162693504.8662341
    ));
    assert_eq!(v.get_incident_edge()?.0, 65);
    let v = output.vertices()[17].get();
    assert!(almost_equal(
        v.x(),
        125796679.1337659,
        v.y(),
        1377441868.8662341
    ));
    assert_eq!(v.get_incident_edge()?.0, 67);
    let v = output.vertices()[18].get();
    assert!(almost_equal(
        v.x(),
        607400097.7324682,
        v.y(),
        466341722.2675318
    ));
    assert_eq!(v.get_incident_edge()?.0, 69);
    let v = output.vertices()[19].get();
    assert!(almost_equal(
        v.x(),
        644245092.0000000,
        v.y(),
        481603418.5987024
    ));
    assert_eq!(v.get_incident_edge()?.0, 71);
    let v = output.vertices()[20].get();
    assert!(almost_equal(
        v.x(),
        644245092.0000000,
        v.y(),
        -429496728.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 73);
    let v = output.vertices()[21].get();
    assert!(almost_equal(
        v.x(),
        966367638.0000000,
        v.y(),
        536870910.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 75);
    let v = output.vertices()[22].get();
    assert!(almost_equal(
        v.x(),
        -429496728.0000000,
        v.y(),
        3435973824.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 77);
    let v = output.vertices()[23].get();
    assert!(almost_equal(
        v.x(),
        -644245092.0000000,
        v.y(),
        4509715644.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 79);
    Ok(())
}

#[test]
fn sample_primary_062() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
644245092 214748364 -214748364 214748364
0 429496728 214748364 644245092
-429496728 644245092 214748364 644245092
"#;
        let mut vb = Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = BV::read_boost_input_buffer::<I, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 8);
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
    assert_eq!(output.vertices().len(), 7);
    assert_eq!(output.edges().len(), 28);
    let v = output.vertices()[0].get();
    assert!(almost_equal(
        v.x(),
        -429496728.0000000,
        v.y(),
        375809637.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(
        v.x(),
        -214748364.0000000,
        v.y(),
        429496728.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(
        v.x(),
        -88951684.8662341,
        v.y(),
        518448412.8662342
    ));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(
        v.x(),
        88951684.8662341,
        v.y(),
        340545043.1337659
    ));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(
        v.x(),
        214748364.0000000,
        v.y(),
        644245092.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(
        v.x(),
        392651733.7324682,
        v.y(),
        466341722.2675318
    ));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[6].get();
    assert!(almost_equal(
        v.x(),
        644245092.0000000,
        v.y(),
        644245092.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 27);
    Ok(())
}

#[test]
fn sample_primary_063() -> Result<(), BvError> {
    let output = {
        let input = r#"0
2
-5 0 -1 -1
-4 0 -6 3
"#;
        let mut vb = Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = BV::read_boost_input_buffer::<I, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 6);
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
    assert_eq!(output.vertices().len(), 6);
    assert_eq!(output.edges().len(), 22);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -11.0000000, v.y(), -0.3333333));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -4.8840179, v.y(), 0.4639282));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -4.1159821, v.y(), -0.0773214));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -3.2240179, v.y(), 0.5173214));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 0.8106248, v.y(), 6.2424992));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 2.7857143, v.y(), 8.8571429));
    assert_eq!(v.get_incident_edge()?.0, 21);
    Ok(())
}

#[test]
fn sample_primary_064() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
-8 15 24 28
-37 4 -39 36
12 -30 44 8
"#;
        let mut vb = Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = BV::read_boost_input_buffer::<I, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
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
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.id().0, 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, BV::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
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
    assert_eq!(output.vertices().len(), 9);
    assert_eq!(output.edges().len(), 34);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -20.7978947, v.y(), 5.0126316));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -16.4622005, v.y(), 35.8300321));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -15.4291742, v.y(), 37.4731766));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -13.4475410, v.y(), -14.3655738));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -8.9431818, v.y(), -12.3636364));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 0.8438020, v.y(), -6.7693587));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 29.6278740, v.y(), 14.1467718));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 33.1428571, v.y(), 17.1428571));
    assert_eq!(v.get_incident_edge()?.0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -0.3855814, v.y(), 88.0260465));
    assert_eq!(v.get_incident_edge()?.0, 33);
    Ok(())
}
