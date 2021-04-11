// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.75.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

use super::diagram as VD;
pub use super::{InputType, OutputType};
use crate::BvError;
use std::marker::PhantomData;
use std::ops::Neg;

///! See <https://www.boost.org/doc/libs/1_75_0/libs/polygon/doc/voronoi_diagram.htm>

/// Sync version of the boostvoronoi::diagram::VoronoiDiagram struct.
/// This is useful when traversing the diagram in a multi threaded environment.
#[derive(Default, Debug)]
pub struct SyncVoronoiDiagram<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    pub cells: Vec<VD::VoronoiCell<I1, F1>>, // indexed by VoronoiCellIndex
    pub vertices: Vec<VD::VoronoiVertex<I1, F1>>, // indexed by VoronoiVertexIndex
    pub edges: Vec<VD::VoronoiEdge<I1, F1>>, // indexed by VoronoiEdgeIndex
}

impl<I1, F1> SyncVoronoiDiagram<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    /// Returns a reference to the list of cells
    #[inline]
    pub fn cells(&self) -> &Vec<VD::VoronoiCell<I1, F1>> {
        &self.cells
    }

    #[inline]
    /// Returns an edge iterator, the edges will all originate at the same vertex as 'edge_id'.
    ///  'edge_id' will be the first edge returned by the iterator.
    pub fn edge_rot_next_iterator(
        &self,
        edge_id: Option<VD::VoronoiEdgeIndex>,
    ) -> EdgeRotNextIterator<I1, F1> {
        EdgeRotNextIterator::new(self, edge_id)
    }

    #[inline]
    /// Returns a pointer to the rotation next edge
    /// over the starting point of the half-edge.
    pub fn edge_rot_next(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
    ) -> Result<Option<VD::VoronoiEdgeIndex>, BvError> {
        let prev_id = self.edge_get(edge_id)?.prev();
        if let Some(prev_id) = prev_id {
            let prev = self.edge_get(prev_id)?;
            Ok(prev.twin())
        } else {
            Err(BvError::IdError {
                txt: format!("The edge id {} does not have any prev edge", edge_id.0),
            })
        }
    }

    #[inline]
    /// Returns a pointer to the rotation next edge
    /// over the starting point of the half-edge.
    /// This method returns None at any error
    fn edge_rot_next_no_err(
        &self,
        edge_id: Option<VD::VoronoiEdgeIndex>,
    ) -> Option<VD::VoronoiEdgeIndex> {
        self.edges
            .get(self.edges.get(edge_id?.0)?.prev()?.0)?
            .twin()
    }

    #[inline]
    /// Returns a pointer to the rotation previous edge
    /// over the starting point of the half-edge.
    pub fn edge_rot_prev(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
    ) -> Result<Option<VD::VoronoiEdgeIndex>, BvError> {
        if let Some(twin_id) = self.edge_get(edge_id)?.twin() {
            let twin = self.edge_get(twin_id)?;
            Ok(twin.next())
        } else {
            Err(BvError::IdError {
                txt: format!("The edge id {} does not have any twin edge", edge_id.0),
            })
        }
    }

    /// Returns the next edge or an error
    #[inline]
    pub fn edge_get_next_err(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
    ) -> Result<VD::VoronoiEdgeIndex, BvError> {
        if let Some(edge_id) = self.edge_get(edge_id)?.next() {
            Ok(edge_id)
        } else {
            Err(BvError::ValueError {
                txt: format!("The edge id {} does not have a next edge", edge_id.0),
            })
        }
    }

    /// Returns the previous edge or an BvError if it does not exist
    #[inline]
    pub fn edge_get_prev_err(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
    ) -> Result<VD::VoronoiEdgeIndex, BvError> {
        if let Some(prev) = self.edge_get(edge_id)?.prev() {
            Ok(prev)
        } else {
            Err(BvError::ValueError {
                txt: format!("The edge id {} does not have a prev edge", edge_id.0),
            })
        }
    }

    /// Returns the twin edge as a Result or a BvError if it does not exists
    #[inline]
    pub fn edge_get_twin_err(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
    ) -> Result<VD::VoronoiEdgeIndex, BvError> {
        if let Some(twin_id) = self.edge_get(edge_id)?.twin() {
            Ok(twin_id)
        } else {
            Err(BvError::ValueError {
                txt: format!("The edge id {} does not have a twin", edge_id.0),
            })
        }
    }

    /// Returns true if the edge is finite (segment, parabolic arc).
    /// Returns false if the edge is infinite (ray, line).
    #[inline]
    pub fn edge_is_finite(&self, edge_id: VD::VoronoiEdgeIndex) -> Result<bool, BvError> {
        Ok(self.edge_get_vertex0(edge_id)?.is_some() && self.edge_get_vertex1(edge_id)?.is_some())
    }

    /// Returns true if the edge is infinite (ray, line).
    /// Returns false if the edge is finite (segment, parabolic arc).
    #[inline]
    pub fn edge_is_infinite(&self, edge_id: VD::VoronoiEdgeIndex) -> Result<bool, BvError> {
        Ok(!self.edge_is_finite(edge_id)?)
    }

    pub fn edges(&self) -> &Vec<VD::VoronoiEdge<I1, F1>> {
        &self.edges
    }

    #[inline]
    pub fn edge_get(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
    ) -> Result<&VD::VoronoiEdge<I1, F1>, BvError> {
        if let Some(edge) = self.edges.get(edge_id.0) {
            Ok(edge)
        } else {
            Err(BvError::IdError {
                txt: format!("The edge id {} does not exists", edge_id.0),
            })
        }
    }

    #[inline]
    pub fn edge_get_mut(
        &mut self,
        edge_id: VD::VoronoiEdgeIndex,
    ) -> Result<&mut VD::VoronoiEdge<I1, F1>, BvError> {
        if let Some(edge) = self.edges.get_mut(edge_id.0) {
            Ok(edge)
        } else {
            Err(BvError::IdError {
                txt: format!("The edge id {} does not exists", edge_id.0),
            })
        }
    }

    /// Returns the vertex0 of the edge
    #[inline]
    pub fn edge_get_vertex0(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
    ) -> Result<Option<VD::VoronoiVertexIndex>, BvError> {
        Ok(self.edge_get(edge_id)?.vertex0())
    }

    /// Returns the vertex1 of the edge
    #[inline]
    pub fn edge_get_vertex1(
        &self,
        edge_id: VD::VoronoiEdgeIndex,
    ) -> Result<Option<VD::VoronoiVertexIndex>, BvError> {
        let twin = self.edge_get(edge_id)?.twin().map_or(
            Err(BvError::IdError {
                txt: format!("the edge {} does not have any twin", edge_id.0),
            }),
            Ok,
        )?;
        self.edge_get_vertex0(twin)
    }

    #[inline]
    pub fn cell_get(
        &self,
        cell_id: VD::VoronoiCellIndex,
    ) -> Result<&VD::VoronoiCell<I1, F1>, BvError> {
        if let Some(cell) = self.cells.get(cell_id.0) {
            Ok(cell)
        } else {
            Err(BvError::IdError {
                txt: format!("The cell id {} does not exists", cell_id.0),
            })
        }
    }

    #[inline]
    /// Returns a reference to all of the vertices
    pub fn vertices(&self) -> &Vec<VD::VoronoiVertex<I1, F1>> {
        &self.vertices
    }

    #[inline]
    pub fn vertex_get(
        &self,
        vertex_id: VD::VoronoiVertexIndex,
    ) -> Result<&VD::VoronoiVertex<I1, F1>, BvError> {
        if let Some(vertex) = self.vertices.get(vertex_id.0) {
            Ok(vertex)
        } else {
            Err(BvError::IdError {
                txt: format!("The vertex id {} does not exists", vertex_id.0),
            })
        }
    }

    #[inline]
    pub fn vertex_get_mut(
        &mut self,
        vertex_id: VD::VoronoiVertexIndex,
    ) -> Result<&mut VD::VoronoiVertex<I1, F1>, BvError> {
        if let Some(vertex) = self.vertices.get_mut(vertex_id.0) {
            Ok(vertex)
        } else {
            Err(BvError::IdError {
                txt: format!("The vertex id {} does not exists", vertex_id.0),
            })
        }
    }
}

