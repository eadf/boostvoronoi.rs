use boostvoronoi::builder as VB;
use boostvoronoi::diagram as VD;
use boostvoronoi::file_reader as FR;
use boostvoronoi::BvError;
use std::io::{BufReader, Cursor};

#[allow(dead_code)]
fn almost_equal(x1: F1, x2: F1, y1: F1, y2: F1) -> bool {
    let delta = 0.000001;
    assert!(F1::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F1::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F1::abs(x1 - x2) < delta) && (F1::abs(y1 - y2) < delta)
}

type I1 = i32;
type F1 = f64;

//#[ignore]
#[test]
fn sample_primary_031() -> Result<(), BvError> {
    let output = {
        let input = r#"1
0 0
12
-1 10 1 10
10 -1 10 1
-1 -10 1 -10
-10 -1 -10 1
-6 8 -2 11
-8 6 -11 2
6 8 2 11
8 6 11 2
6 -8 2 -11
8 -6 11 -2
-6 -8 -2 -11
-8 -6 -11 -2
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 37);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    let cell = output.cells()[9].get();
    assert_eq!(cell.id().0, 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[10].get();
    assert_eq!(cell.id().0, 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[11].get();
    assert_eq!(cell.id().0, 11);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[12].get();
    assert_eq!(cell.id().0, 12);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[18].get();
    assert_eq!(cell.id().0, 18);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[19].get();
    assert_eq!(cell.id().0, 19);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[20].get();
    assert_eq!(cell.id().0, 20);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[21].get();
    assert_eq!(cell.id().0, 21);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[22].get();
    assert_eq!(cell.id().0, 22);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[23].get();
    assert_eq!(cell.id().0, 23);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[24].get();
    assert_eq!(cell.id().0, 24);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[25].get();
    assert_eq!(cell.id().0, 25);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[26].get();
    assert_eq!(cell.id().0, 26);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[27].get();
    assert_eq!(cell.id().0, 27);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[28].get();
    assert_eq!(cell.id().0, 28);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[29].get();
    assert_eq!(cell.id().0, 29);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[30].get();
    assert_eq!(cell.id().0, 30);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[31].get();
    assert_eq!(cell.id().0, 31);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[32].get();
    assert_eq!(cell.id().0, 32);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[33].get();
    assert_eq!(cell.id().0, 33);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[34].get();
    assert_eq!(cell.id().0, 34);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[35].get();
    assert_eq!(cell.id().0, 35);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[36].get();
    assert_eq!(cell.id().0, 36);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 48);
    assert_eq!(output.edges().len(), 168);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -11.0000000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -12.5000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -11.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -10.4285714, v.y(), -1.5714286));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -10.4285714, v.y(), 1.5714286));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -1.5714286, v.y(), -10.4285714));
    assert_eq!(v.get_incident_edge()?.0, 47);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -1.5714286, v.y(), 10.4285714));
    assert_eq!(v.get_incident_edge()?.0, 49);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -11.0000000));
    assert_eq!(v.get_incident_edge()?.0, 51);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 11.0000000));
    assert_eq!(v.get_incident_edge()?.0, 53);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), -4.9500000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 57);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -4.9500000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 59);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), -4.8923311, v.y(), -1.5766889));
    assert_eq!(v.get_incident_edge()?.0, 61);
    let v = output.vertices()[12].get();
    assert!(almost_equal(v.x(), -4.8923311, v.y(), 1.5766889));
    assert_eq!(v.get_incident_edge()?.0, 63);
    let v = output.vertices()[13].get();
    assert!(almost_equal(v.x(), -4.0000000, v.y(), -3.0000000));
    assert_eq!(v.get_incident_edge()?.0, 65);
    let v = output.vertices()[14].get();
    assert!(almost_equal(v.x(), -4.0000000, v.y(), 3.0000000));
    assert_eq!(v.get_incident_edge()?.0, 67);
    let v = output.vertices()[15].get();
    assert!(almost_equal(v.x(), -3.5714286, v.y(), -3.5714286));
    assert_eq!(v.get_incident_edge()?.0, 73);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), -3.5714286, v.y(), 3.5714286));
    assert_eq!(v.get_incident_edge()?.0, 75);
    let v = output.vertices()[17].get();
    assert!(almost_equal(v.x(), -3.0000000, v.y(), -4.0000000));
    assert_eq!(v.get_incident_edge()?.0, 77);
    let v = output.vertices()[18].get();
    assert!(almost_equal(v.x(), -3.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge()?.0, 79);
    let v = output.vertices()[19].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), -11.0000000));
    assert_eq!(v.get_incident_edge()?.0, 83);
    let v = output.vertices()[20].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), 11.0000000));
    assert_eq!(v.get_incident_edge()?.0, 87);
    let v = output.vertices()[21].get();
    assert!(almost_equal(v.x(), 1.5714286, v.y(), -10.4285714));
    assert_eq!(v.get_incident_edge()?.0, 93);
    let v = output.vertices()[22].get();
    assert!(almost_equal(v.x(), 1.5714286, v.y(), 10.4285714));
    assert_eq!(v.get_incident_edge()?.0, 95);
    let v = output.vertices()[23].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), -12.5000000));
    assert_eq!(v.get_incident_edge()?.0, 97);
    let v = output.vertices()[24].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 12.5000000));
    assert_eq!(v.get_incident_edge()?.0, 99);
    let v = output.vertices()[25].get();
    assert!(almost_equal(v.x(), -1.5766889, v.y(), -4.8923311));
    assert_eq!(v.get_incident_edge()?.0, 101);
    let v = output.vertices()[26].get();
    assert!(almost_equal(v.x(), -1.5766889, v.y(), 4.8923311));
    assert_eq!(v.get_incident_edge()?.0, 103);
    let v = output.vertices()[27].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -4.9500000));
    assert_eq!(v.get_incident_edge()?.0, 105);
    let v = output.vertices()[28].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 4.9500000));
    assert_eq!(v.get_incident_edge()?.0, 107);
    let v = output.vertices()[29].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), -4.9500000));
    assert_eq!(v.get_incident_edge()?.0, 113);
    let v = output.vertices()[30].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), 4.9500000));
    assert_eq!(v.get_incident_edge()?.0, 115);
    let v = output.vertices()[31].get();
    assert!(almost_equal(v.x(), 1.5766889, v.y(), -4.8923311));
    assert_eq!(v.get_incident_edge()?.0, 117);
    let v = output.vertices()[32].get();
    assert!(almost_equal(v.x(), 1.5766889, v.y(), 4.8923311));
    assert_eq!(v.get_incident_edge()?.0, 119);
    let v = output.vertices()[33].get();
    assert!(almost_equal(v.x(), 3.0000000, v.y(), -4.0000000));
    assert_eq!(v.get_incident_edge()?.0, 121);
    let v = output.vertices()[34].get();
    assert!(almost_equal(v.x(), 3.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge()?.0, 123);
    let v = output.vertices()[35].get();
    assert!(almost_equal(v.x(), 3.5714286, v.y(), -3.5714286));
    assert_eq!(v.get_incident_edge()?.0, 133);
    let v = output.vertices()[36].get();
    assert!(almost_equal(v.x(), 3.5714286, v.y(), 3.5714286));
    assert_eq!(v.get_incident_edge()?.0, 135);
    let v = output.vertices()[37].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), -3.0000000));
    assert_eq!(v.get_incident_edge()?.0, 137);
    let v = output.vertices()[38].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), 3.0000000));
    assert_eq!(v.get_incident_edge()?.0, 139);
    let v = output.vertices()[39].get();
    assert!(almost_equal(v.x(), 4.9500000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 145);
    let v = output.vertices()[40].get();
    assert!(almost_equal(v.x(), 4.9500000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 149);
    let v = output.vertices()[41].get();
    assert!(almost_equal(v.x(), 4.8923311, v.y(), -1.5766889));
    assert_eq!(v.get_incident_edge()?.0, 151);
    let v = output.vertices()[42].get();
    assert!(almost_equal(v.x(), 4.8923311, v.y(), 1.5766889));
    assert_eq!(v.get_incident_edge()?.0, 153);
    let v = output.vertices()[43].get();
    assert!(almost_equal(v.x(), 10.4285714, v.y(), -1.5714286));
    assert_eq!(v.get_incident_edge()?.0, 159);
    let v = output.vertices()[44].get();
    assert!(almost_equal(v.x(), 10.4285714, v.y(), 1.5714286));
    assert_eq!(v.get_incident_edge()?.0, 161);
    let v = output.vertices()[45].get();
    assert!(almost_equal(v.x(), 11.0000000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 163);
    let v = output.vertices()[46].get();
    assert!(almost_equal(v.x(), 11.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 165);
    let v = output.vertices()[47].get();
    assert!(almost_equal(v.x(), 12.5000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 167);
    Ok(())
}

#[test]
fn sample_primary_032() -> Result<(), BvError> {
    let output = {
        let input = r#"3
0 -4
2 8
-16 15
1
7 20 7 -20
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 6);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.id().0, 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 6);
    assert_eq!(output.edges().len(), 22);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -9.9826087, v.y(), 3.8304348));
    assert_eq!(v.get_incident_edge()?.0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -14.7857143, v.y(), -20.0000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 0.9166506, v.y(), 2.0138916));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -4.6426618, v.y(), 17.5617267));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -5.0434783, v.y(), 20.0000000));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -63.6869919, v.y(), -41.3943089));
    assert_eq!(v.get_incident_edge()?.0, 21);
    Ok(())
}

