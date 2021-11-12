// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code..

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! A std::cell::Cell based version of the output data.
//! See <https://www.boost.org/doc/libs/1_76_0/libs/polygon/doc/voronoi_diagram.htm> for diagram description.

use crate::circle_event as VC;
use crate::ctypes as CT;
use crate::site_event as VSE;
use crate::visual_utils as VU;
use crate::TypeConverter2 as TC2;
use crate::{sync_diagram as SD, BvError};

#[allow(unused_imports)]
use crate::{t, tln};
pub use crate::{InputType, OutputType};
use num::NumCast;
use std::cell;
use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

pub type SourceIndex = usize;

///! See <https://www.boost.org/doc/libs/1_76_0/libs/polygon/doc/voronoi_diagram.htm>

/// Typed container for cell indices
#[derive(Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct CellIndex(pub usize);

impl fmt::Debug for CellIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CellIndex({})", self.0)
    }
}

/// Typed container for edge indices
#[derive(Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct EdgeIndex(pub usize);

impl fmt::Debug for EdgeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EdgeIndex({})", self.0)
    }
}

/// Typed container for vertex indices
#[derive(Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct VertexIndex(pub usize);

impl fmt::Debug for VertexIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VertexIndex({})", self.0)
    }
}

pub type ColorType = u32;

/// Represents category of the input source that forms Voronoi cell.
/// Todo: sort out all of these bits, seems like they overlap in functionality
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct ColorBits(pub ColorType);

#[allow(clippy::upper_case_acronyms)]
impl ColorBits {
    pub(crate) const ZERO: Self = ColorBits(0x0);
    // Point subtypes.
    pub(crate) const SINGLE_POINT__BIT: Self = ColorBits(0x0); // 0b_00000000
    pub(crate) const SEGMENT_START_POINT__BIT: Self = ColorBits(0x1); // 0b_00000001
    pub(crate) const SEGMENT_END_POINT__BIT: Self = ColorBits(0x2); // 0b_00000010
    /// Vertex subtype (does not exists not in c++ boost)
    pub(crate) const SITE_VERTEX__BIT: Self = ColorBits(0x4); // 0b_00000100

    // Segment subtypes.
    pub(crate) const INITIAL_SEGMENT: Self = ColorBits(0x8); // 0b1_00001000
                                                             // todo: not used for anything?
    pub(crate) const REVERSE_SEGMENT: Self = ColorBits(0x9); // 0b1_00001001

    /// 5 color bits are reserved for internal use.
    pub(crate) const RESERVED_BITS__SHIFT: Self = Self(0x5);
    /// Used for clearing custom color
    pub(crate) const RESERVED__MASK: Self = ColorBits(0x1F); // 0b_00011111

    // todo: why have a GEOMETRY_SHIFT when GEOMETRY_CATEGORY_POINT and GEOMETRY_CATEGORY_SEGMENT could just indicate the bits directly?
    pub(crate) const GEOMETRY__SHIFT: Self = ColorBits(0x3);
    pub(crate) const GEOMETRY_CATEGORY_POINT__BIT: Self = ColorBits(0x0); // 0b_00000000
    pub(crate) const GEOMETRY_CATEGORY_SEGMENT__BIT: Self = ColorBits(0x1); // 0b_00000001

    /// Used on the site points (beach-line keys etc.) value exceeds the reserved bit field,
    /// but these site points are not public.
    pub(crate) const IS_INVERSE__BIT: Self = Self(0x20); // 0b_00100000

    // todo: remove this
    pub(crate) const TEMPORARY_CELL: Self = ColorBits(u32::MAX << ColorBits::GEOMETRY__SHIFT.0);
}

/// Represents the type of input geometry a cell was created from
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SourceCategory {
    SinglePoint,
    SegmentStart,
    SegmentEnd,
    Segment,
}

/// Represents Voronoi cell.
/// Data members:
///   1) index of the source within the initial input set
///   2) id of the incident edge
///   3) mutable color member
/// Cell may contain point or segment site inside.
///
/// TODO! fix the name confusion "initial index" & "source index" referring to the same thing.
#[derive(Copy, Clone)]
pub struct Cell {
    // sorted_index of the site event
    id_: CellIndex,
    // source_index/initial_index of the site event
    source_index_: SourceIndex,
    incident_edge_: Option<EdgeIndex>,
    color_: ColorType,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(id:{:?} ii:{} ie:{} col:{})",
            self.id_.0,
            self.source_index_,
            super::format_id(self.incident_edge_.map(|x| x.0)),
            self.color_
        )
    }
}

impl Cell {
    pub fn new(id: CellIndex, source_index: SourceIndex, source_category: ColorType) -> Self {
        Cell {
            id_: id,
            source_index_: source_index,
            incident_edge_: None,
            color_: source_category,
        }
    }

    #[inline(always)]
    pub(crate) fn internal_color(&self) -> ColorBits {
        ColorBits(self.color_ & ColorBits::RESERVED__MASK.0)
    }

    #[inline(always)]
    pub fn source_category(&self) -> SourceCategory {
        match self.internal_color() {
            ColorBits::SINGLE_POINT__BIT => SourceCategory::SinglePoint,
            ColorBits::SEGMENT_START_POINT__BIT => SourceCategory::SegmentStart,
            ColorBits::SEGMENT_END_POINT__BIT => SourceCategory::SegmentEnd,
            _ => SourceCategory::Segment,
        }
    }

    /// Returns true if the cell contains point site, false else.
    #[inline(always)]
    pub fn contains_point(&self) -> bool {
        let geometry = self.internal_color().0 >> ColorBits::GEOMETRY__SHIFT.0;
        geometry == ColorBits::GEOMETRY_CATEGORY_POINT__BIT.0
    }

    /// Returns true if the cell contains segment site, false otherwise.
    #[inline(always)]
    pub fn contains_segment(&self) -> bool {
        let geometry = self.internal_color().0 >> ColorBits::GEOMETRY__SHIFT.0;
        geometry == ColorBits::GEOMETRY_CATEGORY_SEGMENT__BIT.0
    }

    /// Returns true if the cell contains segment start point, false otherwise.
    #[inline(always)]
    pub fn contains_segment_startpoint(&self) -> bool {
        self.internal_color().0 == ColorBits::SEGMENT_START_POINT__BIT.0
    }

    /// Returns true if the cell contains segment end point, false otherwise.
    #[inline(always)]
    pub fn contains_segment_endpoint(&self) -> bool {
        self.internal_color().0 == ColorBits::SEGMENT_END_POINT__BIT.0
    }

    #[inline(always)]
    pub fn id(&self) -> CellIndex {
        self.id_
    }

    /// Returns the origin index of the cell.
    #[inline(always)]
    pub fn source_index(&self) -> SourceIndex {
        self.source_index_
    }

    /// Returns the origin index of the point that created this cell.
    /// It also returns the source category
    #[inline(always)]
    pub fn source_index_2(&self) -> (SourceIndex, SourceCategory) {
        (self.source_index_, self.source_category())
    }

    /// Degenerate cells don't have any incident edges.
    pub fn is_degenerate(&self) -> bool {
        self.incident_edge_.is_none()
    }

    /// returns a random edge defined by this cell.
    #[inline(always)]
    pub fn get_incident_edge(&self) -> Option<EdgeIndex> {
        self.incident_edge_
    }
}

/// Iterator over edges of a Cell
/// Do *NOT* use this while altering the std::cell::Cell values of next, prev or twin edges.
pub struct EdgeNextIterator<'s, F: OutputType> {
    diagram_: &'s Diagram<F>,
    start_edge_: EdgeIndex,
    next_edge_: Option<EdgeIndex>,
}

impl<'s, F: OutputType> EdgeNextIterator<'s, F> {
    pub(crate) fn new(diagram: &'s Diagram<F>, starting_edge: Option<EdgeIndex>) -> Self {
        if let Some(starting_edge) = starting_edge {
            Self {
                diagram_: diagram,
                start_edge_: starting_edge,
                next_edge_: Some(starting_edge),
            }
        } else {
            Self {
                diagram_: diagram,
                // Value does not matter next edge is None
                start_edge_: EdgeIndex(0),
                next_edge_: None,
            }
        }
    }
}

