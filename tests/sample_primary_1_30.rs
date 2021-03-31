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
fn sample_primary_001() -> Result<(), BvError> {
    let output = {
        let input = r#"1
0 0
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 1);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), true);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 0);
    Ok(())
}

#[test]
fn sample_primary_002() -> Result<(), BvError> {
    let output = {
        let input = r#"2
0 0
1 0
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 2);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 2);
    Ok(())
}

#[test]
fn sample_primary_003() -> Result<(), BvError> {
    let output = {
        let input = r#"2
0 0
0 1
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 2);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 2);
    Ok(())
}

#[test]
fn sample_primary_004() -> Result<(), BvError> {
    let output = {
        let input = r#"2
0 0
1 1
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 2);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 2);
    Ok(())
}

#[test]
fn sample_primary_005() -> Result<(), BvError> {
    let output = {
        let input = r#"10
0 0
0 1
0 2
0 3
0 4
0 -1
0 -2
0 -3
0 -4
0 -5
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 10);
    assert_eq!(
        format!("{:?}", output.cells()[0].get()),
        "(id:0 ii:9 ie:0 col:0)"
    );
    assert_eq!(
        format!("{:?}", output.cells()[1].get()),
        "(id:1 ii:8 ie:2 col:0)"
    );
    assert_eq!(
        format!("{:?}", output.cells()[2].get()),
        "(id:2 ii:7 ie:4 col:0)"
    );
    assert_eq!(
        format!("{:?}", output.cells()[3].get()),
        "(id:3 ii:6 ie:6 col:0)"
    );
    assert_eq!(
        format!("{:?}", output.cells()[4].get()),
        "(id:4 ii:5 ie:8 col:0)"
    );
    assert_eq!(
        format!("{:?}", output.cells()[5].get()),
        "(id:5 ii:0 ie:10 col:0)"
    );
    assert_eq!(
        format!("{:?}", output.cells()[6].get()),
        "(id:6 ii:1 ie:12 col:0)"
    );
    assert_eq!(
        format!("{:?}", output.cells()[7].get()),
        "(id:7 ii:2 ie:14 col:0)"
    );
    assert_eq!(
        format!("{:?}", output.cells()[8].get()),
        "(id:8 ii:3 ie:16 col:0)"
    );
    assert_eq!(
        format!("{:?}", output.cells()[9].get()),
        "(id:9 ii:4 ie:17 col:0)"
    );
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 18);
    Ok(())
}
#[test]
fn sample_primary_006() -> Result<(), BvError> {
    let output = {
        let input = r#"10
0 0
1 0
2 0
3 0
4 0
5 0
-1 0
-2 0
-3 0
-4 0
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 10);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 18);
    Ok(())
}

#[test]
fn sample_primary_007() -> Result<(), BvError> {
    let output = {
        let input = r#"11
0 0
1 1
2 2
3 3
4 4
5 5
6 6
7 7
8 8
9 9
10 10
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 11);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[10].get();
    assert_eq!(cell.get_id(), 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 20);
    Ok(())
}

#[test]
fn sample_primary_008() -> Result<(), BvError> {
    let output = {
        let input = r#"10
-46 -37
-40 -30
-34 -23
-28 -16
-22 -9
-16 -2
-10 5
-4 12
2 19
8 26
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 10);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 18);
    Ok(())
}

#[test]
fn sample_primary_009() -> Result<(), BvError> {
    let output = {
        let input = r#"10
33333 11111
66666 0
99999 -11111
133332 -22222
166665 -33333
199998 -44444
233331 -55555
266664 -66666
299997 -77777
333330 -88888
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 10);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 18);
    Ok(())
}

#[test]
fn sample_primary_010() -> Result<(), BvError> {
    let output = {
        let input = r#"3
0 0
2005 2005
10025 10025
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 4);
    Ok(())
}

#[test]
fn sample_primary_011() -> Result<(), BvError> {
    let output = {
        let input = r#"3
0 0
0 4
1 1
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 1);
    assert_eq!(output.edges().len(), 6);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 5);
    Ok(())
}

#[test]
fn sample_primary_012() -> Result<(), BvError> {
    let output = {
        let input = r#"4
0 0
0 1
1 0
1 1
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 4);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 1);
    assert_eq!(output.edges().len(), 8);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 0.5000000, v.y(), 0.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    Ok(())
}

