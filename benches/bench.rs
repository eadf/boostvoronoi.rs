use boostvoronoi as BV;
use boostvoronoi::prelude::*;
use std::io::{BufReader, Cursor};

use criterion::{criterion_group, criterion_main, Criterion};

type I = i32;
type F = f64;

#[cfg(test)]
pub fn bench_segments(c: &mut Criterion) {
    c.bench_function("bench_segments", |b| {
        b.iter(|| {
            // same test as cpp voronoi bench : voronoi_segment.txt
            let _output = {
                let input = include_str!("input_data/voronoi_segment.txt");
                let br = BufReader::new(Cursor::new(input));
                let (points, segments) =
                    BV::read_boost_input_buffer::<I, _>(br).expect("bench_segments");
                Builder::<I, F>::default()
                    .with_vertices(points.iter())
                    .expect("bench_segments")
                    .with_segments(segments.iter())
                    .expect("bench_segments")
                    .build()
                    .expect("bench_segments")
            };
        })
    });
}

#[cfg(test)]
pub fn bench_points(c: &mut Criterion) {
    c.bench_function("bench_points", |b| {
        b.iter(|| {
            // same test as cpp voronoi bench : voronoi_segment.txt
            let _output = {
                let input = include_str!("input_data/voronoi_point.txt");
                let br = BufReader::new(Cursor::new(input));
                let (points, segments) =
                    BV::read_boost_input_buffer::<I, _>(br).expect("bench_points");
                Builder::<I, F>::default()
                    .with_vertices(points.iter())
                    .expect("bench_points")
                    .with_segments(segments.iter())
                    .expect("bench_points")
                    .build()
                    .expect("bench_points")
            };
        })
    });
}

criterion_group! {name=benches1; config = Criterion::default().sample_size(40); targets=bench_segments,bench_points}
criterion_main!(benches1);
