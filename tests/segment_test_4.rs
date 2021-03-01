use boostvoronoi::builder as VB;
use boostvoronoi::diagram as VD;
use boostvoronoi::{BvError, InputType};
use boostvoronoi::{Line, Point};

type I1 = i32;
type F1 = f64;
type I2 = i64;
type F2 = f64;

#[allow(dead_code)]
fn almost_equal(x1: F1, x2: F1, y1: F1, y2: F1) -> bool {
    let delta = 0.00001;
    assert!(F1::abs(x1 - x2) < delta, "{} != {}", x1, x2);
    assert!(F1::abs(y1 - y2) < delta, "{} != {}", y1, y2);

    (F1::abs(x1 - x2) < delta) && (F1::abs(y1 - y2) < delta)
}

fn to_points<T: InputType>(points: &[[T; 2]]) -> Vec<boostvoronoi::Point<T>> {
    points.iter().map(|x| x.into()).collect()
}

fn to_segments<T: InputType>(points: &[[T; 4]]) -> Vec<boostvoronoi::Line<T>> {
    points.iter().map(|x| x.into()).collect()
}

fn retrieve_point<T>(
    point_data_: &Vec<Point<T>>,
    segment_data_: &Vec<Line<T>>,
    source: (VD::SourceIndex, VD::SourceCategory),
) -> Point<T>
where
    T: VD::InputType,
{
    match source.1 {
        VD::SourceCategory::SinglePoint => point_data_[source.0],
        VD::SourceCategory::SegmentStart => segment_data_[source.0 - point_data_.len()].start,
        VD::SourceCategory::Segment | VD::SourceCategory::SegmentEnd => {
            segment_data_[source.0 - point_data_.len()].end
        }
    }
}

#[ignore]
#[test]
fn segment_4_1_intersecting() {
    let _output = {
        let segments: [[I1; 4]; 9] = [
            [207, 208, 405, 400],
            [409, 401, 200, 201],
            [400, 402, 403, 230],
            [410, 203, 204, 220],
            [529, 244, 367, 107],
            [94, 628, 512, 632],
            [680, 608, 669, 291],
            [572, 96, 363, 51],
            [147, 103, 96, 170],
        ];
        let segments = to_segments(&segments);

        let mut vb = VB::Builder::<I1, F1, I2, F2>::new();
        vb.with_segments(segments.iter()).expect("should not fail");
        vb.construct()
    };
    _output.expect_err("should fail");
}

