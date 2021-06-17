![Rusty voronoi](img/title.png)

[![Crates.io](https://meritbadge.herokuapp.com/boostvoronoi)](https://crates.io/crates/boostvoronoi)
[![Documentation](https://docs.rs/boostvoronoi/badge.svg)](https://docs.rs/boostvoronoi)
[![Workflow](https://github.com/eadf/boostvoronoi.rs/workflows/Rust/badge.svg)](https://github.com/eadf/boostvoronoi.rs/workflows/Rust/badge.svg)
[![Workflow](https://github.com/eadf/boostvoronoi.rs/workflows/Clippy/badge.svg)](https://github.com/eadf/boostvoronoi.rs/workflows/Clippy/badge.svg)
[![dependency status](https://deps.rs/crate/boostvoronoi/0.8.4/status.svg)](https://deps.rs/crate/boostvoronoi/0.8.4)

# Segmented Voronoi for Rust


[Boost 1.76.0 polygon::voronoi](https://www.boost.org/doc/libs/1_76_0/libs/polygon/doc/voronoi_main.htm) ported to 100% rust.
This implementation of [Fortune's algorithm](https://en.wikipedia.org/wiki/Fortune%27s_algorithm) works for line segments as well as points, making it useful for calculating centerlines [(like the title image above)](https://github.com/eadf/toxicblend.rs).

Code still in development, not ready for any purpose.

The code uses ```#![feature(map_first_last)]``` if run on +nightly, this is only emulated on +stable.
So +nightly should be somewhat faster.

![Rusty voronoi](img/img.png)

Gui example:
```fish
cargo run --example fltk_gui
```
* Mouse click to place new points. 
* Press and hold 'L' + mouse click to add a single line. 
* Press and hold 'S' + mouse click to add strings of lines.
* Use mouse wheel to zoom.
* Mouse click and drag to pan.

API example:
```rust
use boostvoronoi::{Point,Line};
use boostvoronoi::builder::Builder;
type I = i32; // this is the integer input type
type F = f64; // this is the float output type (circle event coordinates)

// Only unique Points will be used. Points should not intersect lines
let p = vec![Point{x:9_i32, y:10}];
// Lines may only intersect at the endpoints.
let s = vec![Line::new(Point{x:10_i32, y:11}, Point{x:12, y:13})];
let mut vb = Builder::<I, F>::default();

// you will have to keep track of the input geometry. it will be referenced as
// input geometry indices in the output.
vb.with_vertices(p.iter())?;
vb.with_segments(s.iter())?;

// this will generate the list of cells, edges and circle events (aka vertices)
let result = vb.construct()?;
```
Edges may become curves when line segments are used as input, see the example code for discretization and interpolation. 

## Todo
- [x] Fix the degenerate vertex key problem
- [ ] Fix the beach-line key problem
- [x] Error handling
- [X] Evaluate the generic API. Is <I1, F1, I2, F2> really needed?
- [ ] Verify the builtin ulp implementation
- [x] Replace num::BigInt with something lighter
- [ ] Add many more test cases for voronoi_robust_ftp.rs, specially for ulp
- [x] Remove use of vec_map::VecMap where not absolutely needed.
- [ ] Benchmark and optimize
- [x] Example GUI with more features. fltk?
- [x] Fix the beach-line bug found with main.rs example

All credit goes to the original author ([Andrii Sydorchuk](https://github.com/asydorchuk)) and the [boost contributors](https://github.com/boostorg/polygon), except the porting mistakes. They are all mine.

