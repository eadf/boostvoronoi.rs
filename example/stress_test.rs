use boostvoronoi::builder as VB;
use boostvoronoi::BvError;
use geo::Line;
use rand::{thread_rng, Rng};
use geo::prelude::Intersects;

type I = i64;
type F = f64;

/// Looking for failing examples
fn main() -> Result<(), BvError> {

    let l1 = Line::from([(-5500_i64, -5343),(-5120_i64, -5120)]);
    let l2 = Line::from([(-5472_i64, -5270),(-5380_i64, -5380)]);
    if l1.intersects(&l2) {
        println!("intersects");
    }
    //panic!();
    let mut rng = thread_rng();
    //#[allow(unused_variables)]
    loop {
        let mut segments = Vec::<geo::Line<I>>::new();
        'gen_loop: while segments.len() < 2 {
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
            for s in segments.iter() {
                if line.intersects(s) {
                    continue 'gen_loop;
                } else {
                    println!("{:?} does NOT intersect {:?}", s,line);
                }
            }
            segments.push(line);
        }
        let vertices: [[I; 2]; 0] = [];
        let vertices = VB::to_points::<I, I>(&vertices);
        let segments = segments.into_iter().map(|l|boostvoronoi::geometry::Line::from(l)).collect::<Vec<boostvoronoi::geometry::Line<i64>>>();

        let mut vb = VB::Builder::<I, F>::default();
        vb.with_vertices(vertices.iter())?;
        vb.with_segments(segments.iter())?;
        let result =vb.build();
        if result.is_err() {
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

            println!("{:?}", result);
            break
        } else {
            println!("got {} vertices", result?.vertices().len());
        }
    }
    Ok(())
}
