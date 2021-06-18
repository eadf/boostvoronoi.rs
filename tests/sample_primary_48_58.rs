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

#[test]
fn sample_primary_048() -> Result<(), BvError> {
    let output = {
        let input = r#"0
9
-50 -29 -49 -73
-48 -29 -46 -78
-46 -46 -45 -42
-35 -49 -34 -49
-30 -2 -29 3
-43 16 -40 6
-36 38 -34 49
-35 39 -31 37
-28 34 -27 -9
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 27);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
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
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 42);
    assert_eq!(output.edges().len(), 136);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -49.0000000, v.y(), -28.9772727));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -48.9989917, v.y(), -29.0407752));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -47.6034504, v.y(), -72.9682602));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -52.0802920, v.y(), -78.2481752));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -46.6461377, v.y(), -45.8384656));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -46.2216040, v.y(), -41.6945990));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -93.5853659, v.y(), 0.8243902));
    assert_eq!(v.get_incident_edge()?.0, 33);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -72.3170732, v.y(), -3.6951220));
    assert_eq!(v.get_incident_edge()?.0, 37);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -41.0643516, v.y(), -49.5692893));
    assert_eq!(v.get_incident_edge()?.0, 51);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), -40.4680851, v.y(), -47.3829787));
    assert_eq!(v.get_incident_edge()?.0, 53);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -35.2812500, v.y(), 38.4375000));
    assert_eq!(v.get_incident_edge()?.0, 55);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), -124.8333333, v.y(), 54.1515152));
    assert_eq!(v.get_incident_edge()?.0, 57);
    let v = output.vertices()[12].get();
    assert!(almost_equal(v.x(), -34.9687500, v.y(), 37.8125000));
    assert_eq!(v.get_incident_edge()?.0, 63);
    let v = output.vertices()[13].get();
    assert!(almost_equal(v.x(), -34.5000000, v.y(), 40.0000000));
    assert_eq!(v.get_incident_edge()?.0, 65);
    let v = output.vertices()[14].get();
    assert!(almost_equal(v.x(), -38.6595745, v.y(), -43.5851064));
    assert_eq!(v.get_incident_edge()?.0, 67);
    let v = output.vertices()[15].get();
    assert!(almost_equal(v.x(), -229.0000000, v.y(), 84.4545455));
    assert_eq!(v.get_incident_edge()?.0, 71);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), -49.0000000, v.y(), -10.3571429));
    assert_eq!(v.get_incident_edge()?.0, 73);
    let v = output.vertices()[17].get();
    assert!(almost_equal(v.x(), -37.0689655, v.y(), -0.5862069));
    assert_eq!(v.get_incident_edge()?.0, 79);
    let v = output.vertices()[18].get();
    assert!(almost_equal(v.x(), -34.6034483, v.y(), 4.1206897));
    assert_eq!(v.get_incident_edge()?.0, 83);
    let v = output.vertices()[19].get();
    assert!(almost_equal(v.x(), -31.0714286, v.y(), 33.9285714));
    assert_eq!(v.get_incident_edge()?.0, 89);
    let v = output.vertices()[20].get();
    assert!(almost_equal(v.x(), -33.4620473, v.y(), 32.0759053));
    assert_eq!(v.get_incident_edge()?.0, 91);
    let v = output.vertices()[21].get();
    assert!(almost_equal(v.x(), -34.8275980, v.y(), 31.2098492));
    assert_eq!(v.get_incident_edge()?.0, 93);
    let v = output.vertices()[22].get();
    assert!(almost_equal(v.x(), -39.3835122, v.y(), 26.9629357));
    assert_eq!(v.get_incident_edge()?.0, 95);
    let v = output.vertices()[23].get();
    assert!(almost_equal(v.x(), -35.4834582, v.y(), 18.2549625));
    assert_eq!(v.get_incident_edge()?.0, 97);
    let v = output.vertices()[24].get();
    assert!(almost_equal(v.x(), -45.5217391, v.y(), -11.1521739));
    assert_eq!(v.get_incident_edge()?.0, 99);
    let v = output.vertices()[25].get();
    assert!(almost_equal(v.x(), -33.8325811, v.y(), 7.8502257));
    assert_eq!(v.get_incident_edge()?.0, 101);
    let v = output.vertices()[26].get();
    assert!(almost_equal(v.x(), -33.7512228, v.y(), 7.2455163));
    assert_eq!(v.get_incident_edge()?.0, 103);
    let v = output.vertices()[27].get();
    assert!(almost_equal(v.x(), -28.1461219, v.y(), 2.8292244));
    assert_eq!(v.get_incident_edge()?.0, 105);
    let v = output.vertices()[28].get();
    assert!(almost_equal(v.x(), -28.5922550, v.y(), -2.2815490));
    assert_eq!(v.get_incident_edge()?.0, 107);
    let v = output.vertices()[29].get();
    assert!(almost_equal(v.x(), -37.2213115, v.y(), -9.2377049));
    assert_eq!(v.get_incident_edge()?.0, 111);
    let v = output.vertices()[30].get();
    assert!(almost_equal(v.x(), -44.0217391, v.y(), -12.1521739));
    assert_eq!(v.get_incident_edge()?.0, 113);
    let v = output.vertices()[31].get();
    assert!(almost_equal(v.x(), -35.0000000, v.y(), -38.3571429));
    assert_eq!(v.get_incident_edge()?.0, 115);
    let v = output.vertices()[32].get();
    assert!(almost_equal(v.x(), -35.0000000, v.y(), -60.6965253));
    assert_eq!(v.get_incident_edge()?.0, 117);
    let v = output.vertices()[33].get();
    assert!(almost_equal(v.x(), -34.0000000, v.y(), -36.8571429));
    assert_eq!(v.get_incident_edge()?.0, 119);
    let v = output.vertices()[34].get();
    assert!(almost_equal(v.x(), -34.0000000, v.y(), -61.6565416));
    assert_eq!(v.get_incident_edge()?.0, 121);
    let v = output.vertices()[35].get();
    assert!(almost_equal(v.x(), -27.8333333, v.y(), 43.3333333));
    assert_eq!(v.get_incident_edge()?.0, 123);
    let v = output.vertices()[36].get();
    assert!(almost_equal(v.x(), -31.4048640, v.y(), -32.7790720));
    assert_eq!(v.get_incident_edge()?.0, 125);
    let v = output.vertices()[37].get();
    assert!(almost_equal(v.x(), -28.7254443, v.y(), -28.2132834));
    assert_eq!(v.get_incident_edge()?.0, 127);
    let v = output.vertices()[38].get();
    assert!(almost_equal(v.x(), -27.4171377, v.y(), -29.5395009));
    assert_eq!(v.get_incident_edge()?.0, 129);
    let v = output.vertices()[39].get();
    assert!(almost_equal(v.x(), -19.2368421, v.y(), 46.3157895));
    assert_eq!(v.get_incident_edge()?.0, 131);
    let v = output.vertices()[40].get();
    assert!(almost_equal(v.x(), -18.5000000, v.y(), 46.5000000));
    assert_eq!(v.get_incident_edge()?.0, 133);
    let v = output.vertices()[41].get();
    assert!(almost_equal(v.x(), -8.6431889, v.y(), -76.4752322));
    assert_eq!(v.get_incident_edge()?.0, 135);
    Ok(())
}

