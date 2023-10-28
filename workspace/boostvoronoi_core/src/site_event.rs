// Boost.Polygon library detail/voronoi_structures.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

#[cfg(test)]
mod tests;

use crate::diagram as VD;
use crate::predicate as VP;
use std::cmp::Ordering;

use super::geometry::{Line, Point};
use super::{InputType, OutputType};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;

pub type SiteEventIndexType = usize;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
pub(crate) enum Site<I: InputType> {
    Point(Point<I>),
    Segment(Point<I>, Point<I>),
}

impl<I: InputType> From<Line<I>> for Site<I> {
    fn from(l: Line<I>) -> Self {
        Site::Segment(l.start, l.end)
    }
}

impl<I: InputType> Site<I> {
    /// Reverse the order of a `Segment`, does nothing for a `Point`.
    pub(crate) fn reverse(self) -> Self {
        match self {
            Site::Segment(a, b) => Site::Segment(b, a),
            _ => self,
        }
    }
}

/// Site event type.
/// Occurs when the sweepline sweeps over one of the initial sites:
///   1) point site
///   2) start-point of the segment site
///   3) endpoint of the segment site
/// Implicit segment direction is defined: the start-point of
/// the segment compares less than its endpoint.
/// Each input segment is divided onto two site events:
///   1) One going from the start-point to the endpoint
///      (is_inverse() = false)
///   2) Another going from the endpoint to the start-point
///      (is_inverse() = true)
/// In beach line data structure segment sites of the first
/// type precede sites of the second type for the same segment.
/// Members:
///   `point0_` - point site or segment's start-point
///   `point1_` - segment's endpoint if site is a segment
///   `sorted_index_` - the last bit encodes information if the site is inverse;
///                     the other VS::Bits encode site event index among the sorted site events
///   `initial_index_` - site index among the initial input set
/// Note: for all sites `is_inverse_` flag is set to `false` by default.
#[derive(Copy, Clone)]
pub struct SiteEvent<I: InputType, F: OutputType> {
    site_: Site<I>,
    sorted_index_: SiteEventIndexType,
    initial_index_: SiteEventIndexType,
    flags_: VD::ColorType,
    #[doc(hidden)]
    pd_: PhantomData<fn(F) -> F>,
}

impl<I: InputType, F: OutputType> fmt::Debug for SiteEvent<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.site_ {
            Site::Point(point) => write!(
                f,
                "#{:?}({},{}),ii:{:?},f:{:?}",
                self.sorted_index_, point.x, point.y, self.initial_index_, self.flags_
            ),
            Site::Segment(point0, point1) => write!(
                f,
                "#{:?}({},{}){}({},{}),ii:{:?},f:{:?}",
                self.sorted_index_,
                point0.x,
                point0.y,
                if self.is_inverse() { "¿" } else { "-" },
                point1.x,
                point1.y,
                self.initial_index_,
                self.flags_
            ),
        }
    }
}

#[allow(clippy::incorrect_partial_ord_impl_on_ord_type)]
impl<I: InputType, F: OutputType> PartialOrd for SiteEvent<I, F> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(VP::event_comparison_predicate::event_comparison_ii::<I, F>(
            self, other,
        ))
    }
}

impl<I: InputType, F: OutputType> Ord for SiteEvent<I, F> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        VP::event_comparison_predicate::event_comparison_ii::<I, F>(self, other)
    }
}

impl<I: InputType, F: OutputType> PartialEq for SiteEvent<I, F> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.site_.eq(&other.site_)
    }
}

impl<I: InputType, F: OutputType> Eq for SiteEvent<I, F> {}

impl<I: InputType, F: OutputType> Hash for SiteEvent<I, F> {
    #[inline]
    /// sorted_index_ is not part of the hash
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.site_.hash(state);
        self.initial_index_.hash(state);
        self.flags_.hash(state);
    }
}

