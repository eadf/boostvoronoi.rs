use boostvoronoi::builder as VB;
use boostvoronoi::diagram as VD;
use boostvoronoi::file_reader as FR;
use boostvoronoi::BvError;
use std::io::{BufReader, Cursor};

mod common;
use common::almost_equal;

type I = i64;
type F = f64;

#[test]
fn pps_problem_1() -> Result<(), BvError> {
    let output: VD::Diagram<F> = {
        let input = r#"0
3
-5138 -5149 -5038 -5142
-5042 -5069 -5165 -5162
-5011 -5195 -5404 -5134
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 40);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -5161.8029011, v.y(), -5166.2284211));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -5182.8168392, v.y(), -5138.4357933));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -5138.2247187, v.y(), -5145.7897328));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -5142.4025686, v.y(), -5161.3427932));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -5137.0815829, v.y(), -5162.1202444));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -5036.2947365, v.y(), -5166.3609069));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -5040.3041060, v.y(), -5109.0842007));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -5015.4207571, v.y(), -5104.1531922));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -5005.3744656, v.y(), -5158.7568033));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), -5279.3601678, v.y(), -4755.0720361));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -5317.3746719, v.y(), -4575.9056733));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), -4884.0605458, v.y(), -5096.9553724));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}

#[test]
fn pps_problem_2() -> Result<(), BvError> {
    let output: VD::Diagram<F> = {
        let input = r#"0
3
-5205 -5210 -5095 -5152
-5166 -5197 -5099 -5209
-5029 -5002 -5500 -5319
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 40);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -5222.3739979, v.y(), -5177.0493144));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -5165.3404816, v.y(), -5193.3176891));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -5171.5766569, v.y(), -5228.1363343));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -5115.4849717, v.y(), -5113.1491916));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -5161.6211612, v.y(), -5292.2702116));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -5094.2137357, v.y(), -5182.2766911));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -5079.3184829, v.y(), -5181.7408082));
    assert_eq!(v.get_incident_edge().unwrap().0, 27);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -5149.3525019, v.y(), -5490.1348021));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -4943.4008205, v.y(), -5129.1836390));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), -5146.2212550, v.y(), -5822.0469693));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -5136.0921495, v.y(), -5859.6958915));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), -4775.4962049, v.y(), -5203.0616698));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}

#[test]
fn pps_problem_3() -> Result<(), BvError> {
    let output: VD::Diagram<F> = {
        let input = r#"0
3
759 -242 631 128
189 -303 843 693
-911 -920 921 853
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 40);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 52.9277420, v.y(), -213.6513486));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 553.0127242, v.y(), 101.0206181));
    assert_eq!(v.get_incident_edge().unwrap().0, 13);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 499.0591582, v.y(), -331.9254804));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 817.3065196, v.y(), 709.8710203));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 499.0515283, v.y(), -506.5880517));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 953.4824281, v.y(), 239.5614886));
    assert_eq!(v.get_incident_edge().unwrap().0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 1102.4156925, v.y(), 665.5473499));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 1669.4370715, v.y(), 150.3395133));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 1840.4339679, v.y(), 132.1176970));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 715.4384935, v.y(), -2528.5646114));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 764.4981726, v.y(), -2651.2536109));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 2277.1023701, v.y(), 92.8875946));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}

#[test]
fn pps_problem_4() -> Result<(), BvError> {
    let output: VD::Diagram<F> = {
        let input = r#"0
3
580 -833 552 -566
-671 955 604 -936
535 -110 412 -549
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert!(almost_equal(v.x(), 378.5956542, v.y(), -539.6406958));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 298.5557206, v.y(), -43.7525140));
    assert_eq!(v.get_incident_edge().unwrap().0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 480.6949930, v.y(), -568.2471165));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 480.0516611, v.y(), -573.5451442));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 474.5253602, v.y(), -599.8312189));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 560.0965540, v.y(), -835.0872528));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 727.0543920, v.y(), -853.0310155));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 1003.7223326, v.y(), -788.5646992));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 1254.1922629, v.y(), -311.5048937));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 1981.7104195, v.y(), 2743.5805314));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 3930.0577834, v.y(), -211.7467493));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 4638.9421043, v.y(), -183.0587104));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}