#[test]
fn sample_primary_033() -> Result<(), BvError> {
    let output = {
        let input = r#"4
-6 6
-5 6
-4 6
-3 6
1
0 0 0 7
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 7);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 8);
    assert_eq!(output.edges().len(), 28);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -6.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -5.5000000, v.y(), 0.5227744));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -4.5000000, v.y(), 1.5278640));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -3.5000000, v.y(), 2.5358984));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -1.6666667, v.y(), 7.0000000));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -3.5000000, v.y(), 12.5000000));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -4.5000000, v.y(), 16.5000000));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -5.5000000, v.y(), 21.5000000));
    assert_eq!(v.get_incident_edge()?.0, 27);
    Ok(())
}

#[test]
fn sample_primary_034() -> Result<(), BvError> {
    let output = {
        let input = r#"0
4
0 -4 2 8
2 8 -16 15
0 -4 -16 15
7 20 7 -20
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[8].get();
    assert_eq!(cell.id().0, 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 40);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -16.0000000, v.y(), 15.0000000));
    assert_eq!(v.get_incident_edge()?.0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), -4.0000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -2.5741126, v.y(), 5.3969379));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 2.0000000, v.y(), 8.0000000));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -14.7857143, v.y(), -20.0000000));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 3.4760263, v.y(), -4.5793377));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 4.4828759, v.y(), 7.5861873));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 3.3301305, v.y(), 11.4203355));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -2.5389358, v.y(), 20.0000000));
    assert_eq!(v.get_incident_edge()?.0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), -23.5569106, v.y(), -23.8373984));
    assert_eq!(v.get_incident_edge()?.0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -8.2749004, v.y(), 34.8645418));
    assert_eq!(v.get_incident_edge()?.0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), -151.4715447, v.y(), -99.0813008));
    assert_eq!(v.get_incident_edge()?.0, 39);
    Ok(())
}

