//          Copyright Eadf (github.com/eadf) 2021.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! A Sync version of the output data.
//! See <https://www.boost.org/doc/libs/1_76_0/libs/polygon/doc/voronoi_diagram.htm> for diagram description.

use crate::diagram as VD;
use crate::BvError;
pub use crate::{InputType, OutputType};
use std::marker::PhantomData;

/// Sync version of the boostvoronoi::diagram::VoronoiDiagram struct.
/// This is useful when traversing the diagram in a multi threaded environment.
#[derive(Default, Debug)]
pub struct SyncDiagram<I: InputType, F: OutputType> {
    cells_: Vec<VD::Cell<I, F>>,      // indexed by CellIndex
    vertices_: Vec<VD::Vertex<I, F>>, // indexed by VertexIndex
    edges_: Vec<VD::Edge<I, F>>,      // indexed by EdgeIndex
}

impl<I: InputType, F: OutputType> SyncDiagram<I, F> {
    pub fn new(
        cells: Vec<VD::Cell<I, F>>,
        vertices: Vec<VD::Vertex<I, F>>,
        edges: Vec<VD::Edge<I, F>>,
    ) -> Self {
        Self {
            cells_: cells,
            vertices_: vertices,
            edges_: edges,
        }
    }

    /// Returns a reference to the list of cells
    #[inline]
    pub fn cells(&self) -> &Vec<VD::Cell<I, F>> {
        &self.cells_
    }

    #[inline]
    /// Returns an edge iterator, the edges will all originate at the same vertex as 'edge_id'.
    /// 'edge_id' will be the first edge returned by the iterator.
    pub fn edge_rot_next_iterator(&self, edge_id: VD::EdgeIndex) -> EdgeRotNextIterator<'_, I, F> {
        EdgeRotNextIterator::new(self, edge_id)
    }

    #[inline]
    /// Returns a pointer to the rotation next edge
    /// over the starting point of the half-edge.
    pub fn edge_rot_next(&self, edge_id: VD::EdgeIndex) -> Result<VD::EdgeIndex, BvError> {
        let prev_id = self.edge_get(edge_id)?.prev()?;
        self.edge_get(prev_id)?.twin()
    }

    #[inline]
    /// Returns a pointer to the rotation next edge
    /// over the starting point of the half-edge.
    /// This method returns None at any error
    fn edge_rot_next_no_err(&self, edge_id: Option<VD::EdgeIndex>) -> Option<VD::EdgeIndex> {
        self.edges_
            .get(self.edges_.get(edge_id?.0)?.prev_()?.0)?
            .twin_()
    }

    #[inline]
    /// Returns a pointer to the rotation previous edge
    /// over the starting point of the half-edge.
    pub fn edge_rot_prev(&self, edge_id: VD::EdgeIndex) -> Result<VD::EdgeIndex, BvError> {
        self.edge_get(self.edge_get(edge_id)?.twin()?)?.next()
    }

    /// Returns the next edge or an error
    #[inline]
    pub fn edge_get_next(&self, edge_id: VD::EdgeIndex) -> Result<VD::EdgeIndex, BvError> {
        self.edge_get(edge_id)?.next()
    }

    /// Returns the previous edge or an BvError if it does not exist
    #[inline]
    pub fn edge_get_prev(&self, edge_id: VD::EdgeIndex) -> Result<VD::EdgeIndex, BvError> {
        self.edge_get(edge_id)?.prev_().ok_or_else(|| {
            BvError::ValueError(format!(
                "The edge id {} does not have a prev edge",
                edge_id.0
            ))
        })
    }

    /// Returns the twin edge or a BvError if it does not exists
    #[inline]
    pub fn edge_get_twin(&self, edge_id: VD::EdgeIndex) -> Result<VD::EdgeIndex, BvError> {
        self.edge_get(edge_id)?.twin_().ok_or_else(|| {
            BvError::ValueError(format!("The edge id {} does not have a twin", edge_id.0))
        })
    }

    /// Returns true if the edge is finite (segment, parabolic arc).
    /// Returns false if the edge is infinite (ray, line).
    #[inline]
    pub fn edge_is_finite(&self, edge_id: VD::EdgeIndex) -> Result<bool, BvError> {
        Ok(self.edge_get_vertex0(edge_id)?.is_some() && self.edge_get_vertex1(edge_id)?.is_some())
    }

    /// Returns true if the edge is infinite (ray, line).
    /// Returns false if the edge is finite (segment, parabolic arc).
    #[inline]
    pub fn edge_is_infinite(&self, edge_id: VD::EdgeIndex) -> Result<bool, BvError> {
        Ok(!self.edge_is_finite(edge_id)?)
    }

    pub fn edges(&self) -> &Vec<VD::Edge<I, F>> {
        &self.edges_
    }

    #[inline]
    pub fn edge_get(&self, edge_id: VD::EdgeIndex) -> Result<&VD::Edge<I, F>, BvError> {
        self.edges_
            .get(edge_id.0)
            .ok_or_else(|| BvError::IdError(format!("The edge id {} does not exists", edge_id.0)))
    }

    #[inline]
    pub fn edge_get_mut(&mut self, edge_id: VD::EdgeIndex) -> Result<&mut VD::Edge<I, F>, BvError> {
        self.edges_
            .get_mut(edge_id.0)
            .ok_or_else(|| BvError::IdError(format!("The edge id {} does not exists", edge_id.0)))
    }

    /// Returns the optional vertex0 of the edge
    #[inline]
    pub fn edge_get_vertex0(
        &self,
        edge_id: VD::EdgeIndex,
    ) -> Result<Option<VD::VertexIndex>, BvError> {
        Ok(self.edge_get(edge_id)?.vertex0())
    }

    /// Returns the optional vertex1 of the edge
    #[inline]
    pub fn edge_get_vertex1(
        &self,
        edge_id: VD::EdgeIndex,
    ) -> Result<Option<VD::VertexIndex>, BvError> {
        self.edge_get_vertex0(self.edge_get(edge_id)?.twin()?)
    }

    #[inline]
    pub fn cell_get(&self, cell_id: VD::CellIndex) -> Result<&VD::Cell<I, F>, BvError> {
        self.cells_
            .get(cell_id.0)
            .ok_or_else(|| BvError::IdError(format!("The cell id {} does not exists", cell_id.0)))
    }

    #[inline]
    /// Returns a reference to all of the vertices
    pub fn vertices(&self) -> &Vec<VD::Vertex<I, F>> {
        &self.vertices_
    }

    #[inline]
    /// Returns a reference to a vertex
    pub fn vertex_get(&self, vertex_id: VD::VertexIndex) -> Result<&VD::Vertex<I, F>, BvError> {
        self.vertices_.get(vertex_id.0).ok_or_else(|| {
            BvError::IdError(format!("The vertex id {} does not exists", vertex_id.0))
        })
    }

    #[inline]
    /// Returns a mutable reference to a vertex
    pub fn vertex_get_mut(
        &mut self,
        vertex_id: VD::VertexIndex,
    ) -> Result<&mut VD::Vertex<I, F>, BvError> {
        self.vertices_.get_mut(vertex_id.0).ok_or_else(|| {
            BvError::IdError(format!("The vertex id {} does not exists", vertex_id.0))
        })
    }
}

