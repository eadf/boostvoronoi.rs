use boostvoronoi::builder::{to_segments, Builder};
use boostvoronoi::diagram as VD;
use boostvoronoi::BvError;

type I1 = i64;
type F1 = f64;

#[allow(dead_code)]
fn almost_equal(x1: F1, x2: F1, y1: F1, y2: F1) -> bool {
    let delta = 0.00001;
    assert!(F1::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F1::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F1::abs(x1 - x2) < delta) && (F1::abs(y1 - y2) < delta)
}

//#[ignore]
#[test]
fn segment_5_1() -> Result<(), BvError> {
    let output = {
        let segments: [[I1; 4]; 4] = [
            [-251891, 127298, -237685, 182389],
            [-237685, 182389, -182594, 168183],
            [-182594, 168183, -196800, 113093],
            [-196800, 113093, -251891, 127298],
        ];
        let segments = to_segments(&segments);

        let mut vb = Builder::<I1, F1>::default();
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
    assert_eq!(output.vertices().len(), 6);
    assert_eq!(output.edges().len(), 26);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -251891.0000000, v.y(), 127298.0000000));
    assert_eq!(v.get_incident_edge()?.0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -237685.0000000, v.y(), 182389.0000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -196800.0000000, v.y(), 113093.0000000));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -217242.6739670, v.y(), 147740.7948610));
    assert_eq!(v.get_incident_edge()?.0, 19);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -217242.3260300, v.y(), 147740.7051436));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -182594.0000000, v.y(), 168183.0000000));
    assert_eq!(v.get_incident_edge()?.0, 25);
    Ok(())
}