//#[ignore]
#[test]
fn sample_primary_035() -> Result<(), BvError> {
    let output = {
        let input = r#"0
20
100 0 95 30
95 30 80 58
80 58 58 80
58 80 30 95
30 95 0 99
0 99 -30 95
-30 95 -58 80
-58 80 -80 58
-80 58 -95 30
-95 30 -99 0
-99 0 -95 -30
-95 -30 -80 -58
-80 -58 -58 -80
-58 -80 -30 -95
-30 -95 0 -99
0 -99 30 -95
30 -95 58 -80
58 -80 80 -58
80 -58 95 -30
95 -30 100 0
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 40);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    let cell = output.cells()[9].get();
    assert_eq!(cell.id().0, 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[10].get();
    assert_eq!(cell.id().0, 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[13].get();
    assert_eq!(cell.id().0, 13);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[14].get();
    assert_eq!(cell.id().0, 14);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[18].get();
    assert_eq!(cell.id().0, 18);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[19].get();
    assert_eq!(cell.id().0, 19);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[20].get();
    assert_eq!(cell.id().0, 20);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[21].get();
    assert_eq!(cell.id().0, 21);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[22].get();
    assert_eq!(cell.id().0, 22);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[23].get();
    assert_eq!(cell.id().0, 23);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[24].get();
    assert_eq!(cell.id().0, 24);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[25].get();
    assert_eq!(cell.id().0, 25);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[26].get();
    assert_eq!(cell.id().0, 26);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[27].get();
    assert_eq!(cell.id().0, 27);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[28].get();
    assert_eq!(cell.id().0, 28);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[29].get();
    assert_eq!(cell.id().0, 29);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[30].get();
    assert_eq!(cell.id().0, 30);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[31].get();
    assert_eq!(cell.id().0, 31);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[32].get();
    assert_eq!(cell.id().0, 32);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[33].get();
    assert_eq!(cell.id().0, 33);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[34].get();
    assert_eq!(cell.id().0, 34);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[35].get();
    assert_eq!(cell.id().0, 35);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[36].get();
    assert_eq!(cell.id().0, 36);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[37].get();
    assert_eq!(cell.id().0, 37);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[38].get();
    assert_eq!(cell.id().0, 38);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[39].get();
    assert_eq!(cell.id().0, 39);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 29);
    assert_eq!(output.edges().len(), 136);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -99.0000000, v.y(), -0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -95.0000000, v.y(), -30.0000000));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -95.0000000, v.y(), 30.0000000));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -80.0000000, v.y(), -58.0000000));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -80.0000000, v.y(), 58.0000000));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -58.0000000, v.y(), -80.0000000));
    assert_eq!(v.get_incident_edge()?.0, 37);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -58.0000000, v.y(), 80.0000000));
    assert_eq!(v.get_incident_edge()?.0, 41);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -30.0000000, v.y(), -95.0000000));
    assert_eq!(v.get_incident_edge()?.0, 49);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -30.0000000, v.y(), 95.0000000));
    assert_eq!(v.get_incident_edge()?.0, 53);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), -99.0000000));
    assert_eq!(v.get_incident_edge()?.0, 61);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), 99.0000000));
    assert_eq!(v.get_incident_edge()?.0, 65);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 30.0000000, v.y(), -95.0000000));
    assert_eq!(v.get_incident_edge()?.0, 73);
    let v = output.vertices()[12].get();
    assert!(almost_equal(v.x(), 30.0000000, v.y(), 95.0000000));
    assert_eq!(v.get_incident_edge()?.0, 77);
    let v = output.vertices()[13].get();
    assert!(almost_equal(v.x(), 58.0000000, v.y(), -80.0000000));
    assert_eq!(v.get_incident_edge()?.0, 85);
    let v = output.vertices()[14].get();
    assert!(almost_equal(v.x(), 58.0000000, v.y(), 80.0000000));
    assert_eq!(v.get_incident_edge()?.0, 89);
    let v = output.vertices()[15].get();
    assert!(almost_equal(v.x(), 80.0000000, v.y(), -58.0000000));
    assert_eq!(v.get_incident_edge()?.0, 97);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), 80.0000000, v.y(), 58.0000000));
    assert_eq!(v.get_incident_edge()?.0, 101);
    let v = output.vertices()[17].get();
    assert!(almost_equal(v.x(), -2.0439806, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 103);
    let v = output.vertices()[18].get();
    assert!(almost_equal(v.x(), -1.8724422, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 105);
    let v = output.vertices()[19].get();
    assert!(almost_equal(v.x(), 95.0000000, v.y(), -30.0000000));
    assert_eq!(v.get_incident_edge()?.0, 113);
    let v = output.vertices()[20].get();
    assert!(almost_equal(v.x(), 95.0000000, v.y(), 30.0000000));
    assert_eq!(v.get_incident_edge()?.0, 117);
    let v = output.vertices()[21].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), -2.0439806));
    assert_eq!(v.get_incident_edge()?.0, 119);
    let v = output.vertices()[22].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 2.0439806));
    assert_eq!(v.get_incident_edge()?.0, 121);
    let v = output.vertices()[23].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 1.8724422));
    assert_eq!(v.get_incident_edge()?.0, 123);
    let v = output.vertices()[24].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), -1.8724422));
    assert_eq!(v.get_incident_edge()?.0, 125);
    let v = output.vertices()[25].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 127);
    let v = output.vertices()[26].get();
    assert!(almost_equal(v.x(), 1.8724422, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 129);
    let v = output.vertices()[27].get();
    assert!(almost_equal(v.x(), 6.9786032, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 131);
    let v = output.vertices()[28].get();
    assert!(almost_equal(v.x(), 100.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 135);
    Ok(())
}

#[test]
fn sample_primary_040() -> Result<(), BvError> {
    let output = {
        let input = r#"10
858993458 1717986916
-429496729 1288490187
-2147483645 -1288490187
-2147483645 -858993458
-1717986916 858993458
-2147483645 -429496729
429496729 -2147483645
858993458 -429496729
-429496729 1717986916
1717986916 -858993458
0
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 10);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.id().0, 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.id().0, 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.id().0, 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.id().0, 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.id().0, 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 9);
    assert_eq!(output.edges().len(), 36);
    let v = output.vertices()[0].get();
    assert!(almost_equal(
        v.x(),
        -1216907398.8333333,
        v.y(),
        1503238551.5000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[1].get();
    assert!(almost_equal(
        v.x(),
        -644245093.5000000,
        v.y(),
        -644245093.5000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[2].get();
    assert!(almost_equal(
        v.x(),
        -644245093.5000000,
        v.y(),
        -214748364.4999999
    ));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[3].get();
    assert!(almost_equal(
        v.x(),
        -644245093.5000000,
        v.y(),
        -1073741822.5000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[4].get();
    assert!(almost_equal(
        v.x(),
        214748364.5000000,
        v.y(),
        1503238551.5000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[5].get();
    assert!(almost_equal(
        v.x(),
        -596523234.7222221,
        v.y(),
        -978298104.9444439
    ));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(
        v.x(),
        501079517.1666666,
        v.y(),
        644245093.5000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(
        v.x(),
        930576246.1666664,
        v.y(),
        -1360072975.1666667
    ));
    assert_eq!(v.get_incident_edge()?.0, 33);
    let v = output.vertices()[8].get();
    assert!(almost_equal(
        v.x(),
        1932735280.5000002,
        v.y(),
        644245093.5000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 35);
    Ok(())
}

#[test]
fn sample_primary_041() -> Result<(), BvError> {
    let output = {
        let input = r#"0
6
0 0 0 10
-5 0 -1 0
-6 2 -1 2
-4 5 -2 5
-8 8 -1 8
-7 -2 -7 7
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 18);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[9].get();
    assert_eq!(cell.id().0, 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[10].get();
    assert_eq!(cell.id().0, 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[13].get();
    assert_eq!(cell.id().0, 13);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[14].get();
    assert_eq!(cell.id().0, 14);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[15].get();
    assert_eq!(cell.id().0, 15);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[16].get();
    assert_eq!(cell.id().0, 16);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[17].get();
    assert_eq!(cell.id().0, 17);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 25);
    assert_eq!(output.edges().len(), 84);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -57.5000000, v.y(), -2.0000000));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -8.0000000, v.y(), 7.0000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -6.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -6.0000000, v.y(), 3.0000000));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -6.0000000, v.y(), 7.0000000));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -5.8284271, v.y(), 0.8284271));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -5.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -5.2426407, v.y(), 3.7573593));
    assert_eq!(v.get_incident_edge()?.0, 35);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -5.2426407, v.y(), 6.2426407));
    assert_eq!(v.get_incident_edge()?.0, 37);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), -5.0000000, v.y(), -2.0000000));
    assert_eq!(v.get_incident_edge()?.0, 39);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -4.0000000, v.y(), 3.5000000));
    assert_eq!(v.get_incident_edge()?.0, 41);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), -4.0000000, v.y(), 6.5000000));
    assert_eq!(v.get_incident_edge()?.0, 43);
    let v = output.vertices()[12].get();
    assert!(almost_equal(v.x(), -2.0000000, v.y(), 3.5000000));
    assert_eq!(v.get_incident_edge()?.0, 53);
    let v = output.vertices()[13].get();
    assert!(almost_equal(v.x(), -2.0000000, v.y(), 6.5000000));
    assert_eq!(v.get_incident_edge()?.0, 55);
    let v = output.vertices()[14].get();
    assert!(almost_equal(v.x(), -0.5000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 61);
    let v = output.vertices()[15].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 63);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 3.0000000));
    assert_eq!(v.get_incident_edge()?.0, 65);
    let v = output.vertices()[17].get();
    assert!(almost_equal(v.x(), -1.5358984, v.y(), 3.5358984));
    assert_eq!(v.get_incident_edge()?.0, 67);
    let v = output.vertices()[18].get();
    assert!(almost_equal(v.x(), -1.5358984, v.y(), 6.4641016));
    assert_eq!(v.get_incident_edge()?.0, 69);
    let v = output.vertices()[19].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 7.0000000));
    assert_eq!(v.get_incident_edge()?.0, 71);
    let v = output.vertices()[20].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 9.0000000));
    assert_eq!(v.get_incident_edge()?.0, 73);
    let v = output.vertices()[21].get();
    assert!(almost_equal(v.x(), -2.0000000, v.y(), 10.0000000));
    assert_eq!(v.get_incident_edge()?.0, 77);
    let v = output.vertices()[22].get();
    assert!(almost_equal(v.x(), -8.0000000, v.y(), 25.0000000));
    assert_eq!(v.get_incident_edge()?.0, 79);
    let v = output.vertices()[23].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -10.0000000));
    assert_eq!(v.get_incident_edge()?.0, 81);
    let v = output.vertices()[24].get();
    assert!(almost_equal(v.x(), -0.5000000, v.y(), -11.5000000));
    assert_eq!(v.get_incident_edge()?.0, 83);
    Ok(())
}

