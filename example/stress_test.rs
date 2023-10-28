#![cfg_attr(feature = "map_first_last", feature(map_first_last))]
use boostvoronoi::geo::{
    algorithm::euclidean_distance::*, prelude::Intersects, Coord, Line as GLine,
};
use boostvoronoi::prelude::*;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use rand::{Rng, SeedableRng};
use rand::prelude::StdRng;

type I = i64;
type F = f64;
const NUMBER_OF_THREADS: usize = 6;
const REPORT_FREQUENCY: usize = 1000_000;
const TESTS_PER_SEED: usize = 1000_000;
const NUMBER_OF_SEGMENTS_PER_TEST: usize = 2;
const SEED_START: u64 = 121;

/// Messages sent from worker threads to manager
enum SeedRequest {
    RequestNewSeed(usize),
    ErrorFrom(usize),
}

/// Check if all the vertices really are at the midpoint between (at least) two segments.
///
/// I've repeated this 10_000_000_000 times with NUMBER_OF_SEGMENTS_PER_TEST=2 w/o any problems.
///
/// With NUMBER_OF_SEGMENTS_PER_TEST=3 it finds an error per million iterations.
fn fault_check(
    diagram: &Result<Diagram<F>, BvError>,
    segments: Vec<GLine<I>>,
) -> Result<(), String> {
    let mut heap: Vec<f64> = Vec::new();
    let diagram = diagram.as_ref().unwrap();
    // is there no easier way to cast Vec<geo::Line<i64>> to Vec<geo::Line<f64>>??
    let segments: Vec<GLine<f64>> = segments
        .iter()
        .map(|s| {
            GLine::from([
                (s.start.x as f64, s.start.y as f64),
                (s.end.x as f64, s.end.y as f64),
            ])
        })
        .collect();
    for v in diagram.vertices().iter() {
        let v = v.get();
        let v = Coord { x: v.x(), y: v.y() };
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
            //eprintln!("{}", err_msg);
            return Err(err_msg);
        }
        heap.clear();
    }
    Ok(())
}

/// Looking for failing examples by generating random voronoi test data.
fn boostvoronoi_test(rnd_seed: [u8;32], printout_lock: Arc<Mutex<()>>) -> Result<(), BvError> {
    let to_r = 1000_i64;
    let mut rng: StdRng = SeedableRng::from_seed(rnd_seed);

    for _ in 0..TESTS_PER_SEED {
        let mut geo_segments = Vec::<GLine<I>>::with_capacity(4);
        'gen_loop: while geo_segments.len() < NUMBER_OF_SEGMENTS_PER_TEST {
            let line = GLine::from([
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
        let vertices = []
            .iter()
            .map(|p: &[i64; 2]| -> boostvoronoi::Point<i64> { p.into() })
            .collect::<Vec<boostvoronoi::Point<i64>>>();
        let segments = geo_segments
            .iter()
            .map(|l| l.into())
            .collect::<Vec<boostvoronoi::Line<i64>>>();

        let result = Builder::<I, F>::default()
            .with_vertices(vertices.iter())?
            .with_segments(segments.iter())?
            .build();
        if result.is_err() || fault_check(&result, geo_segments).is_err() {
            let _ = printout_lock.lock();
            println!("\nfound a bad example:");
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

            /*let result = result?;
            for e in result.edges() {
                println!("e:{:?}", e.get());
            }
            for v in result.vertices() {
                println!("v:{:?}", v.get());
            }*/
            return Err(BvError::InternalError("found error".to_string()));
        }
    }
    Ok(())
}

#[inline]
fn worker_thread_loop(
    id: usize,
    request_tx: Sender<SeedRequest>,
    seed_rx: Receiver<[u8;32]>,
    printout_lock: Arc<Mutex<()>>,
) {
    loop {
        //println!("thread id {} starting", id);
        // assume that there is already a seed waiting for the first loop
        let new_seed = seed_rx.recv().unwrap();
        if boostvoronoi_test(new_seed, Arc::clone(&printout_lock)).is_err() {
            request_tx.send(SeedRequest::ErrorFrom(id)).unwrap();
            //break;
        };
        request_tx.send(SeedRequest::RequestNewSeed(id)).unwrap();
    }
}

fn u64_to_u8_array(value: u64) -> [u8; 32] {
    let u64_bytes: [u8; 8] = value.to_le_bytes(); // Convert u64 to little-endian bytes
    let mut result: [u8; 32] = [0; 32]; // Initialize a 32-byte array with zeros
    result[..8].copy_from_slice(&u64_bytes); // Copy the u64 bytes to the first 8 bytes of the result
    result[..32].copy_from_slice(&u64_bytes); // Copy the u64 bytes to the entire result
    result
}

/// spawn off a number of threads and let those test for faulty voronoi builds.
///
/// run with 'cargo run --example stress_test --features geo --release"
fn main() {
    let mut next_seed = SEED_START;
    let printout_lock = Arc::new(Mutex::new(()));

    // used when requesting and sending new seeds
    let (request_tx, request_rx) = mpsc::channel::<SeedRequest>();

    let handles: Vec<_> = (0..NUMBER_OF_THREADS)
        .map(|n| {
            let (thread_tx, thread_rx) = mpsc::channel::<[u8;32]>();
            // prime the seed channel
            let _ = thread_tx.send(u64_to_u8_array(next_seed)).unwrap();
            next_seed += 1;

            let thread_handle = thread::spawn({
                let request_tx = request_tx.clone();
                let printout_lock = Arc::clone(&printout_lock);
                move || worker_thread_loop(n, request_tx, thread_rx, printout_lock)
            });
            (thread_handle, thread_tx)
        })
        .collect();

    let mut iterations = 0_usize;
    let duration = std::time::Instant::now();
    let mut detected_errors = 0;
    loop {
        match request_rx
            .recv_timeout(std::time::Duration::from_millis(100))
            .ok()
        {
            Some(SeedRequest::RequestNewSeed(requesting_id)) => {
                //println!("Got request from : {}", requesting_thread);
                handles[requesting_id as usize].1.send(u64_to_u8_array(next_seed)).unwrap();
                next_seed += 1;
                iterations += TESTS_PER_SEED;
                if iterations % REPORT_FREQUENCY == 0 {
                    let tdelta = duration.elapsed().as_secs_f64()
                        / (iterations as f64 / REPORT_FREQUENCY as f64);
                    println!(
                        "report: {}*{} tests at an average {:.4} seconds per {} tests. Next seed:{}, errors detected:{}",
                        iterations / REPORT_FREQUENCY,
                        REPORT_FREQUENCY,
                        tdelta,
                        REPORT_FREQUENCY,
                        next_seed,
                        detected_errors,
                    );
                }
            }
            None => continue,
            _ => {
                detected_errors += 1;
                if detected_errors >= 5 {
                    break;
                }
            }
        }
    }
}
