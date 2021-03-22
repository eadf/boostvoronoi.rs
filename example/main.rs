use boostvoronoi::builder as VB;
use boostvoronoi::diagram as VD;
use boostvoronoi::BvError;
//use boostvoronoi::InputType;

type I1 = i64;
type F1 = f64;

const EXTERNAL: VD::ColorType = 1;

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
        // problematic polygon from https://github.com/boostorg/polygon/issues/43
        // not even 1.75 C++ boost handles this correctly
        let segments: [[I1; 4]; 6] = [
            [0, 10000000, 700000, 1],
            [700000, 1, 700000, 9000000],
            [700000, 9000000, 9100000, 9000000],
            [9100000, 9000000, 9100000, 0],
            [9100000, 0, 10000000, 10000000],
            [10000000, 10000000, 0, 10000000],
        ];
        /*
        let shift:I1 = 35058881;
        let a:I1 = 0; // nope: -5,-4,-3,-1,0

        let segments: [[I1; 4]; 4] = [
            [ a + shift, -35000000, a + shift, -25732145 ],
            [a + shift, -25732145 , a + shift, -19586070],
            [a + shift, -19586070, -66716086 + shift, -35000000],
            [ -66716086 + shift, -35000000 , a + shift, -35000000],
        ];*/

        let _v = VB::to_points::<I1, I1>(&points);
        let _s = VB::to_segments::<I1, I1>(&segments);
        println!("-------\n0\n{}", segments.len());
        for s in segments.iter() {
            println!("{} {} {} {}", s[0], s[1], s[2], s[3] );
        }
        println!("-------");

        let mut vb = VB::Builder::<I1, F1>::new();
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
    output.color_exterior_edges(EXTERNAL);
    output.debug_print_edges();
    Ok(())
}