#[test]
fn sample_primary_013() -> Result<(), BvError> {
    let output = {
        let input = r#"13
0 5
0 -5
-4 -3
4 -3
4 3
-4 3
3 -4
-3 4
-3 -4
3 4
-5 0
5 0
0 0
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 13);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[10].get();
    assert_eq!(cell.get_id(), 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[11].get();
    assert_eq!(cell.get_id(), 11);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[12].get();
    assert_eq!(cell.get_id(), 12);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 48);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -2.5000000, v.y(), -0.8333333));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -2.5000000, v.y(), 0.8333333));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -1.7857143, v.y(), -1.7857143));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -1.7857143, v.y(), 1.7857143));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -0.8333333, v.y(), -2.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -0.8333333, v.y(), 2.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 0.8333333, v.y(), -2.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 0.8333333, v.y(), 2.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 1.7857143, v.y(), -1.7857143));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 1.7857143, v.y(), 1.7857143));
    assert_eq!(v.get_incident_edge().unwrap().0, 41);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 2.5000000, v.y(), -0.8333333));
    assert_eq!(v.get_incident_edge().unwrap().0, 45);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 2.5000000, v.y(), 0.8333333));
    assert_eq!(v.get_incident_edge().unwrap().0, 47);
    Ok(())
}

#[test]
fn sample_primary_014() -> Result<(), BvError> {
    let output = {
        let input = r#"12
0 5
0 -5
-4 -3
4 -3
4 3
-4 3
3 -4
-3 4
-3 -4
3 4
-5 0
5 0
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 12);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[10].get();
    assert_eq!(cell.get_id(), 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[11].get();
    assert_eq!(cell.get_id(), 11);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 1);
    assert_eq!(output.edges().len(), 24);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), -0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    Ok(())
}

#[test]
fn sample_primary_015() -> Result<(), BvError> {
    let output = {
        let input = r#"4
4 3
4 8
9 2
9 9
0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 4);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 1);
    assert_eq!(output.edges().len(), 8);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 7.1000000, v.y(), 5.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    Ok(())
}

#[test]
fn sample_primary_017() -> Result<(), BvError> {
    let output = {
        let input = r#"0
1
0 0 1 1
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 3);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 0);
    assert_eq!(output.edges().len(), 4);
    Ok(())
}

#[test]
fn sample_primary_018() -> Result<(), BvError> {
    let output = {
        let input = r#"2
3 1
1 3
1
0 0 4 4
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 5);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 16);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -2.5000000, v.y(), 2.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 1.5000000, v.y(), 6.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 2.5000000, v.y(), -2.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 6.5000000, v.y(), 1.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    Ok(())
}

#[test]
fn sample_primary_019() -> Result<(), BvError> {
    let output = {
        let input = r#"2
3 2
2 3
1
4 0 0 4
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 5);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 3);
    assert_eq!(output.edges().len(), 14);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 2.5000000, v.y(), 2.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 2.5000000, v.y(), 6.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 6.5000000, v.y(), 2.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    Ok(())
}

#[test]
fn sample_primary_020() -> Result<(), BvError> {
    let output = {
        let input = r#"3
-2 -2
-2 4
-2 10
1
0 0 0 8
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 6);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -2.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -3.2500000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -3.2500000, v.y(), 7.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -2.0000000, v.y(), 8.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    Ok(())
}

#[test]
fn sample_primary_021() -> Result<(), BvError> {
    let output = {
        let input = r#"1
-1 1
1
1 0 1 2
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 4);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 2);
    assert_eq!(output.edges().len(), 10);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -0.2500000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -0.2500000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    Ok(())
}

#[test]
fn sample_primary_022() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
0 0 4 0
4 0 0 4
0 4 4 4
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 7);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 6);
    assert_eq!(output.edges().len(), 24);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -2.0000000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 1.6568542));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), 2.3431458));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 6.0000000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    Ok(())
}

#[test]
fn sample_primary_023() -> Result<(), BvError> {
    let output = {
        let input = r#"0
4
0 0 4 0
4 0 4 4
4 4 0 4
0 4 0 0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 8);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 5);
    assert_eq!(output.edges().len(), 24);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), -0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 2.0000000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    Ok(())
}

