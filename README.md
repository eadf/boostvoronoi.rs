[![Crates.io](https://meritbadge.herokuapp.com/boostvoronoi)](https://crates.io/crates/boostvoronoi)
[![Documentation](https://docs.rs/boostvoronoi/badge.svg)](https://docs.rs/boostvoronoi)
[![Workflow](https://github.com/eadf/boostvoronoi.rs/workflows/Rust/badge.svg)](https://github.com/eadf/boostvoronoi.rs/workflows/Rust/badge.svg)
[![Workflow](https://github.com/eadf/boostvoronoi.rs/workflows/Clippy/badge.svg)](https://github.com/eadf/boostvoronoi.rs/workflows/Clippy/badge.svg)
[![dependency status](https://deps.rs/crate/boostvoronoi/0.6.0/status.svg)](https://deps.rs/crate/boostvoronoi/0.6.0)

# Segmented Voronoi for Rust
[Boost 1.75.0 polygon::voronoi](https://www.boost.org/doc/libs/1_75_0/libs/polygon/doc/voronoi_main.htm) ported to 100% rust.
This implementation of [Fortune's algorithm](https://en.wikipedia.org/wiki/Fortune%27s_algorithm) works on line segments as well as points, making it useful for calculating centerlines.

Code still in development, not ready for any purpose.

# Note
The code uses ```#![feature(map_first_last)]``` i.e. rust nightly.

![Rusty voronoi](img.png)

Gui example:
```fish
cargo run --example fltk_gui
```
Mouse click to place new points. Press and hold 'L' + mouse click to add single line. 
\
Press and hold 'S' + mouse click to add strings of lines.

API example:
```rust
use boostvoronoi::{Point,Line};
use boostvoronoi::builder::{Builder};
type I1 = i32; // this is the integer input type
type F1 = f64; // this is the float output type (circle event coordinates)

// Points should be unique,
let p = vec![Point{x:9_i32, y:10}];
// Lines should never intersect with other lines.
// The only points that can intersect are the endpoints.
let s = vec![Line::new(Point{x:10_i32, y:11}, Point{x:12, y:13})];
let mut vb = Builder::<I1, F1>::new();

// you will have to keep track of the input geometry. it will be referenced as
// input geometry indices in the output.
vb.with_vertices(p.iter()).unwrap();
vb.with_segments(s.iter()).unwrap();

// this will generate a the list of cells, edges and circle events (aka vertices)
let result = vb.construct().unwrap();
```
Edges may become curves when line segments are used as input, see the example code for discretization and interpolation. 

## Todo
- [ ] Error handling
- [X] Evaluate the generic API. Is <I1, F1, I2, F2> really needed?
- [ ] ~Replace~ Verify the builtin ulp implementation
- [x] Replace num::BigInt with something lighter
- [ ] Add many more test cases for voronoi_robust_ftp.rs, specially for ulp
- [x] Remove use of vec_map::VecMap where not absolutely needed.
- [ ] Benchmark and optimize
- [x] Example GUI with more features. fltk?
- [x] Fix the beach-line bug found with main.rs example

All credit goes to the original author ([Andrii Sydorchuk](https://github.com/asydorchuk)) and the [boost contributors](https://github.com/boostorg/polygon), except the porting mistakes. They are all mine.