#[test]
fn sample_primary_049() -> Result<(), BvError> {
    let output = {
        let input = r#"0
9
-5 4 -7 8
-5 4 -5 2
-5 4 -2 7
-1 -6 -5 -2
-1 -6 -3 -10
-1 -6 5 -6
5 -1 5 -4
5 -1 3 4
5 -1 8 6
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 21);
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
    assert_eq!(cell.is_degenerate(), true);
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
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[11].get();
    assert_eq!(cell.id().0, 11);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), true);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
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
    assert_eq!(cell.is_degenerate(), true);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[19].get();
    assert_eq!(cell.id().0, 19);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[20].get();
    assert_eq!(cell.id().0, 20);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 29);
    assert_eq!(output.edges().len(), 92);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -13.4721360, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -5.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -24.3333333, v.y(), -0.6666667));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -20.7459667, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -10.6666667, v.y(), -7.6666667));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -8.5497035, v.y(), -7.2251482));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -4.1622777, v.y(), 9.1622777));
    assert_eq!(v.get_incident_edge()?.0, 31);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -6.0000000));
    assert_eq!(v.get_incident_edge()?.0, 35);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -4.1111111, v.y(), 9.4444444));
    assert_eq!(v.get_incident_edge()?.0, 37);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), -3.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 39);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 0.1250000, v.y(), 4.8750000));
    assert_eq!(v.get_incident_edge()?.0, 45);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), -0.7679780, v.y(), 2.4928088));
    assert_eq!(v.get_incident_edge()?.0, 47);
    let v = output.vertices()[12].get();
    assert!(almost_equal(v.x(), -0.8144713, v.y(), 2.2662972));
    assert_eq!(v.get_incident_edge()?.0, 49);
    let v = output.vertices()[13].get();
    assert!(almost_equal(v.x(), -0.7631871, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge()?.0, 51);
    let v = output.vertices()[14].get();
    assert!(almost_equal(v.x(), -0.6000812, v.y(), 0.2806945));
    assert_eq!(v.get_incident_edge()?.0, 53);
    let v = output.vertices()[15].get();
    assert!(almost_equal(v.x(), 0.7521293, v.y(), -1.7699856));
    assert_eq!(v.get_incident_edge()?.0, 55);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), 3.0000000, v.y(), -4.0000000));
    assert_eq!(v.get_incident_edge()?.0, 63);
    let v = output.vertices()[17].get();
    assert!(almost_equal(v.x(), 0.8074176, v.y(), -1.8074176));
    assert_eq!(v.get_incident_edge()?.0, 65);
    let v = output.vertices()[18].get();
    assert!(almost_equal(v.x(), 5.0000000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 69);
    let v = output.vertices()[19].get();
    assert!(almost_equal(v.x(), 5.0000000, v.y(), -5.0000000));
    assert_eq!(v.get_incident_edge()?.0, 71);
    let v = output.vertices()[20].get();
    assert!(almost_equal(v.x(), 5.0710678, v.y(), 4.8284271));
    assert_eq!(v.get_incident_edge()?.0, 73);
    let v = output.vertices()[21].get();
    assert!(almost_equal(v.x(), 4.5000000, v.y(), 7.5000000));
    assert_eq!(v.get_incident_edge()?.0, 77);
    let v = output.vertices()[22].get();
    assert!(almost_equal(v.x(), 3.3800000, v.y(), 10.3000000));
    assert_eq!(v.get_incident_edge()?.0, 79);
    let v = output.vertices()[23].get();
    assert!(almost_equal(v.x(), 3.4721360, v.y(), -13.2360680));
    assert_eq!(v.get_incident_edge()?.0, 81);
    let v = output.vertices()[24].get();
    assert!(almost_equal(v.x(), 5.0000000, v.y(), -16.0000000));
    assert_eq!(v.get_incident_edge()?.0, 83);
    let v = output.vertices()[25].get();
    assert!(almost_equal(v.x(), 19.6157731, v.y(), -4.0000000));
    assert_eq!(v.get_incident_edge()?.0, 85);
    let v = output.vertices()[26].get();
    assert!(almost_equal(v.x(), 24.1652541, v.y(), -5.0000000));
    assert_eq!(v.get_incident_edge()?.0, 87);
    let v = output.vertices()[27].get();
    assert!(almost_equal(v.x(), 43.7000000, v.y(), -9.3000000));
    assert_eq!(v.get_incident_edge()?.0, 89);
    let v = output.vertices()[28].get();
    assert!(almost_equal(v.x(), 10.7000000, v.y(), 83.5000000));
    assert_eq!(v.get_incident_edge()?.0, 91);
    Ok(())
}