#[test]
fn sample_primary_024() -> Result<(), BvError> {
    let output = {
        let input = r#"0
2
0 0 4 0
2 2 2 4
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 6);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -3.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 7.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    Ok(())
}

#[test]
fn sample_primary_025() -> Result<(), BvError> {
    let output = {
        let input = r#"1
5 6
2
0 0 4 0
2 2 2 4
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 7);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 5);
    assert_eq!(output.edges().len(), 22);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -3.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 4.1666667, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 5.1020134, v.y(), 2.8996644));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    Ok(())
}

#[test]
fn sample_primary_026() -> Result<(), BvError> {
    let output = {
        let input = r#"2
0 0
1 6
2
-4 5 5 -1
3 -11 13 -1
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 8);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 9);
    assert_eq!(output.edges().len(), 32);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -9.8571429, v.y(), -3.7857143));
    assert_eq!(v.get_incident_edge().unwrap().0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -2.0000000, v.y(), 8.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -18.9827586, v.y(), -11.0862069));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -1.6428571, v.y(), -6.3571429));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 1.5560757, v.y(), -5.0163094));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 2.3766275, v.y(), -4.9350587));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 9.0000000, v.y(), 3.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 9.0000000, v.y(), 5.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 9.0167098, v.y(), 5.9572167));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    Ok(())
}