#[test]
fn sample_primary_042() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
-6 -1 -5 3
-1 0 4 -1
3 0 4 1
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(output.vertices().len(), 11);
    assert_eq!(output.edges().len(), 38);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -3.2631579, v.y(), -1.6842105));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -2.3684211, v.y(), 2.3421053));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 1.7543219, v.y(), 1.2456781));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 3.3567892, v.y(), -0.3567892));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -2.3000000, v.y(), -6.5000000));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 4.1783946, v.y(), -0.1080271));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 0.8858048, v.y(), 4.1141952));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 0.1363636, v.y(), 5.6818182));
    assert_eq!(v.get_incident_edge()?.0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 5.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 0.3965572, v.y(), 6.0345072));
    assert_eq!(v.get_incident_edge()?.0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -26.0000000));
    assert_eq!(v.get_incident_edge()?.0, 37);
    Ok(())
}

#[test]
fn sample_primary_043() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
-5 -2 -4 2
-5 3 -2 2
-5 5 -2 2
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
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
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.id().0, 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.id().0, 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.id().0, 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.id().0, 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(output.vertices().len(), 9);
    assert_eq!(output.edges().len(), 32);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -15.0000000, v.y(), 0.5000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -6.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -5.4153374, v.y(), 1.7539877));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -4.7639320, v.y(), 3.7082039));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -4.6645399, v.y(), 2.1661350));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -3.4301347, v.y(), 1.8575337));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -2.0000000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -2.5698653, v.y(), 0.2904041));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 1.2500000, v.y(), -3.5625000));
    assert_eq!(v.get_incident_edge()?.0, 31);
    Ok(())
}

