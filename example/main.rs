use boostvoronoi::builder as VB;
use boostvoronoi::BvError;

type I = i64;
type F = f64;

/// This example will fail, something is wrong with the beach-line ordering
fn main() -> Result<(), BvError> {
    #[allow(unused_variables)]
    let output = {
        let points: [[I; 2]; 0] = [];
        let segments: [[I; 4]; 2] = [
            [-5357, -5417, -5111, -5027],
            [-5330, -5287, -5312, -5283],
        ];

        let _v = VB::to_points::<I, I>(&points);
        let _s = VB::to_segments::<I, I>(&segments);
        println!("-------\n{}", points.len());
        for p in points.iter() {
            println!("{} {}", p[0], p[1]);
        }
        println!("{}", segments.len());
        for s in segments.iter() {
            println!("{} {} {} {}", s[0], s[1], s[2], s[3]);
        }
        println!("-------");
        println!("int INPUT_PTS[{}][2] = {{", points.len());
        for p in points.iter() {
            print!("{{{},{}}},", p[0], p[1]);
        }
        println!("}};");
        println!("int INPUT_SGS[{}][4] = {{", segments.len());
        for s in segments.iter() {
            print!("{{{},{},{},{}}},", s[0], s[1], s[2], s[3]);
        }
        println!("}};");
        println!("-------");
        let mut vb = VB::Builder::<I, F>::default();
        vb.with_vertices(_v.iter())?;
        vb.with_segments(_s.iter())?;
        vb.build()?
    };
    println!();
    for (i, v) in output.vertices().iter().enumerate() {
        println!(
            "vertex #{} contains a point: ({:.12}, {:.12}) ie:{:?}",
            i,
            v.get().x(),
            v.get().y(),
            v.get().get_incident_edge()?.0
        );
    }

    println!("cells:{}", output.cells().len());
    println!("vertices:{}", output.vertices().len());
    println!("edges:{}", output.edges().len());

    Ok(())
}
