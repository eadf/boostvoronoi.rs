use boostvoronoi::builder as VB;
use boostvoronoi::diagram as VD;
use boostvoronoi::file_reader as FR;
use boostvoronoi::BvError;
use std::io::{BufReader, Cursor};

mod common;
use common::almost_equal;

type I = i32;
type F = f64;

#[test]
fn sample_primary_problem_1() -> Result<(), BvError> {
    let output = {
        let input = r#"0
2
-5152 -5035 -5402 -5487
-5284 -5195 -5396 -5232
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 6);
    assert_eq!(output.edges().len(), 22);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -5642.8956060, v.y(), -5353.7612799));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -5374.6238114, v.y(), -5296.7063005));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -5277.1134478, v.y(), -5215.8457796));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -5328.9045467, v.y(), -5059.0727235));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -5515.6633321, v.y(), -4833.8587765));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -5535.5660095, v.y(), -4809.5299172));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);

    Ok(())
}

#[test]
fn sample_primary_problem_2() -> Result<(), BvError> {
    let output = {
        let input = r#"0
2
-5093 -5402 -5362 -5008
-5200 -5273 -5273 -5265
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 6);
    assert_eq!(output.edges().len(), 22);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -5789.7479672, v.y(), -5300.0411248));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -5268.2867774, v.y(), -5221.9918434));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -5198.9683124, v.y(), -5263.5858507));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -5204.8715852, v.y(), -5317.4532150));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -5295.2556381, v.y(), -5468.0826977));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -5388.8961165, v.y(), -5604.0204450));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    Ok(())
}

#[test]
fn sample_primary_problem_3() -> Result<(), BvError> {
    let output = {
        let input = r#"0
2
-5357 -5417 -5111 -5027
-5330 -5287 -5312 -5283
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 6);
    assert_eq!(output.edges().len(), 22);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -5517.2636364, v.y(), -5315.9106294));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -5324.0788056, v.y(), -5313.6453749));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -5307.7450214, v.y(), -5302.1474037));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -5336.5219414, v.y(), -5172.6512636));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -5364.1245390, v.y(), -5133.4395747));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -6161.5545455, v.y(), -4364.3425175));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    Ok(())
}

#[test]
fn sample_primary_problem_5() -> Result<(), BvError> {
    let output = {
        let input = r#"0
2
-5279 -5300 -5286 -5286
-5065 -5490 -5191 -5238
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -5238.6008403, v.y(), -5261.8004202));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -5238.4000000, v.y(), -5262.2000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -5231.4000000, v.y(), -5276.2000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -5409.1008403, v.y(), -5662.0504202));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);

    Ok(())
}

#[test]
fn sample_primary_problem_6() -> Result<(), BvError> {
    let output = {
        let input = r#"0
2
-5498 -5081 -5481 -5122
-5229 -5308 -5348 -5021
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -5423.3765690, v.y(), -5050.0585774));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -5422.6116751, v.y(), -5051.9365482));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -5406.3883249, v.y(), -5091.0634518));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -5509.4811715, v.y(), -5424.2970711));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    Ok(())
}

#[test]
fn sample_primary_problem_7() -> Result<(), BvError> {
    let output = {
        let input = r#"0
2
-5465 -5344 -5467 -5340
-5153 -5406 -5241 -5230
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -5354.5871886, v.y(), -5283.7935943));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -5353.4000000, v.y(), -5286.2000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -5352.6000000, v.y(), -5287.8000000));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -5333.0498221, v.y(), -5496.0249110));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    Ok(())
}

#[test]
fn sample_primary_problem_9() -> Result<(), BvError> {
    let output = {
        let input = r#"0
2
129 455 264 104
192 20 32 436
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 78.8325959, v.y(), 454.0125369));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 83.5670103, v.y(), 437.5257732));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 218.5670103, v.y(), 86.5257732));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 250.6725664, v.y(), 42.5663717));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    Ok(())
}

#[test]
fn sample_primary_problem_10() -> Result<(), BvError> {
    let output = {
        let input = r#"0
2
356 -306 -52 136
-297 -62 -69 -309
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 4);
    assert_eq!(output.edges().len(), 18);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -181.0157346, v.y(), 45.0623988));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -167.4840256, v.y(), 29.3993610));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 46.4840256, v.y(), -202.3993610));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 144.8651322, v.y(), -500.8937242));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    Ok(())
}