#[test]
fn pps_problem_5() -> Result<(), BvError> {
    let output: VD::Diagram<F> = {
        let input = r#"0
3
963 -74 -944 707
694 281 853 211
326 220 803 441
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 40);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 333.6679379, v.y(), 203.4497449));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 285.6930020, v.y(), 306.9974573));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 594.5600917, v.y(), 209.3420269));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 648.3465444, v.y(), 177.3014365));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 718.1008817, v.y(), 335.7434313));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 808.2569812, v.y(), 109.3694288));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 851.1762530, v.y(), 337.0177706));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 911.6332023, v.y(), 344.1811309));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 1042.6429352, v.y(), 120.4674487));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 256.0019816, v.y(), 1621.6246822));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 478.6761312, v.y(), 4180.8071476));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 2538.6900452, v.y(), 697.8891403));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}

#[test]
fn pps_problem_6() -> Result<(), BvError> {
    let output: VD::Diagram<F> = {
        let input = r#"0
3
415 -54 955 703
976 38 -916 -467
909 424 962 401
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 40);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 216.6564884, v.y(), 87.4867851));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 440.6431061, v.y(), -72.2923082));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 846.3323599, v.y(), 279.5919598));
    assert_eq!(v.get_incident_edge().unwrap().0, 21);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 950.0315350, v.y(), 518.5509284));
    assert_eq!(v.get_incident_edge().unwrap().0, 23);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 862.8079517, v.y(), 220.2615401));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 882.9167375, v.y(), 218.7646559));
    assert_eq!(v.get_incident_edge().unwrap().0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 927.9775252, v.y(), 217.9178660));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 1021.6423352, v.y(), 538.4366855));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 1160.1280771, v.y(), 556.6734985));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), -1348.6705305, v.y(), 2346.3052661));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), -1885.0838893, v.y(), 3163.7063734));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 22591.5995851, v.y(), 1053.4294606));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}

#[test]
fn pps_problem_7() -> Result<(), BvError> {
    let output: VD::Diagram<F> = {
        let input = r#"0
3
365 113 741 366
768 -67 601 187
-814 662 817 -285
"#;
        let mut vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;
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
    assert_eq!(output.vertices().len(), 12);
    assert_eq!(output.edges().len(), 40);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 248.7208934, v.y(), 285.8100556));
    assert_eq!(v.get_incident_edge().unwrap().0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 410.5368053, v.y(), 45.3247478));
    assert_eq!(v.get_incident_edge().unwrap().0, 9);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), 542.8345857, v.y(), 148.7573851));
    assert_eq!(v.get_incident_edge().unwrap().0, 15);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), 522.7827325, v.y(), 49.0434072));
    assert_eq!(v.get_incident_edge().unwrap().0, 17);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 660.4201120, v.y(), 226.0675540));
    assert_eq!(v.get_incident_edge().unwrap().0, 19);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 695.7078595, v.y(), -114.5306593));
    assert_eq!(v.get_incident_edge().unwrap().0, 25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 893.4648396, v.y(), -153.3060682));
    assert_eq!(v.get_incident_edge().unwrap().0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 862.5002096, v.y(), 185.4305185));
    assert_eq!(v.get_incident_edge().unwrap().0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 220.4336065, v.y(), 1139.6480788));
    assert_eq!(v.get_incident_edge().unwrap().0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 1133.2038256, v.y(), 173.1143263));
    assert_eq!(v.get_incident_edge().unwrap().0, 35);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 384.6363162, v.y(), 2726.3884179));
    assert_eq!(v.get_incident_edge().unwrap().0, 37);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 2811.2146957, v.y(), 277.7477986));
    assert_eq!(v.get_incident_edge().unwrap().0, 39);
    Ok(())
}
