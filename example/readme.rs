use boostvoronoi::builder::Builder;
use boostvoronoi::geometry::*;
use boostvoronoi::BvError;

type I = i32; // this is the integer input type
type F = f64; // this is the float output type (circle event coordinates)

/// This is the readme example
fn main() -> Result<(), BvError> {
    // Only unique Points will be used. Points should not intersect lines
    let p = vec![Point::from([9_i32, 10])];
    // Lines may only intersect at the endpoints.
    let s = vec![Line::from([10_i32, 11, 12, 33])];
    let mut vb = Builder::<I, F>::default();
    // you will have to keep track of the input geometry. it will be referenced as
    // input geometry indices in the output.
    vb.with_vertices(p.iter())?;
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