#[test]
fn sample_primary_044() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
-9 -8 -4 -3
-8 -3 -4 -3
-2 7 -1 3
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[7].get();
    assert_eq!(cell.id().0, 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 7);
    assert_eq!(output.edges().len(), 28);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -12.2500000, v.y(), -4.7500000));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -8.0000000, v.y(), -4.6568542));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -4.0000000, v.y(), -3.0000000));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -10.0000000, v.y(), 5.0000000));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -8.0000000, v.y(), 3.6365995));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -5.6846584, v.y(), 1.8288354));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -4.0000000, v.y(), 0.7500000));
    assert_eq!(v.get_incident_edge()?.0, 27);
    Ok(())
}

#[test]
fn sample_primary_045() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
14 76 38 29
37 47 61 50
39 37 41 35
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert!(almost_equal(v.x(), 37.3588820, v.y(), 35.3588820));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 39.0430183, v.y(), 33.0430183));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 37.2324883, v.y(), 41.8464977));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 37.6341463, v.y(), 41.9268293));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 41.7105263, v.y(), 30.8947368));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 35.6273058, v.y(), 57.9815538));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 43.4560969, v.y(), 41.4560969));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 46.4342645, v.y(), 40.4342645));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 64.1250000, v.y(), 25.0000000));
    assert_eq!(v.get_incident_edge()?.0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 55.3487783, v.y(), 95.2097735));
    assert_eq!(v.get_incident_edge()?.0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 56.7744479, v.y(), 97.8422713));
    assert_eq!(v.get_incident_edge()?.0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 70.5000000, v.y(), 16.5000000));
    assert_eq!(v.get_incident_edge()?.0, 39);
    Ok(())
}

