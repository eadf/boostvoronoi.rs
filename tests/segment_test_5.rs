use boostvoronoi::builder::Builder;
use boostvoronoi::Line;
use boostvoronoi::{BvError, InputType};

type I1 = i64;
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

fn to_segments<T: InputType>(points: &[[T; 4]]) -> Vec<Line<T>> {
    points.iter().map(|x| x.into()).collect()
}

#[ignore]
#[test]
fn segment_5_1() -> Result<(), BvError> {
    let _output = {
        let segments: [[I1; 4]; 4] = [
            [-251891, 127298, -237685, 182389],
            [-237685, 182389, -182594, 168183],
            [-182594, 168183, -196800, 113093],
            [-196800, 113093, -251891, 127298],
        ];
        let segments = to_segments(&segments);

        let mut vb = Builder::<I1, F1, I2, F2>::new();
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    Ok(())
}

#[ignore]
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

        let mut vb = Builder::<I1, F1, I2, F2>::new();
        vb.with_segments(segments.iter())?;
        vb.construct()?
    };
    Ok(())
}