#[test]
fn sample_primary_050() -> Result<(), BvError> {
    let output = {
        let input = r#"0
4
2134582590 2134582590 2134582590 2141031480
2134582590 2134582590 2141031480 2134582590
2141031480 2134582590 2141031480 2141031480
2134582590 2141031480 2141031480 2141031480
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
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
    assert_eq!(output.vertices().len(), 5);
    assert_eq!(output.edges().len(), 24);
    let v = output.vertices()[0].get();
    assert!(almost_equal(
        v.x(),
        2134582590.0000000,
        v.y(),
        2134582590.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(
        v.x(),
        2134582590.0000000,
        v.y(),
        2141031480.0000002
    ));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(
        v.x(),
        2141031480.0000002,
        v.y(),
        2134582590.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(
        v.x(),
        2137807035.0000005,
        v.y(),
        2137807035.0000002
    ));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(
        v.x(),
        2141031480.0000002,
        v.y(),
        2141031480.0000002
    ));
    assert_eq!(v.get_incident_edge()?.0, 23);
    Ok(())
}

#[test]
fn sample_primary_051() -> Result<(), BvError> {
    let output = {
        let input = r#"0
4
-1073741800 -1073741800 -687194752 -1159641144
-1073741800 -1073741800 -408021884 -923417948
-1073741800 -1073741800 -343597376 -2061584256
-2147483600 -837518604 -1073741800 -1073741800
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
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
    assert_eq!(cell.is_degenerate(), true);
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
    let cell = output.cells()[8].get();
    assert_eq!(cell.id().0, 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 10);
    assert_eq!(output.edges().len(), 34);
    let v = output.vertices()[0].get();
    assert!(almost_equal(
        v.x(),
        -1073741800.0000001,
        v.y(),
        -1073741800.0000001
    ));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(
        v.x(),
        -667952126.2601250,
        v.y(),
        -1073049328.1705620
    ));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(
        v.x(),
        -719312873.5433259,
        v.y(),
        -1304172690.9449666
    ));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(
        v.x(),
        -320834049.8400000,
        v.y(),
        -1309535499.2800000
    ));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(
        v.x(),
        -2777223954.8168521,
        v.y(),
        -3699974762.2584195
    ));
    assert_eq!(v.get_incident_edge()?.0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(
        v.x(),
        -267406425.8674650,
        v.y(),
        -1372677236.7020869
    ));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(
        v.x(),
        -2986306750.1121492,
        v.y(),
        -4014891184.6915884
    ));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(
        v.x(),
        492862972.8605770,
        v.y(),
        -1443330954.6682692
    ));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(
        v.x(),
        -1082417952.1604242,
        v.y(),
        2063193210.9961636
    ));
    assert_eq!(v.get_incident_edge()?.0, 31);
    let v = output.vertices()[9].get();
    assert!(almost_equal(
        v.x(),
        -1023286812.6685963,
        v.y(),
        4272466792.9609261
    ));
    assert_eq!(v.get_incident_edge()?.0, 33);
    Ok(())
}

