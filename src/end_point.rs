// Boost.Polygon library detail/voronoi_structures.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code..

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

use crate::beach_line as VB;
use crate::geometry::Point;
use crate::predicate as VP;
use crate::InputType;
use std::cmp::Ordering;

///
/// This was declared as "typedef std::pair<point_type, beach_line_iterator> end_point_type" in C++
///
#[derive(Debug)]
pub(crate) struct EndPointPair<I>
where
    I: InputType,
{
    site_: Point<I>,
    beachline_index_: VB::BeachLineIndex,
}

impl<I> EndPointPair<I>
where
    I: InputType,
{
    pub(crate) fn new(first: Point<I>, second: VB::BeachLineIndex) -> Self {
        Self {
            site_: first,
            beachline_index_: second,
        }
    }

    /// Returns a reference to the site point
    pub(crate) fn site(&self) -> &Point<I> {
        &self.site_
    }

    /// Returns a reference to the beachline index
    pub(crate) fn beachline_index(&self) -> &VB::BeachLineIndex {
        &self.beachline_index_
    }
}

impl<I> PartialOrd for EndPointPair<I>
where
    I: InputType,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I> Ord for EndPointPair<I>
where
    I: InputType,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if VP::PointComparisonPredicate::<I>::point_comparison_predicate(&self.site_, &other.site_)
        {
            Ordering::Greater
        } else if self.site_ == other.site_ {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl<I> PartialEq for EndPointPair<I>
where
    I: InputType,
{
    fn eq(&self, other: &Self) -> bool {
        self.site_.eq(&other.site_)
    }
}

impl<I> Eq for EndPointPair<I> where I: InputType {}