#[ignore]
#[test]
fn segment_4_1() -> Result<(), BvError> {
    let _output = {
        let segments: [[I1; 4]; 9] = [
            [200, 200, 200, 400],
            [200, 400, 400, 400],
            [400, 400, 400, 200],
            [400, 200, 200, 200],
            [529, 242, 367, 107],
            [94, 628, 512, 632],
            [680, 608, 669, 291],
            [572, 96, 363, 51],
            [147, 103, 96, 170],
        ];
        let segments = to_segments(&segments);

        let mut vb = VB::Builder::<I1, F1, I2, F2>::new();
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    Ok(())
}

//#[ignore]
#[test]
fn vertical_segment_1() -> Result<(), BvError> {
    let points: [[I1; 2]; 0] = [];
    let segments: [[I1; 4]; 3] = [
        [200, 200, 200, 400],
        [94, 628, 512, 632],
        [147, 103, 96, 170],
    ];

    let _v = to_points::<I1>(&points);
    let _s = to_segments::<I1>(&segments);

    let mut vb = VB::Builder::<I1, F1, I2, F2>::new();
    vb.with_vertices(_v.iter())?;
    vb.with_segments(_s.iter())?;
    let output = vb.construct()?;

    assert_eq!(output.cells().len(), 9);
    let cell = output.cells()[0].get();
    assert_eq!(cell.get_id(), 0);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&_v, &_s, (source_index, cat));
    assert_eq!(p, Point { x: 94, y: 628 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[1].get();
    assert_eq!(cell.get_id(), 1);
    let (source_index, cat) = cell.source_index_2();
    assert_eq!(cat, VD::SourceCategory::Segment);
    let p = retrieve_point(&_v, &_s, (source_index, cat));
    assert_eq!(p, Point { x: 512, y: 632 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[2].get();
    assert_eq!(cell.get_id(), 2);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&_v, &_s, (source_index, cat));
    assert_eq!(p, Point { x: 96, y: 170 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[3].get();
    assert_eq!(cell.get_id(), 3);
    let (source_index, cat) = cell.source_index_2();
    assert_eq!(cat, VD::SourceCategory::Segment);
    let p = retrieve_point(&_v, &_s, (source_index, cat));
    assert_eq!(p, Point { x: 96, y: 170 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[4].get();
    assert_eq!(cell.get_id(), 4);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&_v, &_s, (source_index, cat));
    assert_eq!(p, Point { x: 147, y: 103 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[5].get();
    assert_eq!(cell.get_id(), 5);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&_v, &_s, (source_index, cat));
    assert_eq!(p, Point { x: 200, y: 200 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[6].get();
    assert_eq!(cell.get_id(), 6);
    let (source_index, cat) = cell.source_index_2();
    assert_eq!(cat, VD::SourceCategory::Segment);
    let p = retrieve_point(&_v, &_s, (source_index, cat));
    assert_eq!(p, Point { x: 200, y: 400 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), false);
    assert_eq!(cell.contains_segment(), true);
    let cell = output.cells()[7].get();
    assert_eq!(cell.get_id(), 7);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&_v, &_s, (source_index, cat));
    assert_eq!(p, Point { x: 200, y: 400 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    let cell = output.cells()[8].get();
    assert_eq!(cell.get_id(), 8);
    let (source_index, cat) = cell.source_index_2();
    let p = retrieve_point(&_v, &_s, (source_index, cat));
    assert_eq!(p, Point { x: 512, y: 632 });
    assert_eq!(cell.is_degenerate(), false);
    assert_eq!(cell.contains_point(), true);
    assert_eq!(cell.contains_segment(), false);
    assert_eq!(output.vertices().len(), 10);
    assert_eq!(output.edges().len(), 36);
    /*let v = output.vertices()[0].get();
    assert!(almost_equal(v.x(), 143.7970909, v.y(), 200.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0,13);
    let v = output.vertices()[1].get();
    assert!(almost_equal(v.x(), 142.0840045, v.y(), 205.0788691));
    assert_eq!(v.get_incident_edge().unwrap().0,15);
    let v = output.vertices()[2].get();
    assert!(almost_equal(v.x(), -102.2273417, v.y(), 398.1387452));
    assert_eq!(v.get_incident_edge().unwrap().0,17);
    let v = output.vertices()[3].get();
    assert!(almost_equal(v.x(), -98.2075472, v.y(), 400.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0,21);
    let v = output.vertices()[4].get();
    assert!(almost_equal(v.x(), 95.3208257, v.y(), 489.9737172));
    assert_eq!(v.get_incident_edge().unwrap().0,23);
    let v = output.vertices()[5].get();
    assert!(almost_equal(v.x(), 195.1646270, v.y(), 139.6626265));
    assert_eq!(v.get_incident_edge().unwrap().0,25);
    let v = output.vertices()[6].get();
    assert!(almost_equal(v.x(), 431.2163645, v.y(), 400.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0,29);
    let v = output.vertices()[7].get();
    assert!(almost_equal(v.x(), 515.0143534, v.y(), 317.0000690));
    assert_eq!(v.get_incident_edge().unwrap().0,31);
    let v = output.vertices()[8].get();
    assert!(almost_equal(v.x(), 655.0769231, v.y(), 200.0000000));
    assert_eq!(v.get_incident_edge().unwrap().0,33);
    let v = output.vertices()[9].get();
    assert!(almost_equal(v.x(), 2427.4071661, v.y(), -1080.0162866));
    assert_eq!(v.get_incident_edge().unwrap().0,35);*/
    Ok(())
}