/// Iterator over edges pointing away from the vertex indicated by the initial edge.
/// edge.vertex()
pub struct EdgeRotNextIterator<'s, I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    diagram: &'s SyncVoronoiDiagram<I1, F1>,
    starting_edge: VD::VoronoiEdgeIndex,
    next_edge: Option<VD::VoronoiEdgeIndex>,
    #[doc(hidden)]
    _pdi: PhantomData<I1>,
    #[doc(hidden)]
    _pdf: PhantomData<F1>,
}

impl<'s, I1, F1> EdgeRotNextIterator<'s, I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    pub(crate) fn new(
        diagram: &'s SyncVoronoiDiagram<I1, F1>,
        starting_edge: Option<VD::VoronoiEdgeIndex>,
    ) -> Self {
        if let Some(starting_edge) = starting_edge {
            Self {
                diagram,
                starting_edge,
                next_edge: Some(starting_edge),
                _pdf: PhantomData,
                _pdi: PhantomData,
            }
        } else {
            Self {
                diagram,
                // Value does not matter next edge is None
                starting_edge: VD::VoronoiEdgeIndex(0),
                next_edge: None,
                _pdf: PhantomData,
                _pdi: PhantomData,
            }
        }
    }
}

impl<'s, I1, F1> Iterator for EdgeRotNextIterator<'s, I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    type Item = VD::VoronoiEdgeIndex;
    fn next(&mut self) -> Option<VD::VoronoiEdgeIndex> {
        let rv = self.next_edge;
        let new_next_edge = self.diagram.edge_rot_next_no_err(self.next_edge);
        self.next_edge = if let Some(nne) = new_next_edge {
            if nne.0 == self.starting_edge.0 {
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