impl<'s, F: OutputType> Iterator for EdgeNextIterator<'s, F> {
    type Item = EdgeIndex;
    fn next(&mut self) -> Option<EdgeIndex> {
        let rv = self.next_edge_;
        let new_next_edge = self.diagram_.edge_get_next_(self.next_edge_);

        self.next_edge_ = if let Some(nne) = new_next_edge {
            if nne.0 == self.start_edge_.0 {
                // Break the loop when we see starting edge again
                None
            } else {
                Some(nne)
            }
        } else {
            None
        };
        rv
    }
}

/// Iterator over edges pointing away from the vertex indicated by the initial edge.
/// edge.vertex()
/// Do *NOT* use this when altering the std::cell::Cell values of next, prev or twin edges.
pub struct EdgeRotNextIterator<'s, F: OutputType> {
    diagram_: &'s Diagram<F>,
    start_edge: EdgeIndex,
    next_edge: Option<EdgeIndex>,
}

impl<'s, F: OutputType> EdgeRotNextIterator<'s, F> {
    pub(crate) fn new(diagram: &'s Diagram<F>, starting_edge: Option<EdgeIndex>) -> Self {
        if let Some(starting_edge) = starting_edge {
            Self {
                diagram_: diagram,
                start_edge: starting_edge,
                next_edge: Some(starting_edge),
            }
        } else {
            Self {
                diagram_: diagram,
                // Value does not matter; next edge is None
                start_edge: EdgeIndex(0),
                next_edge: None,
            }
        }
    }
}

