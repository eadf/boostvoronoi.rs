// Boost.Polygon library voronoi_diagram.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

#[cfg(test)]
mod tests;

use super::diagram as VD;
use super::predicate as VP;
use std::cmp::Ordering;

use super::geometry::Point;
use super::{InputType, OutputType};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;
use std::ops::Neg;

pub type SiteEventIndexType = usize;

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
///   point0_ - point site or segment's start-point
///   point1_ - segment's endpoint if site is a segment
///   sorted_index_ - the last bit encodes information if the site is inverse;
///     the other VS::Bits encode site event index among the sorted site events
///   initial_index_ - site index among the initial input set
/// Note: for all sites is_inverse_ flag is equal to false by default.

#[derive(Copy, Clone)]
pub struct SiteEvent<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    point0_: Point<I>,
    point1_: Point<I>,
    sorted_index_: SiteEventIndexType,
    initial_index_: SiteEventIndexType,
    flags_: VD::ColorType,
    #[doc(hidden)]
    pdf_: PhantomData<F>,
}

impl<I, F> fmt::Debug for SiteEvent<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_point() {
            write!(
                f,
                "#{:?}({},{}),ii:{:?},f:{:?}",
                self.sorted_index_,
                self.point0_.x,
                self.point0_.y,
                self.initial_index_,
                self.flags_
            )
        } else {
            write!(
                f,
                "#{:?}({},{}){}({},{}),ii:{:?},f:{:?}",
                self.sorted_index_,
                self.point0_.x,
                self.point0_.y,
                if self.is_inverse() { "¿" } else { "-" },
                self.point1_.x,
                self.point1_.y,
                self.initial_index_,
                self.flags_
            )
        }
    }
}

impl<I, O> PartialOrd for SiteEvent<I, O>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(VP::EventComparisonPredicate::<I, O>::event_comparison_predicate_ii(self, other))
    }
}

impl<I, O> Ord for SiteEvent<I, O>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        VP::EventComparisonPredicate::<I, O>::event_comparison_predicate_ii(self, other)
    }
}

impl<I, O> PartialEq for SiteEvent<I, O>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
{
    fn eq(&self, other: &Self) -> bool {
        (self.point0_ == other.point0_) && (self.point1_ == other.point1_)
    }
}

impl<I, O> Eq for SiteEvent<I, O>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
{
}

impl<I, O> Hash for SiteEvent<I, O>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point0_.hash(state);
        self.point1_.hash(state);
        self.initial_index_.hash(state);
        self.flags_.hash(state);
    }
}

impl<I, F> SiteEvent<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    #[cfg(test)]
    /// only used by unit test code
    pub(crate) fn new_2(point: Point<I>, initial_index: SiteEventIndexType) -> SiteEvent<I, F> {
        Self {
            point0_: point,
            point1_: point,
            sorted_index_: 0,
            initial_index_: initial_index,
            flags_: VD::ColorBits::SINGLE_POINT__BIT.0,
            #[doc(hidden)]
            pdf_: PhantomData,
        }
    }

    pub(crate) fn new_3(
        a: Point<I>,
        b: Point<I>,
        initial_index: SiteEventIndexType,
    ) -> SiteEvent<I, F> {
        Self {
            point0_: a,
            point1_: b,
            sorted_index_: 0,
            initial_index_: initial_index,
            flags_: 0,
            #[doc(hidden)]
            pdf_: PhantomData,
        }
    }

    /// Only used by test code
    #[cfg(test)]
    pub fn new_7(
        x1: I,
        y1: I,
        x2: I,
        y2: I,
        initial_index: SiteEventIndexType,
        sorted_index: SiteEventIndexType,
        flags: VD::ColorType,
    ) -> SiteEvent<I, F> {
        Self {
            point0_: Point { x: x1, y: y1 },
            point1_: Point { x: x2, y: y2 },
            sorted_index_: sorted_index,
            initial_index_: initial_index,
            flags_: flags,
            pdf_: PhantomData,
        }
    }

    #[inline(always)]
    pub fn x(&self) -> I {
        self.point0_.x
    }

    #[inline(always)]
    pub fn y(&self) -> I {
        self.point0_.y
    }

    #[inline(always)]
    pub fn x0(&self) -> I {
        self.point0_.x
    }

    #[inline(always)]
    pub fn y0(&self) -> I {
        self.point0_.y
    }

    #[inline(always)]
    pub fn x1(&self) -> I {
        self.point1_.x
    }

    #[inline(always)]
    pub fn y1(&self) -> I {
        self.point1_.y
    }

    #[inline(always)]
    pub fn point0(&self) -> &Point<I> {
        &self.point0_
    }

    #[inline(always)]
    pub fn point1(&self) -> &Point<I> {
        &self.point1_
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
        mem::swap(&mut self.point0_, &mut self.point1_);
        self.flags_ ^= VD::ColorBits::IS_INVERSE__BIT.0;
        self
    }

    pub(crate) fn source_category(&self) -> VD::ColorBits {
        VD::ColorBits(self.flags_ & VD::ColorBits::RESERVED__MASK.0)
    }

    pub(crate) fn or_source_category(&mut self, source_category: &VD::ColorBits) {
        self.flags_ |= source_category.0;
    }

    /// only for basic test purposes
    #[inline(always)]
    #[cfg(test)]
    pub(crate) fn set_flags(&mut self, flags: u32) {
        self.flags_ = flags;
    }

    #[inline(always)]
    pub fn is_point(&self) -> bool {
        (self.point0_.x == self.point1_.x) && (self.point0_.y == self.point1_.y)
    }

    #[inline(always)]
    pub fn is_segment(&self) -> bool {
        (self.point0_.x != self.point1_.x) || (self.point0_.y != self.point1_.y)
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
}

impl<I, O> fmt::Display for SiteEvent<I, O>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
