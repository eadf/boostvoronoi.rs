use boostvoronoi::builder as VB;
use boostvoronoi::file_reader as FR;
use boostvoronoi::BvError;
use std::io::{BufReader, Cursor};

type I = i64;
type F = f64;

/// This example will fail, something is wrong with the beach-line ordering
fn main() -> Result<(), BvError> {
    #[allow(unused_variables)]
    let output = {
        let input = r#"0
6
0 10000000 700000 1
700000 1 700000 9000000
700000 9000000 9100000 9000000
9100000 9000000 9100000 0
9100000 0 10000000 10000000
10000000 10000000 0 10000000
"#;
        let vb = VB::Builder::<I, F>::default();
        let br = BufReader::new(Cursor::new(input));
        let (points, segments) = FR::read_boost_input_buffer::<I, _>(br)?;

        println!("-------\n{}", points.len());
        for p in points.iter() {
            println!("{} {}", p.x, p.y);
        }
        println!("{}", segments.len());
        for s in segments.iter() {
            println!("{} {} {} {}", s.start.x, s.start.y, s.end.x, s.end.y);
        }
        println!("-------");
        println!("int INPUT_PTS[{}][2] = {{", points.len());
        for p in points.iter() {
            print!("{{{},{}}},", p.x, p.y);
        }
        println!("}};");
        println!("int INPUT_SGS[{}][4] = {{", segments.len());
        for s in segments.iter() {
            print!("{{{},{},{},{}}},", s.start.x, s.start.y, s.end.x, s.end.y);
        }
        println!("}};");
        println!("-------");
        let mut vb = VB::Builder::<I, F>::default();
        vb.with_vertices(points.iter())?;
        vb.with_segments(segments.iter())?;
        //panic!();
        vb.build()?
    };
    /*println!();
    for (i, v) in output.vertices().iter().enumerate() {
        println!(
            "vertex #{} contains a point: ({:.12}, {:.12}) ie:{:?}",
            i,
            v.get().x(),
            v.get().y(),
            v.get().get_incident_edge()?.0
        );
    }
    */
    println!("cells:{}", output.cells().len());
    println!("vertices:{}", output.vertices().len());
    println!("edges:{}", output.edges().len());

    for (i, e) in output.edges().iter().enumerate() {
        println!("Edge:#{}=>{:?}", i, e.get());
    }
    Ok(())
}