impl<'s, F: OutputType> Iterator for EdgeRotNextIterator<'s, F> {
    type Item = EdgeIndex;
    fn next(&mut self) -> Option<EdgeIndex> {
        let rv = self.next_edge;
        let new_next_edge = self.diagram_.edge_rot_next_(self.next_edge);
        self.next_edge = if let Some(nne) = new_next_edge {
            if nne.0 == self.start_edge.0 {
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

/// Iterator over edges pointing away from the vertex indicated by the initial edge.
/// edge.vertex()
/// Do *NOT* use this when altering the std::cell::Cell values of next, prev or twin edges.
pub struct EdgeRotPrevIterator<'s, F: OutputType> {
    diagram_: &'s Diagram<F>,
    start_edge: EdgeIndex,
    next_edge: Option<EdgeIndex>,
}

impl<'s, F: OutputType> EdgeRotPrevIterator<'s, F> {
    #[allow(dead_code)]
    pub(crate) fn new(diagram: &'s Diagram<F>, starting_edge: Option<EdgeIndex>) -> Self {
        if let Some(starting_edge) = starting_edge {
            Self {
                diagram_: diagram,
                start_edge: starting_edge,
                next_edge: Some(starting_edge),
            }
        } else {
            Self {
                diagram_: diagram,
                // Value does not matter next edge is None
                start_edge: EdgeIndex(0),
                next_edge: None,
            }
        }
    }
}

impl<'s, F: OutputType> Iterator for EdgeRotPrevIterator<'s, F> {
    type Item = EdgeIndex;
    fn next(&mut self) -> Option<EdgeIndex> {
        let rv = self.next_edge;
        let new_next_edge = self.diagram_.edge_rot_prev(self.next_edge);
        self.next_edge = if let Some(nne) = new_next_edge {
            if nne.0 == self.start_edge.0 {
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

/// Represents Voronoi vertex aka. Circle event.
/// Data members:
///   1) vertex coordinates
///   2) id of the incident edge
///   3) mutable color member
#[derive(Copy, Clone)]
pub struct Vertex<F: OutputType> {
    pub(crate) id_: VertexIndex,
    pub(crate) x_: F,
    pub(crate) y_: F,
    pub(crate) incident_edge_: Option<EdgeIndex>,
    pub(crate) color_: ColorType,
}

impl<F: OutputType> fmt::Debug for Vertex<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(id:{} x:{} y:{} ie:{} co:{})",
            self.id_.0,
            self.x_,
            self.y_,
            super::format_id(self.incident_edge_.map(|x| x.0)),
            self.color_
        )
    }
}

impl<F: OutputType> Vertex<F> {
    pub fn new_3(id: VertexIndex, x: F, y: F, is_site_vertex: bool) -> Rc<cell::Cell<Vertex<F>>> {
        let color = if is_site_vertex {
            ColorBits::SITE_VERTEX__BIT.0
        } else {
            ColorBits::ZERO.0
        };
        Rc::new(cell::Cell::new(Self {
            id_: id,
            x_: x,
            y_: y,
            incident_edge_: None,
            color_: color,
        }))
    }

    fn vertex_equality_predicate_eq(&self, other: &Self) -> bool {
        let ulp = 128;
        let x1: f64 = NumCast::from(self.x()).unwrap();
        let y1: f64 = NumCast::from(self.y()).unwrap();
        let x2: f64 = NumCast::from(other.x()).unwrap();
        let y2: f64 = NumCast::from(other.y()).unwrap();

        CT::UlpComparison::ulp_comparison(x1, x2, ulp) == Ordering::Equal
            && CT::UlpComparison::ulp_comparison(y1, y2, ulp) == Ordering::Equal
    }

    pub fn get_id(&self) -> VertexIndex {
        self.id_
    }

    #[inline]
    #[cfg(feature = "console_debug")]
    pub(crate) fn get_incident_edge_(&self) -> Option<EdgeIndex> {
        self.incident_edge_
    }

    #[inline]
    pub fn get_incident_edge(&self) -> Result<EdgeIndex, BvError> {
        self.incident_edge_.ok_or_else(|| {
            BvError::InternalError("Vertex didn't have an incident_edge".to_string())
        })
    }

    /// returns the x coordinate of the circle event
    #[inline]
    pub fn x(&self) -> F {
        self.x_
    }

    /// returns the x coordinate of the circle event
    #[inline]
    pub fn y(&self) -> F {
        self.y_
    }

    /// get_color returns the custom edge info. (does not contain the reserved bits)
    pub fn get_color(&self) -> ColorType {
        self.color_ >> ColorBits::RESERVED_BITS__SHIFT.0
    }

    /// set_color sets the custom edge info. (does not affect the reserved bits)
    pub fn set_color(&mut self, color: ColorType) -> ColorType {
        self.color_ &= ColorBits::RESERVED__MASK.0;
        self.color_ |= color << ColorBits::RESERVED_BITS__SHIFT.0;
        self.color_
    }

    /// or_color sets the custom vertex info together with the previous value. (does not affect the reserved bits)
    /// This is a Cell operation, remember to set() the entire cell
    #[inline(always)]
    pub fn or_color(&mut self, color: ColorType) -> ColorType {
        self.set_color(self.get_color() | color)
    }

    /// Returns true if this vertex coincides with an input site.
    #[inline]
    pub fn is_site_point(&self) -> bool {
        (self.color_ & ColorBits::SITE_VERTEX__BIT.0) != 0
    }
}

/// Half-edge data structure. Represents a Voronoi edge.
/// Data members:
///   1) id of the corresponding cell
///   2) id of to the vertex that is the starting
///      point of the half-edge (optional)
///   3) id of to the twin edge
///   4) id of of the CCW next edge
///   5) id of to the CCW prev edge
///   6) mutable color member
#[derive(Copy, Clone)]
pub struct Edge {
    id_: EdgeIndex,
    cell_: Option<CellIndex>,
    vertex_: Option<VertexIndex>,
    twin_: Option<EdgeIndex>,
    next_ccw_: Option<EdgeIndex>,
    prev_ccw_: Option<EdgeIndex>,
    color_: ColorType,
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id:{} cell:{} v0:{} t:{} n:{} p:{} c:{}",
            self.id_.0,
            super::format_id(self.cell_.map(|c| c.0)),
            super::format_id(self.vertex_.map(|v| v.0)),
            super::format_id(self.twin_.map(|e| e.0)),
            super::format_id(self.next_ccw_.map(|e| e.0)),
            super::format_id(self.prev_ccw_.map(|e| e.0)),
            self.color_
        )
    }
}

impl Edge {
    // todo: this is super suspicious, doesn't this collide with SEGMENT_START_POINT & SEGMENT_END_POINT?
    const BIT_IS_LINEAR: ColorType = 0x1; // linear is opposite to curved
    const BIT_IS_PRIMARY: ColorType = 0x2; // primary is opposite to secondary

    fn new_(id: EdgeIndex, cell: CellIndex, is_linear: bool, is_primary: bool) -> EdgeType {
        let mut rv = Self {
            id_: id,
            cell_: Some(cell),
            vertex_: None,
            twin_: None,
            next_ccw_: None,
            prev_ccw_: None,
            color_: 0,
        };
        if is_linear {
            rv.color_ |= Self::BIT_IS_LINEAR;
        }
        if is_primary {
            rv.color_ |= Self::BIT_IS_PRIMARY;
        }
        Rc::from(cell::Cell::from(rv))
    }

    /// Returns the edge index
    pub fn id(&self) -> EdgeIndex {
        self.id_
    }

    pub(crate) fn cell_(&self) -> Option<CellIndex> {
        self.cell_
    }

    /// Returns the cell index of this edge, or a BvError
    pub fn cell(&self) -> Result<CellIndex, BvError> {
        self.cell_.ok_or_else(|| {
            BvError::ValueError("Edge didn't have any valid cell associated to it.".to_string())
        })
    }

    /// Returns vertex0, it is perfectly ok for an edge to not contain a vertex0 so no
    /// Result<..> is needed here.
    pub fn vertex0(&self) -> Option<VertexIndex> {
        self.vertex_
    }

    /// Returns the twin edge
    pub(crate) fn twin_(&self) -> Option<EdgeIndex> {
        self.twin_
    }

    /// Returns the twin edge or an error
    pub fn twin(&self) -> Result<EdgeIndex, BvError> {
        self.twin_.ok_or_else(|| {
            BvError::ValueError(
                "Edge didn't have any valid twin edge associated to it.".to_string(),
            )
        })
    }

    /// returns the next edge (counter clockwise winding)
    pub(crate) fn next_(&self) -> Option<EdgeIndex> {
        self.next_ccw_
    }

    /// returns the next edge (counter clockwise winding) or an error
    pub fn next(&self) -> Result<EdgeIndex, BvError> {
        self.next_ccw_.ok_or_else(|| {
            BvError::ValueError(format!(
                "Edge {} didn't have any valid next edge associated to it. {}:{}",
                self.id_.0,
                file!(),
                line!()
            ))
        })
    }

    /// returns the previous edge (counter clockwise winding)
    pub(crate) fn prev_(&self) -> Option<EdgeIndex> {
        self.prev_ccw_
    }

    /// returns the previous edge (counter clockwise winding)
    pub fn prev(&self) -> Result<EdgeIndex, BvError> {
        self.prev_().ok_or_else(|| {
            BvError::InternalError("The edge does not have a previous edge".to_string())
        })
    }

    /// Returns true if the edge is linear (segment, ray, line).
    /// Returns false if the edge is curved (parabolic arc).
    #[inline]
    pub fn is_linear(&self) -> bool {
        (self.color_ & Self::BIT_IS_LINEAR) != 0
    }

    /// Returns true if the edge is curved (parabolic arc).
    /// Returns false if the edge is linear (segment, ray, line).
    #[inline]
    pub fn is_curved(&self) -> bool {
        !self.is_linear()
    }

    /// Returns false if edge goes through the endpoint of the segment.
    /// Returns true else.
    #[inline]
    pub fn is_primary(&self) -> bool {
        (self.color_ & Self::BIT_IS_PRIMARY) != 0
    }

    /// Returns true if edge goes through the endpoint of the segment.
    /// Returns false else.
    #[inline]
    pub fn is_secondary(&self) -> bool {
        !self.is_primary()
    }

    /// get_color returns the custom edge info. (does not contain the reserved bits)
    #[inline(always)]
    pub fn get_color(&self) -> ColorType {
        self.color_ >> ColorBits::RESERVED_BITS__SHIFT.0
    }

    /// set_color sets the custom edge info. (does not affect the reserved bits)
    #[inline(always)]
    pub fn set_color(&mut self, color: ColorType) -> ColorType {
        self.color_ &= ColorBits::RESERVED__MASK.0;
        self.color_ |= color << ColorBits::RESERVED_BITS__SHIFT.0;
        self.color_
    }

    /// or_color sets the custom edge info together with the previous value. (does not affect the reserved bits)
    #[inline(always)]
    pub fn or_color(&mut self, color: ColorType) -> ColorType {
        self.set_color(self.get_color() | color)
    }
}

pub type CellType = Rc<cell::Cell<Cell>>;
pub type EdgeType = Rc<cell::Cell<Edge>>;
pub type VertexType<F> = Rc<cell::Cell<Vertex<F>>>;

/// Voronoi output data structure.
/// CCW ordering is used on the faces perimeter and around the vertices.
/// Mandatory reading: <https://www.boost.org/doc/libs/1_76_0/libs/polygon/doc/voronoi_diagram.htm>
#[derive(Default, Debug)]
pub struct Diagram<F: OutputType> {
    cells_: Vec<CellType>,         // indexed by CellIndex
    vertices_: Vec<VertexType<F>>, // indexed by VertexIndex
    edges_: Vec<EdgeType>,         // indexed by EdgeIndex
}

impl<F: OutputType> Diagram<F> {
    pub fn new(input_size: usize) -> Self {
        Self {
            cells_: Vec::<CellType>::with_capacity(input_size),
            vertices_: Vec::<VertexType<F>>::with_capacity(input_size),
            edges_: Vec::<EdgeType>::with_capacity(input_size * 2),
        }
    }

    /// clear the list of cells, vertices and edges
    pub fn clear(&mut self) {
        self.cells_.clear();
        self.vertices_.clear();
        self.edges_.clear();
    }

    #[inline(always)]
    /// Returns a reference to the list of cells
    pub fn cells(&self) -> &Vec<CellType> {
        &self.cells_
    }

    #[inline(always)]
    /// Returns a reference to all of the vertices
    pub fn vertices(&self) -> &Vec<VertexType<F>> {
        &self.vertices_
    }

    #[inline(always)]
    /// Computes an AABB large enough to contain all the vertices
    pub fn vertices_get_aabb<I: InputType>(&self) -> VU::Aabb2<F> {
        let mut rv = VU::Aabb2::<F>::default();
        for v in self.vertices_.iter() {
            let v = v.get();
            rv.update_vertex(v.x(), v.y());
        }
        rv
    }

    #[inline(always)]
    /// Returns a reference to the list of edges
    pub fn edges(&self) -> &Vec<EdgeType> {
        &self.edges_
    }

    #[inline(always)]
    /// Returns a Rc<cell::Cell<>> belonging to the cell_id
    pub fn get_cell(&self, cell_id: CellIndex) -> Result<Rc<cell::Cell<Cell>>, BvError> {
        Ok(Rc::clone(self.cells_.get(cell_id.0).ok_or_else(|| {
            BvError::IdError(format!("The cell with id:{} does not exist", cell_id.0))
        })?))
    }

    #[inline(always)]
    /// Returns the edge associated with the edge id
    pub(crate) fn get_edge_(&self, edge_id: Option<EdgeIndex>) -> Option<EdgeType> {
        let edge_id = edge_id?;
        self.edges_.get(edge_id.0).map(|x| Rc::clone(x))
    }

    /// Returns the edge associated with the edge id
    pub fn get_edge(&self, edge_id: EdgeIndex) -> Result<EdgeType, BvError> {
        if let Some(edge) = self.edges_.get(edge_id.0) {
            Ok(Rc::clone(edge))
        } else {
            Err(BvError::IdError(format!(
                "The edge with id:{} does not exist",
                edge_id.0
            )))
        }
    }

    /// Return the edge represented as an straight line
    /// if the edge does not exists or if it lacks v0 or v1; None will be returned.
    /// TODO: this looks like an into() candidate
    pub(crate) fn edge_as_line_(&self, edge: Option<EdgeIndex>) -> Option<[F; 4]> {
        let v0 = self.vertex_get_(self.edge_get_vertex0_(edge));
        let v1 = self.vertex_get_(self.edge_get_vertex1_(edge));
        if let Some(v0) = v0 {
            if let Some(v1) = v1 {
                let v0 = v0.get();
                let v1 = v1.get();
                return Some([v0.x(), v0.y(), v1.x(), v1.y()]);
            }
        }
        None
    }

    /// Return the edge represented as an straight line
    /// if the edge does not exists or if it lacks v0 or v1; None will be returned.
    #[inline]
    pub fn edge_as_line(&self, edge_id: EdgeIndex) -> Result<[F; 4], BvError> {
        self.edge_as_line_(Some(edge_id)).ok_or_else(|| {
            BvError::IdError(format!(
                "Edge id:{} (probably) does not exists, or some vertex is missing",
                edge_id.0
            ))
        })
    }

    /// Iterates over all edges, colors each edge as exterior if it has an unbroken primary edge
    /// link connection to an infinite edge.
    pub fn color_exterior_edges(&self, external_color: ColorType) {
        for it in self.edges().iter() {
            let edge_id = Some(it.get().id());
            if !self.edge_is_finite_(edge_id).unwrap() {
                self.recurse_color_exterior(edge_id, external_color);
            }
        }
    }

    /// Mark all edges connected to an 'infinite' edge via primary edges as 'external'
    /// You should only call this on edges that you know are infinite. i.e. lacks one or two vertexes
    fn recurse_color_exterior(&self, edge_id: Option<EdgeIndex>, external_color: ColorType) {
        if edge_id.is_none() || (self.edge_get_color_(edge_id).unwrap() & external_color) != 0 {
            // This edge has already been colored, break recursion
            return;
        }
        // Color edge as EXTERNAL
        self.edge_or_color_(edge_id, external_color);

        let v1 = self.edge_get_vertex1_(edge_id);
        if self.edge_get_vertex0_(edge_id).is_some() && v1.is_none() {
            // this edge leads to nowhere, break recursion
            return;
        }
        // Color twin edge as EXTERNAL
        self.edge_or_color_(self.edge_get_twin_(edge_id), external_color);
        if v1.is_none()
            || self.vertex_is_site_point_(v1).unwrap_or(true)
            || !self
                .get_edge_(edge_id)
                .map_or(false, |x| x.get().is_primary())
        {
            // stop recursion if this edge does not have a vertex1 (e.g is infinite)
            // or if this edge isn't a primary edge.
            return;
        }
        self.vertex_set_color_(v1, external_color);
        let incident_edge = self.vertex_get_incident_edge(v1);
        for e in self.edge_rot_next_iterator_(incident_edge) {
            // mark all surrounding edges as EXTERNAL, but only recurse on primary edges
            self.recurse_color_exterior(Some(e), external_color);
        }
    }

    /// Returns an iterator over all cells
    pub fn cell_iter(&self) -> core::slice::Iter<'_, CellType> {
        self.cells_.iter()
    }

    /// Returns an iterator over all vertices
    pub fn vertex_iter(&self) -> core::slice::Iter<'_, VertexType<F>> {
        self.vertices_.iter()
    }

