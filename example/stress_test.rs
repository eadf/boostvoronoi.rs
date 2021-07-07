#![feature(map_first_last)]
use boostvoronoi::builder as VB;
use boostvoronoi::diagram as VD;
use boostvoronoi::BvError;
use geo::algorithm::euclidean_distance::*;
use geo::prelude::Intersects;
use geo::Line;
use rand::{thread_rng, Rng};

type I = i64;
type F = f64;

/// Check if all the vertices really are at the midpoint between (at least) two segments.
fn fault_check(
    diagram: &Result<VD::Diagram<I, F>, BvError>,
    segments: Vec<geo::Line<I>>,
) -> Result<(), String> {
    let mut heap: Vec<f64> = Vec::new();
    let diagram = diagram.as_ref().unwrap();
    // is there no easier way to cast Vec<geo::Line<i64>> to Vec<geo::Line<f64>>??
    let segments: Vec<geo::Line<f64>> = segments
        .iter()
        .map(|s| {
            Line::from([
                (s.start.x as f64, s.start.y as f64),
                (s.end.x as f64, s.end.y as f64),
            ])
        })
        .collect();
    for v in diagram.vertices().iter() {
        let v = v.get();
        let v = geo::Coordinate { x: v.x(), y: v.y() };
        //println!("v {:?}", v);
        for s in segments.iter() {
            let distance = v.euclidean_distance(s);
            //print!("s{:?} -> v {:?} = {:?}", s, v, distance);
            if let Some(peek) = heap.first() {
                if distance <= *peek {
                    if *peek - distance > 0.0001 {
                        // this sample is smaller than anything before
                        heap.clear();
                    }
                } else {
                    if distance - *peek > 0.0001 {
                        // ignore this sample, get a new sample
                        continue;
                    }
                }
            }
            //println!();
            heap.push(distance);
        }
        if heap.len() < 2 {
            let mut err_msg = format!(
                "Got a vertex with only one close neighbour: {:?}, dist:{:?}",
                v,
                heap.get(0)
            );
            for s in segments.iter() {
                err_msg += format!("\n {:?}, dist:{}", s, v.euclidean_distance(s)).as_str();
            }
            eprintln!("{}", err_msg);
            return Err(err_msg);
        }
        heap.clear();
    }
    Ok(())
}

/// Looking for failing examples
fn main() -> Result<(), BvError> {
    let to_r = 1000_i64;

    let mut rng = thread_rng();
    loop {
        let mut geo_segments = Vec::<geo::Line<I>>::new();
        'gen_loop: while geo_segments.len() < 2 {
            let line = Line::from([
                (rng.gen_range(-to_r..to_r), rng.gen_range(-to_r..to_r)),
                (rng.gen_range(-to_r..to_r), rng.gen_range(-to_r..to_r)),
            ]);
            for s in geo_segments.iter() {
                if line.intersects(s) {
                    // this line was of no use to us, generate a new one
                    continue 'gen_loop;
                }
            }
            geo_segments.push(line);
        }
        let vertices: [[I; 2]; 0] = [];
        let vertices = VB::to_points::<I, I>(&vertices);
        let segments = geo_segments
            .iter()
            .map(|l| boostvoronoi::geometry::Line::from(*l))
            .collect::<Vec<boostvoronoi::geometry::Line<i64>>>();

        let mut vb = VB::Builder::<I, F>::default();
        vb.with_vertices(vertices.iter())?;
        vb.with_segments(segments.iter())?;
        let result = vb.build();
        if result.is_err() || fault_check(&result, geo_segments).is_err() {
            println!("-------\n{}", vertices.len());
            for p in vertices.iter() {
                println!("{} {}", p.x, p.y);
            }
            println!("{}", segments.len());
            for s in segments.iter() {
                println!("{} {} {} {}", s.start.x, s.start.y, s.end.x, s.end.y);
            }
            println!("-------");
            print!("int INPUT_PTS[{}][2] = {{", vertices.len());
            for p in vertices.iter() {
                print!("{{{},{}}},", p.x, p.y);
            }
            println!("}};");
            print!("int INPUT_SGS[{}][4] = {{", segments.len());
            for s in segments.iter() {
                print!("{{{},{},{},{}}},", s.start.x, s.start.y, s.end.x, s.end.y);
            }
            println!("}};");
            println!("-------");

            let result = result?;
            for e in result.edges() {
                println!("e:{:?}", e.get());
            }
            for v in result.vertices() {
                println!("v:{:?}", v.get());
            }
            break;
        } else {
            println!(
                "Successfully got {} vertices. Trying again",
                result?.vertices().len()
            );
        }
    }
    Ok(())
}
