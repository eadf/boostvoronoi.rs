// Boost.Polygon library voronoi_diagram.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::circle_event as VC;
use super::ctypes as CT;
use super::site_event as VSE;
use super::visual_utils as VU;
use super::TypeConverter4 as TC4;

pub use super::{BigFloatType, BigIntType, InputType, OutputType};
use num::NumCast;
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Neg;
use std::rc::Rc;

pub type SourceIndex = usize;

///! See <https://www.boost.org/doc/libs/1_75_0/libs/polygon/doc/voronoi_diagram.htm>

/// Typed container for cell indices
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct VoronoiCellIndex(pub usize);

impl fmt::Debug for VoronoiCellIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VoronoiCellIndex({})", self.0)
    }
}

/// Typed container for edge indices
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct VoronoiEdgeIndex(pub usize);

impl fmt::Debug for VoronoiEdgeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VoronoiEdgeIndex({})", self.0)
    }
}

/// Typed container for vertex indices
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct VoronoiVertexIndex(pub usize);

impl fmt::Debug for VoronoiVertexIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VoronoiVertexIndex({})", self.0)
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
    pub(crate) const SINGLE_POINT: Self = ColorBits(0x0);
    pub(crate) const SEGMENT_START_POINT: Self = ColorBits(0x1);
    pub(crate) const SEGMENT_END_POINT: Self = ColorBits(0x2);
    pub(crate) const SITE_VERTEX: Self = ColorBits(0x4);

    // Segment subtypes.
    pub(crate) const INITIAL_SEGMENT: Self = ColorBits(0x8);
    pub(crate) const REVERSE_SEGMENT: Self = ColorBits(0x9);

    pub(crate) const BITMASK: Self = ColorBits(0x1F); // 0b1_11111111

    pub(crate) const GEOMETRY_SHIFT: Self = ColorBits(0x3);
    pub(crate) const GEOMETRY_CATEGORY_POINT: Self = ColorBits(0x0);
    pub(crate) const GEOMETRY_CATEGORY_SEGMENT: Self = ColorBits(0x1);

    // 5 color bits are reserved for internal use.
    pub(crate) const BITS_SHIFT: Self = Self(0x5);
    pub(crate) const IS_INVERSE_BITMASK: Self = Self(0x20); // 32

    // todo: remove this
    pub(crate) const TEMPORARY_CELL: Self = ColorBits(u32::MAX);
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
///   2) pointer to the incident edge
///   3) mutable color member
/// Cell may contain point or segment site inside.
///
/// TODO! fix the name confusion "initial index" & "source index" referring to the same thing.
#[derive(Copy, Clone)]
pub struct VoronoiCell<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    // sorted_index of the site event
    id_: VoronoiCellIndex,
    // source_index/initial_index of the site event
    source_index_: SourceIndex,
    incident_edge_: Option<VoronoiEdgeIndex>,
    color_: ColorType,
    #[doc(hidden)]
    _pdi: PhantomData<I1>,
    #[doc(hidden)]
    _pdo: PhantomData<F1>,
}

impl<I1, F1> fmt::Debug for VoronoiCell<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rv = String::new();

        rv.push_str(
            format!(
                "(id:{:?} ii:{} ie:{:?} col:{})",
                self.id_.0,
                self.source_index_,
                super::format_id(self.incident_edge_.map(|x| x.0)),
                self.color_
            )
            .as_str(),
        );
        write!(f, "{}", rv)
    }
}

impl<I1, F1> VoronoiCell<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    pub fn new(
        id: VoronoiCellIndex,
        source_index: SourceIndex,
        source_category: ColorType,
    ) -> Self {
        VoronoiCell {
            id_: id,
            source_index_: source_index,
            incident_edge_: None,
            color_: source_category,
            _pdi: PhantomData,
            _pdo: PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) fn internal_color(&self) -> ColorBits {
        ColorBits(self.color_ & ColorBits::BITMASK.0)
    }

    #[inline(always)]
    pub fn source_category(&self) -> SourceCategory {
        match self.internal_color() {
            ColorBits::SINGLE_POINT => SourceCategory::SinglePoint,
            ColorBits::SEGMENT_START_POINT => SourceCategory::SegmentStart,
            ColorBits::SEGMENT_END_POINT => SourceCategory::SegmentEnd,
            _ => SourceCategory::Segment,
        }
    }

    /// Returns true if the cell contains point site, false else.
    #[inline(always)]
    pub fn contains_point(&self) -> bool {
        let geometry = self.internal_color().0 >> ColorBits::GEOMETRY_SHIFT.0;
        geometry == ColorBits::GEOMETRY_CATEGORY_POINT.0
    }

    /// Returns true if the cell contains segment site, false otherwise.
    #[inline(always)]
    pub fn contains_segment(&self) -> bool {
        let geometry = self.internal_color().0 >> ColorBits::GEOMETRY_SHIFT.0;
        geometry == ColorBits::GEOMETRY_CATEGORY_SEGMENT.0
    }

    /// Returns true if the cell contains segment start point, false otherwise.
    #[inline(always)]
    pub fn contains_segment_startpoint(&self) -> bool {
        self.internal_color().0 == ColorBits::SEGMENT_START_POINT.0
    }

    /// Returns true if the cell contains segment end point, false otherwise.
    #[inline(always)]
    pub fn contains_segment_endpoint(&self) -> bool {
        self.internal_color().0 == ColorBits::SEGMENT_END_POINT.0
    }

    #[inline(always)]
    pub fn get_id(&self) -> SourceIndex {
        self.id_.0
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
    pub fn get_incident_edge(&self) -> Option<VoronoiEdgeIndex> {
        self.incident_edge_
    }
}

/// Iterator over edges pointing away from the vertex indicated by the initial edge.
/// edge.vertex()
/// Do *NOT* use this when altering next, prev or twin edges.
pub struct EdgeRotNextIterator<'s, I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    diagram: &'s VoronoiDiagram<I1, F1, I2, F2>,
    starting_edge: VoronoiEdgeIndex,
    next_edge: Option<VoronoiEdgeIndex>,
    #[doc(hidden)]
    _pdi: PhantomData<I1>,
    #[doc(hidden)]
    _pdf: PhantomData<F1>,
}