    /// Returns an iterator over all edges
    pub fn edge_iter(&self) -> std::slice::Iter<'_, EdgeType> {
        self.edges_.iter()
    }

    /// push a new cell on the output. Nothing but id and source category is initialized
    fn make_new_cell_with_category_(
        &mut self,
        cell_id: CellIndex, // same as sorted_index
        initial_index: SourceIndex,
        sc: ColorBits,
    ) -> CellIndex {
        // fill cell with temporary blocks- they will be over-written later
        // Todo: fix this dirty hack with Option<>
        while self.cells_.len() < cell_id.0 {
            self.cells_.push(Rc::new(cell::Cell::new(Cell::new(
                CellIndex(usize::MAX),
                usize::MAX,
                ColorBits::TEMPORARY_CELL.0,
            ))));
        }
        self.cells_.push(Rc::new(cell::Cell::new(Cell::new(
            cell_id,
            initial_index,
            sc.0,
        ))));
        #[cfg(feature = "console_debug")]
        assert_eq!(self.cells_[cell_id.0].get().id().0, cell_id.0);

        let ccell = &self.cells_[cell_id.0];
        {
            let cell = ccell.get();
            #[cfg(feature = "console_debug")]
            {
                assert_eq!(cell.id_.0, cell_id.0);
                assert_eq!(cell.source_index_, initial_index);
                assert_eq!(cell.color_, sc.0);
            }
            //cell.color_ = sc.get_value();
            ccell.set(cell);
        }
        cell_id
    }

    /// returns the number of cells in the diagram
    pub fn num_cells(&self) -> usize {
        self.cells_.len()
    }

    /// returns the number of edges in the diagram
    pub fn num_edges(&self) -> usize {
        self.edges_.len()
    }

    /// returns the number of vertices in the diagram
    pub fn num_vertices(&self) -> usize {
        self.vertices_.len()
    }

    /// reserves space for an number of additional sites
    pub fn reserve_(&mut self, additional_sites: usize) {
        self.cells_.reserve(additional_sites);
        self.vertices_.reserve(additional_sites << 1);
        self.edges_
            .reserve((additional_sites << 2) + (additional_sites << 1));
    }

    pub(crate) fn process_single_site_<I: InputType>(&mut self, site: &VSE::SiteEvent<I, F>) {
        let _ = self.make_new_cell_with_category_(
            CellIndex(site.sorted_index()),
            site.initial_index(),
            site.source_category(),
        );
    }

    #[inline]
    fn cell_get_(&self, cell_id: Option<CellIndex>) -> Option<&CellType> {
        let _ = cell_id?;
        self.cells_.get(cell_id.unwrap().0)
    }

    fn cell_set_incident_edge_(&self, cell_id: Option<CellIndex>, edge: Option<EdgeIndex>) {
        if cell_id.is_none() {
            return;
        }
        if let Some(cell) = self.cell_get_(cell_id) {
            let mut c = cell.get();
            c.incident_edge_ = edge;
            cell.set(c)
        }
    }

    fn cell_get_incident_edge_(&self, cell_id: Option<CellIndex>) -> Option<EdgeIndex> {
        let _ = cell_id?;
        if let Some(cell) = self.cell_get_(cell_id) {
            return cell.get().incident_edge_;
        }
        None
    }

    fn cell_is_degenerate_(&self, cell_id: Option<CellIndex>) -> bool {
        if cell_id.is_none() {
            return false;
        }
        if let Some(cell) = self.cell_get_(cell_id) {
            return cell.get().is_degenerate();
        }
        false
    }

    /// Returns an edge iterator. This iterates over the edges belonging to this cell starting with
    /// the incident edge.
    pub fn cell_edge_iterator(&self, cell_id: CellIndex) -> EdgeNextIterator<'_, F> {
        self.cell_edge_iterator_(Some(cell_id))
    }

    /// Returns an edge iterator. This iterates over the edges belonging to this cell starting with
    /// the incident edge.
    fn cell_edge_iterator_(&self, cell_id: Option<CellIndex>) -> EdgeNextIterator<'_, F> {
        let incident_edge = self.cell_get_incident_edge_(cell_id);
        EdgeNextIterator::<'_, F>::new(self, incident_edge)
    }

    /// Returns the vertex associated with the vertex_id
    #[inline]
    pub(crate) fn vertex_get_(&self, vertex_id: Option<VertexIndex>) -> Option<&VertexType<F>> {
        let _ = vertex_id?;
        self.vertices_.get(vertex_id.unwrap().0)
    }

    /// Returns the vertex associated with the vertex_id
    #[inline]
    pub fn vertex_get(&self, vertex_id: VertexIndex) -> Result<&VertexType<F>, BvError> {
        self.vertex_get_(Some(vertex_id))
            .ok_or_else(|| BvError::IdError(format!("Vertex id {} does not exists.", vertex_id.0)))
    }

    #[inline]
    /// OR the previous color field value with this new color value
    pub(crate) fn vertex_or_color_(&self, vertex_id: Option<VertexIndex>, color: ColorType) {
        if vertex_id.is_none() {
            return;
        }
        if let Some(vertexcell) = self.vertex_get_(vertex_id) {
            let mut vertex = vertexcell.get();
            let _ = vertex.or_color(color);
            vertexcell.set(vertex);
        }
    }

    #[inline]
    /// OR the previous color field value with this new color value
    pub fn vertex_or_color(&self, vertex_id: VertexIndex, color: ColorType) {
        self.vertex_or_color_(Some(vertex_id), color)
    }

    /// Returns the color field of the vertex.
    pub fn vertex_get_color(&self, vertex_id: Option<VertexIndex>) -> Option<ColorType> {
        let _ = vertex_id?;
        if let Some(vertexcell) = self.vertex_get_(vertex_id) {
            let vertex = vertexcell.get();
            return Some(vertex.get_color());
        }
        None
    }

    /// Overwrites the content of dest with the content of source.
    /// edge_id is compensated accordingly
    fn vertex_copy_(&self, dest: usize, source: usize) {
        let mut v = self.vertices_[source].get();
        v.id_ = VertexIndex(dest);
        self.vertices_[dest].set(v);
    }

    fn vertex_set_incident_edge_(&self, vertex_id: Option<VertexIndex>, edge: Option<EdgeIndex>) {
        if vertex_id.is_none() {
            return;
        }
        if let Some(vertex) = self.vertex_get_(vertex_id) {
            let mut c = vertex.get();
            c.incident_edge_ = edge;
            vertex.set(c)
        }
    }

    /// return one of the edges originating at the vertex
    pub fn vertex_get_incident_edge(&self, vertex_id: Option<VertexIndex>) -> Option<EdgeIndex> {
        let _ = vertex_id?;
        self.vertex_get_(vertex_id)
            .and_then(|x| x.get().incident_edge_)
    }

    /// Set the color of the vertex. This affects only the public bits, not the internal
    pub(crate) fn vertex_set_color_(&self, vertex_id: Option<VertexIndex>, color: ColorType) {
        if let Some(vertex_cell) = self.vertex_get_(vertex_id) {
            let mut vertex = vertex_cell.get();
            let _ = vertex.set_color(color);
            vertex_cell.set(vertex);
        }
    }

    /// Set the color of the vertex. This affects only the public bits, not the internal
    pub fn vertex_set_color(
        &self,
        vertex_id: VertexIndex,
        color: ColorType,
    ) -> Result<(), BvError> {
        self.vertex_set_color_(Some(vertex_id), color);
        Ok(())
    }

    /// returns true if this vertex coincides with an site point
    #[inline]
    pub(crate) fn vertex_is_site_point_(&self, vertex_id: Option<VertexIndex>) -> Option<bool> {
        let _ = vertex_id?;
        self.vertex_get_(vertex_id)
            .map(|cell| cell.get().is_site_point())
    }

    /// returns true if this vertex coincides with an site point
    #[inline]
    pub fn vertex_is_site_point(&self, vertex_id: VertexIndex) -> Result<bool, BvError> {
        self.vertex_is_site_point_(Some(vertex_id)).ok_or_else(|| {
            BvError::IdError(format!(
                "Vertex id {} (probably) does not exists",
                vertex_id.0
            ))
        })
    }

    /// Create and insert a new edge
    fn create_and_insert_edge(
        &mut self,
        cell_id: CellIndex,
        is_linear: bool,
        is_primary: bool,
    ) -> EdgeIndex {
        let new_edge_id = EdgeIndex(self.edges_.len());
        let new_edge = Edge::new_(new_edge_id, cell_id, is_linear, is_primary);
        let _ = self.edges_.push(new_edge);
        tln!("Created and inserted new edge : e={}", new_edge_id.0);
        new_edge_id
    }

    #[inline]
    pub(crate) fn edge_get_(&self, edge_id: Option<EdgeIndex>) -> Option<&EdgeType> {
        let rv = self.edges_.get(edge_id?.0);
        if rv.is_none() {
            dbg!(edge_id.unwrap().0);
            // todo: remove this panic and raise error?
            panic!();
        }
        rv
    }

    /// Overwrites the content of dest with the content of source.
    /// edge_id of the new dest is corrected
    pub(crate) fn edge_copy_(&self, dest: usize, source: usize) {
        let mut e = self.edges_[source].get();
        e.id_ = EdgeIndex(dest);
        self.edges_[dest].set(e);
    }

    #[inline]
    /// Returns the color field of the edge.
    pub(crate) fn edge_get_color_(&self, edge_id: Option<EdgeIndex>) -> Option<ColorType> {
        let _ = edge_id?;
        if let Some(edgecell) = self.edge_get_(edge_id) {
            let edge = edgecell.get();
            return Some(edge.get_color());
        }
        None
    }

    #[inline]
    /// Returns the color field of the edge.
    pub fn edge_get_color(&self, edge_id: EdgeIndex) -> Result<ColorType, BvError> {
        self.edge_get_color_(Some(edge_id)).ok_or_else(|| {
            BvError::IdError(format!("edge id {} (probably) does not exists", edge_id.0))
        })
    }

    #[inline]
    /// Sets the color field with new value
    pub(crate) fn edge_set_color_(&self, edge_id: Option<EdgeIndex>, color: ColorType) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self.edge_get_(edge_id) {
            let mut edge = edgecell.get();
            let _ = edge.set_color(color);
            edgecell.set(edge);
        }
    }

    #[inline]
    /// Sets the color field with new value
    // todo: raise proper error when id not found
    pub fn edge_set_color(&self, edge_id: EdgeIndex, color: ColorType) -> Result<(), BvError> {
        self.edge_set_color_(Some(edge_id), color);
        Ok(())
    }

    #[inline]
    /// OR the previous color field value with this new color value
    pub(crate) fn edge_or_color_(&self, edge_id: Option<EdgeIndex>, color: ColorType) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self.edge_get_(edge_id) {
            let mut edge = edgecell.get();
            let _ = edge.or_color(color);
            edgecell.set(edge);
        }
    }

    #[inline]
    /// OR the previous color field value with this new color value
    // Todo: add error on edge index problems
    pub fn edge_or_color(&self, edge_id: EdgeIndex, color: ColorType) -> Result<(), BvError> {
        self.edge_or_color_(Some(edge_id), color);
        Ok(())
    }

    #[inline]
    pub(crate) fn edge_set_twin_(&self, edge_id: Option<EdgeIndex>, twin_id: Option<EdgeIndex>) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self.edge_get_(edge_id) {
            let mut edge = edgecell.get();
            edge.twin_ = twin_id;
            edgecell.set(edge);
        }
    }

    #[inline]
    /// Returns an edge iterator, the edges will all originate at the same vertex as 'edge_id'.
    ///  'edge_id' will be the first edge returned by the iterator.
    /// Do *NOT* use this when altering next, prev or twin edges.
    pub(crate) fn edge_rot_next_iterator_(
        &self,
        edge_id: Option<EdgeIndex>,
    ) -> EdgeRotNextIterator<'_, F> {
        EdgeRotNextIterator::new(self, edge_id)
    }

    #[inline]
    /// Returns an edge iterator, the edges will all originate at the same vertex as 'edge_id'.
    ///  'edge_id' will be the first edge returned by the iterator.
    /// Do *NOT* use this when altering next, prev or twin edges.
    pub fn edge_rot_next_iterator(&self, edge_id: EdgeIndex) -> EdgeRotNextIterator<'_, F> {
        self.edge_rot_next_iterator_(Some(edge_id))
    }

    #[inline]
    /// Returns an edge iterator, the edges will all originate at the same vertex as 'edge_id'.
    ///  'edge_id' will be the first edge returned by the iterator.
    /// Do *NOT* use this when altering next, prev or twin edges.
    pub(crate) fn edge_rot_prev_iterator_(
        &self,
        edge_id: Option<EdgeIndex>,
    ) -> EdgeRotPrevIterator<'_, F> {
        EdgeRotPrevIterator::new(self, edge_id)
    }

    #[inline]
    /// Returns an edge iterator, the edges will all originate at the same vertex as 'edge_id'.
    ///  'edge_id' will be the first edge returned by the iterator.
    /// Do *NOT* use this when altering next, prev or twin edges.
    pub fn edge_rot_prev_iterator(&self, edge_id: EdgeIndex) -> EdgeRotPrevIterator<'_, F> {
        self.edge_rot_prev_iterator_(Some(edge_id))
    }

    #[inline]
    pub(crate) fn edge_get_twin_(&self, edge_id: Option<EdgeIndex>) -> Option<EdgeIndex> {
        let _ = edge_id?;
        if let Some(edgecell) = self.edge_get_(edge_id) {
            return edgecell.get().twin_();
        }
        None
    }

    #[inline]
    pub fn edge_get_twin(&self, edge_id: EdgeIndex) -> Result<EdgeIndex, BvError> {
        self.edge_get_twin_(Some(edge_id))
            .ok_or_else(|| BvError::IdError(format!("Edge {} does not have a twin", edge_id.0)))
    }

    #[inline]
    pub fn edge_get_next(&self, edge_id: EdgeIndex) -> Result<EdgeIndex, BvError> {
        self.edge_get_next_(Some(edge_id)).ok_or_else(|| {
            BvError::IdError(format!("Edge {} did not have any next edge", edge_id.0))
        })
    }

    #[inline]
    fn edge_set_cell_(&self, edge_id: Option<EdgeIndex>, cell_id: Option<CellIndex>) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self.edge_get_(edge_id) {
            let mut edge = edgecell.get();
            edge.cell_ = cell_id;
            edgecell.set(edge);
        }
    }

    #[inline]
    fn edge_get_cell_(&self, edge_id: Option<EdgeIndex>) -> Option<CellIndex> {
        let _ = edge_id?;
        self.edge_get_(edge_id)?.get().cell_()
    }

    #[inline]
    pub fn edge_get_cell(&self, edge_id: EdgeIndex) -> Result<CellIndex, BvError> {
        self.edge_get_cell_(Some(edge_id)).ok_or_else(|| {
            BvError::IdError(format!(
                "Either the edge id:{} or the cell didn't exists",
                edge_id.0
            ))
        })
    }

    /// Returns true if the edge is finite (segment, parabolic arc).
    /// Returns false if the edge is infinite (ray, line).
    #[inline]
    pub(crate) fn edge_is_finite_(&self, edge_id: Option<EdgeIndex>) -> Option<bool> {
        let _ = edge_id?;
        Some(self.edge_get_vertex0_(edge_id).is_some() && self.edge_get_vertex1_(edge_id).is_some())
    }

    /// Returns true if the edge is finite (segment, parabolic arc).
    /// Returns false if the edge is infinite (ray, line).
    #[inline]
    pub fn edge_is_finite(&self, edge_id: EdgeIndex) -> Result<bool, BvError> {
        self.edge_is_finite_(Some(edge_id))
            .ok_or_else(|| BvError::IdError(format!("Edge id {} doesn't exists", edge_id.0)))
    }

    /// Returns true if the edge is infinite (ray, line).
    /// Returns false if the edge is finite (segment, parabolic arc).
    #[inline]
    pub(crate) fn edge_is_infinite_(&self, edge_id: Option<EdgeIndex>) -> Option<bool> {
        Some(!self.edge_is_finite_(edge_id)?)
    }

    /// Returns true if the edge is infinite (ray, line).
    /// Returns false if the edge is finite (segment, parabolic arc).
    #[inline]
    pub fn edge_is_infinite(&self, edge_id: EdgeIndex) -> Result<bool, BvError> {
        self.edge_is_infinite_(Some(edge_id))
            .ok_or_else(|| BvError::IdError(format!("Edge id {} doesn't exists", edge_id.0)))
    }

    /// Remove degenerate edge.
    fn remove_edge_(&mut self, edge: Option<EdgeIndex>) {
        #[cfg(feature = "console_debug")]
        if let Some(edge_id) = edge {
            tln!("removing edge: {}", edge_id.0);
        } else {
            tln!("removing edge: but it was None!");
            return;
        }
        // Update the endpoints of the incident edges to the second vertex.
        let vertex = self.edge_get_vertex0_(edge);
        let mut updated_edge = self.edge_rot_next_(self.edge_get_twin_(edge));

        while updated_edge != self.edge_get_twin_(edge) {
            self.edge_set_vertex0_(updated_edge, vertex);
            updated_edge = self.edge_rot_next_(updated_edge);
        }
        let edge1 = edge;
        let edge2 = self.edge_get_twin_(edge);

        // Update prev/next pointers for the incident edges.
        //edge1_rot_next->twin()->next(edge2_rot_prev);
        self.edge_set_next_(
            self.edge_get_twin_(self.edge_rot_next_(edge1)),
            self.edge_rot_prev(edge2),
        );
        //edge2_rot_prev->prev(edge1_rot_next->twin());
        self.edge_set_prev_(
            self.edge_rot_prev(edge2),
            self.edge_get_twin_(self.edge_rot_next_(edge1)),
        );

        //edge1_rot_prev->prev(edge2_rot_next->twin());
        self.edge_set_prev_(
            self.edge_rot_prev(edge1),
            self.edge_get_twin_(self.edge_rot_next_(edge2)),
        );

        //edge2_rot_next->twin()->next(edge1_rot_prev);
        self.edge_set_next_(
            self.edge_get_twin_(self.edge_rot_next_(edge2)),
            self.edge_rot_prev(edge1),
        );
    }

    fn vertex_new_2_(&mut self, x: F, y: F, is_site_vertex: bool) -> VertexIndex {
        let new_vertex_id = VertexIndex(self.vertices_.len());
        let new_edge = Vertex::new_3(new_vertex_id, x, y, is_site_vertex);
        let _ = self.vertices_.push(new_edge);
        #[cfg(feature = "console_debug")]
        assert_eq!(self.vertices_.len() - 1, new_vertex_id.0);
        new_vertex_id
    }

    fn edge_set_vertex0_(&self, edge_id: Option<EdgeIndex>, vertex_id: Option<VertexIndex>) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self.edge_get_(edge_id) {
            let mut edge = edgecell.get();
            edge.vertex_ = vertex_id;
            edgecell.set(edge);
        }
    }

    #[inline]
    pub(crate) fn edge_get_vertex0_(&self, edge_id: Option<EdgeIndex>) -> Option<VertexIndex> {
        let _ = edge_id?;
        self.edge_get_(edge_id).and_then(|x| x.get().vertex0())
    }

    #[inline]
    // todo: add error when edge is not found
    pub fn edge_get_vertex0(&self, edge_id: EdgeIndex) -> Result<Option<VertexIndex>, BvError> {
        Ok(self.edge_get_vertex0_(Some(edge_id)))
    }

    #[inline]
    pub(crate) fn edge_get_vertex1_(&self, edge_id: Option<EdgeIndex>) -> Option<VertexIndex> {
        let _ = edge_id?;
        let twin = self.edge_get_twin_(edge_id);
        self.edge_get_vertex0_(twin)
    }

    #[inline]
    // todo: add error when edge is not found
    pub fn edge_get_vertex1(&self, edge_id: EdgeIndex) -> Result<Option<VertexIndex>, BvError> {
        Ok(self.edge_get_vertex1_(Some(edge_id)))
    }

    #[inline]
    fn edge_set_prev_(&self, edge_id: Option<EdgeIndex>, prev_id: Option<EdgeIndex>) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self.edge_get_(edge_id) {
            let mut edge = edgecell.get();
            edge.prev_ccw_ = prev_id;
            edgecell.set(edge);
        }
    }

    #[inline]
    fn edge_set_next_(&self, edge_id: Option<EdgeIndex>, next_id: Option<EdgeIndex>) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self.edge_get_(edge_id) {
            let mut edge = edgecell.get();
            edge.next_ccw_ = next_id;
            edgecell.set(edge);
        }
    }

    #[inline]
    // todo replace with _edge_get_next
    fn edge_get_next_(&self, edge_id: Option<EdgeIndex>) -> Option<EdgeIndex> {
        let _ = edge_id?;
        self.edges_
            .get(edge_id.unwrap().0)
            .and_then(|x| x.get().next_())
    }

    #[inline]
    fn edge_get_prev_(&self, edge_id: Option<EdgeIndex>) -> Option<EdgeIndex> {
        let _ = edge_id?;
        self.edges_
            .get(edge_id.unwrap().0)
            .and_then(|x| x.get().prev_())
    }

    #[inline]
    /// Returns a pointer to the rotation next edge
    /// over the starting point of the half-edge.
    pub(crate) fn edge_rot_next_(&self, edge_id: Option<EdgeIndex>) -> Option<EdgeIndex> {
        let _ = edge_id?;
        let prev = self.edge_get_prev_(edge_id);
        self.edge_get_twin_(prev)
    }

    #[inline]
    /// Returns a pointer to the rotation next edge
    /// over the starting point of the half-edge.
    pub fn edge_rot_next(&self, edge_id: EdgeIndex) -> Result<EdgeIndex, BvError> {
        self.edge_rot_next_(Some(edge_id)).ok_or_else(|| {
            BvError::IdError(format!("Edge id {} (probably) doesn't exists", edge_id.0))
        })
    }

    #[inline]
    /// Returns a pointer to the rotation previous edge
    /// over the starting point of the half-edge.
    pub fn edge_rot_prev(&self, edge_id: Option<EdgeIndex>) -> Option<EdgeIndex> {
        let _ = edge_id?;
        let twin = self.edge_get_twin_(edge_id);
        self.edge_get_next_(twin)
    }

    /// Insert a new half-edge into the output data structure.
    /// Takes as input left and right sites that form a new bisector.
    /// Returns a pair of pointers to new half-edges.
    pub(crate) fn insert_new_edge_2_<I: InputType>(
        &mut self,
        site1: VSE::SiteEvent<I, F>,
        site2: VSE::SiteEvent<I, F>,
    ) -> (EdgeIndex, EdgeIndex) {
        //tln!("-> insert_new_edge_2()");
        //tln!("site1:{:?}\nsite2:{:?}", &site1, &site2);
        // Get sites' indexes.
        let site1_index = site1.sorted_index();
        let site2_index = site2.sorted_index();

        let is_linear = VSE::SiteEvent::is_linear_edge(&site1, &site2);
        let is_primary = VSE::SiteEvent::is_primary_edge(&site1, &site2);

        // Create a new half-edge that belongs to the first site.
        let edge1_id = self.create_and_insert_edge(CellIndex(site1_index), is_linear, is_primary);

        // Create a new half-edge that belongs to the second site.
        let edge2_id = self.create_and_insert_edge(CellIndex(site2_index), is_linear, is_primary);

        // Add the initial cell during the first edge insertion.
        if self.cells_.is_empty() {
            let _ = self.make_new_cell_with_category_(
                CellIndex(site1_index),
                site1.initial_index(),
                site1.source_category(),
            );
        }

        // The second site represents a new site during site event
        // processing. Add a new cell to the cell records.
        let _ = self.make_new_cell_with_category_(
            CellIndex(site2_index),
            site2.initial_index(),
            site2.source_category(),
        );

        // Set up pointers to cells. Todo! is this needed? Didn't we do this already?
        self.edge_set_cell_(Some(edge1_id), Some(CellIndex(site1_index)));
        self.edge_set_cell_(Some(edge2_id), Some(CellIndex(site2_index)));

        // Set up twin pointers.
        self.edge_set_twin_(Some(edge1_id), Some(edge2_id));
        self.edge_set_twin_(Some(edge2_id), Some(edge1_id));

        //tln!("edge1: {:?}", self.get_edge_(edge1_id).get());
        //tln!("edge2: {:?}", self.get_edge_(edge2_id).get());
        //tln!("edges.len():{}", self.edges_.len());
        (edge1_id, edge2_id)
    }

    /// Insert a new half-edge into the output data structure with the
    /// start at the point where two previously added half-edges intersect.
    /// Takes as input two sites that create a new bisector, circle event
    /// that corresponds to the intersection point of the two old half-edges,
    /// pointers to those half-edges. Half-edges' direction goes out of the
    /// new Voronoi vertex point. Returns a pair of pointers to a new half-edges.
    pub(crate) fn insert_new_edge_5_<I: InputType>(
        &mut self,
        site1: VSE::SiteEvent<I, F>,
        site3: VSE::SiteEvent<I, F>,
        circle: VC::CircleEvent,
        edge12_id: EdgeIndex,
        edge23_id: EdgeIndex,
    ) -> (EdgeIndex, EdgeIndex) {
        /*tln!("-> insert_new_edge_5()");
        tln!(
            "site1:{:?}\nsite3:{:?}\ncircle:{:?}\nedge12_id:{:?}\nedge23_id{:?}\n",
            &site1,
            &site3,
            &circle,
            edge12_id,
            edge23_id
        );*/
        tln!("new vertex@CE{:?}", circle);

        let is_linear = VSE::SiteEvent::<I, F>::is_linear_edge(&site1, &site3);
        let is_primary = VSE::SiteEvent::<I, F>::is_primary_edge(&site1, &site3);

        // Add a new half-edge.
        let new_edge1_id =
            self.create_and_insert_edge(CellIndex(site1.sorted_index()), is_linear, is_primary);

        // Add a new half-edge.
        let new_edge2_id =
            self.create_and_insert_edge(CellIndex(site3.sorted_index()), is_linear, is_primary);

        // Add a new Voronoi vertex.
        let new_vertex_id = self.vertex_new_2_(
            TC2::<I, F>::f64_to_f(circle.raw_x()),
            TC2::<I, F>::f64_to_f(circle.raw_y()),
            circle.is_site_point(),
        );

        // Update vertex pointers of the old edges.
        self.edge_set_vertex0_(Some(edge12_id), Some(new_vertex_id));
        self.edge_set_vertex0_(Some(edge23_id), Some(new_vertex_id));

        // Update twin pointers.
        self.edge_set_twin_(Some(new_edge1_id), Some(new_edge2_id));
        self.edge_set_twin_(Some(new_edge2_id), Some(new_edge1_id));

        // Update vertex pointer.
        //new_edge2.vertex0(&new_vertex);
        self.edge_set_vertex0_(Some(new_edge2_id), Some(new_vertex_id));

        // Update Voronoi prev/next pointers.
        //edge12->prev(&new_edge1);
        self.edge_set_prev_(Some(edge12_id), Some(new_edge1_id));

        //new_edge1.next(edge12);
        self.edge_set_next_(Some(new_edge1_id), Some(edge12_id));

        //edge12->twin()->next(edge23);
        let edge12_twin_id = self.edge_get_twin_(Some(edge12_id));
        self.edge_set_next_(edge12_twin_id, Some(edge23_id));

        //edge23->prev(edge12->twin());
        self.edge_set_prev_(Some(edge23_id), edge12_twin_id);

        //edge23->twin()->next(&new_edge2);
        let edge23_twin_id = self.edge_get_twin_(Some(edge23_id));
        self.edge_set_next_(edge23_twin_id, Some(new_edge2_id));

        //new_edge2.prev(edge23->twin());
        self.edge_set_prev_(Some(new_edge2_id), edge23_twin_id);

        //tln!("edge12: {:?}", self.get_edge_(edge12_id).get());
        //tln!("edge23: {:?}", self.get_edge_(edge23_id).get());
        //tln!("edges.len():{}", self.edges_.len());
        // Return a pointer to the new half-edge.
        (new_edge1_id, new_edge2_id)
    }

    /// Make sure the diagram is consistent. Removes degenerate edges, connects incident
    /// edges etc. etc
    pub(crate) fn build_(&mut self) {
        // Remove degenerate edges.
        #[cfg(feature = "console_debug")]
        self.debug_print_edges("b4 degenerate");
        if !self.edges_.is_empty() {
            let mut last_edge: usize = 0;
            let mut it: usize = last_edge;
            let edges_end: usize = self.edges_.len();

            //let mut edges_to_erase: Vec<usize> = Vec::new();
            while it < edges_end {
                let is_equal = {
                    let v1 = self.edge_get_vertex0_(Some(EdgeIndex(it)));
                    let v1 = self.vertex_get_(v1);
                    let v2 = self.edge_get_vertex1_(Some(EdgeIndex(it)));
                    let v2 = self.vertex_get_(v2);
                    //tln!("looking at edge:{}, v1={:?}, v2={:?}", it, v1, v2);
                    v1.is_some()
                        && v2.is_some()
                        && v1
                            .unwrap()
                            .get()
                            .vertex_equality_predicate_eq(&v2.unwrap().get())
                };

                if is_equal {
                    self.remove_edge_(Some(EdgeIndex(it)));
                } else {
                    if it != last_edge {
                        //edge_type * e1 = &(*last_edge = *it);
                        self.edge_copy_(last_edge, it);
                        //edge_type * e2 = &(*(last_edge + 1) = *(it + 1));
                        self.edge_copy_(last_edge + 1, it + 1);
                        let e1 = Some(EdgeIndex(last_edge));
                        let e2 = Some(EdgeIndex(last_edge + 1));

                        // e1->twin(e2);
                        self.edge_set_twin_(e1, e2);

                        // e2->twin(e1);
                        self.edge_set_twin_(e2, e1);

                        if self.edge_get_prev_(e1).is_some() {
                            // e1 -> prev() -> next(e1);
                            self.edge_set_next_(self.edge_get_prev_(e1), e1);

                            //e2 -> next() -> prev(e2);
                            self.edge_set_prev_(self.edge_get_next_(e2), e2);
                        }
                        if self.edge_get_prev_(e2).is_some() {
                            //e1 -> next() -> prev(e1);
                            self.edge_set_prev_(self.edge_get_next_(e1), e1);

                            //e2 -> prev() -> next(e2);
                            self.edge_set_next_(self.edge_get_prev_(e2), e2);
                        }
                    }
                    last_edge += 2;
                }
                it += 2;
            }
            for e in (last_edge..edges_end).rev() {
                let _ = self.edges_.remove(e);
            }
        }
        #[cfg(feature = "console_debug")]
        self.debug_print_edges("after degenerate");
        tln!();

        // Set up incident edge pointers for cells and vertices.
        for edge_it in self.edge_iter().enumerate().map(|x| EdgeIndex(x.0)) {
            let cell = self.edge_get_cell_(Some(edge_it));
            if self.cell_get_incident_edge_(cell).is_none() {
                self.cell_set_incident_edge_(cell, Some(edge_it));
            }
            let vertex = self.edge_get_vertex0_(Some(edge_it));
            self.vertex_set_incident_edge_(vertex, Some(edge_it));
        }

        #[cfg(feature = "console_debug")]
        for (i, v) in self.vertices_.iter().enumerate() {
            tln!(
                "vertex #{} contains a point: ({:.12}, {:.12}) ie:{}",
                i,
                v.get().x(),
                v.get().y(),
                v.get()
                    .get_incident_edge_()
                    .map_or("-".to_string(), |x| x.0.clone().to_string())
            );
        }

        tln!("vertices b4 degenerate {}", self.vertices_.len());
        // Remove degenerate vertices.
        if !self.vertices_.is_empty() {
            let mut last_vertex_iterator = (0..self.vertices_.len()).map(VertexIndex);
            let mut last_vertex = last_vertex_iterator.next();
            for it in (0..self.vertices_.len()).map(VertexIndex) {
                let it = Some(it);
                if self.vertex_get_incident_edge(it).is_some() {
                    if it != last_vertex {
                        self.vertex_copy_(last_vertex.unwrap().0, it.unwrap().0);
                        let v = last_vertex;
                        let mut e = self.vertex_get_incident_edge(last_vertex);
                        loop {
                            //e->vertex0(v);
                            self.edge_set_vertex0_(e, v);
                            // e = e->rot_next();
                            e = self.edge_rot_next_(e);
                            if self.vertex_get_incident_edge(v) == e {
                                break;
                            }
                        }
                    }
                    last_vertex = last_vertex_iterator.next();
                }
            }
            if let Some(last_vertex) = last_vertex {
                for v in (last_vertex.0..self.vertices_.len()).rev() {
                    let _ = self.vertices_.remove(v);
                }
            }
        }
        tln!("vertices after degenerate {}", self.vertices_.len());

        // Set up next/prev pointers for infinite edges.
        if self.vertices_.is_empty() {
            if !self.edges_.is_empty() {
                // Update prev/next pointers for the line edges.
                let mut edge_it = self.edges_.iter().enumerate().map(|x| x.0);

                let mut edge1 = edge_it.next().map(EdgeIndex);
                self.edge_set_next_(edge1, edge1);
                self.edge_set_prev_(edge1, edge1);

                edge1 = edge_it.next().map(EdgeIndex);
                let mut edge_it_value = edge_it.next();
                while edge_it_value.is_some() {
                    let edge2 = edge_it_value.map(EdgeIndex);
                    edge_it_value = edge_it.next();
                    //dbg!(edge1.unwrap(),edge2.unwrap());

                    self.edge_set_next_(edge1, edge2);
                    self.edge_set_prev_(edge1, edge2);
                    self.edge_set_next_(edge2, edge1);
                    self.edge_set_prev_(edge2, edge1);

                    edge1 = edge_it_value.map(EdgeIndex);
                    edge_it_value = edge_it.next();
                }
                self.edge_set_next_(edge1, edge1);
                self.edge_set_prev_(edge1, edge1);
            }
        } else {
            // Update prev/next pointers for the ray edges.
            // let mut cell_it_keys = self.cells_.keys();
            for cell_it in 0..self.cells_.len() {
                if self.cell_is_degenerate_(Some(CellIndex(cell_it))) {
                    continue;
                }
                // Move to the previous edge while
                // it is possible in the CW direction.
                let mut left_edge = self.cell_get_incident_edge_(Some(CellIndex(cell_it)));
                let terminal_edge = left_edge;
                while let Some(new_left_edge) = self.edge_get_prev_(left_edge) {
                    left_edge = Some(new_left_edge);
                    // Terminate if this is not a boundary cell.
                    if left_edge == terminal_edge {
                        break;
                    }
                }

                if self.edge_get_prev_(left_edge).is_some() {
                    continue;
                }

                let mut right_edge = self.cell_get_incident_edge_(Some(CellIndex(cell_it)));
                while let Some(new_right_edge) = self.edge_get_next_(right_edge) {
                    right_edge = Some(new_right_edge);
                }

                self.edge_set_prev_(left_edge, right_edge);
                self.edge_set_next_(right_edge, left_edge);
            }
        }
    }

    /// prints cells and vertices to the console
    /// edges will be printed if the 'edge_filter' returns true for that edge id.
    #[cfg(feature = "console_debug")]
    pub fn debug_print_all<FN: Fn(usize) -> bool>(&self, edge_filter: FN) {
        tln!();
        tln!("output:");
        for (i, c) in self.cells_.iter().enumerate() {
            let cc = c.get();
            print!("cell#{} {:?} ", i, &cc);
            if cc.contains_point() {
                tln!("point");
            } else if cc.contains_segment() {
                tln!("segment");
            } else if cc.contains_segment_startpoint() {
                tln!("startpoint");
            } else if cc.contains_segment_endpoint() {
                tln!("endpoint");
            } else {
                tln!();
            }
        }
        for (i, v) in self.vertices_.iter().enumerate() {
            assert_eq!(i, v.get().id_.0);

            let edges1: Vec<usize> = self
                .edge_rot_next_iterator_(v.get().get_incident_edge_())
                .map(|x| x.0)
                .filter(|x| edge_filter(*x))
                .collect();
            let edges2: Vec<usize> = self
                .edge_rot_next_iterator_(v.get().get_incident_edge_())
                .map(|x| self.edge_get_twin_(Some(x)))
                .flatten()
                .map(|x| x.0)
                .filter(|x| edge_filter(*x))
                .collect();
            //if !(edges1.is_empty() && edges2.is_empty()) {
            t!("vertex#{} {:?}", i, &v.get());
            t!(" outgoing edges:{:?}", edges1);
            tln!(" incoming edges:{:?}", edges2);
            //}
        }
    }

    #[cfg(feature = "console_debug")]
    pub fn debug_print_edges(&self, text: &str) {
        tln!("edges {} {}", text, self.edges_.len());
        for (i, e) in self.edges_.iter().enumerate() {
            let e = e.get();
            tln!("edge{} ({:?})", e.id_.0, &e);
            assert_eq!(i, e.id_.0);
        }
    }
}

impl<F: OutputType> From<Diagram<F>> for SD::SyncDiagram<F> {
    fn from(other: Diagram<F>) -> SD::SyncDiagram<F> {
        SD::SyncDiagram::new(
            other.cells_.into_iter().map(|x| x.get()).collect(),
            other.vertices_.into_iter().map(|x| x.get()).collect(),
            other.edges_.into_iter().map(|x| x.get()).collect(),
        )
    }
}
