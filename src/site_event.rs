// Boost.Polygon library voronoi_diagram.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

mod tests;

use super::diagram as VD;
use super::predicate as VP;
use std::cmp::Ordering;

use super::Point;
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
pub struct SiteEvent<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    pub(crate) point0_: Point<I1>,
    pub(crate) point1_: Point<I1>,
    pub sorted_index_: SiteEventIndexType,
    initial_index_: SiteEventIndexType,
    flags_: VD::ColorType,
    #[doc(hidden)]
    _pdo: PhantomData<F1>,
}

impl<I1, F1> fmt::Debug for SiteEvent<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rv = String::new();

        if self.is_point() {
            rv.push_str(
                format!(
                    "#{:?}({},{}),ii:{:?},f:{:?}",
                    self.sorted_index_,
                    self.point0_.x,
                    self.point0_.y,
                    self.initial_index_,
                    self.flags_
                )
                .as_str(),
            );
        } else {
            rv.push_str(
                format!(
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
                .as_str(),
            );
        }
        write!(f, "{}", rv)
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
        self.sorted_index_.hash(state);
        self.initial_index_.hash(state);
        self.flags_.hash(state);
    }
}

impl<I1, F1> SiteEvent<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    pub fn new_2(a: Point<I1>, initial_index_: SiteEventIndexType) -> SiteEvent<I1, F1> {
        Self {
            point0_: a,
            point1_: a,
            sorted_index_: 0,
            initial_index_,
            flags_: VD::ColorBits::SINGLE_POINT.0,
            #[doc(hidden)]
            _pdo: PhantomData,
        }
    }

    pub fn new_3(
        a: Point<I1>,
        b: Point<I1>,
        initial_index_: SiteEventIndexType,
    ) -> SiteEvent<I1, F1> {
        Self {
            point0_: a,
            point1_: b,
            sorted_index_: 0,
            initial_index_,
            flags_: 0,
            #[doc(hidden)]
            _pdo: PhantomData,
        }
    }

    /// used by test code
    pub fn new_7(
        x1: I1,
        y1: I1,
        x2: I1,
        y2: I1,
        initial_index: usize,
        sorted_index: usize,
        flags: u32,
    ) -> SiteEvent<I1, F1> {
        Self {
            point0_: Point { x: x1, y: y1 },
            point1_: Point { x: x2, y: y2 },
            sorted_index_: sorted_index,
            initial_index_: initial_index,
            flags_: flags,
            _pdo: PhantomData,
        }
    }
    /*
    pub(crate) fn is_single_point(&self) -> bool {
        self.flags_ & VD::SourceCategory::SOURCE_CATEGORY_BITMASK.0
            == VD::SourceCategory::SOURCE_CATEGORY_SINGLE_POINT.0
    }*/
    /*
    pub(crate) fn is_segment_start_point(&self) -> bool {
        self.flags_ & VD::SourceCategory::SOURCE_CATEGORY_BITMASK.0
            == VD::SourceCategory::SOURCE_CATEGORY_SEGMENT_START_POINT.0
    }*/
    /*
    pub(crate) fn is_segment_end_point(&self) -> bool {
        self.flags_ & VD::SourceCategory::SOURCE_CATEGORY_BITMASK.0
            == VD::SourceCategory::SOURCE_CATEGORY_SEGMENT_END_POINT.0
    }*/

    #[inline(always)]
    pub fn x(&self) -> I1 {
        self.point0_.x
    }

    #[inline(always)]
    pub fn y(&self) -> I1 {
        self.point0_.y
    }

    #[inline(always)]
    pub fn x0(&self) -> I1 {
        self.point0_.x
    }

    #[inline(always)]
    pub fn y0(&self) -> I1 {
        self.point0_.y
    }

    #[inline(always)]
    pub fn x1(&self) -> I1 {
        self.point1_.x
    }

    #[inline(always)]
    pub fn y1(&self) -> I1 {
        self.point1_.y
    }

    #[inline(always)]
    pub fn point0(&self) -> &Point<I1> {
        &self.point0_
    }
    #[inline(always)]
    pub fn point1(&self) -> &Point<I1> {
        &self.point1_
    }

    #[inline(always)]
    pub fn sorted_index(&self) -> usize {
        self.sorted_index_
    }

    #[inline(always)]
    pub fn set_sorted_index(&mut self, index: usize) {
        self.sorted_index_ = index;
    }

    #[inline(always)]
    pub(crate) fn initial_index(&self) -> usize {
        self.initial_index_
    }

    pub(crate) fn is_inverse(&self) -> bool {
        (self.flags_ & VD::ColorBits::IS_INVERSE_BITMASK.0) != 0
    }

    pub(crate) fn inverse(&mut self) -> &mut Self {
        mem::swap(&mut self.point0_, &mut self.point1_);
        self.flags_ ^= VD::ColorBits::IS_INVERSE_BITMASK.0;
        self
    }

    pub(crate) fn source_category(&self) -> VD::ColorBits {
        VD::ColorBits(self.flags_ & VD::ColorBits::BITMASK.0)
    }

    pub(crate) fn or_source_category(&mut self, source_category: &VD::ColorBits) {
        self.flags_ |= source_category.0;
    }

    /// only for basic test purposes
    #[inline(always)]
    #[allow(dead_code)]
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
    pub fn is_primary_edge(site1: &SiteEvent<I1, F1>, site2: &SiteEvent<I1, F1>) -> bool {
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

    pub fn is_linear_edge(site1: &SiteEvent<I1, F1>, site2: &SiteEvent<I1, F1>) -> bool {
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