//#[ignore]
#[test]
fn segment_5_2() -> Result<(), BvError> {
    let _output = {
        let segments: [[I1; 4]; 26] = [
            [304929, 68078, 276145, 82335],
            [276145, 82335, 252204, 88476],
            [252204, 88476, 230711, 88894],
            [230711, 88894, 217560, 81543],
            [217560, 81543, 212501, 66078],
            [212501, 66078, 216206, 45512],
            [216206, 45512, 255989, -31265],
            [255989, -31265, 225328, -28812],
            [225328, -28812, 222262, -31265],
            [222262, -31265, 230234, -39850],
            [230234, -39850, 261509, -40464],
            [261509, -40464, 313634, -125703],
            [313634, -125703, 353886, -141605],
            [353886, -141605, 354528, -137987],
            [354528, -137987, 333789, -125284],
            [333789, -125284, 280519, -40464],
            [280519, -40464, 356560, -40464],
            [356560, -40464, 358093, -36938],
            [358093, -36938, 275613, -32492],
            [275613, -32492, 242345, 39716],
            [242345, 39716, 240608, 63785],
            [240608, 63785, 249040, 76663],
            [249040, 76663, 272668, 77342],
            [272668, 77342, 310874, 59265],
            [310874, 59265, 313497, 62603],
            [313497, 62603, 304929, 68078],
        ];

        let segments = to_segments(&segments);

        let mut vb = Builder::<I1, F1>::default();
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    Ok(())
}

#[test]
// this is the old gui test 'A_test'
fn segment_5_3() -> Result<(), BvError> {
    let output = {
        let segments: [[I1; 4]; 12] = [
            [61580, -50720, 56712, -55735],
            [56712, -55735, -148074, -55735],
            [-148074, -55735, -148480, 39809],
            [-148480, 39809, -65636, 40871],
            [-65636, 40871, -65636, 17536],
            [-65636, 17536, 14319, 17536],
            [14319, 17536, 33598, 22174],
            [33598, 22174, 42095, 33233],
            [42095, 33233, 40371, 46433],
            [40371, 46433, 140722, 55735],
            [140722, 55735, 148480, 38812],
            [148480, 38812, 61580, -50720],
        ];

        let segments = to_segments(&segments);

        let mut vb = Builder::<I1, F1>::default();
        vb.with_segments(segments.iter())?;
        vb.build()?
    };
    assert_eq!(output.cells().len(), 24);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[16].get();
    assert_eq!(cell.id().0, 16);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[17].get();
    assert_eq!(cell.id().0, 17);
    let (_source_index, _cat) = cell.source_index_2();
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
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
    assert_eq!(_cat, VD::SourceCategory::Segment);
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
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
    assert_eq!(output.vertices().len(), 35);
    assert_eq!(output.edges().len(), 116);
    let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), -148480.0000000, v.y(), 39809.0000000));
    assert_eq!(v.get_incident_edge()?.0, 5);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), -148074.0000000, v.y(), -55735.0000000));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -88673.7795089, v.y(), 17536.0000000));
    assert_eq!(v.get_incident_edge()?.0, 17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -65636.0000000, v.y(), 40871.0000000));
    assert_eq!(v.get_incident_edge()?.0, 21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), -65636.0000000, v.y(), 17536.0000000));
    assert_eq!(v.get_incident_edge()?.0, 25);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), -104242.7606461, v.y(), -3679.5735594));
    assert_eq!(v.get_incident_edge()?.0, 27);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), -102406.4802008, v.y(), -9873.0105690));
    assert_eq!(v.get_incident_edge()?.0, 29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), -65636.0000000, v.y(), -19099.5000000));
    assert_eq!(v.get_incident_edge()?.0, 31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), -42301.0000000, v.y(), 40871.0000000));
    assert_eq!(v.get_incident_edge()?.0, 33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 14319.0000000, v.y(), 17536.0000000));
    assert_eq!(v.get_incident_edge()?.0, 39);
    let v = output.vertices()[10].get();
    assert!(almost_equal(v.x(), 33598.0000000, v.y(), 22174.0000000));
    assert_eq!(v.get_incident_edge()?.0, 45);
    let v = output.vertices()[11].get();
    assert!(almost_equal(v.x(), 40371.0000000, v.y(), 46433.0000000));
    assert_eq!(v.get_incident_edge()?.0, 53);
    let v = output.vertices()[12].get();
    assert!(almost_equal(v.x(), 10812.6428286, v.y(), 47101.9253153));
    assert_eq!(v.get_incident_edge()?.0, 55);
    let v = output.vertices()[13].get();
    assert!(almost_equal(v.x(), 15967.2253635, v.y(), 43245.7191308));
    assert_eq!(v.get_incident_edge()?.0, 57);
    let v = output.vertices()[14].get();
    assert!(almost_equal(v.x(), 23102.2251381, v.y(), 38339.0179906));
    assert_eq!(v.get_incident_edge()?.0, 59);
    let v = output.vertices()[15].get();
    assert!(almost_equal(v.x(), 42095.0000000, v.y(), 33233.0000000));
    assert_eq!(v.get_incident_edge()?.0, 63);
    let v = output.vertices()[16].get();
    assert!(almost_equal(v.x(), 14319.0000000, v.y(), -19099.5000000));
    assert_eq!(v.get_incident_edge()?.0, 65);
    let v = output.vertices()[17].get();
    assert!(almost_equal(v.x(), -14781.8565240, v.y(), 84616.9113708));
    assert_eq!(v.get_incident_edge()?.0, 67);
    let v = output.vertices()[18].get();
    assert!(almost_equal(v.x(), 56712.0000000, v.y(), -55735.0000000));
    assert_eq!(v.get_incident_edge()?.0, 73);
    let v = output.vertices()[19].get();
    assert!(almost_equal(v.x(), 23008.5401908, v.y(), -18584.2340099));
    assert_eq!(v.get_incident_edge()?.0, 75);
    let v = output.vertices()[20].get();
    assert!(almost_equal(v.x(), 61580.0000000, v.y(), -50720.0000000));
    assert_eq!(v.get_incident_edge()?.0, 81);
    let v = output.vertices()[21].get();
    assert!(almost_equal(v.x(), 49723.2533225, v.y(), -39211.3044224));
    assert_eq!(v.get_incident_edge()?.0, 83);
    let v = output.vertices()[22].get();
    assert!(almost_equal(v.x(), 54809.8124517, v.y(), 34893.6315657));
    assert_eq!(v.get_incident_edge()?.0, 85);
    let v = output.vertices()[23].get();
    assert!(almost_equal(v.x(), 40139.1173982, v.y(), -16552.6411677));
    assert_eq!(v.get_incident_edge()?.0, 87);
    let v = output.vertices()[24].get();
    assert!(almost_equal(v.x(), 42579.7873427, v.y(), -15161.0319493));
    assert_eq!(v.get_incident_edge()?.0, 89);
    let v = output.vertices()[25].get();
    assert!(almost_equal(v.x(), 61781.9189142, v.y(), 519.3513867));
    assert_eq!(v.get_incident_edge()?.0, 91);
    let v = output.vertices()[26].get();
    assert!(almost_equal(v.x(), 70917.4531383, v.y(), 11087.7441617));
    assert_eq!(v.get_incident_edge()?.0, 93);
    let v = output.vertices()[27].get();
    assert!(almost_equal(v.x(), 72297.8985020, v.y(), 12733.0450056));
    assert_eq!(v.get_incident_edge()?.0, 95);
    let v = output.vertices()[28].get();
    assert!(almost_equal(v.x(), 140722.0000000, v.y(), 55735.0000000));
    assert_eq!(v.get_incident_edge()?.0, 101);
    let v = output.vertices()[29].get();
    assert!(almost_equal(v.x(), 132523.3881263, v.y(), 41556.3157791));
    assert_eq!(v.get_incident_edge()?.0, 103);
    let v = output.vertices()[30].get();
    assert!(almost_equal(v.x(), 148480.0000000, v.y(), 38812.0000000));
    assert_eq!(v.get_incident_edge()?.0, 107);
    let v = output.vertices()[31].get();
    assert!(almost_equal(v.x(), -82102.4218576, v.y(), 1367689.7573458));
    assert_eq!(v.get_incident_edge()?.0, 109);
    let v = output.vertices()[32].get();
    assert!(almost_equal(v.x(), -82819.7798653, v.y(), 1381335.2741606));
    assert_eq!(v.get_incident_edge()?.0, 111);
    let v = output.vertices()[33].get();
    assert!(almost_equal(v.x(), -192329.1012672, v.y(), 3460369.2122219));
    assert_eq!(v.get_incident_edge()?.0, 113);
    let v = output.vertices()[34].get();
    assert!(almost_equal(v.x(), -216595.6375268, v.y(), 3910517.0085418));
    assert_eq!(v.get_incident_edge()?.0, 115);
    Ok(())
}

#[test]
// this is the old gui test 'A_test'
fn segment_5_4() -> Result<(), BvError> {
    let output = {
        let segments: [[I1; 4]; 2] = [
            [35058881, -35000000, 31058881, -35000000],
            [31058881, -35000000, 25058881, -35000001],
        ];

        let segments = to_segments(&segments);

        let mut vb = Builder::<I1, F1>::default();
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
    assert_eq!(output.vertices().len(), 3);
    assert_eq!(output.edges().len(), 14);
    let v = output.vertices()[0].get();
    assert!(almost_equal(
        v.x(),
        31058881.0000000,
        v.y(),
        -35000000.0000000
    ));
    assert_eq!(v.get_incident_edge()?.0, 7);
    let v = output.vertices()[1].get();
    assert!(almost_equal(
        v.x(),
        35058881.0000000,
        v.y(),
        -48000035000000.3281250
    ));
    assert_eq!(v.get_incident_edge()?.0, 11);
    let v = output.vertices()[2].get();
    assert!(almost_equal(
        v.x(),
        37558881.0000001,
        v.y(),
        -75000035000001.7500000
    ));
    assert_eq!(v.get_incident_edge()?.0, 13);
    Ok(())
}
