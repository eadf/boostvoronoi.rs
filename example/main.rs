use boostvoronoi::{prelude::*, read_boost_input_buffer};
use std::io::{BufReader, Cursor};

type I = i64;
type F = f64;

fn main() -> Result<(), BvError> {
    let output = {
        let input = r#"0
3
310 407 365 177
754 177 -893 79
300 558 109 347
"#;
        let (points, segments) =
            read_boost_input_buffer::<I, _>(BufReader::new(Cursor::new(input)))?;

        println!("Input data:\n{}", points.len());
        for p in points.iter() {
            println!("{} {}", p.x, p.y);
        }
        println!("{}", segments.len());
        for s in segments.iter() {
            println!("{} {} {} {}", s.start.x, s.start.y, s.end.x, s.end.y);
        }
        if true {
            // this will generate some C++ code from the example data
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
        }
        Builder::<I, F>::default()
            .with_vertices(points.iter())?
            .with_segments(segments.iter())?
            .build()?
    };
    println!("\nResult:");
    for (i, v) in output.vertices().iter().enumerate() {
        println!(
            "vertex #{} contains a point: ({:.12}, {:.12}), incident edge#:{:?}",
            i,
            v.get().x(),
            v.get().y(),
            v.get().get_incident_edge()?.0
        );
    }

    println!("\ncells:{}", output.cells().len());
    println!("vertices:{}", output.vertices().len());
    println!("edges:{}\n", output.edges().len());

    for (i, e) in output.edges().iter().enumerate() {
        println!("Edge:#{}=>{:?}", i, e.get());
    }
    Ok(())
}