#[test]
fn sample_primary_svn_problem_12067() -> Result<(), BvError> {
    let output = {
        let input = r#"6
-10 -20
10 -20
5 0
10 20
-10 20
-5 0
0
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
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
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -47.5000000, v.y(), -0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), -11.8750000));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), 11.8750000));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 47.5000000, v.y(), -0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 17);
    Ok(())
}

#[test]
fn sample_primary_052() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
0 0 0 -1
0 0 1 1
0 0 1 -1
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
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
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 20);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 0.4142136, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 0.5000000, v.y(), -1.5000000));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 2.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge()?.0, 19);
    Ok(())
}

#[test]
fn sample_primary_053() -> Result<(), BvError> {
    let output = {
        let input = r#"2
1 0
0 10
1
-2 10 -1 0
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
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
    assert_eq!(output.vertices().len(), 3);
    assert_eq!(output.edges().len(), 14);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 10.1000000));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 0.1000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 5.8224010, v.y(), 5.5322401));
    assert_eq!(v.get_incident_edge()?.0, 13);
    Ok(())
}

#[test]
fn sample_primary_054() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
-1073741800 -1073741800 -687194752 -1159641144
-1073741800 -1073741800 -408021884 -923417948
-1073741800 -1073741800 -343597376 -2061584256
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
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
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.id().0, 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 6);
    assert_eq!(output.edges().len(), 24);
    let v = output.vertices()[0].get();
    assert!(almost_equal(
        v.x(),
        -1073741800.0000001,
        v.y(),
        -1073741800.0000001
    ));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(
        v.x(),
        -667952126.2601250,
        v.y(),
        -1073049328.1705620
    ));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(
        v.x(),
        -719312873.5433259,
        v.y(),
        -1304172690.9449666
    ));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(
        v.x(),
        -320834049.8400000,
        v.y(),
        -1309535499.2800000
    ));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(
        v.x(),
        -267406425.8674650,
        v.y(),
        -1372677236.7020869
    ));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(
        v.x(),
        492862972.8605770,
        v.y(),
        -1443330954.6682692
    ));
    assert_eq!(v.get_incident_edge()?.0, 23);
    Ok(())
}

#[test]
fn sample_primary_055() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
-3 -4 -1 -2
-2 4 4 -1
0 -2 0 1
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
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
    assert!(almost_equal(v.x(), -7.6428571, v.y(), 0.6428571));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -4.9056829, v.y(), 0.5131805));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -0.5000000, v.y(), -2.0000000));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -3.4142136, v.y(), 0.4142136));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -3.7767695, v.y(), 0.5643891));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -2.8467222, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -0.5000000, v.y(), -2.5000000));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 0.6244999, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 2.0296248, v.y(), -2.0000000));
    assert_eq!(v.get_incident_edge()?.0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 2.3653846, v.y(), -2.9615385));
    assert_eq!(v.get_incident_edge()?.0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 3.5000000, v.y(), -10.5000000));
    assert_eq!(v.get_incident_edge()?.0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 4.7000000, v.y(), -12.3000000));
    assert_eq!(v.get_incident_edge()?.0, 39);
    Ok(())
}