impl<'s, I1, F1, I2, F2> EdgeRotNextIterator<'s, I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    pub(crate) fn new(
        diagram: &'s VoronoiDiagram<I1, F1, I2, F2>,
        starting_edge: Option<VoronoiEdgeIndex>,
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
                starting_edge: VoronoiEdgeIndex(0),
                next_edge: None,
                _pdf: PhantomData,
                _pdi: PhantomData,
            }
        }
    }
}

impl<'s, I1, F1, I2, F2> Iterator for EdgeRotNextIterator<'s, I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    type Item = VoronoiEdgeIndex;
    fn next(&mut self) -> Option<VoronoiEdgeIndex> {
        let rv = self.next_edge;
        let new_next_edge = self.diagram.edge_rot_next(self.next_edge);
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

/// Iterator over edges pointing away from the vertex indicated by the initial edge.
/// edge.vertex()
/// Do *NOT* use this when altering next, prev or twin edges.
pub struct EdgeRotPrevIterator<'s, I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    diagram: &'s VoronoiDiagram<I1, F1, I2, F2>,
    starting_edge: VoronoiEdgeIndex,
    next_edge: Option<VoronoiEdgeIndex>,
    #[doc(hidden)]
    _pdi: PhantomData<I1>,
    #[doc(hidden)]
    _pdf: PhantomData<F1>,
}

impl<'s, I1, F1, I2, F2> EdgeRotPrevIterator<'s, I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    #[allow(dead_code)]
    pub(crate) fn new(
        diagram: &'s VoronoiDiagram<I1, F1, I2, F2>,
        starting_edge: Option<VoronoiEdgeIndex>,
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
                starting_edge: VoronoiEdgeIndex(0),
                next_edge: None,
                _pdf: PhantomData,
                _pdi: PhantomData,
            }
        }
    }
}

impl<'s, I1, F1, I2, F2> Iterator for EdgeRotPrevIterator<'s, I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    type Item = VoronoiEdgeIndex;
    fn next(&mut self) -> Option<VoronoiEdgeIndex> {
        let rv = self.next_edge;
        let new_next_edge = self.diagram.edge_rot_prev(self.next_edge);
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

/// Represents Voronoi vertex.
/// Data members:
///   1) vertex coordinates
///   2) pointer to the incident edge
///   3) mutable color member
#[derive(Copy, Clone)]
pub struct VoronoiVertex<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    pub(crate) id_: VoronoiVertexIndex,
    pub(crate) x_: F1,
    pub(crate) y_: F1,
    pub(crate) incident_edge_: Option<VoronoiEdgeIndex>,
    pub(crate) color_: ColorType,
    #[doc(hidden)]
    _pdi: PhantomData<I1>,
}

impl<I1, F1> fmt::Debug for VoronoiVertex<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rv = String::new();

        rv.push_str(
            format!(
                "(id:{} x:{} y:{} ie:{} co:{})",
                self.id_.0,
                self.x_,
                self.y_,
                super::format_id(self.incident_edge_.map(|x| x.0)),
                self.color_
            )
            .as_str(),
        );
        write!(f, "{}", rv)
    }
}

impl<I1, F1> VoronoiVertex<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    pub fn new_3(id: VoronoiVertexIndex, x: F1, y: F1, is_site_vertex:bool) -> Rc<Cell<VoronoiVertex<I1, F1>>> {
        let color = if is_site_vertex {ColorBits::SITE_VERTEX.0} else {ColorBits::ZERO.0};
        Rc::new(Cell::new(Self {
            id_: id,
            x_: x,
            y_: y,
            incident_edge_: None,
            color_: color,
            _pdi: PhantomData,
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

    pub fn get_id(&self) -> VoronoiVertexIndex {
        self.id_
    }

    #[inline]
    pub fn get_incident_edge(&self) -> Option<VoronoiEdgeIndex> {
        self.incident_edge_
    }

    #[inline]
    pub fn x(&self) -> F1 {
        self.x_
    }

    #[inline]
    pub fn y(&self) -> F1 {
        self.y_
    }

    /// get_color returns the custom edge info. (not the internal bits)
    pub fn get_color(&self) -> ColorType {
        self.color_ >> ColorBits::BITS_SHIFT.0
    }

    /// set_color sets the custom edge info. (not the internal bits)
    pub fn set_color(&mut self, color: ColorType) -> ColorType {
        self.color_ &= ColorBits::BITMASK.0;
        self.color_ |= color << ColorBits::BITS_SHIFT.0;
        self.color_
    }

    /// or_color sets the custom vertex info together with the previous value. (not the internal bits)
    /// This is a Cell operation, remember to set() the entire cell
    #[inline(always)]
    pub fn or_color(&mut self, color: ColorType) -> ColorType {
        self.set_color(self.get_color() | color)
    }

    /// Returns true if vertex coincides with an input site.
    #[inline]
    pub fn is_site_vertex(&self) -> bool {
        (self.color_ & ColorBits::SITE_VERTEX.0) != 0
    }
}

/// Half-edge data structure. Represents Voronoi edge.
/// Data members:
///   1) pointer to the corresponding cell
///   2) pointer to the vertex that is the starting
///      point of the half-edge
///   3) pointer to the twin edge
///   4) pointer to the CCW next edge
///   5) pointer to the CCW prev edge
///   6) mutable color member
#[derive(Copy, Clone)]
pub struct VoronoiEdge<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    id: VoronoiEdgeIndex,
    cell_: Option<VoronoiCellIndex>,
    vertex_: Option<VoronoiVertexIndex>,
    twin_: Option<VoronoiEdgeIndex>,
    next_ccw_: Option<VoronoiEdgeIndex>,
    prev_ccw_: Option<VoronoiEdgeIndex>,
    color_: ColorType,
    #[doc(hidden)]
    _pdi: PhantomData<I1>,
    #[doc(hidden)]
    _pdo: PhantomData<F1>,
    #[doc(hidden)]
    _pdbi: PhantomData<I2>,
    #[doc(hidden)]
    _pdbf: PhantomData<F2>,
}

impl<I1, F1, I2, F2> fmt::Debug for VoronoiEdge<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rv = String::new();

        rv.push_str(
            format!(
                "id:{} cell:{} v0:{} t:{} n:{} p:{} c:{}",
                self.id.0,
                super::format_id(self.cell_.map(|c| c.0)),
                super::format_id(self.vertex_.map(|v| v.0)),
                super::format_id(self.twin_.map(|e| e.0)),
                super::format_id(self.next_ccw_.map(|e| e.0)),
                super::format_id(self.prev_ccw_.map(|e| e.0)),
                self.color_
            )
            .as_str(),
        );
        write!(f, "{}", rv)
    }
}

