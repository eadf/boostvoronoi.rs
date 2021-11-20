![Rusty voronoi](img/title.png)

[![crates.io](https://img.shields.io/crates/v/boostvoronoi.svg)](https://crates.io/crates/boostvoronoi)
[![Documentation](https://docs.rs/boostvoronoi/badge.svg)](https://docs.rs/boostvoronoi)
[![Workflow](https://github.com/eadf/boostvoronoi.rs/workflows/Rust/badge.svg)](https://github.com/eadf/boostvoronoi.rs/workflows/Rust/badge.svg)
[![Workflow](https://github.com/eadf/boostvoronoi.rs/workflows/Clippy/badge.svg)](https://github.com/eadf/boostvoronoi.rs/workflows/Clippy/badge.svg)
[![dependency status](https://deps.rs/crate/boostvoronoi/0.9.3/status.svg)](https://deps.rs/crate/boostvoronoi/0.9.3)
![license](https://img.shields.io/crates/l/boostvoronoi)

# Segmented Voronoi for Rust

[Boost 1.76.0 polygon::voronoi](https://www.boost.org/doc/libs/1_76_0/libs/polygon/doc/voronoi_main.htm) ported to 100% rust.
This implementation of [Fortune's algorithm](https://en.wikipedia.org/wiki/Fortune%27s_algorithm) works for line segments as well as points, making it useful for calculating centerlines [(like the title image above)](https://github.com/eadf/toxicblend.rs).

Code still in development, there are still bugs. However, all the remaining bugs I've noticed are also present in C++ boost voronoi.

Gui example:
```sh
cargo run --example fltk_gui
```
* Mouse click to place new points. 
* Press and hold 'L' + mouse click to add a single line. 
* Press and hold 'S' + mouse click to add strings of lines.
* Use mouse wheel to zoom.
* Mouse click and drag to pan.

API example:
```rust
use boostvoronoi::builder::Builder;
use boostvoronoi::BvError;

type I = i32; // this is the integer input type
type F = f64; // this is the float output type (circle event coordinates)

fn main() -> Result<(), BvError> {
    // Only unique Points will be used. Points should not intersect lines
    let p = vec![[9_i32, 10]];
    // Lines may only intersect at the endpoints.
    let s = vec![[10_i32, 11, 12, 33]];
    let mut vb = Builder::<I, F>::default();
    // you will have to keep track of the input geometry. it will be referenced as
    // input geometry indices in the output.
    // `with_vertices()` accepts iterators of anything that implements `Into<boostvoronoi::geometry::Point>`
    vb.with_vertices(p.iter())?;
    // `with_segments()` accepts iterators of anything that implements `Into<boostvoronoi::geometry::Line>`
    vb.with_segments(s.iter())?;
    // this will generate the list of cells, edges and circle events (aka vertices)
    let result = vb.build()?;
    println!(
        "Result: cells:{}, edges:{}, vertices:{}",
        result.cells().len(),
        result.edges().len(),
        result.vertices().len()
    );
    for cell in result.cell_iter() {
        let cell = cell.get(); // Get the std::cell:Cell value
        println!("Cell : {}", cell.id().0);
        for edge_id in result.cell_edge_iterator(cell.id()) {
            let edge = result.get_edge(edge_id)?.get();
            // the vertices of an edge will have the value None if they are infinitely far away.
            println!(
                "  edge: {}, from:{:?} to:{:?}",
                edge_id.0,
                edge.vertex0(),
                result.edge_get_vertex1(edge_id)?
            );
        }
    }
    Ok(())
}
```
Edges will become curves when line segments are used as input, see the example code for discretization and interpolation. 

## Rust toolchain
The crate is only tested on 1.56 and above.
The code uses ```#![feature(map_first_last)]``` if run on +nightly, this is only emulated on +stable.
So +nightly should be somewhat faster.

## Todo
- [ ] Try to fix the known problems in C++ Boost voronoi and port over.  
- [ ] Add many more test cases for voronoi_robust_ftp.rs.
- [ ] Benchmark and optimize.
- [ ] Replace C++ style boolean ordering functors.
- [ ] Replace builtin ulp with some rust crate (approx?).
- [ ] Take care of the "todo:" tags.

All credit goes to the original author ([Andrii Sydorchuk](https://github.com/asydorchuk)) and the [boost contributors](https://github.com/boostorg/polygon), except the porting mistakes. They are all mine.
