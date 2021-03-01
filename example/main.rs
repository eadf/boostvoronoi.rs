use boostvoronoi::builder as VB;
use boostvoronoi::BvError;
use boostvoronoi::InputType;

type I1 = i64;
type F1 = f64;
type I2 = i64;
type F2 = f64;

#[allow(dead_code)]
fn almost_equal(x1: F1, x2: F1, y1: F1, y2: F1) -> bool {
    let delta = 0.001;
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

/// This example will fail, something is wrong with the beach-line ordering
fn main() -> Result<(), BvError> {
    #[allow(unused_variables)]
    let output = {
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
        vb.construct()?
    };
    println!("cells:{}", output.cells().len());
    println!("vertices:{}", output.vertices().len());
    println!("edges:{}", output.edges().len());
    for (i,v) in output.vertices().iter().enumerate() {
       println!("vertex #{} contains a point: ({:?}, {:?})", i, v.get().x(),v.get().y() );
    }
    Ok(())
}
