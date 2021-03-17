use boostvoronoi::builder as VB;
use boostvoronoi::BvError;
//use boostvoronoi::InputType;

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

/// This example will fail, something is wrong with the beach-line ordering
fn main() -> Result<(), BvError> {
    #[allow(unused_variables)]
    let output = {
        let points: [[I1; 2]; 0] = [];
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

        let _v = VB::to_points::<I1, I1>(&points);
        let _s = VB::to_segments::<I1, I1>(&segments);

        let mut vb = VB::Builder::<I1, F1, I2, F2>::new();
        vb.with_vertices(_v.iter())?;
        vb.with_segments(_s.iter())?;
        vb.construct()?
    };

    println!();
    for (i, v) in output.vertices().iter().enumerate() {
        println!(
            "vertex #{} contains a point: ({:.12}, {:.12}) ie:{:?}",
            i,
            v.get().x(),
            v.get().y(),
            v.get().get_incident_edge().unwrap().0
        );
    }

    println!("cells:{}", output.cells().len());
    println!("vertices:{}", output.vertices().len());
    println!("edges:{}", output.edges().len());
    Ok(())
}
