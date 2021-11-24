use boostvoronoi::prelude::*;

type I = i32; // this is the integer input type
type F = f64; // this is the float output type (circle event coordinates)

fn main() -> Result<(), BvError> {
    // Only unique Points will be used. Points should not intersect lines
    let p = vec!([9_i32, 10]);
    // Lines may only intersect at the endpoints.
    let s = vec!([10_i32, 11, 12, 33]);
    let diagram = Builder::<I, F>::default()
        // You will have to keep track of the input geometry. it will be referenced as
        // input geometry indices in the output.
        // `with_vertices()` accepts iterators of anything that implements
        // `Into<boostvoronoi::Point>`
        .with_vertices(p.iter())?
        // `with_segments()` accepts iterators of anything that implements
        // `Into<boostvoronoi::Line>`
        .with_segments(s.iter())?
        // this will generate the list of cells, edges and circle events (aka vertices)
        .build()?;
    println!(
        "Result: cells:{}, edges:{}, vertices:{}",
        diagram.cells().len(),
        diagram.edges().len(),
        diagram.vertices().len()
    );
    // The values inside the diagram are wrapped in `Rc<Cell<T>>`
    for cell in diagram.cell_iter().map(|c| c.get()) {
        println!("Cell : {}", cell.id().0);
        for edge_id in diagram.cell_edge_iterator(cell.id()) {
            let edge = diagram.get_edge(edge_id)?.get();
            // the vertices of an edge will have the value `None` if they are infinitely far away.
            println!(
                "  edge: {}, from:{:?} to:{:?}",
                edge_id.0,
                edge.vertex0(),
                diagram.edge_get_vertex1(edge_id)?
            );
        }
    }
    Ok(())
}