#[test]
fn sample_primary_027() -> Result<(), BvError> {
    let output = {
        let input = r#"2
0 0
1 6
8
-6 5 2 -7
3 -11 13 -1
-4 5 5 -1
4 4 11 4
4 4 8 10
11 4 8 10
8 10 5 13
8 10 11 13
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 21);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::SinglePoint);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[10].get();
    assert_eq!(cell.get_id(), 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[11].get();
    assert_eq!(cell.get_id(), 11);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[12].get();
    assert_eq!(cell.get_id(), 12);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[13].get();
    assert_eq!(cell.get_id(), 13);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[14].get();
    assert_eq!(cell.get_id(), 14);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[15].get();
    assert_eq!(cell.get_id(), 15);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), true);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[16].get();
    assert_eq!(cell.get_id(), 16);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[17].get();
    assert_eq!(cell.get_id(), 17);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[18].get();
    assert_eq!(cell.get_id(), 18);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[19].get();
    assert_eq!(cell.get_id(), 19);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[20].get();
    assert_eq!(cell.get_id(), 20);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 30);
    assert_eq!(output.edges().len(), 98);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -5.0000000, v.y(), 5.6666667));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -4.4800000, v.y(), 4.2800000));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -1.3263027, v.y(), 1.1263027));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -2.0000000, v.y(), 8.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -3.1000000, v.y(), -10.4000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 1.3000000, v.y(), -9.3000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 1.7500000, v.y(), 3.8750000));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 2.5000000, v.y(), 5.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 1.7263027, v.y(), -1.9263027));
    assert_eq!(v.get_incident_edge().unwrap().0, 41);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -2.9838710, v.y(), 12.9193548));
    assert_eq!(v.get_incident_edge().unwrap().0, 49);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), -5.0000000, v.y(), 15.1875000));
    assert_eq!(v.get_incident_edge().unwrap().0, 51);
    let v = output.vertices()[12].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), 2.0319584));
    assert_eq!(v.get_incident_edge().unwrap().0, 53);
    let v = output.vertices()[13].get();
    assert!(almost_equal(v.x(), 2.0454545, v.y(), 10.0454545));
    assert_eq!(v.get_incident_edge().unwrap().0, 55);
    let v = output.vertices()[14].get();
    assert!(almost_equal(v.x(), 3.3200000, v.y(), -3.5200000));
    assert_eq!(v.get_incident_edge().unwrap().0, 57);
    let v = output.vertices()[15].get();
    assert!(almost_equal(v.x(), 2.8890100, v.y(), 9.4939123));
    assert_eq!(v.get_incident_edge().unwrap().0, 59);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), 4.4594117, v.y(), -5.3603922));
    assert_eq!(v.get_incident_edge().unwrap().0, 61);
    let v = output.vertices()[17].get();
    assert!(almost_equal(v.x(), 8.0000000, v.y(), 10.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 67);
    let v = output.vertices()[18].get();
    assert!(almost_equal(v.x(), 4.5461461, v.y(), -4.4844594));
    assert_eq!(v.get_incident_edge().unwrap().0, 69);
    let v = output.vertices()[19].get();
    assert!(almost_equal(v.x(), 6.5138782, v.y(), 1.2708173));
    assert_eq!(v.get_incident_edge().unwrap().0, 71);
    let v = output.vertices()[20].get();
    assert!(almost_equal(v.x(), 7.7514493, v.y(), 6.0077147));
    assert_eq!(v.get_incident_edge().unwrap().0, 73);
    let v = output.vertices()[21].get();
    assert!(almost_equal(v.x(), 11.0000000, v.y(), 4.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 77);
    let v = output.vertices()[22].get();
    assert!(almost_equal(v.x(), 8.0000000, v.y(), 16.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 81);
    let v = output.vertices()[23].get();
    assert!(almost_equal(v.x(), 8.6854142, v.y(), 0.1417722));
    assert_eq!(v.get_incident_edge().unwrap().0, 83);
    let v = output.vertices()[24].get();
    assert!(almost_equal(v.x(), 10.9289322, v.y(), 1.0710678));
    assert_eq!(v.get_incident_edge().unwrap().0, 87);
    let v = output.vertices()[25].get();
    assert!(almost_equal(v.x(), 11.0000000, v.y(), 1.1000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 89);
    let v = output.vertices()[26].get();
    assert!(almost_equal(v.x(), -107.1000000, v.y(), -62.4000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 91);
    let v = output.vertices()[27].get();
    assert!(almost_equal(v.x(), 15.1622777, v.y(), 8.8377223));
    assert_eq!(v.get_incident_edge().unwrap().0, 93);
    let v = output.vertices()[28].get();
    assert!(almost_equal(v.x(), 20.0000000, v.y(), 8.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 95);
    let v = output.vertices()[29].get();
    assert!(almost_equal(v.x(), 29.5000000, v.y(), 8.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 97);
    Ok(())
}

#[test]
fn sample_primary_028() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
0 0 4 2
4 2 4 -2
4 -2 0 0
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 6);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), -2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 2.7639320, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 4.0000000, v.y(), 2.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    Ok(())
}

#[test]
fn sample_primary_029() -> Result<(), BvError> {
    let output = {
        let input = r#"0
8
0 0 0 1
0 0 1 0
0 0 -1 0
0 0 0 -1
0 0 1 1
0 0 1 -1
0 0 -1 1
0 0 -1 -1
"#;
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 17);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), true);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[10].get();
    assert_eq!(cell.get_id(), 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[11].get();
    assert_eq!(cell.get_id(), 11);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[12].get();
    assert_eq!(cell.get_id(), 12);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[13].get();
    assert_eq!(cell.get_id(), 13);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[14].get();
    assert_eq!(cell.get_id(), 14);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[15].get();
    assert_eq!(cell.get_id(), 15);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[16].get();
    assert_eq!(cell.get_id(), 16);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 17);
    assert_eq!(output.edges().len(), 64);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -1.5000000, v.y(), -0.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -1.5000000, v.y(), 0.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -0.4142136));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 0.4142136));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -0.4142136, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -0.4142136, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -0.5000000, v.y(), -1.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -0.5000000, v.y(), 1.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 41);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 0.4142136, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 43);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 0.4142136, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 45);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 0.5000000, v.y(), -1.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 53);
    let v = output.vertices()[12].get();
    assert!(almost_equal(v.x(), 0.5000000, v.y(), 1.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 55);
    let v = output.vertices()[13].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), -0.4142136));
    assert_eq!(v.get_incident_edge().unwrap().0, 57);
    let v = output.vertices()[14].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), 0.4142136));
    assert_eq!(v.get_incident_edge().unwrap().0, 59);
    let v = output.vertices()[15].get();
    assert!(almost_equal(v.x(), 1.5000000, v.y(), 0.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 61);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), 1.5000000, v.y(), -0.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 63);
    Ok(())
}