#[test]
fn sample_primary_056() -> Result<(), BvError> {
    let output = {
        let input = r#"1
9 72
2
-2 0 -1 -50
-1 -50 0 -99
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
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
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -50.0000000));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 211.2234727, v.y(), 4.2644695));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 600.0484487, v.y(), -37.8563686));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 1173.8382353, v.y(), -75.0441176));
    assert_eq!(v.get_incident_edge()?.0, 17);
    Ok(())
}

#[test]
fn sample_primary_057() -> Result<(), BvError> {
    let output = {
        let input = r#"5
-4 -4
-6 0
-2 2
0 4
-10 2
0
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 5);
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
    assert_eq!(output.vertices().len(), 5);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -11.0000000, v.y(), -5.0000000));
    assert_eq!(v.get_incident_edge()?.0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -6.0000000, v.y(), 5.0000000));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -3.0000000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -6.0000000, v.y(), 8.0000000));
    assert_eq!(v.get_incident_edge()?.0, 15);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 6.0000000, v.y(), -4.0000000));
    assert_eq!(v.get_incident_edge()?.0, 17);
    Ok(())
}

//#[ignore]
#[test]
fn sample_primary_058() -> Result<(), BvError> {
    let output = {
        let input = r#"0
8
644245092 214748364 858993456 214748364
-644245092 -214748364 0 644245092
-858993456 -214748364 -429496728 -644245092
0 644245092 -214748364 644245092
644245092 644245092 0 644245092
858993456 858993456 644245092 644245092
644245092 214748364 644245092 644245092
858993456 214748364 644245092 214748364
"#;
        let mut vb = VB::Builder::<I1, F1>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 16);
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
    assert_eq!(cell.is_degenerate(), true);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
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
    assert_eq!(output.vertices().len(), 19);
    assert_eq!(output.edges().len(), 66);
    let v = output.vertices()[0].get();
    assert!(almost_equal(
        v.x(),
        -750673816.9249936,
        v.y(),
        -134926820.3062548
    ));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(
        v.x(),
        -752564731.0750064,
        v.y(),
        -108319639.0750065
    ));
    assert_eq!(v.get_incident_edge()?.0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(
        v.x(),
        -502755409.6872513,
        v.y(),
        -320865625.7345614
    ));
    assert_eq!(v.get_incident_edge()?.0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(
        v.x(),
        -1139240071.0200000,
        v.y(),
        666525234.7650000
    ));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(
        v.x(),
        -214748364.0000000,
        v.y(),
        536870910.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), 644245092.0000000));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(
        v.x(),
        -75772522.2181284,
        v.y(),
        -290520886.2181284
    ));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(
        v.x(),
        214748364.0000000,
        v.y(),
        214748364.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 35);
    let v = output.vertices()[8].get();
    assert!(almost_equal(
        v.x(),
        644245091.9999999,
        v.y(),
        214748364.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 39);
    let v = output.vertices()[9].get();
    assert!(almost_equal(
        v.x(),
        644245092.0000000,
        v.y(),
        644245092.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 43);
    let v = output.vertices()[10].get();
    assert!(almost_equal(
        v.x(),
        167307337.9062425,
        v.y(),
        -289664808.8828030
    ));
    assert_eq!(v.get_incident_edge()?.0, 49);
    let v = output.vertices()[11].get();
    assert!(almost_equal(
        v.x(),
        340545043.1337659,
        v.y(),
        1377441868.8662341
    ));
    assert_eq!(v.get_incident_edge()?.0, 51);
    let v = output.vertices()[12].get();
    assert!(almost_equal(
        v.x(),
        858993456.0000001,
        v.y(),
        429496728.0000001
    ));
    assert_eq!(v.get_incident_edge()?.0, 53);
    let v = output.vertices()[13].get();
    assert!(almost_equal(
        v.x(),
        959596601.5758756,
        v.y(),
        513622219.8188434
    ));
    assert_eq!(v.get_incident_edge()?.0, 55);
    let v = output.vertices()[14].get();
    assert!(almost_equal(
        v.x(),
        1181116002.0000000,
        v.y(),
        536870910.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 57);
    let v = output.vertices()[15].get();
    assert!(almost_equal(
        v.x(),
        644245092.0000000,
        v.y(),
        -885837001.5000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 59);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), 2469606186.0000000));
    assert_eq!(v.get_incident_edge()?.0, 61);
    let v = output.vertices()[17].get();
    assert!(almost_equal(
        v.x(),
        858993456.0000001,
        v.y(),
        -1181116002.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 63);
    let v = output.vertices()[18].get();
    assert!(almost_equal(
        v.x(),
        -214748364.0000000,
        v.y(),
        3435973824.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 65);
    Ok(())
}