#[test]
fn sample_primary_046() -> Result<(), BvError> {
    let output = {
        let input = r#"2
2 -5
3 -3
1
0 0 2 -7
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 5);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.id().0, 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 16);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 2.3000793, v.y(), -3.9000397));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 4.2000000, v.y(), 1.2000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 5.5000000, v.y(), -6.0000000));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 6.5000000, v.y(), -6.0000000));
    assert_eq!(v.get_incident_edge()?.0, 15);
    Ok(())
}

#[test]
fn sample_primary_047() -> Result<(), BvError> {
    let output = {
        let input = r#"1
-35 -49
2
-48 -29 -46 -78
-46 -46 -45 -42
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 7);
    let cell = output.cells()[0].get();
    assert_eq!(cell.id().0, 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.id().0, 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 8);
    assert_eq!(output.edges().len(), 28);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -46.6461377, v.y(), -45.8384656));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -46.2216040, v.y(), -41.6945990));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -41.0643516, v.y(), -49.5692893));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -40.4680851, v.y(), -47.3829787));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -38.6595745, v.y(), -43.5851064));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -31.0032271, v.y(), -32.6474673));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -24.6490787, v.y(), -28.0469012));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -6.5209380, v.y(), -76.3886097));
    assert_eq!(v.get_incident_edge()?.0, 27);
    Ok(())
}