impl<I1, F1, I2, F2> VoronoiEdge<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    const BIT_IS_LINEAR: ColorType = 0x1; // linear is opposite to curved
    const BIT_IS_PRIMARY: ColorType = 0x2; // primary is opposite to secondary

    fn new_4(
        id: VoronoiEdgeIndex,
        cell: VoronoiCellIndex,
        is_linear: bool,
        is_primary: bool,
    ) -> EdgeType<I1, F1, I2, F2> {
        let mut rv = Self {
            id,
            cell_: Some(cell),
            vertex_: None,
            twin_: None,
            next_ccw_: None,
            prev_ccw_: None,
            color_: 0,
            _pdi: PhantomData,
            _pdo: PhantomData,
            _pdbi: PhantomData,
            _pdbf: PhantomData,
        };
        if is_linear {
            rv.color_ |= Self::BIT_IS_LINEAR;
        }
        if is_primary {
            rv.color_ |= Self::BIT_IS_PRIMARY;
        }
        Rc::new(Cell::new(rv))
    }

    pub fn get_id(&self) -> VoronoiEdgeIndex {
        self.id
    }

    pub fn cell(&self) -> Option<VoronoiCellIndex> {
        self.cell_
    }

    pub fn vertex0(&self) -> Option<VoronoiVertexIndex> {
        self.vertex_
    }

    pub fn twin(&self) -> Option<VoronoiEdgeIndex> {
        self.twin_
    }

    pub fn next(&self) -> Option<VoronoiEdgeIndex> {
        self.next_ccw_
    }

    pub fn prev(&self) -> Option<VoronoiEdgeIndex> {
        self.prev_ccw_
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

    /// get_color returns the custom edge info. (not the internal bits)
    #[inline(always)]
    pub fn get_color(&self) -> ColorType {
        self.color_ >> ColorBits::BITS_SHIFT.0
    }

    /// set_color sets the custom edge info. (not the internal bits)
    /// This is a Cell operation, remember to set() the entire cell
    #[inline(always)]
    pub fn set_color(&mut self, color: ColorType) -> ColorType {
        self.color_ &= ColorBits::BITMASK.0;
        self.color_ |= color << ColorBits::BITS_SHIFT.0;
        self.color_
    }

    /// or_color sets the custom edge info together with the previous value. (not the internal bits)
    /// This is a Cell operation, remember to set() the entire cell
    #[inline(always)]
    pub fn or_color(&mut self, color: ColorType) -> ColorType {
        self.set_color(self.get_color() | color)
    }
}

pub type CellType<I1, F1> = Rc<Cell<VoronoiCell<I1, F1>>>;
pub type EdgeType<I1, F1, I2, F2> = Rc<Cell<VoronoiEdge<I1, F1, I2, F2>>>;
pub type VertexType<I1, F1> = Rc<Cell<VoronoiVertex<I1, F1>>>;

/// Voronoi output data structure.
/// CCW ordering is used on the faces perimeter and around the vertices.
/// Mandatory reading: <https://www.boost.org/doc/libs/1_75_0/libs/polygon/doc/voronoi_diagram.htm>
#[derive(Default, Debug)]
pub struct VoronoiDiagram<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    cells_: Vec<CellType<I1, F1>>,         // indexed by VoronoiCellIndex
    vertices_: Vec<VertexType<I1, F1>>,    // indexed by VoronoiVertexIndex
    edges_: Vec<EdgeType<I1, F1, I2, F2>>, // indexed by VoronoiEdgeIndex
}