//#[ignore]
#[test]
fn sample_primary_030() -> Result<(), BvError> {
    let output = {
        let input = r#"0
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
        let mut vb = VB::Builder::<I1, F1>::new();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I1, _>(br)?;
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    assert_eq!(output.cells().len(), 36);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[9].get();
    assert_eq!(cell.get_id(), 9);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[10].get();
    assert_eq!(cell.get_id(), 10);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[11].get();
    assert_eq!(cell.get_id(), 11);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[12].get();
    assert_eq!(cell.get_id(), 12);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[13].get();
    assert_eq!(cell.get_id(), 13);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[14].get();
    assert_eq!(cell.get_id(), 14);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[15].get();
    assert_eq!(cell.get_id(), 15);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[16].get();
    assert_eq!(cell.get_id(), 16);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[17].get();
    assert_eq!(cell.get_id(), 17);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[18].get();
    assert_eq!(cell.get_id(), 18);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[19].get();
    assert_eq!(cell.get_id(), 19);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[20].get();
    assert_eq!(cell.get_id(), 20);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[21].get();
    assert_eq!(cell.get_id(), 21);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[22].get();
    assert_eq!(cell.get_id(), 22);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[23].get();
    assert_eq!(cell.get_id(), 23);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[24].get();
    assert_eq!(cell.get_id(), 24);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[25].get();
    assert_eq!(cell.get_id(), 25);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[26].get();
    assert_eq!(cell.get_id(), 26);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[27].get();
    assert_eq!(cell.get_id(), 27);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[28].get();
    assert_eq!(cell.get_id(), 28);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[29].get();
    assert_eq!(cell.get_id(), 29);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[30].get();
    assert_eq!(cell.get_id(), 30);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[31].get();
    assert_eq!(cell.get_id(), 31);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[32].get();
    assert_eq!(cell.get_id(), 32);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[33].get();
    assert_eq!(cell.get_id(), 33);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[34].get();
    assert_eq!(cell.get_id(), 34);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[35].get();
    assert_eq!(cell.get_id(), 35);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 29);
    assert_eq!(output.edges().len(), 128);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -11.0000000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -12.5000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -11.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -10.4285714, v.y(), -1.5714286));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -10.4285714, v.y(), 1.5714286));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -1.5714286, v.y(), -10.4285714));
    assert_eq!(v.get_incident_edge().unwrap().0, 47);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -1.5714286, v.y(), 10.4285714));
    assert_eq!(v.get_incident_edge().unwrap().0, 49);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -11.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 51);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 11.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 53);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), -11.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 61);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), 11.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 65);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 1.5714286, v.y(), -10.4285714));
    assert_eq!(v.get_incident_edge().unwrap().0, 71);
    let v = output.vertices()[12].get();
    assert!(almost_equal(v.x(), 1.5714286, v.y(), 10.4285714));
    assert_eq!(v.get_incident_edge().unwrap().0, 73);
    let v = output.vertices()[13].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), -12.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 75);
    let v = output.vertices()[14].get();
    assert!(almost_equal(v.x(), 0.0000000, v.y(), 12.5000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 77);
    let v = output.vertices()[15].get();
    assert!(almost_equal(v.x(), -3.0000000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 79);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), -3.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 81);
    let v = output.vertices()[17].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), -3.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 83);
    let v = output.vertices()[18].get();
    assert!(almost_equal(v.x(), -1.0000000, v.y(), 3.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 85);
    let v = output.vertices()[19].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), -3.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 91);
    let v = output.vertices()[20].get();
    assert!(almost_equal(v.x(), 1.0000000, v.y(), 3.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 93);
    let v = output.vertices()[21].get();
    assert!(almost_equal(v.x(), 3.0000000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 107);
    let v = output.vertices()[22].get();
    assert!(almost_equal(v.x(), -0.0000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 109);
    let v = output.vertices()[23].get();
    assert!(almost_equal(v.x(), 3.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 113);
    let v = output.vertices()[24].get();
    assert!(almost_equal(v.x(), 10.4285714, v.y(), -1.5714286));
    assert_eq!(v.get_incident_edge().unwrap().0, 119);
    let v = output.vertices()[25].get();
    assert!(almost_equal(v.x(), 10.4285714, v.y(), 1.5714286));
    assert_eq!(v.get_incident_edge().unwrap().0, 121);
    let v = output.vertices()[26].get();
    assert!(almost_equal(v.x(), 11.0000000, v.y(), -1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 123);
    let v = output.vertices()[27].get();
    assert!(almost_equal(v.x(), 11.0000000, v.y(), 1.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 125);
    let v = output.vertices()[28].get();
    assert!(almost_equal(v.x(), 12.5000000, v.y(), 0.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 127);
    Ok(())
}