/// Iterator over edges pointing away from the vertex indicated by the initial edge.
/// edge.vertex()
pub struct EdgeRotNextIterator<'s, I: InputType, F: OutputType> {
    diagram_: &'s SyncDiagram<I, F>,
    starting_edge_: VD::EdgeIndex,
    next_edge_: Option<VD::EdgeIndex>,
    #[doc(hidden)]
    pdi_: PhantomData<I>,
    #[doc(hidden)]
    pdf_: PhantomData<F>,
}

impl<'s, I: InputType, F: OutputType> EdgeRotNextIterator<'s, I, F> {
    pub(crate) fn new(diagram: &'s SyncDiagram<I, F>, starting_edge: VD::EdgeIndex) -> Self {
        Self {
            diagram_: diagram,
            starting_edge_: starting_edge,
            next_edge_: Some(starting_edge),
            pdf_: PhantomData,
            pdi_: PhantomData,
        }
    }
}

impl<'s, I: InputType, F: OutputType> Iterator for EdgeRotNextIterator<'s, I, F> {
    type Item = VD::EdgeIndex;
    fn next(&mut self) -> Option<VD::EdgeIndex> {
        let rv = self.next_edge_;
        let new_next_edge = self.diagram_.edge_rot_next_no_err(self.next_edge_);
        self.next_edge_ = if let Some(nne) = new_next_edge {
            if nne.0 == self.starting_edge_.0 {
                // Break the loop when we see starting edge again
                None
            } else {
                new_next_edge
            }
        } else {
            None
        };
        rv
    }
}