impl<I1, F1, I2, F2> VoronoiDiagram<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    pub fn new(input_size: usize) -> Self {
        Self {
            cells_: Vec::<CellType<I1, F1>>::with_capacity(input_size),
            vertices_: Vec::<VertexType<I1, F1>>::with_capacity(input_size),
            edges_: Vec::<EdgeType<I1, F1, I2, F2>>::with_capacity(input_size * 2),
        }
    }

    pub fn clear(&mut self) {
        self.cells_.clear();
        self.vertices_.clear();
        self.edges_.clear();
    }

    /// Returns a reference to all of the cells
    pub fn cells(&self) -> &Vec<CellType<I1, F1>> {
        &self.cells_
    }

    /// Returns a reference to all of the vertices
    pub fn vertices(&self) -> &Vec<VertexType<I1, F1>> {
        &self.vertices_
    }

    /// Returns an aabb large enough to contain all the vertices
    pub fn vertices_get_aabb(&self) -> VU::Aabb2<I1, F1> {
        let mut rv = VU::Aabb2::<I1, F1>::default();
        for v in self.vertices_.iter() {
            let v = v.get();
            rv.update_vertex(v.x(), v.y());
        }
        rv
    }

    pub fn edges(&self) -> &Vec<EdgeType<I1, F1, I2, F2>> {
        &self.edges_
    }

    pub fn get_cell(&self, cell: VoronoiCellIndex) -> Rc<Cell<VoronoiCell<I1, F1>>> {
        self.cells_.get(cell.0).unwrap().clone()
    }

    pub fn get_edge(&self, edge: VoronoiEdgeIndex) -> EdgeType<I1, F1, I2, F2> {
        self.edges_.get(edge.0).unwrap().clone()
    }

    /// Return the edge represented as an straight line
    /// if the edge does not exists or if it lacks v0 or v1; None will be returned.
    /// TODO: this looks like an into() candidate
    pub fn edge_as_line(&self, edge: Option<VoronoiEdgeIndex>) -> Option<[F1; 4]> {
        let v0 = self.vertex_get(self.edge_get_vertex0(edge));
        let v1 = self.vertex_get(self.edge_get_vertex1(edge));
        if let Some(v0) = v0 {
            if let Some(v1) = v1 {
                let v0 = v0.get();
                let v1 = v1.get();
                return Some([v0.x(), v0.y(), v1.x(), v1.y()]);
            }
        }
        None
    }

    /// Iterates over all edges, colors each edge as exterior if it has an unbroken primary edge
    /// link connection to an infinite edge.
    pub fn color_exterior_edges(&self, external_color: ColorType) {
        for it in self.edges().iter() {
            let edge_id = Some(it.get().get_id());
            if !self.edge_is_finite(edge_id).unwrap() {
                self.color_exterior(edge_id, external_color);
            }
        }
    }

    /// Todo: something is wrong here, some external edges will remain unmarked.
    /// The same bug exists in C++ too.
    /// Some secondary internal edges are inevitably marked too.
    fn color_exterior(&self, edge_id: Option<VoronoiEdgeIndex>, external_color: ColorType) {
        if edge_id.is_none() || (self.edge_get_color(edge_id).unwrap() & external_color) != 0 {
            // This edge has already been colored, break recursion
            return;
        }
        // Color edge as EXTERNAL
        self.edge_or_color(edge_id, external_color);

        let v1 = self.edge_get_vertex1(edge_id);
        if  self.edge_get_vertex0(edge_id).is_some() && v1.is_none() {
            // this edge leads to nowhere, break recursion
            return
        }
        // Color twin edge as EXTERNAL
        self.edge_or_color(self.edge_get_twin(edge_id), external_color);
        if v1.is_none()
            || self.vertex_is_site_point(v1).unwrap_or(true)
            || !self.get_edge(edge_id.unwrap()).get().is_primary() {
            // stop recursion if this edge does not have a vertex1 (e.g is infinite)
            // or if this edge isn't a primary edge.
            return;
        }
        self.vertex_set_color(v1, external_color);
        let incident_edge = self.vertex_get_incident_edge(v1);
        for e in self.edge_rot_next_iterator(incident_edge) {
            // mark all surrounding edges as EXTERNAL, but only recurse on primary edges
            self.color_exterior(Some(e), external_color);
        }
    }

    pub fn cell_iter(&self) -> core::slice::Iter<CellType<I1, F1>> {
        self.cells_.iter()
    }

    pub fn vertex_iter(&self) -> core::slice::Iter<VertexType<I1, F1>> {
        self.vertices_.iter()
    }

    pub fn edge_iter(&self) -> std::slice::Iter<EdgeType<I1, F1, I2, F2>> {
        self.edges_.iter()
    }

    /// push a new cell on the output. Nothing but id and source category is initialized
    fn _make_new_cell_with_category(
        &mut self,
        cell_id: VoronoiCellIndex, // same as sorted_index
        initial_index: SourceIndex,
        sc: ColorBits,
    ) -> VoronoiCellIndex {
        // fill cell with temporary blocks- they will be over-written later
        // Todo: fix this dirty hack with Option<>
        while self.cells_.len() < cell_id.0 {
            self.cells_
                .push(Rc::new(Cell::new(VoronoiCell::<I1, F1>::new(
                    VoronoiCellIndex(usize::MAX),
                    usize::MAX,
                    ColorBits::TEMPORARY_CELL.0,
                ))));
        }
        self.cells_
            .push(Rc::new(Cell::new(VoronoiCell::<I1, F1>::new(
                cell_id,
                initial_index,
                sc.0,
            ))));
        assert_eq!(self.cells_[cell_id.0].get().get_id(), cell_id.0);

        let ccell = &self.cells_[cell_id.0];
        {
            let cell = ccell.get();
            assert_eq!(cell.id_.0, cell_id.0);
            assert_eq!(cell.source_index_, initial_index);
            assert_eq!(cell.color_, sc.0);
            //cell.color_ = sc.get_value();
            ccell.set(cell);
        }
        cell_id
    }

    pub fn num_cells(&self) -> usize {
        self.cells_.len()
    }

    pub fn num_edges(&self) -> usize {
        self.edges_.len()
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices_.len()
    }

    pub fn _reserve(&mut self, num_sites: usize) {
        self.cells_.reserve(num_sites);
        self.vertices_.reserve(num_sites << 1);
        self.edges_.reserve((num_sites << 2) + (num_sites << 1));
    }

    pub(crate) fn _process_single_site(&mut self, site: &VSE::SiteEvent<I1, F1, I2, F2>) {
        let _ = self._make_new_cell_with_category(
            VoronoiCellIndex(site.sorted_index()),
            site.initial_index(),
            site.source_category(),
        );
    }

    #[inline]
    fn _cell_get(&self, cell_id: Option<VoronoiCellIndex>) -> Option<&CellType<I1, F1>> {
        let _ = cell_id?;
        self.cells_.get(cell_id.unwrap().0)
    }

    fn _cell_set_incident_edge(
        &self,
        cell_id: Option<VoronoiCellIndex>,
        edge: Option<VoronoiEdgeIndex>,
    ) {
        if cell_id.is_none() {
            return;
        }
        if let Some(cell) = self._cell_get(cell_id) {
            let mut c = cell.get();
            c.incident_edge_ = edge;
            cell.set(c)
        }
    }

    fn _cell_get_incident_edge(
        &self,
        cell_id: Option<VoronoiCellIndex>,
    ) -> Option<VoronoiEdgeIndex> {
        let _ = cell_id?;
        if let Some(cell) = self._cell_get(cell_id) {
            return cell.get().incident_edge_;
        }
        None
    }

    fn _cell_is_degenerate(&self, cell_id: Option<VoronoiCellIndex>) -> bool {
        if cell_id.is_none() {
            return false;
        }
        if let Some(cell) = self._cell_get(cell_id) {
            return cell.get().is_degenerate();
        }
        false
    }

    #[inline]
    pub fn vertex_get(&self, vertex_id: Option<VoronoiVertexIndex>) -> Option<&VertexType<I1, F1>> {
        let _ = vertex_id?;
        self.vertices_.get(vertex_id.unwrap().0)
    }

    /// OR the previous color field value with this new color value
    pub fn vertex_or_color(&self, vertex_id: Option<VoronoiVertexIndex>, color: ColorType) {
        if vertex_id.is_none() {
            return;
        }
        if let Some(vertexcell) = self.vertex_get(vertex_id) {
            let mut vertex = vertexcell.get();
            let _ = vertex.or_color(color);
            vertexcell.set(vertex);
        }
    }

    /// Returns the color field of the vertex.
    pub fn vertex_get_color(&self, vertex_id: Option<VoronoiVertexIndex>) -> Option<ColorType> {
        let _ = vertex_id?;
        if let Some(vertexcell) = self.vertex_get(vertex_id) {
            let vertex = vertexcell.get();
            return Some(vertex.get_color());
        }
        None
    }

    /// Overwrites the content of dest with the content of source.
    /// edge_id is compensated accordingly
    fn _vertex_copy(&self, dest: usize, source: usize) {
        let mut v = self.vertices_[source].get();
        v.id_ = VoronoiVertexIndex(dest);
        self.vertices_[dest].set(v);
    }

    fn _vertex_set_incident_edge(
        &self,
        vertex_id: Option<VoronoiVertexIndex>,
        edge: Option<VoronoiEdgeIndex>,
    ) {
        if vertex_id.is_none() {
            return;
        }
        if let Some(vertex) = self.vertex_get(vertex_id) {
            let mut c = vertex.get();
            c.incident_edge_ = edge;
            vertex.set(c)
        }
    }

    /// return one of the edges originating at the vertex
    pub fn vertex_get_incident_edge(
        &self,
        vertex_id: Option<VoronoiVertexIndex>,
    ) -> Option<VoronoiEdgeIndex> {
        let _ = vertex_id?;
        self.vertex_get(vertex_id)
            .and_then(|x| x.get().incident_edge_)
    }

    /// Set the color of the vertex. This affects only the public bits, not the internal
    pub fn vertex_set_color(&self, vertex_id: Option<VoronoiVertexIndex>, color: ColorType) {
        if vertex_id.is_none() {
            return;
        }
        if let Some(cell) = self.vertex_get(vertex_id) {
            let mut vertex = cell.get();
            let _ = vertex.set_color(color);
            cell.set(vertex);
        }
    }

    /// returns true if this vertex coincides with an site point
    pub fn vertex_is_site_point(&self, vertex_id: Option<VoronoiVertexIndex>)-> Option<bool> {
        let _ = vertex_id?;
        if let Some(cell) = self.vertex_get(vertex_id) {
            Some(cell.get().is_site_vertex())
        } else {
            None
        }
    }

    fn _edge_new_3(
        &mut self,
        cell_id: VoronoiCellIndex,
        is_linear: bool,
        is_primary: bool,
    ) -> VoronoiEdgeIndex {
        let new_edge_id = VoronoiEdgeIndex(self.edges_.len());
        let new_edge = VoronoiEdge::new_4(new_edge_id, cell_id, is_linear, is_primary);
        let _ = self.edges_.insert(new_edge_id.0, new_edge);
        new_edge_id
    }

    #[inline]
    fn _edge_get(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<&EdgeType<I1, F1, I2, F2>> {
        let _ = edge_id?;
        let rv = self.edges_.get(edge_id.unwrap().0);
        if rv.is_none() {
            dbg!(edge_id.unwrap().0);
            panic!();
        }
        rv
    }

    /// Overwrites the content of dest with the content of source.
    /// edge_id is compensated accordingly
    fn _edge_copy(&self, dest: usize, source: usize) {
        let mut e = self.edges_[source].get();
        e.id = VoronoiEdgeIndex(dest);
        self.edges_[dest].set(e);
    }

    #[inline]
    /// Returns the color field of the edge.
    pub fn edge_get_color(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<ColorType> {
        let _ = edge_id?;
        if let Some(edgecell) = self._edge_get(edge_id) {
            let edge = edgecell.get();
            return Some(edge.get_color());
        }
        None
    }

    #[inline]
    /// Sets the color field with new value
    pub fn edge_set_color(&self, edge_id: Option<VoronoiEdgeIndex>, color: ColorType) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self._edge_get(edge_id) {
            let mut edge = edgecell.get();
            let _ = edge.set_color(color);
            edgecell.set(edge);
        }
    }

    #[inline]
    /// OR the previous color field value with this new color value
    pub fn edge_or_color(&self, edge_id: Option<VoronoiEdgeIndex>, color: ColorType) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self._edge_get(edge_id) {
            let mut edge = edgecell.get();
            let _ = edge.or_color(color);
            edgecell.set(edge);
        }
    }

    #[inline]
    fn _edge_set_twin(&self, edge_id: Option<VoronoiEdgeIndex>, twin_id: Option<VoronoiEdgeIndex>) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self._edge_get(edge_id) {
            let mut edge = edgecell.get();
            edge.twin_ = twin_id;
            edgecell.set(edge);
        }
    }

    #[inline]
    /// Returns an edge iterator, the edges will all originate at the same vertex as 'edge_id'.
    ///  'edge_id' will be the first edge returned by the iterator.
    /// Do *NOT* use this when altering next, prev or twin edges.
    pub fn edge_rot_next_iterator(
        &self,
        edge_id: Option<VoronoiEdgeIndex>,
    ) -> EdgeRotNextIterator<I1, F1, I2, F2> {
        EdgeRotNextIterator::new(self, edge_id)
    }

    #[inline]
    /// Returns an edge iterator, the edges will all originate at the same vertex as 'edge_id'.
    ///  'edge_id' will be the first edge returned by the iterator.
    /// Do *NOT* use this when altering next, prev or twin edges.
    pub fn edge_rot_prev_iterator(
        &self,
        edge_id: Option<VoronoiEdgeIndex>,
    ) -> EdgeRotPrevIterator<I1, F1, I2, F2> {
        EdgeRotPrevIterator::new(self, edge_id)
    }

    #[inline]
    pub fn edge_get_twin(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<VoronoiEdgeIndex> {
        let _ = edge_id?;
        if let Some(edgecell) = self._edge_get(edge_id) {
            return edgecell.get().twin();
        }
        None
    }

    #[inline]
    pub fn edge_get_next(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<VoronoiEdgeIndex> {
        let _ = edge_id?;
        if let Some(edgecell) = self._edge_get(edge_id) {
            return edgecell.get().next();
        }
        None
    }

    #[inline]
    fn _edge_set_cell(&self, edge_id: Option<VoronoiEdgeIndex>, cell_id: Option<VoronoiCellIndex>) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self._edge_get(edge_id) {
            let mut edge = edgecell.get();
            edge.cell_ = cell_id;
            edgecell.set(edge);
        }
    }

    #[inline]
    pub fn edge_get_cell(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<VoronoiCellIndex> {
        let _ = edge_id?;
        if let Some(edgecell) = self._edge_get(edge_id) {
            return edgecell.get().cell();
        }
        None
    }

    /// Returns true if the edge is finite (segment, parabolic arc).
    /// Returns false if the edge is infinite (ray, line).
    #[inline]
    pub fn edge_is_finite(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<bool> {
        let _ = edge_id?;
        Some(self.edge_get_vertex0(edge_id).is_some() && self.edge_get_vertex1(edge_id).is_some())
    }

    /// Returns true if the edge is infinite (ray, line).
    /// Returns false if the edge is finite (segment, parabolic arc).
    #[inline]
    pub fn edge_is_infinite(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<bool> {
        let _ = edge_id?;
        Some(self.edge_get_vertex0(edge_id).is_none() || self.edge_get_vertex1(edge_id).is_none())
    }

    /// Remove degenerate edge.
    fn _remove_edge(&mut self, edge: Option<VoronoiEdgeIndex>) {
        // Update the endpoints of the incident edges to the second vertex.
        let vertex = self.edge_get_vertex0(edge);
        let edge_twin = self.edge_get_twin(edge);
        let mut updated_edge = self.edge_rot_next(edge_twin);

        while updated_edge != edge_twin {
            self._edge_set_vertex0(updated_edge, vertex);
            updated_edge = self.edge_rot_next(updated_edge);
        }
        let edge1 = edge;
        let edge2 = edge_twin;

        // Update prev/next pointers for the incident edges.
        //edge1_rot_next->twin()->next(edge2_rot_prev);
        self._edge_set_next(
            self.edge_get_twin(self.edge_rot_next(edge1)),
            self.edge_rot_prev(edge2),
        );
        //edge2_rot_prev->prev(edge1_rot_next->twin());
        self._edge_set_prev(
            self.edge_rot_prev(edge2),
            self.edge_get_twin(self.edge_rot_next(edge1)),
        );

        //edge1_rot_prev->prev(edge2_rot_next->twin());
        self._edge_set_prev(
            self.edge_rot_prev(edge1),
            self.edge_get_twin(self.edge_rot_next(edge2)),
        );

        //edge2_rot_next->twin()->next(edge1_rot_prev);
        self._edge_set_next(
            self.edge_get_twin(self.edge_rot_next(edge2)),
            self.edge_rot_prev(edge1),
        );
    }

    fn _vertex_new_2(&mut self, x: F1, y: F1, is_site_vertex:bool) -> VoronoiVertexIndex {
        let new_vertex_id = VoronoiVertexIndex(self.vertices_.len());
        let new_edge = VoronoiVertex::new_3(new_vertex_id, x, y, is_site_vertex);
        let _ = self.vertices_.insert(new_vertex_id.0, new_edge);
        new_vertex_id
    }

    fn _edge_set_vertex0(
        &self,
        edge_id: Option<VoronoiEdgeIndex>,
        vertex_id: Option<VoronoiVertexIndex>,
    ) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self._edge_get(edge_id) {
            let mut edge = edgecell.get();
            edge.vertex_ = vertex_id;
            edgecell.set(edge);
        }
    }

    #[inline]
    pub fn edge_get_vertex0(
        &self,
        edge_id: Option<VoronoiEdgeIndex>,
    ) -> Option<VoronoiVertexIndex> {
        let _ = edge_id?;
        self._edge_get(edge_id).and_then(|x| x.get().vertex0())
    }

    #[inline]
    pub fn edge_get_vertex1(
        &self,
        edge_id: Option<VoronoiEdgeIndex>,
    ) -> Option<VoronoiVertexIndex> {
        let _ = edge_id?;
        let twin = self.edge_get_twin(edge_id);
        self.edge_get_vertex0(twin)
    }

    #[inline]
    fn _edge_set_prev(&self, edge_id: Option<VoronoiEdgeIndex>, prev_id: Option<VoronoiEdgeIndex>) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self._edge_get(edge_id) {
            let mut edge = edgecell.get();
            edge.prev_ccw_ = prev_id;
            edgecell.set(edge);
        }
    }

    #[inline]
    fn _edge_set_next(&self, edge_id: Option<VoronoiEdgeIndex>, next_id: Option<VoronoiEdgeIndex>) {
        if edge_id.is_none() {
            return;
        }
        if let Some(edgecell) = self._edge_get(edge_id) {
            let mut edge = edgecell.get();
            edge.next_ccw_ = next_id;
            edgecell.set(edge);
        }
    }

    #[inline]
    fn _edge_get_next(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<VoronoiEdgeIndex> {
        let _ = edge_id?;
        self.edges_
            .get(edge_id.unwrap().0)
            .and_then(|x| x.get().next())
    }

    #[inline]
    fn _edge_get_prev(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<VoronoiEdgeIndex> {
        let _ = edge_id?;
        self.edges_
            .get(edge_id.unwrap().0)
            .and_then(|x| x.get().prev())
    }

    #[inline]
    fn _edge_get_cell(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<VoronoiCellIndex> {
        let _ = edge_id?;
        self.edges_
            .get(edge_id.unwrap().0)
            .and_then(|x| x.get().cell())
    }

    #[inline]
    /// Returns a pointer to the rotation next edge
    /// over the starting point of the half-edge.
    pub fn edge_rot_next(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<VoronoiEdgeIndex> {
        let _ = edge_id?;
        let prev = self._edge_get_prev(edge_id);
        self.edge_get_twin(prev)
    }

    #[inline]
    /// Returns a pointer to the rotation previous edge
    /// over the starting point of the half-edge.
    pub fn edge_rot_prev(&self, edge_id: Option<VoronoiEdgeIndex>) -> Option<VoronoiEdgeIndex> {
        let _ = edge_id?;
        let twin = self.edge_get_twin(edge_id);
        self._edge_get_next(twin)
    }

    /// Insert a new half-edge into the output data structure.
    /// Takes as input left and right sites that form a new bisector.
    /// Returns a pair of pointers to new half-edges.
    pub(crate) fn _insert_new_edge_2(
        &mut self,
        site1: VSE::SiteEvent<I1, F1, I2, F2>,
        site2: VSE::SiteEvent<I1, F1, I2, F2>,
    ) -> (VoronoiEdgeIndex, VoronoiEdgeIndex) {
        // Get sites' indexes.
        let site1_index = site1.sorted_index();
        let site2_index = site2.sorted_index();

        let is_linear = VSE::SiteEvent::is_linear_edge(&site1, &site2);
        let is_primary = VSE::SiteEvent::is_primary_edge(&site1, &site2);

        // Create a new half-edge that belongs to the first site.
        let edge1_id = self._edge_new_3(VoronoiCellIndex(site1_index), is_linear, is_primary);

        // Create a new half-edge that belongs to the second site.
        let edge2_id = self._edge_new_3(VoronoiCellIndex(site2_index), is_linear, is_primary);

        // Add the initial cell during the first edge insertion.
        if self.cells_.is_empty() {
            let _ = self._make_new_cell_with_category(
                VoronoiCellIndex(site1_index),
                site1.initial_index(),
                site1.source_category(),
            );
        }

        // The second site represents a new site during site event
        // processing. Add a new cell to the cell records.
        let _ = self._make_new_cell_with_category(
            VoronoiCellIndex(site2_index),
            site2.initial_index(),
            site2.source_category(),
        );

        // Set up pointers to cells. Todo! is this needed? Didn't we do this already?
        self._edge_set_cell(Some(edge1_id), Some(VoronoiCellIndex(site1_index)));
        self._edge_set_cell(Some(edge2_id), Some(VoronoiCellIndex(site2_index)));

        // Set up twin pointers.
        self._edge_set_twin(Some(edge1_id), Some(edge2_id));
        self._edge_set_twin(Some(edge2_id), Some(edge1_id));

        (edge1_id, edge2_id)
    }

    /// Insert a new half-edge into the output data structure with the
    /// start at the point where two previously added half-edges intersect.
    /// Takes as input two sites that create a new bisector, circle event
    /// that corresponds to the intersection point of the two old half-edges,
    /// pointers to those half-edges. Half-edges' direction goes out of the
    /// new Voronoi vertex point. Returns a pair of pointers to a new half-edges.
    pub(crate) fn _insert_new_edge_5(
        &mut self,
        site1: VSE::SiteEvent<I1, F1, I2, F2>,
        site3: VSE::SiteEvent<I1, F1, I2, F2>,
        circle: VC::CircleEvent<F2>,
        edge12_id: VoronoiEdgeIndex,
        edge23_id: VoronoiEdgeIndex,
    ) -> (VoronoiEdgeIndex, VoronoiEdgeIndex) {
        //println!("-> insert_new_edge_5()");
        //dbg!(&site1, &site3, &circle, edge12_id, edge23_id);
        #[cfg(feature = "console_debug")]
        println!("new vertex@CE{:?}", circle);

        let is_linear = VSE::SiteEvent::<I1, F1, I2, F2>::is_linear_edge(&site1, &site3);
        let is_primary = VSE::SiteEvent::<I1, F1, I2, F2>::is_primary_edge(&site1, &site3);

        // Add a new half-edge.
        let new_edge1_id = self._edge_new_3(
            VoronoiCellIndex(site1.sorted_index()),
            is_linear,
            is_primary,
        );

        // Add a new half-edge.
        let new_edge2_id = self._edge_new_3(
            VoronoiCellIndex(site3.sorted_index()),
            is_linear,
            is_primary,
        );

        // Add a new Voronoi vertex.
        let new_vertex_id = self._vertex_new_2(
            TC4::<I1, F1, I2, F2>::f2_to_f1(circle.raw_x()),
            TC4::<I1, F1, I2, F2>::f2_to_f1(circle.raw_y()),
            circle.is_site_point()
        );

        // Update vertex pointers of the old edges.
        self._edge_set_vertex0(Some(edge12_id), Some(new_vertex_id));
        self._edge_set_vertex0(Some(edge23_id), Some(new_vertex_id));

        // Update twin pointers.
        self._edge_set_twin(Some(new_edge1_id), Some(new_edge2_id));
        self._edge_set_twin(Some(new_edge2_id), Some(new_edge1_id));

        // Update vertex pointer.
        //new_edge2.vertex0(&new_vertex);
        self._edge_set_vertex0(Some(new_edge2_id), Some(new_vertex_id));

        // Update Voronoi prev/next pointers.
        //edge12->prev(&new_edge1);
        self._edge_set_prev(Some(edge12_id), Some(new_edge1_id));

        //new_edge1.next(edge12);
        self._edge_set_next(Some(new_edge1_id), Some(edge12_id));

        //edge12->twin()->next(edge23);
        let edge12_twin_id = self.edge_get_twin(Some(edge12_id));
        self._edge_set_next(edge12_twin_id, Some(edge23_id));

        //edge23->prev(edge12->twin());
        self._edge_set_prev(Some(edge23_id), edge12_twin_id);

        //edge23->twin()->next(&new_edge2);
        let edge23_twin_id = self.edge_get_twin(Some(edge23_id));
        self._edge_set_next(edge23_twin_id, Some(new_edge2_id));

        //new_edge2.prev(edge23->twin());
        self._edge_set_prev(Some(new_edge2_id), edge23_twin_id);

        // Return a pointer to the new half-edge.
        (new_edge1_id, new_edge2_id)
    }

    /// Make sure the diagram is consistent. Removes degenerate edges, connects incident
    /// edges etc. etc
    pub(crate) fn _build(&mut self) {
        // Remove degenerate edges.
        if !self.edges_.is_empty() {
            let mut last_edge: usize = 0;
            let mut it: usize = last_edge;
            let edges_end: usize = self.edges_.len();

            //let mut edges_to_erase: Vec<usize> = Vec::new();
            while it < edges_end {
                let is_equal = {
                    let v1 = self.edge_get_vertex0(Some(VoronoiEdgeIndex(it)));
                    let v1 = self.vertex_get(v1);
                    let v2 = self.edge_get_vertex1(Some(VoronoiEdgeIndex(it)));
                    let v2 = self.vertex_get(v2);
                    v1.is_some()
                        && v2.is_some()
                        && v1
                            .unwrap()
                            .get()
                            .vertex_equality_predicate_eq(&v2.unwrap().get())
                };
                if is_equal {
                    self._remove_edge(Some(VoronoiEdgeIndex(it)));
                } else {
                    if it != last_edge {
                        //edge_type * e1 = &(*last_edge = *it);
                        self._edge_copy(last_edge, it);
                        //edge_type * e2 = &(*(last_edge + 1) = *(it + 1));
                        self._edge_copy(last_edge + 1, it + 1);
                        let e1 = Some(VoronoiEdgeIndex(last_edge));
                        let e2 = Some(VoronoiEdgeIndex(last_edge + 1));

                        // e1->twin(e2);
                        self._edge_set_twin(e1, e2);

                        // e2->twin(e1);
                        self._edge_set_twin(e2, e1);

                        if self._edge_get_prev(e1).is_some() {
                            // e1 -> prev() -> next(e1);
                            self._edge_set_next(self._edge_get_prev(e1), e1);

                            //e2 -> next() -> prev(e2);
                            self._edge_set_prev(self._edge_get_next(e2), e2);
                        }
                        if self._edge_get_prev(e2).is_some() {
                            //e1 -> next() -> prev(e1);
                            self._edge_set_prev(self._edge_get_next(e1), e1);

                            //e2 -> prev() -> next(e2);
                            self._edge_set_next(self._edge_get_prev(e2), e2);
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

        // Set up incident edge pointers for cells and vertices.
        for edge_it in self.edge_iter().enumerate().map(|x| VoronoiEdgeIndex(x.0)) {
            let cell = self._edge_get_cell(Some(edge_it));
            self._cell_set_incident_edge(cell, Some(edge_it));
            let vertex = self.edge_get_vertex0(Some(edge_it));
            self._vertex_set_incident_edge(vertex, Some(edge_it));
        }

        // Remove degenerate vertices.
        if !self.vertices_.is_empty() {
            let mut last_vertex_iterator = (0..self.vertices_.len()).map(VoronoiVertexIndex);
            let mut last_vertex = last_vertex_iterator.next();
            for it in (0..self.vertices_.len()).map(VoronoiVertexIndex) {
                let it = Some(it);
                if self.vertex_get_incident_edge(it).is_some() {
                    if it != last_vertex {
                        self._vertex_copy(last_vertex.unwrap().0, it.unwrap().0);
                        let v = last_vertex;
                        let mut e = self.vertex_get_incident_edge(last_vertex);
                        loop {
                            //e->vertex0(v);
                            self._edge_set_vertex0(e, v);
                            // e = e->rot_next();
                            e = self.edge_rot_next(e);
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
        // Set up next/prev pointers for infinite edges.
        if self.vertices_.is_empty() {
            if !self.edges_.is_empty() {
                // Update prev/next pointers for the line edges.
                let mut edge_it = self.edges_.iter().enumerate().map(|x| x.0);

                let mut edge1 = edge_it.next().map(VoronoiEdgeIndex);
                self._edge_set_next(edge1, edge1);
                self._edge_set_prev(edge1, edge1);

                edge1 = edge_it.next().map(VoronoiEdgeIndex);
                let mut edge_it_value = edge_it.next();
                while edge_it_value.is_some() {
                    let edge2 = edge_it_value.map(VoronoiEdgeIndex);
                    edge_it_value = edge_it.next();
                    //dbg!(edge1.unwrap(),edge2.unwrap());

                    self._edge_set_next(edge1, edge2);
                    self._edge_set_prev(edge1, edge2);
                    self._edge_set_next(edge2, edge1);
                    self._edge_set_prev(edge2, edge1);

                    edge1 = edge_it_value.map(VoronoiEdgeIndex);
                    edge_it_value = edge_it.next();
                }
                self._edge_set_next(edge1, edge1);
                self._edge_set_prev(edge1, edge1);
            }
        } else {
            // Update prev/next pointers for the ray edges.
            //let mut cell_it_keys = self.cells_.keys();
            #[allow(clippy::while_let_on_iterator)]
            for cell_it in 0..self.cells_.len() {
                if self._cell_is_degenerate(Some(VoronoiCellIndex(cell_it))) {
                    continue;
                }
                // Move to the previous edge while
                // it is possible in the CW direction.
                let mut left_edge = self._cell_get_incident_edge(Some(VoronoiCellIndex(cell_it)));

                while let Some(new_left_edge) = self._edge_get_prev(left_edge) {
                    left_edge = Some(new_left_edge);
                    // Terminate if this is not a boundary cell.
                    if left_edge == self._cell_get_incident_edge(Some(VoronoiCellIndex(cell_it))) {
                        break;
                    }
                }

                if self._edge_get_prev(left_edge).is_some() {
                    continue;
                }

                let mut right_edge = self._cell_get_incident_edge(Some(VoronoiCellIndex(cell_it)));
                while let Some(new_right_edge) = self._edge_get_next(right_edge) {
                    right_edge = Some(new_right_edge);
                }

                self._edge_set_prev(left_edge, right_edge);
                self._edge_set_next(right_edge, left_edge);
            }
        }
    }

    /// prints cells and vertices to the console
    /// edges will be printed if the 'edge_filter' returns true for that edge id.
    pub fn debug_print_all<F>(&self, edge_filter: F)
    where
        F: Fn(usize) -> bool,
    {
        println!();
        println!("output:");
        for (i, c) in self.cells_.iter().enumerate() {
            let cc = c.get();
            print!("cell#{} {:?} ", i, &cc);
            if cc.contains_point() {
                println!("point");
            } else if cc.contains_segment() {
                println!("segment");
            } else if cc.contains_segment_startpoint() {
                println!("startpoint");
            } else if cc.contains_segment_endpoint() {
                println!("endpoint");
            } else {
                println!();
            }
        }
        for (i, v) in self.vertices_.iter().enumerate() {
            assert_eq!(i, v.get().id_.0);

            let edges1: Vec<usize> = self
                .edge_rot_next_iterator(v.get().get_incident_edge())
                .map(|x| x.0)
                .filter(|x| edge_filter(*x))
                .collect();
            let edges2: Vec<usize> = self
                .edge_rot_next_iterator(v.get().get_incident_edge())
                .map(|x| self.edge_get_twin(Some(x)))
                .flatten()
                .map(|x| x.0)
                .filter(|x| edge_filter(*x))
                .collect();
            //if !(edges1.is_empty() && edges2.is_empty()) {
            print!("vertex#{} {:?}", i, &v.get());
            print!(" outgoing edges:{:?}", edges1);
            println!(" incoming edges:{:?}", edges2);
            //}
        }
    }

    pub fn debug_print_edges(&self) {
        println!("edges:{}", self.edges_.len());
        for (i, e) in self.edges_.iter().enumerate() {
            let e = e.get();
            println!("Edge:#{}=>{:?}", e.id.0, &e);
            assert_eq!(i, e.id.0);
        }
    }
}
