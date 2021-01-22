// Boost.Polygon library voronoi_diagram.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::voronoi_beachline as VB;
use super::voronoi_circleevent as VC;
use super::voronoi_diagram as VD;
use super::voronoi_error::BVError;
use super::voronoi_predicate as VP;
use super::voronoi_siteevent as VSE;
use super::voronoi_structures as VS;
use super::{BigFloatType, BigIntType, BoostInputType, BoostOutputType};
use geo::{Point, Coordinate};
use std::cmp::Ordering;
use std::ops::Neg;

///
/// This was declared as "typedef std::pair<point_type, beach_line_iterator> end_point_type" in C++
///
#[derive(Debug)]
pub(crate) struct EndPointPair<I>
where
    I: BoostInputType + Neg<Output = I>,
{
    pub(crate) first: Coordinate<I>,            // TODO: better name
    pub(crate) second: VB::BeachLineIndex, // TODO: better name
}

impl<I> EndPointPair<I>
where
    I: BoostInputType + Neg<Output = I>,
{
    pub(crate) fn new_2(first: Coordinate<I>, second: VB::BeachLineIndex) -> Self {
        Self { first, second }
    }
}

impl<I> PartialOrd for EndPointPair<I>
where
    I: BoostInputType + Neg<Output = I>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I> Ord for EndPointPair<I>
where
    I: BoostInputType + Neg<Output = I>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if VP::PointComparisonPredicate::<I>::point_comparison_predicate(&self.first, &other.first)
        {
            Ordering::Greater
        } else if self.first == other.first {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl<I> PartialEq for EndPointPair<I>
where
    I: BoostInputType + Neg<Output = I>,
{
    fn eq(&self, other: &Self) -> bool {
        self.first.eq(&other.first)
    }
}

impl<I> Eq for EndPointPair<I> where I: BoostInputType + Neg<Output = I> {}