impl<I: InputType, F: OutputType> SiteEvent<I, F> {
    /// Creates a new SiteEvent from a `Site`.
    pub(crate) fn new(site: Site<I>, initial_index: SiteEventIndexType) -> Self {
        match site {
            Site::Point(_) => Self {
                site_: site,
                sorted_index_: 0,
                initial_index_: initial_index,
                flags_: VD::ColorBits::SINGLE_POINT__BIT.0,
                pd_: PhantomData,
            },
            Site::Segment(_, _) => Self {
                site_: site,
                sorted_index_: 0,
                initial_index_: initial_index,
                flags_: 0,
                pd_: PhantomData,
            },
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    /// Only used by test code
    pub fn new_7(
        x0: I,
        y0: I,
        x1: I,
        y1: I,
        initial_index: SiteEventIndexType,
        sorted_index: SiteEventIndexType,
        flags: VD::ColorType,
    ) -> SiteEvent<I, F> {
        let site = if x0 == x1 && y0 == y1 {
            Site::Point(Point { x: x0, y: y0 })
        } else {
            Site::Segment(Point { x: x0, y: y0 }, Point { x: x1, y: y1 })
        };
        Self {
            site_: site,
            sorted_index_: sorted_index,
            initial_index_: initial_index,
            flags_: flags,
            pd_: PhantomData,
        }
    }

    #[cfg(feature = "ce_corruption_check")]
    #[allow(dead_code)]
    pub fn dbg(&self) {
        match &self.site_ {
            Site::Point(point0) => {
                println!(
                    "[{},{}];",
                    super::cast::<I, f64>(point0.x),
                    super::cast::<I, f64>(point0.y)
                );
            }
            Site::Segment(point0, point1) => {
                println!(
                    "[{},{},{},{}];",
                    super::cast::<I, f64>(point0.x),
                    super::cast::<I, f64>(point0.y),
                    super::cast::<I, f64>(point1.x),
                    super::cast::<I, f64>(point1.y)
                );
            }
        }
    }

    /*#[allow(dead_code)]
    pub fn build(&self) {
        //let _ = crate::site_event::SiteEvent::<I,F>::new_7(self.x0(),self.y0(), self.x1(), self.y1(), self.initial_index_, self.sorted_index_, self.flags_);
        print!(
            "({},{},{},{},{},{},{:?})",
            self.x0(),
            self.y0(),
            self.x1(),
            self.y1(),
            self.initial_index_,
            self.sorted_index_,
            self.flags_
        );
    }*/

    #[inline(always)]
    pub(crate) fn x(&self) -> I {
        self.x0()
    }

    #[inline(always)]
    pub(crate) fn y(&self) -> I {
        self.y0()
    }

    #[inline(always)]
    pub fn x0(&self) -> I {
        match &self.site_ {
            Site::Point(p) => p.x,
            Site::Segment(p, _) => p.x,
        }
    }

    #[inline(always)]
    pub fn y0(&self) -> I {
        match &self.site_ {
            Site::Point(p) => p.y,
            Site::Segment(p, _) => p.y,
        }
    }

    #[inline(always)]
    pub fn x1(&self) -> I {
        match &self.site_ {
            Site::Point(p) => {
                // todo: This should not happen (but it happens in some tests)
                // debug_assert!(false, "Site was not a segment");
                p.x
            }
            Site::Segment(_, p) => p.x,
        }
    }

    #[inline(always)]
    pub fn y1(&self) -> I {
        match &self.site_ {
            Site::Point(p) => {
                // todo: This should not happen (but it happens in some tests)
                // debug_assert!(false, "Site was not a segment");
                p.y
            }
            Site::Segment(_, p) => p.y,
        }
    }

    #[inline(always)]
    pub(crate) fn point0(&self) -> Point<I> {
        match self.site_ {
            Site::Point(p) => p,
            Site::Segment(p, _) => p,
        }
    }

    #[inline(always)]
    pub(crate) fn point1(&self) -> Point<I> {
        match self.site_ {
            Site::Point(p) => {
                // todo: This should not happen (but it happens in some tests)
                // debug_assert!(false, "Site was not a segment");
                p
            }
            Site::Segment(_, p) => p,
        }
    }

    #[inline(always)]
    pub fn sorted_index(&self) -> SiteEventIndexType {
        self.sorted_index_
    }

    #[inline(always)]
    pub fn set_sorted_index(&mut self, index: SiteEventIndexType) {
        self.sorted_index_ = index;
    }

    #[inline(always)]
    pub(crate) fn initial_index(&self) -> SiteEventIndexType {
        self.initial_index_
    }

    // todo this looks suspicious
    pub(crate) fn is_inverse(&self) -> bool {
        (self.flags_ & VD::ColorBits::IS_INVERSE__BIT.0) != 0
    }

    // todo this looks suspicious
    pub(crate) fn inverse(&mut self) -> &mut Self {
        match &mut self.site_ {
            Site::Segment(p0, p1) => {
                mem::swap(p0, p1);
            }
            _ => {
                debug_assert!(false, "inverse() should never be called on points");
            }
        }

        self.flags_ ^= VD::ColorBits::IS_INVERSE__BIT.0;
        self
    }

    pub(crate) fn source_category(&self) -> VD::ColorBits {
        VD::ColorBits(self.flags_ & VD::ColorBits::RESERVED__MASK.0)
    }

    pub(crate) fn or_source_category(&mut self, source_category: VD::ColorBits) {
        self.flags_ |= source_category.0;
    }

    /// only for basic test purposes
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn set_flags(&mut self, flags: u32) {
        self.flags_ = flags;
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_point(&self) -> bool {
        !self.is_segment()
    }

    #[inline(always)]
    /// Returns true if the site is a segment, false if it is a point
    pub fn is_segment(&self) -> bool {
        match self.site_ {
            Site::Point(_) => false,
            Site::Segment(_, _) => true,
        }
    }

    #[allow(unknown_lints)]
    #[allow(clippy::suspicious_operation_groupings)]
    pub fn is_primary_edge(site1: &SiteEvent<I, F>, site2: &SiteEvent<I, F>) -> bool {
        let flag1 = site1.is_segment();
        let flag2 = site2.is_segment();
        if flag1 && !flag2 {
            return (site1.point0() != site2.point0()) && (site1.point1() != site2.point0());
        }
        if !flag1 && flag2 {
            return (site2.point0() != site1.point0()) && (site2.point1() != site1.point0());
        }
        true
    }

    pub fn is_linear_edge(site1: &SiteEvent<I, F>, site2: &SiteEvent<I, F>) -> bool {
        if !Self::is_primary_edge(site1, site2) {
            return true;
        }
        !(site1.is_segment() ^ site2.is_segment())
    }

    #[cfg(all(feature = "ce_corruption_check", feature = "geo"))]
    #[inline(always)]
    pub fn distance_to_point(&self, x: f64, y: f64) -> f64 {
        use geo::algorithm::euclidean_distance::*;
        let c = geo::Coord { x, y };

        if self.is_point() {
            c.euclidean_distance(&geo::Coord::from(self.point0().as_f64()))
        } else {
            c.euclidean_distance(&geo::Line::new(
                geo::Coord::from(self.point0().as_f64()),
                geo::Coord::from(self.point1().as_f64()),
            ))
        }
    }

    #[inline(always)]
    pub(crate) fn is_vertical(&self) -> bool {
        self.point0().x == self.point1().x
    }
}

impl<I: InputType, F: OutputType> fmt::Display for SiteEvent<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
