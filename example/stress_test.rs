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

/// check if all the vertices really are at the midpoint between two segments
fn verify_vertices(
    diagram: &Result<VD::Diagram<I, F>, BvError>,
    segments: Vec<geo::Line<I>>,
) -> bool {
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
                let delta: f64 = distance - *peek;
                //print!(" delta:{}", delta);

                if delta > 0.0001 {
                    //println!(" ignore peek:{}", peek);
                    continue;
                }
                if delta < -0.0001 {
                    //println!(" clear peek:{}", peek);
                    heap.clear();
                }
            }
            //println!();
            heap.push(distance);
        }
        if heap.len() < 2 {
            println!("pop1: {:?}, pop2:{:?}", heap.get(0), heap.get(1));
            return true;
        } else {
            //println!("heap:{:?}", heap);
        }
        heap.clear();
    }
    //panic!();
    false
}

/// Looking for failing examples
fn main() -> Result<(), BvError> {
    let l1 = Line::from([(-5500_i64, -5343), (-5120_i64, -5120)]);
    let l2 = Line::from([(-5472_i64, -5270), (-5380_i64, -5380)]);
    if l1.intersects(&l2) {
        println!("intersects");
    }
    //panic!();
    let mut rng = thread_rng();
    //#[allow(unused_variables)]
    loop {
        let mut geo_segments = Vec::<geo::Line<I>>::new();
        'gen_loop: while geo_segments.len() < 2 {
            let line = Line::from([
                (
                    rng.gen_range(-5500_i64..-5000),
                    rng.gen_range(-5500_i64..-5000),
                ),
                (
                    rng.gen_range(-5500_i64..-5000),
                    rng.gen_range(-5500_i64..-5000),
                ),
            ]);
            for s in geo_segments.iter() {
                if line.intersects(s) {
                    continue 'gen_loop;
                } else {
                    //println!("{:?} does NOT intersect {:?}", s,line);
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
        if result.is_err() || verify_vertices(&result, geo_segments) {
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

            println!("{:?}", result?);
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
