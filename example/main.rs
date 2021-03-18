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
        let segments: [[I1; 4]; 19] = [
            [67035, 16168, -51301, 122269],
            [-51301, 122269, -50598, 120727],
            [-50598, 120727, -56132, 110391],
            [-56132, 110391, -102080, 102917],
            [-102080, 102917, -112508, 94666],
            [-112508, 94666, -110974, 81469],
            [-110974, 81469, -83788, 43709],
            [-83788, 43709, -87201, 32462],
            [-87201, 32462, -130792, 16168],
            [-130792, 16168, -139396, 6040],
            [-139396, 6040, -135315, -6597],
            [-135315, -6597, -101213, -38399],
            [-101213, -38399, -102342, -49954],
            [-102342, -49954, -142015, -74507],
            [-142015, -74507, -148480, -86107],
            [-148480, -86107, -142015, -97715],
            [-142015, -97715, -102342, -122269],
            [-102342, -122269, 148479, -56855],
            [148479, -56855, 67035, 16168],
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
    //println!("edges:{}", output.edges().len());

    output.debug_print_edges();
    Ok(())
}
