// Boost.Polygon library detail/voronoi_predicates.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! Predicate utilities

#[cfg(test)]
mod tests;

use crate::beach_line as VB;
use crate::cast;
use crate::circle_event as VC;
use crate::ctypes::ulp_comparison;
use crate::extended_exp_fpt as EX;
use crate::extended_int as EI;
use crate::robust_fpt as RF;
use crate::site_event as VSE;
use crate::{geometry::Point, t, tln, InputType, OutputType};
use num::{Float, NumCast, PrimInt, Zero};
use num_traits::One;
use std::cmp;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::Neg;

// TODO: how to make these generic?
const ULPS: u64 = 64;
const ULPSX2: u64 = 64; // Todo: This is what c++ boost uses. Find a fix for this

#[derive(Copy, Clone, Eq, PartialEq)]
enum SiteIndex {
    One,
    Two,
    Three,
}

impl Debug for SiteIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SiteIndex::One => 1,
                SiteIndex::Two => 2,
                SiteIndex::Three => 3,
            }
        )
    }
}

/// Predicate utilities. Operates with the coordinate types that could
/// be converted to the 32-bit signed integer without precision loss.
/// Todo! give this a lookover
#[derive(Default)]
pub struct Predicates {}

impl Predicates {
    #[inline(always)]
    pub(crate) fn is_vertical_1<I: InputType, F: OutputType>(site: &VSE::SiteEvent<I, F>) -> bool {
        Self::is_vertical_2::<I, F>(site.point0(), site.point1())
    }

    #[inline(always)]
    pub(crate) fn is_vertical_2<I: InputType, F: OutputType>(
        point1: &Point<I>,
        point2: &Point<I>,
    ) -> bool {
        point1.x == point2.x
    }

    /// Compute robust cross_product: a1 * b2 - b1 * a2.
    /// It was mathematically proven that the result is correct
    /// with epsilon relative error equal to 1EPS.
    /// TODO: this is supposed to use u32 if I==i32
    #[inline(always)]
    pub(crate) fn robust_cross_product<I: InputType, F: OutputType>(
        a1: i64,
        b1: i64,
        a2: i64,
        b2: i64,
    ) -> f64 {
        robust_cross_product_f::<i64, f64>(a1, b1, a2, b2)
    }

    #[inline(always)]
    pub(crate) fn ulps() -> u64 {
        // todo figure out how to cache this
        if std::mem::size_of::<f64>() > 4 {
            ULPSX2
        } else {
            ULPS
        }
    }
}

/// Compute robust cross_product: a1 * b2 - b1 * a2.
/// It was mathematically proven that the result is correct
/// with epsilon relative error equal to 1EPS.
#[inline]
fn robust_cross_product_f<T, U>(s_a1: T, s_b1: T, s_a2: T, s_b2: T) -> U
where
    T: PrimInt
        + PartialOrd
        + PartialEq
        + NumCast
        + Copy
        + Clone
        + Display
        + Default
        + Debug
        + Zero
        + Neg<Output = T>,
    U: Float
        + PartialOrd
        + PartialEq
        + NumCast
        + Copy
        + Clone
        + Display
        + Default
        + Debug
        + Zero
        + Neg<Output = U>,
{
    // Why can't *all* integers implement is_negative()? E.g u64 would just always return false.
    // It would make it easier to implement generic code
    let u_a1 = if s_a1 < T::zero() { -s_a1 } else { s_a1 };
    let u_b1 = if s_b1 < T::zero() { -s_b1 } else { s_b1 };
    let u_a2 = if s_a2 < T::zero() { -s_a2 } else { s_a2 };
    let u_b2 = if s_b2 < T::zero() { -s_b2 } else { s_b2 };

    let l = u_a1 * u_b2;
    let r = u_b1 * u_a2;

    if (s_a1 < T::zero()) ^ (s_b2 < T::zero()) {
        return if (s_a2 < T::zero()) ^ (s_b1 < T::zero()) {
            if l > r {
                -num::cast::<T, U>(l - r).unwrap()
            } else {
                num::cast::<T, U>(r - l).unwrap()
            }
        } else {
            -num::cast::<T, U>(l + r).unwrap()
        };
    }
    if (s_a2 < T::zero()) ^ (s_b1 < T::zero()) {
        return num::cast::<T, U>(l + r).unwrap();
    }
    if l < r {
        -num::cast::<T, U>(r - l).unwrap()
    } else {
        num::cast::<T, U>(l - r).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Right,     // = -1,
    Collinear, // = 0,
    Left,      // = 1
}

#[derive(Default)]
pub struct OrientationTest {}

impl OrientationTest {
    /// Value is a determinant of two vectors (e.g. x1 * y2 - x2 * y1).
    /// Return orientation based on the sign of the determinant.
    #[inline(always)]
    fn eval_f<I: InputType, F: OutputType>(value: f64) -> Orientation {
        if value.is_zero() {
            return Orientation::Collinear;
        }
        match value.is_sign_negative() {
            true => Orientation::Right,
            false => Orientation::Left,
        }
    }

    #[inline(always)]
    fn eval_p<I: InputType, F: OutputType>(
        point1: &Point<I>,
        point2: &Point<I>,
        point3: &Point<I>,
    ) -> Orientation {
        let dx1: i64 = cast::<I, i64>(point1.x) - cast::<I, i64>(point2.x);
        let dx2: i64 = cast::<I, i64>(point2.x) - cast::<I, i64>(point3.x);
        let dy1: i64 = cast::<I, i64>(point1.y) - cast::<I, i64>(point2.y);
        let dy2: i64 = cast::<I, i64>(point2.y) - cast::<I, i64>(point3.y);
        let cp: f64 = Predicates::robust_cross_product::<I, F>(dx1, dy1, dx2, dy2);
        Self::eval_f::<I, F>(cp)
    }

    #[inline(always)]
    fn eval_i<I: InputType, F: OutputType>(
        dif_x1: i64,
        dif_y1: i64,
        dif_x2: i64,
        dif_y2: i64,
    ) -> Orientation {
        Self::eval_f::<I, F>(Predicates::robust_cross_product::<I, F>(
            dif_x1, dif_y1, dif_x2, dif_y2,
        ))
    }
}

#[derive(Default)]
pub struct PointComparisonPredicate<I: InputType> {
    #[doc(hidden)]
    pdi_: PhantomData<fn(I) -> I>,
}

impl<I: InputType> PointComparisonPredicate<I> {
    /// returns true if lhs.x < rhs.x, if lhs.x==rhs.x it returns lhs.y < rhs.y
    #[inline(always)]
    pub(crate) fn point_comparison_predicate(lhs: &Point<I>, rhs: &Point<I>) -> bool {
        if lhs.x == rhs.x {
            lhs.y < rhs.y
        } else {
            lhs.x < rhs.x
        }
    }
}

#[derive(Default)]
pub struct EventComparisonPredicate {}

impl EventComparisonPredicate {
    /// boolean predicate between two sites (bool int int)
    pub(crate) fn event_comparison_predicate_bii<I: InputType, F: OutputType>(
        lhs: &VSE::SiteEvent<I, F>,
        rhs: &VSE::SiteEvent<I, F>,
    ) -> bool {
        if lhs.x0() != rhs.x0() {
            return lhs.x0() < rhs.x0();
        }
        if !lhs.is_segment() {
            if !rhs.is_segment() {
                return lhs.y0() < rhs.y0();
            }
            if Predicates::is_vertical_2::<I, F>(rhs.point0(), rhs.point1()) {
                return lhs.y0() <= rhs.y0();
            }
            true
        } else {
            if Predicates::is_vertical_2::<I, F>(rhs.point0(), rhs.point1()) {
                if Predicates::is_vertical_2::<I, F>(lhs.point0(), lhs.point1()) {
                    return lhs.y0() < rhs.y0();
                }
                return false;
            }
            if Predicates::is_vertical_2::<I, F>(lhs.point0(), lhs.point1()) {
                return true;
            }
            if lhs.y0() != rhs.y0() {
                return lhs.y0() < rhs.y0();
            }
            return OrientationTest::eval_p::<I, F>(lhs.point1(), lhs.point0(), rhs.point1())
                == Orientation::Left;
        }
    }

    /// cmp::Ordering predicate between two sites (int int)
    pub(crate) fn event_comparison_predicate_ii<I: InputType, F: OutputType>(
        lhs: &VSE::SiteEvent<I, F>,
        rhs: &VSE::SiteEvent<I, F>,
    ) -> cmp::Ordering {
        #[cfg(feature = "console_debug")]
        // this is technically not needed as ordering of identical point sites is random in C++ boost
        if lhs.is_point() && rhs.is_point() && lhs.point0() == rhs.point0() {
            return if lhs.initial_index() < rhs.initial_index() {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Less
            };
        }
        if Self::event_comparison_predicate_bii(lhs, rhs) {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }

    /// boolean predicate between site and circle (Bool Integer Float)
    #[allow(clippy::let_and_return)]
    pub(crate) fn event_comparison_predicate_bif<I: InputType, F: OutputType>(
        lhs: &VSE::SiteEvent<I, F>,
        rhs: &VC::CircleEvent,
    ) -> bool {
        let lhs = cast::<I, f64>(lhs.x0());
        let rhs = rhs.lower_x().into_inner();
        let ulps = Predicates::ulps();
        let rv = ulp_comparison(lhs, rhs, ulps) == cmp::Ordering::Less;
        tln!(
            "event_comparison_predicate_bif lhs:{:.12} rhs:{:.12} -> {}",
            lhs,
            rhs,
            rv
        );
        rv
    }
}

/// Represents the result of the epsilon robust predicate. If the
/// result is undefined some further processing is usually required.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq)]
enum KPredicateResult {
    LESS,      // = -1,
    UNDEFINED, // = 0,
    MORE,      // = 1
}

pub struct DistancePredicate {}

impl DistancePredicate {
    #[cfg(feature = "console_debug")]
    #[allow(dead_code)]
    #[inline(always)]
    /// Returns true if a horizontal line going through a new site intersects
    /// right arc at first, else returns false. If horizontal line goes
    /// through intersection point of the given two arcs returns false also.
    pub(crate) fn distance_predicate_fake<I: InputType, F: OutputType>(
        left_site: &VSE::SiteEvent<I, F>,
        right_site: &VSE::SiteEvent<I, F>,
        new_point: &Point<I>,
    ) -> bool {
        let rv = Self::distance_predicate(left_site, right_site, new_point);
        tln!(
            "DistancePredicate(L:{:?}, R:{:?}, K:{:?})=={}",
            left_site,
            right_site,
            new_point,
            rv
        );
        rv
    }

    pub(crate) fn distance_predicate<I: InputType, F: OutputType>(
        left_site: &VSE::SiteEvent<I, F>,
        right_site: &VSE::SiteEvent<I, F>,
        new_point: &Point<I>,
    ) -> bool {
        if !left_site.is_segment() {
            if !right_site.is_segment() {
                Self::pp(left_site, right_site, new_point)
            } else {
                Self::ps(left_site, right_site, new_point, false)
            }
        } else if !right_site.is_segment() {
            Self::ps(right_site, left_site, new_point, true)
        } else {
            Self::ss(left_site, right_site, new_point)
        }
    }

    /// Robust predicate, avoids using high-precision libraries.
    /// Returns true if a horizontal line going through the new point site
    /// intersects right arc at first, else returns false. If horizontal line
    /// goes through intersection point of the given two arcs returns false.
    fn pp<I: InputType, F: OutputType>(
        left_site: &VSE::SiteEvent<I, F>,
        right_site: &VSE::SiteEvent<I, F>,
        new_point: &Point<I>,
    ) -> bool {
        let left_point = left_site.point0();
        let right_point = right_site.point0();
        //dbg!(&left_site, &right_site, &new_point);
        //dbg!(left_point.x, left_point.y);
        //dbg!(right_point.x, right_point.y);

        match left_point.x.cmp(&right_point.x) {
            cmp::Ordering::Greater => {
                if new_point.y <= left_point.y {
                    return false;
                }
            }
            cmp::Ordering::Less => {
                if new_point.y >= right_point.y {
                    return true;
                }
            }
            _ => {
                return cast::<I, i64>(left_point.y) + cast::<I, i64>(right_point.y)
                    < cast::<I, i64>(new_point.y) * 2
            }
        }

        let dist1 = Self::find_distance_to_point_arc(left_site, new_point);
        let dist2 = Self::find_distance_to_point_arc(right_site, new_point);

        // The undefined ulp range is equal to 3EPS + 3EPS <= 6ULP.
        dist1 < dist2
    }

    fn ps<I: InputType, F: OutputType>(
        left_site: &VSE::SiteEvent<I, F>,
        right_site: &VSE::SiteEvent<I, F>,
        new_point: &Point<I>,
        reverse_order: bool,
    ) -> bool {
        let fast_res = Self::fast_ps(left_site, right_site, new_point, reverse_order);
        if fast_res != KPredicateResult::UNDEFINED {
            return fast_res == KPredicateResult::LESS;
        }

        let dist1 = Self::find_distance_to_point_arc(left_site, new_point);
        let dist2 = Self::find_distance_to_segment_arc(right_site, new_point);

        // The undefined ulp range is equal to 3EPS + 7EPS <= 10ULP.
        reverse_order ^ (dist1 < dist2)
    }

    fn ss<I: InputType, F: OutputType>(
        left_site: &VSE::SiteEvent<I, F>,
        right_site: &VSE::SiteEvent<I, F>,
        new_point: &Point<I>,
    ) -> bool {
        // Handle temporary segment sites.
        if left_site.sorted_index() == right_site.sorted_index() {
            return OrientationTest::eval_p::<I, F>(
                left_site.point0(),
                left_site.point1(),
                new_point,
            ) == Orientation::Left;
        }

        let dist1 = Self::find_distance_to_segment_arc::<I, F>(left_site, new_point);
        let dist2 = Self::find_distance_to_segment_arc::<I, F>(right_site, new_point);

        // The undefined ulp range is equal to 7EPS + 7EPS <= 14ULP.
        dist1 < dist2
    }

    #[inline(always)]
    fn find_distance_to_point_arc<I: InputType, F: OutputType>(
        site: &VSE::SiteEvent<I, F>,
        point: &Point<I>,
    ) -> f64 {
        let dx = cast::<I, f64>(site.x()) - cast::<I, f64>(point.x);
        let dy = cast::<I, f64>(site.y()) - cast::<I, f64>(point.y);
        // The relative error is at most 3EPS.
        (dx * dx + dy * dy) / (dx * 2_f64)
    }

    fn find_distance_to_segment_arc<I: InputType, F: OutputType>(
        site: &VSE::SiteEvent<I, F>,
        point: &Point<I>,
    ) -> f64 {
        if Predicates::is_vertical_1::<I, F>(site) {
            (cast::<I, f64>(site.x()) - cast::<I, f64>(point.x)) * 0.5_f64
        } else {
            let segment0: &Point<I> = site.point0();
            let segment1: &Point<I> = site.point1();
            let a1: f64 = cast::<I, f64>(segment1.x) - cast::<I, f64>(segment0.x);
            let b1: f64 = cast::<I, f64>(segment1.y) - cast::<I, f64>(segment0.y);
            let mut k: f64 = (a1 * a1 + b1 * b1).sqrt();
            // Avoid subtraction while computing k.
            #[allow(clippy::suspicious_operation_groupings)]
            if !b1.is_sign_negative() {
                k = 1_f64 / (b1 + k);
            } else {
                k = (k - b1) / (a1 * a1);
            }
            // The relative error is at most 7EPS.
            k * Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(segment1.x) - cast::<I, i64>(segment0.x),
                cast::<I, i64>(segment1.y) - cast::<I, i64>(segment0.y),
                cast::<I, i64>(point.x) - cast::<I, i64>(segment0.x),
                cast::<I, i64>(point.y) - cast::<I, i64>(segment0.y),
            )
        }
    }

    fn fast_ps<I: InputType, F: OutputType>(
        left_site: &VSE::SiteEvent<I, F>,
        right_site: &VSE::SiteEvent<I, F>,
        new_point: &Point<I>,
        reverse_order: bool,
    ) -> KPredicateResult {
        let site_point: &Point<I> = left_site.point0();
        let segment_start: &Point<I> = right_site.point0();
        let segment_end: &Point<I> = right_site.point1();
        let eval: Orientation =
            OrientationTest::eval_p::<I, F>(segment_start, segment_end, new_point);
        if eval != Orientation::Right {
            return if !right_site.is_inverse() {
                KPredicateResult::LESS
            } else {
                KPredicateResult::MORE
            };
        }

        let dif_x = cast::<I, f64>(new_point.x) - cast::<I, f64>(site_point.x);
        let dif_y = cast::<I, f64>(new_point.y) - cast::<I, f64>(site_point.y);
        let a = cast::<I, f64>(segment_end.x) - cast::<I, f64>(segment_start.x);
        let b = cast::<I, f64>(segment_end.y) - cast::<I, f64>(segment_start.y);

        if Predicates::is_vertical_1::<I, F>(right_site) {
            if new_point.y < site_point.y && !reverse_order {
                return KPredicateResult::MORE;
            } else if new_point.y > site_point.y && reverse_order {
                return KPredicateResult::LESS;
            }
            return KPredicateResult::UNDEFINED;
        } else {
            let orientation = OrientationTest::eval_i::<I, F>(
                cast::<I, i64>(segment_end.x) - cast::<I, i64>(segment_start.x),
                cast::<I, i64>(segment_end.y) - cast::<I, i64>(segment_start.y),
                cast::<I, i64>(new_point.x) - cast::<I, i64>(site_point.x),
                cast::<I, i64>(new_point.y) - cast::<I, i64>(site_point.y),
            );
            if orientation == Orientation::Left {
                if !right_site.is_inverse() {
                    return if reverse_order {
                        KPredicateResult::LESS
                    } else {
                        KPredicateResult::UNDEFINED
                    };
                }
                return if reverse_order {
                    KPredicateResult::UNDEFINED
                } else {
                    KPredicateResult::MORE
                };
            }
        }

        let fast_left_expr = a * (dif_y + dif_x) * (dif_y - dif_x);
        let fast_right_expr = 2_f64 * b * dif_x * dif_y;

        let expr_cmp = ulp_comparison(fast_left_expr, fast_right_expr, 4);

        if expr_cmp != cmp::Ordering::Equal {
            if (expr_cmp == cmp::Ordering::Greater) ^ reverse_order {
                if reverse_order {
                    KPredicateResult::LESS
                } else {
                    KPredicateResult::MORE
                }
            } else {
                KPredicateResult::UNDEFINED
            }
        } else {
            KPredicateResult::UNDEFINED
        }
    }
}

pub struct NodeComparisonPredicate {}

impl NodeComparisonPredicate {
    /// Compares nodes in the balanced binary search tree. Nodes are
    /// compared based on the y coordinates of the arcs intersection points.
    /// Nodes with less y coordinate of the intersection point go first.
    /// Comparison is only called during the new site events processing.
    /// That's why one of the nodes will always lie on the sweepline and may
    /// be represented as a straight horizontal line.
    pub fn node_comparison_predicate<I: InputType, F: OutputType>(
        node1: &VB::BeachLineNodeKey<I, F>,
        node2: &VB::BeachLineNodeKey<I, F>,
    ) -> bool {
        // Get x coordinate of the rightmost site from both nodes.
        let site1: &VSE::SiteEvent<I, F> =
            NodeComparisonPredicate::comparison_site_::<I, F>(node1);
        let site2: &VSE::SiteEvent<I, F> =
            NodeComparisonPredicate::comparison_site_::<I, F>(node2);
        let point1: &Point<I> = NodeComparisonPredicate::comparison_point_::<I, F>(site1);
        let point2: &Point<I> = NodeComparisonPredicate::comparison_point_::<I, F>(site2);
        let rv = {
            match point1.x.cmp(&point2.x) {
                cmp::Ordering::Less => {
                    //tln!("point1.x < point2.x {}<{}", point1.x, point2.x);
                    // The second node contains a new site.
                    DistancePredicate::distance_predicate::<I, F>(
                        node1.left_site(),
                        node1.right_site(),
                        point2,
                    )
                }
                cmp::Ordering::Greater => {
                    //tln!( "point1.x > point2.x");
                    // The first node contains a new site.
                    !DistancePredicate::distance_predicate::<I, F>(
                        node2.left_site(),
                        node2.right_site(),
                        point1,
                    )
                }
                cmp::Ordering::Equal => {
                    //tln!( "point1.x == point2.x");
                    // These checks were evaluated experimentally.
                    match site1.sorted_index().cmp(&site2.sorted_index()) {
                        cmp::Ordering::Equal => {
                            //tln!( "sorted_index Equal");
                            // Both nodes are new (inserted during same site event processing).
                            let y1 = Self::comparison_y_(node1, true);
                            let y2 = Self::comparison_y_(node2, true);
                            y1 < y2
                        }
                        cmp::Ordering::Less => {
                            let y1 = Self::comparison_y_(node1, false);
                            let y2 = Self::comparison_y_(node2, true);
                            //if (y1.first != y2.first) return y1.first < y2.first;
                            //return (!site1.is_segment()) ? (y1.second < 0) : false;
                            if y1.0 != y2.0 {
                                //tln!( "sorted_index Less 1 y1:{:?} y2:{:?}", y1, y2);
                                y1.0 < y2.0
                            } else if !site1.is_segment() {
                                //tln!( "sorted_index Less 2");
                                y1.1 < 0
                            } else {
                                //tln!( "sorted_index Less 3");
                                false
                            }
                        }
                        cmp::Ordering::Greater => {
                            //tln!( "sorted_index Greater");
                            let y1 = Self::comparison_y_(node1, true);
                            let y2 = Self::comparison_y_(node2, false);
                            //if (y1.first != y2.first) return y1.first < y2.first;
                            //return (!site2.is_segment()) ? (y2.second > 0) : true;
                            if y1.0 != y2.0 {
                                y1.0 < y2.0
                            } else if !site2.is_segment() {
                                y2.1 > 0
                            } else {
                                true
                            }
                        }
                    }
                }
            }
        };
        //tln!("node_comparison_predicate(L:{:?},R:{:?}, {}:{:?}, {}:{:?})=={}", node1.left_site(), node1.right_site(), site1.sorted_index(), point1, site2.sorted_index(), point2,rv);
        rv
    }

    /// Get the newer site.
    fn comparison_site_<I: InputType, F: OutputType>(
        node: &VB::BeachLineNodeKey<I, F>,
    ) -> &VSE::SiteEvent<I, F> {
        if node.left_site().sorted_index() > node.right_site().sorted_index() {
            node.left_site()
        } else {
            node.right_site()
        }
    }

    fn comparison_point_<I: InputType, F: OutputType>(
        site: &VSE::SiteEvent<I, F>,
    ) -> &Point<I> {
        if PointComparisonPredicate::<I>::point_comparison_predicate(site.point0(), site.point1()) {
            site.point0()
        } else {
            site.point1()
        }
    }

    /// Get comparison pair: tuple of y coordinate and direction of the newer site.
    fn comparison_y_<I: InputType, F: OutputType>(
        node: &VB::BeachLineNodeKey<I, F>,
        is_new_node: bool,
    ) -> (I, i8) {
        if node.left_site().sorted_index() == node.right_site().sorted_index() {
            return (node.left_site().y0(), 0);
        }
        if node.left_site().sorted_index() > node.right_site().sorted_index() {
            if !is_new_node
                && node.left_site().is_segment()
                && Predicates::is_vertical_1::<I, F>(node.left_site())
            {
                return (node.left_site().y0(), 1);
            }
            return (node.left_site().y1(), 1);
        }
        return (node.right_site().y0(), -1);
    }
}

pub struct CircleExistencePredicate {}

impl CircleExistencePredicate {
    #[inline(always)]
    pub(crate) fn ppp<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
    ) -> bool {
        OrientationTest::eval_p::<I, F>(site1.point0(), site2.point0(), site3.point0())
            == Orientation::Right
    }

    #[cfg(all(feature = "geo", feature = "ce_corruption_check"))]
    #[inline(always)]
    pub(crate) fn validate_circle_formation_predicate<I: InputType, F: OutputType>(
        _site1: &VSE::SiteEvent<I, F>,
        _site2: &VSE::SiteEvent<I, F>,
        _site3: &VSE::SiteEvent<I, F>,
        c_event: &VC::CircleEventType,
    ) {
        // only do this if the circle event is outside the site event x range.
        println!(
            "\nvalidate CE x={} y:{} xl:{}",
            c_event.0.get().x().0,
            c_event.0.get().y().0,
            c_event.0.get().lower_x().0
        );
        #[allow(clippy::match_like_matches_macro)]
        if match (_site1.is_point(), _site2.is_point(), _site3.is_point()) {
            // only validate pps
            (false, true, true) => true,
            (true, false, true) => true,
            (true, true, false) => true,
            _ => false,
        } {
            use approx::AbsDiffEq;

            let c = geo::Coordinate {
                x: c_event.0.get().x().0 as f64,
                y: c_event.0.get().y().0 as f64,
            };
            let d1 = _site1.distance_to_point(c.x, c.y);
            let d2 = _site2.distance_to_point(c.x, c.y);
            let d3 = _site3.distance_to_point(c.x, c.y);

            if d1.abs_diff_ne(&d2, 0.001) || d1.abs_diff_ne(&d3, 0.001) {
                println!("circle_formation_predicate should return false but doesn't");
                println!("c={:?} lx:{}", c, c_event.0.get().lower_x().0);
                println!("site1:{:?} distance={:.12}", _site1, d1);
                println!("site2:{:?} distance={:.12}", _site2, d2);
                println!("site3:{:?}, distance={:.12}", _site3, d3);
                println!("there were no three point vertex!");
            }
        }
    }

    #[inline(always)]
    fn pps<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
        segment_index: SiteIndex,
    ) -> bool {
        #[allow(clippy::suspicious_operation_groupings)]
        if segment_index != SiteIndex::Two {
            let orient1 =
                OrientationTest::eval_p::<I, F>(site1.point0(), site2.point0(), site3.point0());
            let orient2 =
                OrientationTest::eval_p::<I, F>(site1.point0(), site2.point0(), site3.point1());
            if segment_index == SiteIndex::One && site1.x0() >= site2.x0() {
                if orient1 != Orientation::Right {
                    return false;
                }
            } else if segment_index == SiteIndex::Three && site2.x0() >= site1.x0() {
                if orient2 != Orientation::Right {
                    return false;
                }
            } else if orient1 != Orientation::Right && orient2 != Orientation::Right {
                return false;
            }
        } else {
            return (site3.point0() != site1.point0()) || (site3.point1() != site2.point0());
        }
        true
    }

    #[inline(always)]
    fn pss<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
        point_index: SiteIndex,
    ) -> bool {
        if site2.sorted_index() == site3.sorted_index() {
            return false;
        }
        if point_index == SiteIndex::Two {
            if !site2.is_inverse() && site3.is_inverse() {
                return false;
            }
            if site2.is_inverse() == site3.is_inverse()
                && OrientationTest::eval_p::<I, F>(site2.point0(), site1.point0(), site3.point1())
                    != Orientation::Right
            {
                return false;
            }
        }
        true
    }

    pub(crate) fn sss<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
    ) -> bool {
        (site1.sorted_index() != site2.sorted_index())
            && (site2.sorted_index() != site3.sorted_index())
    }
}

#[derive(Default)]
pub struct LazyCircleFormationFunctor {}

impl LazyCircleFormationFunctor {
    /// Lazy evaluation of point, point, point circle events
    fn ppp<I: InputType, F: OutputType>(
        point1: &Point<I>,
        point2: &Point<I>,
        point3: &Point<I>,
        c_event: &VC::CircleEventType,
    ) {
        let dif_x1 = cast::<I, f64>(point1.x) - cast::<I, f64>(point2.x);
        let dif_x2 = cast::<I, f64>(point2.x) - cast::<I, f64>(point3.x);
        let dif_y1 = cast::<I, f64>(point1.y) - cast::<I, f64>(point2.y);
        let dif_y2 = cast::<I, f64>(point2.y) - cast::<I, f64>(point3.y);
        let orientation = Predicates::robust_cross_product::<I, F>(
            cast::<I, i64>(point1.x) - cast::<I, i64>(point2.x),
            cast::<I, i64>(point2.x) - cast::<I, i64>(point3.x),
            cast::<I, i64>(point1.y) - cast::<I, i64>(point2.y),
            cast::<I, i64>(point2.y) - cast::<I, i64>(point3.y),
        );
        let inv_orientation: RF::RobustFpt = RF::RobustFpt::new_2(
            num::cast::<f32, f64>(0.5f32).unwrap() / orientation,
            num::cast::<f32, f64>(2.0f32).unwrap(),
        );
        let sum_x1: f64 = cast::<I, f64>(point1.x) + cast::<I, f64>(point2.x);
        let sum_x2: f64 = cast::<I, f64>(point2.x) + cast::<I, f64>(point3.x);
        let sum_y1: f64 = cast::<I, f64>(point1.y) + cast::<I, f64>(point2.y);
        let sum_y2: f64 = cast::<I, f64>(point2.y) + cast::<I, f64>(point3.y);
        let dif_x3: f64 = cast::<I, f64>(point1.x) - cast::<I, f64>(point3.x);
        let dif_y3: f64 = cast::<I, f64>(point1.y) - cast::<I, f64>(point3.y);
        let mut c_x = RF::RobustDif::new();
        let mut c_y = RF::RobustDif::new();
        let error = 2_f64;
        c_x += RF::RobustFpt::new_2(dif_x1 * sum_x1 * dif_y2, error);
        c_x += RF::RobustFpt::new_2(dif_y1 * sum_y1 * dif_y2, error);
        c_x -= RF::RobustFpt::new_2(dif_x2 * sum_x2 * dif_y1, error);
        c_x -= RF::RobustFpt::new_2(dif_y2 * sum_y2 * dif_y1, error);
        c_y += RF::RobustFpt::new_2(dif_x2 * sum_x2 * dif_x1, error);
        c_y += RF::RobustFpt::new_2(dif_y2 * sum_y2 * dif_x1, error);
        c_y -= RF::RobustFpt::new_2(dif_x1 * sum_x1 * dif_x2, error);
        c_y -= RF::RobustFpt::new_2(dif_y1 * sum_y1 * dif_x2, error);
        let mut lower_x = RF::RobustDif::new_from(c_x);
        lower_x -= RF::RobustFpt::new_2(
            ((dif_x1 * dif_x1 + dif_y1 * dif_y1)
                * (dif_x2 * dif_x2 + dif_y2 * dif_y2)
                * (dif_x3 * dif_x3 + dif_y3 * dif_y3))
                .sqrt(),
            num::cast::<f32, f64>(5.0f32).unwrap(),
        );

        c_event.cell_set_3_raw(
            c_x.dif().fpv() * inv_orientation.fpv(),
            c_y.dif().fpv() * inv_orientation.fpv(),
            lower_x.dif().fpv() * inv_orientation.fpv(),
        );
        let ulps = Predicates::ulps() as f64;
        let recompute_c_x = c_x.dif().ulp() > ulps;
        let recompute_c_y = c_y.dif().ulp() > ulps;
        let recompute_lower_x = lower_x.dif().ulp() > ulps;
        #[cfg(feature = "console_debug")]
        {
            assert!(!c_x.dif().ulp().is_nan());
            assert!(!c_y.dif().ulp().is_nan());
            assert!(!lower_x.dif().ulp().is_nan());
        }

        if recompute_c_x || recompute_c_y || recompute_lower_x {
            ExactCircleFormationFunctor::ppp::<I, F>(
                point1,
                point2,
                point3,
                c_event,
                recompute_c_x,
                recompute_c_y,
                recompute_lower_x,
            );
        }
    }

    /// Lazy evaluation of point, point, segment circle events
    fn pps<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
        segment_index: SiteIndex,
        c_event: &VC::CircleEventType,
    ) {
        #[cfg(feature = "ce_corruption_check")]
        println!("\n->LazyCircleFormationFunctor::pps(site1:{:?}, site2:{:?}, site3:{:?}, segment_index:{:?})", site1, site2, site3, segment_index);
        tln!("->LazyCircleFormationFunctor::pps(site1:{:?}, site2:{:?}, site3:{:?}, segment_index:{:?})", site1, site2, site3, segment_index);

        // (line_a,line_b) it the perpendicular vector of site3-point0 -> site3-point1
        let line_a = cast::<I, f64>(site3.y1()) - cast::<I, f64>(site3.y0());
        let line_b = cast::<I, f64>(site3.x0()) - cast::<I, f64>(site3.x1());
        // (vec_x,vec_y) it the perpendicular vector of site1->site2
        // t*(vec_x,vec_y) + midpoint(site1->site2) is our circle event position
        let vec_x = cast::<I, f64>(site2.y()) - cast::<I, f64>(site1.y());
        let vec_y = cast::<I, f64>(site1.x()) - cast::<I, f64>(site2.x());

        let teta = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(site3.y1()) - cast::<I, i64>(site3.y0()),
                cast::<I, i64>(site3.x0()) - cast::<I, i64>(site3.x1()),
                cast::<I, i64>(site2.x()) - cast::<I, i64>(site1.x()),
                cast::<I, i64>(site2.y()) - cast::<I, i64>(site1.y()),
            ),
            1_f64,
        );
        let a = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(site3.y0()) - cast::<I, i64>(site3.y1()),
                cast::<I, i64>(site3.x0()) - cast::<I, i64>(site3.x1()),
                cast::<I, i64>(site3.y1()) - cast::<I, i64>(site1.y()),
                cast::<I, i64>(site3.x1()) - cast::<I, i64>(site1.x()),
            ),
            1_f64,
        );
        let b = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(site3.y0()) - cast::<I, i64>(site3.y1()),
                cast::<I, i64>(site3.x0()) - cast::<I, i64>(site3.x1()),
                cast::<I, i64>(site3.y1()) - cast::<I, i64>(site2.y()),
                cast::<I, i64>(site3.x1()) - cast::<I, i64>(site2.x()),
            ),
            1_f64,
        );
        let denom = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(site1.y()) - cast::<I, i64>(site2.y()),
                cast::<I, i64>(site1.x()) - cast::<I, i64>(site2.x()),
                cast::<I, i64>(site3.y1()) - cast::<I, i64>(site3.y0()),
                cast::<I, i64>(site3.x1()) - cast::<I, i64>(site3.x0()),
            ),
            1_f64,
        );
        let inv_segm_len =
            RF::RobustFpt::new_2(1_f64 / (line_a * line_a + line_b * line_b).sqrt(), 3_f64);
        let mut t = RF::RobustDif::default();
        tln!("0t:{:?}", t);
        if OrientationTest::eval_f::<I, F>(denom.fpv()) == Orientation::Collinear {
            t += teta / (RF::RobustFpt::new_1(8_f64) * a);
            tln!("1t:{:?}", t);
            t -= a / (RF::RobustFpt::new_1(2_f64) * teta);
            tln!("2t:{:?}", t);
        } else {
            let det = ((teta * teta + denom * denom) * a * b).sqrt();
            //tln!("det:{:?}", det);
            if segment_index == SiteIndex::Two {
                tln!("3 det:{:?}", det);
                tln!("3 denom:{:?}", denom);
                tln!("3 det/denom:{:?}", det / (denom * denom));
                t -= det / (denom * denom);
                tln!("3t:{:?}", t);
            } else {
                t += det / (denom * denom);
                tln!("4t:{:?}", t);
            }
            tln!("5teta:{:?}", teta);
            tln!("A:{:?}", a);
            tln!("B:{:?}", b);
            t += teta * (a + b) / (RF::RobustFpt::new_1(2_f64) * denom * denom);
            tln!("5t:{:?}", t);
        }
        tln!("6t:{:?}", t);
        let mut c_x = RF::RobustDif::default();
        tln!("0: c_x:{:?}", c_x);
        let mut c_y = RF::RobustDif::default();
        c_x += RF::RobustFpt::new_1(0.5 * (cast::<I, f64>(site1.x()) + cast::<I, f64>(site2.x())));
        tln!("1: c_x:{:?}", c_x);
        c_x += t * RF::RobustFpt::new_1(vec_x);
        tln!("2: c_x:{:?}", c_x);
        c_y += RF::RobustFpt::new_1(0.5 * (cast::<I, f64>(site1.y()) + cast::<I, f64>(site2.y())));
        c_y += t * RF::RobustFpt::new_1(vec_y);

        let mut r = RF::RobustDif::default();
        let mut lower_x = RF::RobustDif::new_from(c_x);
        r -= RF::RobustFpt::new_1(line_a) * RF::RobustFpt::new_1(cast::<I, f64>(site3.x0()));
        r -= RF::RobustFpt::new_1(line_b) * RF::RobustFpt::new_1(cast::<I, f64>(site3.y0()));
        r += c_x * RF::RobustFpt::new_1(line_a);
        r += c_y * RF::RobustFpt::new_1(line_b);
        if r.positive().fpv() < r.negative().fpv() {
            r = -r;
        }
        lower_x += r * inv_segm_len;

        #[cfg(feature = "ce_corruption_check")]
        {
            println!("let site1=[{},{}];", site1.x(), site1.y());
            println!("let site2=[{},{}];", site2.x(), site2.y());
            println!(
                "let site3=[{},{},{},{}];",
                site3.point0().x,
                site3.point0().y,
                site3.point1().x,
                site3.point1().y
            );
            println!(
                "let c1=[{:.12},{:.12}];//lx={:.12}",
                c_x.dif().fpv(),
                c_y.dif().fpv(),
                lower_x.dif().fpv()
            );
        }
        c_event.cell_set_3_raw(c_x.dif().fpv(), c_y.dif().fpv(), lower_x.dif().fpv());

        tln!("  c_x:{:?}, c_y:{:?}, l_x:{:?}", c_x, c_y, lower_x);

        let ulps = Predicates::ulps() as f64;
        let recompute_c_x = c_x.dif().ulp() > ulps;
        let recompute_c_y = c_y.dif().ulp() > ulps;
        let recompute_lower_x = lower_x.dif().ulp() > ulps;
        tln!(
            "  recompute_c_x:{}, recompute_c_y:{}, recompute_lower_x:{}",
            recompute_c_x,
            recompute_c_y,
            recompute_lower_x
        );

        #[cfg(feature = "console_debug")]
        {
            assert!(!c_x.dif().ulp().is_nan());
            assert!(!c_y.dif().ulp().is_nan());
            assert!(!lower_x.dif().ulp().is_nan());
        }

        if recompute_c_x || recompute_c_y || recompute_lower_x {
            ExactCircleFormationFunctor::pps::<I, F>(
                site1,
                site2,
                site3,
                segment_index,
                c_event,
                recompute_c_x,
                recompute_c_y,
                recompute_lower_x,
            );
        }
        #[allow(clippy::suspicious_operation_groupings)]
        // All sites needs to be unique, or ppp will return NaN
        let unique_endpoints = !(site3.point0() == site1.point0()
            || site3.point0() == site2.point0()
            || site3.point1() == site1.point0()
            || site3.point1() == site2.point0()
            || site1.point0() == site2.point0());
        tln!("pps unique_endpoints:{}", unique_endpoints);
        if unique_endpoints {
            #[cfg(feature = "ce_corruption_check")]
            {
                println!(
                    "site1->c distance:{:-12}",
                    site1.distance_to_point(c_event.0.get().x().0, c_event.0.get().y().0)
                );
                println!(
                    "site2->c distance:{:-12}",
                    site2.distance_to_point(c_event.0.get().x().0, c_event.0.get().y().0)
                );
                println!(
                    "site3->c distance:{:-12}",
                    site3.distance_to_point(c_event.0.get().x().0, c_event.0.get().y().0)
                );
            }

            // site3.point0 -> c
            let v_3_c = (
                c_event.0.get().x().0 - cast::<I, f64>(site3.point0().x),
                c_event.0.get().y().0 - cast::<I, f64>(site3.point0().y),
            );
            // site3.point0 -> site3.point1
            let v_3 = (
                cast::<I, f64>(site3.point1().x) - cast::<I, f64>(site3.point0().x),
                cast::<I, f64>(site3.point1().y) - cast::<I, f64>(site3.point0().y),
            );
            #[allow(clippy::suspicious_operation_groupings)]
            let dot = (v_3_c.0 * v_3.0 + v_3_c.1 * v_3.1) / (v_3.0 * v_3.0 + v_3.1 * v_3.1);
            tln!("pps dot:{:.12}", dot);
            #[cfg(feature = "ce_corruption_check")]
            {
                println!("v_a_c:{:?}, v3:{:?}", v_3_c, v_3);
                println!("dot:{:?}", dot);
            }

            if !(-0.0..=1.0).contains(&dot) {
                // The (current) circle event is located outside the length of the segment
                // - re-calculate with ppp
                let site_3_point = if dot < -0.0 {
                    site3.point0()
                } else {
                    site3.point1()
                };

                #[cfg(feature = "ce_corruption_check")]
                {
                    println!(
                        "dot_n:{:?} was bad ---------------- calling ppp on point0",
                        dot
                    );
                    println!(
                        "point0 distance {}",
                        site3.point0().distance_to(&c_event.0.get())
                    );
                    println!(
                        "point1 distance {}",
                        site3.point1().distance_to(&c_event.0.get())
                    );
                }
                match segment_index {
                    SiteIndex::One => {
                        LazyCircleFormationFunctor::ppp::<I, F>(
                            site_3_point,
                            site1.point0(),
                            site2.point0(),
                            c_event,
                        );
                    }
                    SiteIndex::Two => {
                        LazyCircleFormationFunctor::ppp::<I, F>(
                            site1.point0(),
                            site_3_point,
                            site2.point0(),
                            c_event,
                        );
                    }
                    SiteIndex::Three => {
                        LazyCircleFormationFunctor::ppp::<I, F>(
                            site1.point0(),
                            site2.point0(),
                            site_3_point,
                            c_event,
                        );
                    }
                };
                #[cfg(feature = "ce_corruption_check")]
                {
                    println!("//c after ppp");
                    println!(
                        "let c2=[{:.12},{:.12}];//l_x={:12}",
                        c_event.0.get().x().0,
                        c_event.0.get().y().0,
                        c_event.0.get().lower_x().0
                    );
                    println!(
                        "site1->c distance:{:-12}",
                        site1.distance_to_point(c_event.0.get().x().0, c_event.0.get().y().0)
                    );
                    println!(
                        "site2->c distance:{:-12}",
                        site2.distance_to_point(c_event.0.get().x().0, c_event.0.get().y().0)
                    );
                    println!(
                        "site3->c distance:{:-12}",
                        site3.distance_to_point(c_event.0.get().x().0, c_event.0.get().y().0)
                    );
                    assert!(c_event.0.get().x().is_finite());
                    assert!(c_event.0.get().y().is_finite());
                }
            }
        };
    }

    /// Lazy evaluation of point, segment, segment circle events
    #[allow(unused_parens)]
    fn pss<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
        point_index: SiteIndex,
        c_event: &VC::CircleEventType,
    ) {
        let segm_start1 = site2.point1();
        let segm_end1 = site2.point0();
        let segm_start2 = site3.point0();
        let segm_end2 = site3.point1();
        tln!(
            "->LazyCircleFormationFunctor::pss(site1:{:?}, site2:{:?}, site3:{:?}, point_index:{:?})",
            site1,
            site2,
            site3,
            point_index
        );

        // This is a case that does not exists in C++ boost voronoi.
        // If site1 is a point shared by both site2 and site3 there can only be one CE solution.
        // The CE must be the site1 point with zero radius.
        // It seems better to use the pristine int coordinate instead of spending cycles
        // re-calculating it again with lossy floats.
        #[allow(clippy::suspicious_operation_groupings)]
        if (site1.point0() == site2.point0() || site1.point0() == site2.point1())
            && (site1.point0() == site3.point0() || site1.point0() == site3.point1())
        {
            c_event.cell_set_is_site_point();
            let x = cast::<I, f64>(site1.point0().x);
            let y = cast::<I, f64>(site1.point0().y);
            c_event.cell_set_3_raw(x, y, x);
            tln!("<-LazyCircleFormationFunctor::pss shortcut");
            return;
        }

        let a1 = cast::<I, f64>(segm_end1.x) - cast::<I, f64>(segm_start1.x);
        let b1 = cast::<I, f64>(segm_end1.y) - cast::<I, f64>(segm_start1.y);
        let a2 = cast::<I, f64>(segm_end2.x) - cast::<I, f64>(segm_start2.x);
        let b2 = cast::<I, f64>(segm_end2.y) - cast::<I, f64>(segm_start2.y);
        let recompute_c_x: bool;
        let recompute_c_y: bool;
        let recompute_lower_x: bool;

        let orientation = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                cast::<I, i64>(segm_end2.y) - cast::<I, i64>(segm_start2.y),
                cast::<I, i64>(segm_end2.x) - cast::<I, i64>(segm_start2.x),
            ),
            1_f64,
        );
        let is_collinear =
            OrientationTest::eval_f::<I, F>(orientation.fpv()) == Orientation::Collinear;
        if is_collinear {
            tln!("  LazyCircleFormationFunctor::pss collinear");
            let a = RF::RobustFpt::new_2(a1 * a1 + b1 * b1, 2_f64);
            let c = RF::RobustFpt::new_2(
                Predicates::robust_cross_product::<I, F>(
                    cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                    cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                    cast::<I, i64>(segm_start2.y) - cast::<I, i64>(segm_start1.y),
                    cast::<I, i64>(segm_start2.x) - cast::<I, i64>(segm_start1.x),
                ),
                1_f64,
            );
            let det = RF::RobustFpt::new_2(
                Predicates::robust_cross_product::<I, F>(
                    cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                    cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                    cast::<I, i64>(site1.x()) - cast::<I, i64>(segm_start1.x),
                    cast::<I, i64>(site1.y()) - cast::<I, i64>(segm_start1.y),
                ) * Predicates::robust_cross_product::<I, F>(
                    cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                    cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                    cast::<I, i64>(site1.y()) - cast::<I, i64>(segm_start2.y),
                    cast::<I, i64>(site1.x()) - cast::<I, i64>(segm_start2.x),
                ),
                3.0,
            );
            #[cfg(feature = "console_debug")]
            {
                if det.fpv() < 0.0 {
                    println!("det was negative!  {:?}", det);
                }
                assert!(det.fpv() >= 0.0);
                assert!(det.fpv().is_finite());
                assert!(det.sqrt().fpv().is_finite());
            }

            let mut t = RF::RobustDif::default();
            t -= RF::RobustFpt::new_1(a1)
                * RF::RobustFpt::new_1(
                    (cast::<I, f64>(segm_start1.x) + cast::<I, f64>(segm_start2.x)) * 0.5
                        - cast::<I, f64>(site1.x()),
                );
            t -= RF::RobustFpt::new_1(b1)
                * RF::RobustFpt::new_1(
                    (cast::<I, f64>(segm_start1.y) + cast::<I, f64>(segm_start2.y)) * 0.5
                        - cast::<I, f64>(site1.y()),
                );
            if point_index == SiteIndex::Two {
                t += det.sqrt();
            } else {
                t -= det.sqrt();
            }
            t /= a;
            let mut c_x = RF::RobustDif::default();
            let mut c_y = RF::RobustDif::default();
            //tln!("ulps0: x:{:.12}, y:{:.12}", c_x.dif().fpv(), c_y.dif().fpv());
            c_x += RF::RobustFpt::new_1(
                0.5 * (cast::<I, f64>(segm_start1.x) + cast::<I, f64>(segm_start2.x)),
            );
            //tln!("ulps1: x:{:.12}, y:{:.12}", c_x.dif().fpv(), c_y.dif().fpv());
            //tln!("ulps1.5: 1:{:.12}, 2:{:.12}", RF::RobustFpt::new_1(a1).fpv(), t.dif().fpv());
            //tln!("ulps1.6: 1:{:.12}", (t*RF::RobustFpt::new_1(a1)).dif().fpv());
            c_x += t * RF::RobustFpt::new_1(a1);
            c_y += RF::RobustFpt::new_1(
                0.5 * (cast::<I, f64>(segm_start1.y) + cast::<I, f64>(segm_start2.y)),
            );
            //tln!("ulps2: x:{:.12}, y:{:.12}", c_x.dif().fpv(), c_y.dif().fpv());
            c_y += t * RF::RobustFpt::new_1(b1);
            //tln!("ulps3: x:{:.12}, y:{:.12}", c_x.dif().fpv(), c_y.dif().fpv());
            let mut lower_x = RF::RobustDif::new_from(c_x);
            if c.is_neg() {
                lower_x -= RF::RobustFpt::new_1(0.5) * c / a.sqrt();
            } else {
                lower_x += RF::RobustFpt::new_1(0.5) * c / a.sqrt();
            }
            let ulps = Predicates::ulps() as f64;
            recompute_c_x = c_x.dif().ulp() > ulps;
            recompute_c_y = c_y.dif().ulp() > ulps;
            recompute_lower_x = lower_x.dif().ulp() > ulps;
            #[cfg(feature = "console_debug")]
            {
                tln!(
                    "ulps:{}, x:{:.12}, y:{:.12}, lx:{:.12}",
                    ulps,
                    c_x.dif().ulp(),
                    c_y.dif().ulp(),
                    lower_x.dif().ulp()
                );
                tln!(
                    "x:{:.12}, y:{:.12}, lx:{:.12}",
                    c_x.dif().fpv(),
                    c_y.dif().fpv(),
                    lower_x.dif().fpv()
                );
                assert!(!c_x.dif().ulp().is_nan());
                assert!(!c_y.dif().ulp().is_nan());
                assert!(!lower_x.dif().ulp().is_nan());
            }
            c_event.cell_set_3_raw(c_x.dif().fpv(), c_y.dif().fpv(), lower_x.dif().fpv());
        } else {
            tln!("  LazyCircleFormationFunctor::pss !collinear");
            let sqr_sum1 = RF::RobustFpt::new_2((a1 * a1 + b1 * b1).sqrt(), 2_f64);
            let sqr_sum2 = RF::RobustFpt::new_2((a2 * a2 + b2 * b2).sqrt(), 2_f64);
            let mut a = RF::RobustFpt::new_2(
                Predicates::robust_cross_product::<I, F>(
                    cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                    cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                    cast::<I, i64>(segm_start2.y) - cast::<I, i64>(segm_end2.y),
                    cast::<I, i64>(segm_end2.x) - cast::<I, i64>(segm_start2.x),
                ),
                1_f64,
            );
            tln!("0: a:{:?}", a);
            if !a.is_neg() {
                a += sqr_sum1 * sqr_sum2;
                tln!("1: a:{:?}", a);
            } else {
                a = (orientation * orientation) / (sqr_sum1 * sqr_sum2 - a);
                tln!("2: a:{:?}", a);
            }
            let or1 = RF::RobustFpt::new_2(
                Predicates::robust_cross_product::<I, F>(
                    cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                    cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                    cast::<I, i64>(segm_end1.y) - cast::<I, i64>(site1.y()),
                    cast::<I, i64>(segm_end1.x) - cast::<I, i64>(site1.x()),
                ),
                1_f64,
            );
            let or2 = RF::RobustFpt::new_2(
                Predicates::robust_cross_product::<I, F>(
                    cast::<I, i64>(segm_end2.x) - cast::<I, i64>(segm_start2.x),
                    cast::<I, i64>(segm_end2.y) - cast::<I, i64>(segm_start2.y),
                    cast::<I, i64>(segm_end2.x) - cast::<I, i64>(site1.x()),
                    cast::<I, i64>(segm_end2.y) - cast::<I, i64>(site1.y()),
                ),
                1_f64,
            );
            let det = RF::RobustFpt::new_1(2_f64) * a * or1 * or2;
            let c1 = RF::RobustFpt::new_2(
                Predicates::robust_cross_product::<I, F>(
                    cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                    cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                    cast::<I, i64>(segm_end1.y),
                    cast::<I, i64>(segm_end1.x),
                ),
                1_f64,
            );
            let c2 = RF::RobustFpt::new_2(
                Predicates::robust_cross_product::<I, F>(
                    cast::<I, i64>(segm_end2.x) - cast::<I, i64>(segm_start2.x),
                    cast::<I, i64>(segm_end2.y) - cast::<I, i64>(segm_start2.y),
                    cast::<I, i64>(segm_end2.x),
                    cast::<I, i64>(segm_end2.y),
                ),
                1_f64,
            );
            let inv_orientation = RF::RobustFpt::new_1(1_f64) / orientation;
            let mut t = RF::RobustDif::default();
            tln!("0: t:{:?}", t);
            let mut b = RF::RobustDif::default();
            tln!("0: b:{:?}", b);
            let mut ix = RF::RobustDif::default();
            let mut iy = RF::RobustDif::default();

            ix += RF::RobustFpt::new_1(a2) * c1 * inv_orientation;
            ix += RF::RobustFpt::new_1(a1) * c2 * inv_orientation;
            iy += RF::RobustFpt::new_1(b1) * c2 * inv_orientation;
            iy += RF::RobustFpt::new_1(b2) * c1 * inv_orientation;
            tln!("1: ix:{:?}", ix);
            tln!("1: s:{:?}", RF::RobustFpt::new_1(a1) * sqr_sum2);
            tln!("1: p:{:?}", ix * (RF::RobustFpt::new_1(a1) * sqr_sum2));
            b += ix * (RF::RobustFpt::new_1(a1) * sqr_sum2);
            tln!("1: b:{:?}", b);
            b += ix * (RF::RobustFpt::new_1(a2) * sqr_sum1);
            tln!("2: b:{:?}", b);
            b += iy * (RF::RobustFpt::new_1(b1) * sqr_sum2);
            tln!("3: b:{:?}", b);
            b += iy * (RF::RobustFpt::new_1(b2) * sqr_sum1);
            tln!("4: b:{:?}", b);
            b -= sqr_sum1
                * RF::RobustFpt::new_2(
                    Predicates::robust_cross_product::<I, F>(
                        cast::<I, i64>(segm_end2.x) - cast::<I, i64>(segm_start2.x),
                        cast::<I, i64>(segm_end2.y) - cast::<I, i64>(segm_start2.y),
                        -cast::<I, i64>(site1.y()),
                        cast::<I, i64>(site1.x()),
                    ),
                    1_f64,
                );
            tln!("5: b:{:?}", b);
            b -= sqr_sum2
                * RF::RobustFpt::new_2(
                    Predicates::robust_cross_product::<I, F>(
                        cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                        cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                        -cast::<I, i64>(site1.y()),
                        cast::<I, i64>(site1.x()),
                    ),
                    1_f64,
                );
            tln!("6: b:{:?}", b);
            tln!("  LazyCircleFormationFunctor::pss a:{:?} b:{:?}", a, b);
            tln!("1: b:{:?}", b);
            t -= b;
            tln!("1: t:{:?}", t);
            if point_index == SiteIndex::Two {
                t += det.sqrt();
                tln!("2: t:{:?}", t);
            } else {
                t -= det.sqrt();
                tln!("3: t:{:?}", t);
            }

            t /= (a * a);
            tln!("4: t:{:?}", t);
            tln!(
                "  LazyCircleFormationFunctor::pss t:{:.12} det:{:.12}",
                t.dif().fpv(),
                det.fpv()
            );
            let mut c_x = RF::RobustDif::new_from(ix);
            let mut c_y = RF::RobustDif::new_from(iy);
            tln!("0: c_x:{:?}", c_x);
            tln!("0: t:{:?}", t);
            c_x += t * (RF::RobustFpt::new_1(a1) * sqr_sum2);
            tln!("1: c_x:{:?}", c_x);
            c_x += t * (RF::RobustFpt::new_1(a2) * sqr_sum1);
            tln!("2: c_x:{:?}", c_x);
            c_y += t * (RF::RobustFpt::new_1(b1) * sqr_sum2);
            c_y += t * (RF::RobustFpt::new_1(b2) * sqr_sum1);

            if t.positive().fpv() < t.negative().fpv() {
                t = -t;
            }
            let mut lower_x = RF::RobustDif::new_from(c_x);
            if orientation.is_neg() {
                lower_x -= t * orientation;
            } else {
                lower_x += t * orientation;
            }
            tln!(
                "  LazyCircleFormationFunctor::pss c_x:{:?} c_y:{:?} l_x:{:?}",
                c_x,
                c_y,
                lower_x
            );
            /*
            println!(
                "  LazyCircleFormationFunctor::pss c_x:{:?} c_y:{:?} l_x:{:?}",
                c_x.dif().ulp(),
                c_y.dif().ulp(),
                lower_x.dif().ulp()
            );*/

            let ulps = Predicates::ulps() as f64;
            recompute_c_x = c_x.dif().ulp() > ulps;
            recompute_c_y = c_y.dif().ulp() > ulps;
            recompute_lower_x = lower_x.dif().ulp() > ulps;
            #[cfg(feature = "console_debug")]
            {
                assert!(!c_x.dif().ulp().is_nan());
                assert!(!c_y.dif().ulp().is_nan());
                assert!(!lower_x.dif().ulp().is_nan());
            }
            // Todo! Is this correct? it was let c_event = ...
            c_event.cell_set_3_raw(c_x.dif().fpv(), c_y.dif().fpv(), lower_x.dif().fpv());
        }

        if recompute_c_x || recompute_c_y || recompute_lower_x {
            ExactCircleFormationFunctor::pss(
                site1,
                site2,
                site3,
                point_index,
                c_event,
                recompute_c_x,
                recompute_c_y,
                recompute_lower_x,
            );
        }
    }

    /// Lazy evaluation of segment, segment, segment circle events
    fn sss<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
        c_event: &VC::CircleEventType,
    ) {
        let a1 = RF::RobustFpt::new_1(cast::<I, f64>(site1.x1()) - cast::<I, f64>(site1.x0()));
        let b1 = RF::RobustFpt::new_1(cast::<I, f64>(site1.y1()) - cast::<I, f64>(site1.y0()));
        let c1 = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(site1.x0()),
                cast::<I, i64>(site1.y0()),
                cast::<I, i64>(site1.x1()),
                cast::<I, i64>(site1.y1()),
            ),
            1_f64,
        );

        let a2 = RF::RobustFpt::new_1(cast::<I, f64>(site2.x1()) - cast::<I, f64>(site2.x0()));
        let b2 = RF::RobustFpt::new_1(cast::<I, f64>(site2.y1()) - cast::<I, f64>(site2.y0()));
        let c2 = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(site2.x0()),
                cast::<I, i64>(site2.y0()),
                cast::<I, i64>(site2.x1()),
                cast::<I, i64>(site2.y1()),
            ),
            1_f64,
        );

        let a3 = RF::RobustFpt::new_1(cast::<I, f64>(site3.x1()) - cast::<I, f64>(site3.x0()));
        let b3 = RF::RobustFpt::new_1(cast::<I, f64>(site3.y1()) - cast::<I, f64>(site3.y0()));
        let c3 = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(site3.x0()),
                cast::<I, i64>(site3.y0()),
                cast::<I, i64>(site3.x1()),
                cast::<I, i64>(site3.y1()),
            ),
            1_f64,
        );

        let len1 = (a1 * a1 + b1 * b1).sqrt();
        let len2 = (a2 * a2 + b2 * b2).sqrt();
        let len3 = (a3 * a3 + b3 * b3).sqrt();
        let cross_12 = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(site1.x1()) - cast::<I, i64>(site1.x0()),
                cast::<I, i64>(site1.y1()) - cast::<I, i64>(site1.y0()),
                cast::<I, i64>(site2.x1()) - cast::<I, i64>(site2.x0()),
                cast::<I, i64>(site2.y1()) - cast::<I, i64>(site2.y0()),
            ),
            1_f64,
        );
        let cross_23 = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(site2.x1()) - cast::<I, i64>(site2.x0()),
                cast::<I, i64>(site2.y1()) - cast::<I, i64>(site2.y0()),
                cast::<I, i64>(site3.x1()) - cast::<I, i64>(site3.x0()),
                cast::<I, i64>(site3.y1()) - cast::<I, i64>(site3.y0()),
            ),
            1_f64,
        );
        let cross_31 = RF::RobustFpt::new_2(
            Predicates::robust_cross_product::<I, F>(
                cast::<I, i64>(site3.x1()) - cast::<I, i64>(site3.x0()),
                cast::<I, i64>(site3.y1()) - cast::<I, i64>(site3.y0()),
                cast::<I, i64>(site1.x1()) - cast::<I, i64>(site1.x0()),
                cast::<I, i64>(site1.y1()) - cast::<I, i64>(site1.y0()),
            ),
            1_f64,
        );

        // denom = cross_12 * len3 + cross_23 * len1 + cross_31 * len2.
        let mut denom = RF::RobustDif::new();
        denom += cross_12 * len3;
        denom += cross_23 * len1;
        denom += cross_31 * len2;

        // denom * r = (b2 * c_x - a2 * c_y - c2 * denom) / len2.
        let mut r = RF::RobustDif::new();
        r -= cross_12 * c3;
        r -= cross_23 * c1;
        r -= cross_31 * c2;

        let mut c_x = RF::RobustDif::new();
        c_x += a1 * c2 * len3;
        c_x -= a2 * c1 * len3;
        c_x += a2 * c3 * len1;
        c_x -= a3 * c2 * len1;
        c_x += a3 * c1 * len2;
        c_x -= a1 * c3 * len2;

        let mut c_y = RF::RobustDif::new();
        c_y += b1 * c2 * len3;
        c_y -= b2 * c1 * len3;
        c_y += b2 * c3 * len1;
        c_y -= b3 * c2 * len1;
        c_y += b3 * c1 * len2;
        c_y -= b1 * c3 * len2;

        let lower_x = c_x + r;

        let denom_dif = denom.dif();
        //tln!("  denom_dif:{:?}", denom_dif);
        let c_x_dif = c_x.dif() / denom_dif;
        let c_y_dif = c_y.dif() / denom_dif;
        let lower_x_dif = lower_x.dif() / denom_dif;

        let ulps = Predicates::ulps() as f64;
        let recompute_c_x = c_x_dif.ulp() > ulps;
        let recompute_c_y = c_y_dif.ulp() > ulps;
        let recompute_lower_x = lower_x_dif.ulp() > ulps;

        t!(" c_x_dif.ulp():{:.12}", c_x_dif.ulp());
        t!("  c_y_dif.ulp() :{:.12}", c_y_dif.ulp());
        tln!(" lower_x_dif.ulp():{:.12}", lower_x_dif.ulp());

        #[cfg(feature = "console_debug")]
        {
            assert!(!denom_dif.ulp().is_nan());
            assert!(!c_x.dif().ulp().is_nan());
            assert!(!c_y.dif().ulp().is_nan());
            assert!(!lower_x.dif().ulp().is_nan());
        }
        c_event.cell_set_3_raw(c_x_dif.fpv(), c_y_dif.fpv(), lower_x_dif.fpv());

        if recompute_c_x || recompute_c_y || recompute_lower_x {
            ExactCircleFormationFunctor::sss(
                site1,
                site2,
                site3,
                c_event,
                recompute_c_x,
                recompute_c_y,
                recompute_lower_x,
            );
        }

        tln!("<-LazyCircleFormationFunctor::sss(");
        tln!("  site1:{:?}", site1);
        tln!("  site2:{:?}", site2);
        tln!("  site3:{:?}", site3);
        tln!("  c_event:CE{:?}", c_event.0.get());
    }
}

#[derive(Default)]
pub struct CircleFormationFunctor {}

impl CircleFormationFunctor {
    pub(crate) fn lies_outside_vertical_segment<I: InputType, F: OutputType>(
        c: &VC::CircleEventType,
        s: &VSE::SiteEvent<I, F>,
    ) -> bool {
        if !s.is_segment() || !Predicates::is_vertical_1::<I, F>(s) {
            return false;
        }
        let y0 = cast::<I, f64>(if s.is_inverse() { s.y1() } else { s.y0() });
        let y1 = cast::<I, f64>(if s.is_inverse() { s.y0() } else { s.y1() });
        let cc_y = c.0.get().y().into_inner();

        ulp_comparison(cc_y, y0, 64) == cmp::Ordering::Less
            || ulp_comparison(cc_y, y1, 64) == cmp::Ordering::Greater
    }

    #[cfg(feature = "console_debug")]
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn circle_formation_predicate_fake<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
        circle: &VC::CircleEventType,
    ) -> bool {
        tln!(
            "circle_formation_predicate(site1:{:?}, site2:{:?}, site3:{:?}, circle:{:?})",
            site1,
            site2,
            site3,
            circle
        );
        tln!(
            "                           1:{}, 2:{}, 3:{}",
            site1.is_segment(),
            site2.is_segment(),
            site3.is_segment()
        );

        Self::circle_formation_predicate(site1, site2, site3, circle)
    }

    /// Create a circle event from the given three sites.
    /// Returns true if the circle event exists, else false.
    /// If exists circle event is saved into the c_event variable.
    pub(crate) fn circle_formation_predicate<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
        circle: &VC::CircleEventType,
    ) -> bool {
        if !site1.is_segment() {
            if !site2.is_segment() {
                if !site3.is_segment() {
                    // (point, point, point) sites.
                    if !CircleExistencePredicate::ppp::<I, F>(site1, site2, site3) {
                        return false;
                    }
                    LazyCircleFormationFunctor::ppp::<I, F>(
                        site1.point0(),
                        site2.point0(),
                        site3.point0(),
                        circle,
                    );
                } else {
                    // (point, point, segment) sites.
                    if !CircleExistencePredicate::pps::<I, F>(site1, site2, site3, SiteIndex::Three)
                    {
                        return false;
                    }
                    LazyCircleFormationFunctor::pps::<I, F>(
                        site1,
                        site2,
                        site3,
                        SiteIndex::Three,
                        circle,
                    )
                }
            } else if !site3.is_segment() {
                // (point, segment, point) sites.
                if !CircleExistencePredicate::pps::<I, F>(site1, site3, site2, SiteIndex::Two) {
                    return false;
                }
                LazyCircleFormationFunctor::pps::<I, F>(
                    site1,
                    site3,
                    site2,
                    SiteIndex::Two,
                    circle,
                );
            } else {
                // (point, segment, segment) sites.
                if !CircleExistencePredicate::pss::<I, F>(site1, site2, site3, SiteIndex::One) {
                    return false;
                }
                LazyCircleFormationFunctor::pss::<I, F>(
                    site1,
                    site2,
                    site3,
                    SiteIndex::One,
                    circle,
                );
            }
        } else if !site2.is_segment() {
            if !site3.is_segment() {
                // (segment, point, point) sites.
                if !CircleExistencePredicate::pps::<I, F>(site2, site3, site1, SiteIndex::One) {
                    return false;
                }
                LazyCircleFormationFunctor::pps::<I, F>(
                    site2,
                    site3,
                    site1,
                    SiteIndex::One,
                    circle,
                );
            } else {
                // (segment, point, segment) sites.
                if !CircleExistencePredicate::pss::<I, F>(site2, site1, site3, SiteIndex::Two) {
                    return false;
                }
                LazyCircleFormationFunctor::pss::<I, F>(
                    site2,
                    site1,
                    site3,
                    SiteIndex::Two,
                    circle,
                );
            }
        } else if !site3.is_segment() {
            // (segment, segment, point) sites.
            if !CircleExistencePredicate::pss::<I, F>(site3, site1, site2, SiteIndex::Three) {
                return false;
            }
            LazyCircleFormationFunctor::pss::<I, F>(site3, site1, site2, SiteIndex::Three, circle);
        } else {
            // (segment, segment, segment) sites.
            if !CircleExistencePredicate::sss::<I, F>(site1, site2, site3) {
                return false;
            }
            LazyCircleFormationFunctor::sss::<I, F>(site1, site2, site3, circle);
        }

        if Self::lies_outside_vertical_segment(circle, site1)
            || Self::lies_outside_vertical_segment(circle, site2)
            || Self::lies_outside_vertical_segment(circle, site3)
        {
            return false;
        }
        #[cfg(all(feature = "geo", feature = "ce_corruption_check"))]
        CircleExistencePredicate::validate_circle_formation_predicate::<I, F>(
            site1, site2, site3, circle,
        );
        true
    }
}

#[derive(Default)]
pub struct ExactCircleFormationFunctor {}

impl ExactCircleFormationFunctor {
    /// Recompute parameters of the point, point, point circle event using high-precision library.
    fn ppp<I: InputType, F: OutputType>(
        point1: &Point<I>,
        point2: &Point<I>,
        point3: &Point<I>,
        circle: &VC::CircleEventType,
        recompute_c_x: bool,
        recompute_c_y: bool,
        recompute_lower_x: bool,
    ) {
        let dif_x = [
            super::cast_i_to_xi(point1.x) - super::cast_i_to_xi(point2.x),
            super::cast_i_to_xi(point2.x) - super::cast_i_to_xi(point3.x),
            super::cast_i_to_xi(point1.x) - super::cast_i_to_xi(point3.x),
        ];

        let dif_y = [
            super::cast_i_to_xi(point1.y) - super::cast_i_to_xi(point2.y),
            super::cast_i_to_xi(point2.y) - super::cast_i_to_xi(point3.y),
            super::cast_i_to_xi(point1.y) - super::cast_i_to_xi(point3.y),
        ];

        let sum_x = [
            super::cast_i_to_xi(point1.x) + super::cast_i_to_xi(point2.x),
            super::cast_i_to_xi(point2.x) + super::cast_i_to_xi(point3.x),
        ];
        let sum_y = [
            super::cast_i_to_xi(point1.y) + super::cast_i_to_xi(point2.y),
            super::cast_i_to_xi(point2.y) + super::cast_i_to_xi(point3.y),
        ];

        let inv_denom = {
            let tmp = &dif_x[0] * &dif_y[1] - &dif_x[1] * &dif_y[0];
            EX::ExtendedExponentFpt::<f64>::from(0.5) / EX::ExtendedExponentFpt::from(tmp)
        };
        let numer1: EI::ExtendedInt = &dif_x[0] * &sum_x[0] + &dif_y[0] * &sum_y[0];
        let numer2: EI::ExtendedInt = &dif_x[1] * &sum_x[1] + &dif_y[1] * &sum_y[1];

        if recompute_c_x || recompute_lower_x {
            let c_x: EI::ExtendedInt = &numer1 * &dif_y[1] - &numer2 * &dif_y[0];
            circle.cell_set_x_xf(EX::ExtendedExponentFpt::from(&c_x) * inv_denom);

            if recompute_lower_x {
                // Evaluate radius of the circle.
                let sqr_r: EI::ExtendedInt = (&dif_x[0] * &dif_x[0] + &dif_y[0] * &dif_y[0])
                    * (&dif_x[1] * &dif_x[1] + &dif_y[1] * &dif_y[1])
                    * (&dif_x[2] * &dif_x[2] + &dif_y[2] * &dif_y[2]);
                let r = EX::ExtendedExponentFpt::from(&sqr_r).sqrt();

                // If c_x >= 0 then lower_x = c_x + r,
                // else lower_x = (c_x * c_x - r * r) / (c_x - r).
                // To guarantee epsilon relative error.

                // this value will be invalid after call to set_lower_x()
                let tmp_circle_x = circle.0.get().x_as_xf();

                if !tmp_circle_x.is_neg() {
                    if !inv_denom.is_neg() {
                        circle.cell_set_lower_x_xf(tmp_circle_x + r * inv_denom);
                    } else {
                        circle.cell_set_lower_x_xf(tmp_circle_x - r * inv_denom);
                    }
                } else {
                    let numer: EI::ExtendedInt = &c_x * &c_x - &sqr_r;
                    let lower_x = EX::ExtendedExponentFpt::from(numer) * inv_denom
                        / (EX::ExtendedExponentFpt::from(c_x) + r);
                    circle.cell_set_lower_x_xf(lower_x);
                }
            }
        }

        if recompute_c_y {
            let c_y: EI::ExtendedInt = &numer2 * &dif_x[0] - &numer1 * &dif_x[1];
            circle.cell_set_y_xf(EX::ExtendedExponentFpt::from(c_y) * inv_denom);
        }
        #[cfg(feature = "console_debug")]
        {
            let c = circle.0.get();
            tln!(
                "ppp(x:{:.12}, y:{:.12}, lx:{:.12})",
                c.x(),
                c.y(),
                c.lower_x()
            );
        }
    }

    /// Recompute parameters of the point, point, segment circle event using high-precision library.
    #[allow(clippy::too_many_arguments)]
    fn pps<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
        segment_index: SiteIndex,
        c_event: &VC::CircleEventType,
        recompute_c_x: bool,
        recompute_c_y: bool,
        recompute_lower_x: bool,
    ) {
        tln!(
            "->pps site1:{:?} site2:{:?} site3:{:?}",
            site1,
            site2,
            site3
        );
        t!(
            "  segment_index:{:?} recompute_c_x:{}",
            segment_index,
            recompute_c_x
        );
        tln!(
            " recompute_c_y:{} recompute_lower_x:{}",
            recompute_c_y,
            recompute_lower_x
        );

        let i_to_bi = super::cast_i_to_xi;

        // Todo: is 5 the correct size?
        let mut ca: [EI::ExtendedInt; 5] = [
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
        ];
        let mut cb: [EI::ExtendedInt; 5] = [
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
        ];
        let line_a: EI::ExtendedInt = i_to_bi(site3.y1()) - i_to_bi(site3.y0());
        let line_b: EI::ExtendedInt = i_to_bi(site3.x0()) - i_to_bi(site3.x1());
        let segm_len: EI::ExtendedInt = &line_a * &line_a + &line_b * &line_b;
        let vec_x: EI::ExtendedInt = i_to_bi(site2.y()) - i_to_bi(site1.y());
        let vec_y: EI::ExtendedInt = i_to_bi(site1.x()) - i_to_bi(site2.x());
        let sum_x: EI::ExtendedInt = i_to_bi(site1.x()) + i_to_bi(site2.x());
        let sum_y: EI::ExtendedInt = i_to_bi(site1.y()) + i_to_bi(site2.y());
        let teta: EI::ExtendedInt = &line_a * &vec_x + &line_b * &vec_y;
        let mut denom: EI::ExtendedInt = &vec_x * &line_b - &vec_y * &line_a;

        let mut dif0: EI::ExtendedInt = i_to_bi(site3.y1()) - i_to_bi(site1.y());
        let mut dif1: EI::ExtendedInt = i_to_bi(site1.x()) - i_to_bi(site3.x1());
        let a: EI::ExtendedInt = &line_a * &dif1 - &line_b * &dif0;

        dif0 = i_to_bi(site3.y1()) - i_to_bi(site2.y());
        dif1 = i_to_bi(site2.x()) - i_to_bi(site3.x1());
        let b = line_a * dif1 - line_b * dif0;
        let sum_ab = &a + &b;
        tln!("a:{:?} b:{:?} denom:{:?}", a, b, denom);

        if denom.is_zero() {
            let numer: EI::ExtendedInt = &teta * &teta - &sum_ab * &sum_ab;
            denom = &teta * &sum_ab;
            ca[0] = &denom * &sum_x * 2 + &numer * &vec_x;
            cb[0] = segm_len.clone();
            ca[1] = &denom * &sum_ab * 2 + &numer * &teta;
            cb[1] = EI::ExtendedInt::one();
            ca[2] = &denom * &sum_y * 2 + &numer * &vec_y;
            let inv_denom =
                EX::ExtendedExponentFpt::from(1f64) / EX::ExtendedExponentFpt::from(&denom);
            if recompute_c_x {
                c_event.cell_set_x_xf(EX::ExtendedExponentFpt::from(&ca[0]) * inv_denom / 4_f64);
            }
            if recompute_c_y {
                c_event.cell_set_y_xf(EX::ExtendedExponentFpt::from(&ca[2]) * inv_denom / 4_f64);
            }
            if recompute_lower_x {
                c_event.cell_set_lower_x_xf(
                    RF::RobustSqrtExpr::eval2(&ca, &cb) * inv_denom * 0.25f64
                        / (EX::ExtendedExponentFpt::from(&segm_len).sqrt()),
                );
            }
            return;
        }
        let det: EI::ExtendedInt = (&teta * &teta + &denom * &denom) * &a * &b * 4;
        let mut inv_denom_sqr =
            EX::ExtendedExponentFpt::from(1f64) / EX::ExtendedExponentFpt::from(&denom);
        inv_denom_sqr = inv_denom_sqr * inv_denom_sqr;
        tln!("det:{:?} inv_denom_sqr:{:.12}", det, inv_denom_sqr.d());

        if recompute_c_x || recompute_lower_x {
            ca[0] = sum_x * &denom * &denom + &teta * &sum_ab * &vec_x;
            cb[0] = EI::ExtendedInt::from(1_i32);
            ca[1] = if segment_index == SiteIndex::Two {
                -vec_x
            } else {
                vec_x
            };
            cb[1] = det.clone();
            if recompute_c_x {
                c_event.cell_set_x_xf(RF::RobustSqrtExpr::eval2(&ca, &cb) * inv_denom_sqr * 0.5f64);
            }
        }

        if recompute_c_y || recompute_lower_x {
            ca[2] = sum_y * &denom * &denom + &teta * &sum_ab * &vec_y;
            cb[2] = EI::ExtendedInt::one();
            ca[3] = if segment_index == SiteIndex::Two {
                -vec_y
            } else {
                vec_y
            };
            cb[3] = det.clone();
            if recompute_c_y {
                c_event.cell_set_y_xf(
                    RF::RobustSqrtExpr::eval2(&ca[2..], &cb[2..]) * inv_denom_sqr * 0.5f64,
                );
            }
        }

        if recompute_lower_x {
            cb[0] = &cb[0] * &segm_len;
            cb[1] = &cb[1] * &segm_len;
            ca[2] = sum_ab * (&denom * &denom + &teta * &teta);
            cb[2] = EI::ExtendedInt::one();
            ca[3] = if segment_index == SiteIndex::Two {
                -teta
            } else {
                teta
            };
            cb[3] = det;
            let segm_len = EX::ExtendedExponentFpt::from(segm_len).sqrt();
            tln!(" ca[0]:{:?}", ca[0]);
            tln!(" ca[1]:{:?}", ca[1]);
            tln!(" ca[2]:{:?}", ca[2]);
            tln!(" ca[3]:{:?}", ca[3]);
            tln!(" cb[0]:{:?}", cb[0]);
            tln!(" cb[1]:{:?}", cb[1]);
            tln!(" cb[2]:{:?}", cb[2]);
            tln!(" cb[3]:{:?}", cb[3]);
            tln!(" segm_len:{:.12}", segm_len.d());

            let eval4 = RF::RobustSqrtExpr::eval4(&ca, &cb);
            tln!("eval4:{:.12}", eval4.d());

            c_event.cell_set_lower_x_xf(eval4 * inv_denom_sqr * 0.5f64 / segm_len);
        }
        #[cfg(feature = "console_debug")]
        {
            let c = c_event.0.get();
            tln!(
                "<-pps(x:{:.12}, y:{:.12}, lx:{:.12})",
                c.x(),
                c.y(),
                c.lower_x()
            );
        }
    }

    /// Recompute parameters of the point, segment, segment circle event using high-precision library.
    #[allow(non_snake_case)]
    #[allow(clippy::too_many_arguments)]
    fn pss<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
        point_index: SiteIndex,
        c_event: &VC::CircleEventType,
        recompute_c_x: bool,
        recompute_c_y: bool,
        recompute_lower_x: bool,
    ) {
        let mut c: [EI::ExtendedInt; 2] = [EI::ExtendedInt::zero(), EI::ExtendedInt::zero()];
        let mut cA: [EI::ExtendedInt; 4] = [
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
        ];
        let mut cB: [EI::ExtendedInt; 4] = [
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
        ];

        let segm_start1 = site2.point1();
        let segm_end1 = site2.point0();
        let segm_start2 = site3.point0();
        let segm_end2 = site3.point1();
        let a: [EI::ExtendedInt; 2] = [
            super::cast_i_to_xi(segm_end1.x) - super::cast_i_to_xi(segm_start1.x),
            super::cast_i_to_xi(segm_end2.x) - super::cast_i_to_xi(segm_start2.x),
        ];

        let b: [EI::ExtendedInt; 2] = [
            super::cast_i_to_xi(segm_end1.y) - super::cast_i_to_xi(segm_start1.y),
            super::cast_i_to_xi(segm_end2.y) - super::cast_i_to_xi(segm_start2.y),
        ];
        tln!("->ExactCircleFormationFunctor:pss");
        tln!(" a[0]={:?}", a[0]);
        tln!(" a[1]={:?}", a[1]);
        tln!(" b[0]={:?}", b[0]);
        tln!(" b[1]={:?}", b[1]);
        tln!(" recompute_c_x:{}", recompute_c_x);
        tln!(" recompute_c_y:{}", recompute_c_y);
        tln!(" recompute_lower_x:{}", recompute_lower_x);

        let orientation: EI::ExtendedInt = &a[1] * &b[0] - &a[0] * &b[1];
        tln!(" orientation={:?}", orientation);

        if orientation.is_zero() {
            let denom = EX::ExtendedExponentFpt::from(
                EI::ExtendedInt::from(2_i32) * (&a[0] * &a[0] + &b[0] * &b[0]),
            );

            c[0] = (super::cast_i_to_xi(segm_start2.x) - super::cast_i_to_xi(segm_start1.x))
                * &b[0]
                - (super::cast_i_to_xi(segm_start2.y) - super::cast_i_to_xi(segm_start1.y)) * &a[0];
            let dx: EI::ExtendedInt =
                (super::cast_i_to_xi(site1.y()) - super::cast_i_to_xi(segm_start1.y)) * &a[0]
                    - (super::cast_i_to_xi(site1.x()) - super::cast_i_to_xi(segm_start1.x)) * &b[0];
            let dy: EI::ExtendedInt =
                (super::cast_i_to_xi(site1.x()) - super::cast_i_to_xi(segm_start2.x)) * &b[0]
                    - (super::cast_i_to_xi(site1.y()) - super::cast_i_to_xi(segm_start2.y)) * &a[0];
            cB[0] = dx * dy;
            cB[1] = EI::ExtendedInt::one();

            if recompute_c_y {
                cA[0] = if point_index == SiteIndex::Two {
                    EI::ExtendedInt::from(2i32)
                } else {
                    EI::ExtendedInt::from(-2i32)
                } * &b[0];
                tln!(" cA[0]={:?}", cA[0]);
                tln!(" a[0]={:?}", a[0]);
                tln!(" b[0]={:?}", b[0]);
                tln!(
                    " segm_start1.x={:?} segm_start1.y={:?}",
                    segm_start1.x,
                    segm_start1.y
                );
                tln!(
                    " segm_start2.x={:?} segm_start2.y={:?}",
                    segm_start2.x,
                    segm_start2.y
                );
                cA[1] = (super::cast_i_to_xi(segm_start1.y) + super::cast_i_to_xi(segm_start2.y))
                    * &a[0]
                    * &a[0]
                    - (super::cast_i_to_xi(segm_start1.x) + super::cast_i_to_xi(segm_start2.x)
                        - (super::cast_i_to_xi(site1.x()) * EI::ExtendedInt::from(2_i32)))
                        * &a[0]
                        * &b[0]
                    + (super::cast_i_to_xi(site1.y()) * EI::ExtendedInt::from(2_i32))
                        * &b[0]
                        * &b[0];
                tln!("cA[1]={:?}", cA[1]);
                let c_y = RF::RobustSqrtExpr::eval2(&cA, &cB);
                tln!("c_y={:?}", c_y);
                tln!("denom={:?}", denom);
                c_event.cell_set_y_xf(c_y / denom);
            }

            if recompute_c_x || recompute_lower_x {
                cA[0] = EI::ExtendedInt::from(if point_index == SiteIndex::Two {
                    2i32
                } else {
                    -2i32
                }) * &a[0];
                cA[1] = (super::cast_i_to_xi(segm_start1.x) + super::cast_i_to_xi(segm_start2.x))
                    * &b[0]
                    * &b[0]
                    - (super::cast_i_to_xi(segm_start1.y) + super::cast_i_to_xi(segm_start2.y)
                        - super::cast_i_to_xi(site1.y()) * EI::ExtendedInt::from(2_i32))
                        * &a[0]
                        * &b[0]
                    + super::cast_i_to_xi(site1.x()) * &a[0] * &a[0] * EI::ExtendedInt::from(2_i32);
                tln!(" cA[0]={:.0}", cA[0].d());
                tln!(" cA[1]={:.0}", cA[1].d());

                if recompute_c_x {
                    let c_x = RF::RobustSqrtExpr::eval2(&cA, &cB);
                    tln!(" c_x={:.0}", c_x.d());
                    tln!(" denom={:.0}", denom.d());
                    tln!(" c_x/denom={:.0}", (c_x / denom).d());

                    c_event.cell_set_x_xf(c_x / denom);
                }

                if recompute_lower_x {
                    cA[2] = if c[0].is_neg() {
                        c[0].clone() * EI::ExtendedInt::from(-1_i32)
                    } else {
                        c[0].clone()
                    };
                    cB[2] = &a[0] * &a[0] + &b[0] * &b[0];
                    let lower_x = RF::RobustSqrtExpr::eval3(&cA, &cB);
                    c_event.cell_set_lower_x_xf(lower_x / denom);
                }
            }
            return;
        }
        c[0] = super::cast_i_to_xi(segm_end1.x) * &b[0] - super::cast_i_to_xi(segm_end1.y) * &a[0];
        c[1] = super::cast_i_to_xi(segm_end2.y) * &a[1] - super::cast_i_to_xi(segm_end2.x) * &b[1];
        let ix: EI::ExtendedInt = &a[0] * &c[1] + &a[1] * &c[0];
        let iy: EI::ExtendedInt = &b[0] * &c[1] + &b[1] * &c[0];
        let dx: EI::ExtendedInt = ix.clone() - super::cast_i_to_xi(site1.x()) * &orientation;
        let dy: EI::ExtendedInt = iy.clone() - super::cast_i_to_xi(site1.y()) * &orientation;
        tln!(" ix={:?}", ix);
        tln!(" iy={:?}", iy);
        tln!(" dx={:?}", dx);
        tln!(" dy={:?}", dy);

        if dx.is_zero() && dy.is_zero() {
            let denom = EX::ExtendedExponentFpt::from(&orientation);
            let c_x = EX::ExtendedExponentFpt::from(&ix) / denom;
            let c_y = EX::ExtendedExponentFpt::from(&iy) / denom;
            c_event.cell_set_3_ext(c_x, c_y, c_x);
            return;
        }

        let sign: EI::ExtendedInt =
            EI::ExtendedInt::from(if point_index == SiteIndex::Two { 1 } else { -1 })
                * EI::ExtendedInt::from(if orientation.is_neg() { 1_i32 } else { -1 });
        // todo: remove -1*-1
        tln!(" a[1]={:?}", &a[1]);
        tln!(" b[1]={:?}", &b[1]);
        tln!(" cA[0]={:?}", EI::ExtendedInt::from(-1) * &a[1] * &dx);
        tln!(" cA[1]={:?}", EI::ExtendedInt::from(-1) * &b[1] * &dy);

        cA[0] = (EI::ExtendedInt::from(-1) * &a[1] * &dx) - (&b[1] * &dy);
        cA[1] = (EI::ExtendedInt::from(-1) * &a[0] * &dx) - (&b[0] * &dy);
        cA[2] = sign.clone();
        cA[3] = EI::ExtendedInt::zero();

        tln!(" cA[0]={:?}", cA[0]);
        tln!(" cA[1]={:?}", cA[1]);
        tln!(" cA[2]={:?}", cA[2]);
        tln!(" cA[3]={:?}", cA[3]);

        cB[0] = &a[0] * &a[0] + &b[0] * &b[0];
        cB[1] = &a[1] * &a[1] + &b[1] * &b[1];
        cB[2] = &a[0] * &a[1] + &b[0] * &b[1];
        cB[3] = EI::ExtendedInt::from(-2_i32)
            * (&a[0] * &dy - &b[0] * &dx)
            * (&a[1] * &dy - &b[1] * &dx);
        let temp = RF::RobustSqrtExpr::sqrt_expr_evaluator_pss4(&cA[0..], &cB[0..]);
        let denom = temp * EX::ExtendedExponentFpt::from(&orientation);

        if recompute_c_y {
            cA[0] = (&dx * &dx + &dy * &dy)*&b[1] - (&dx * &a[1] + &dy * &b[1])*&iy;
            cA[1] = (&dx * &dx + &dy * &dy)*&b[0] - (&dx * &a[0] + &dy * &b[0])*&iy;
            cA[2] = iy * &sign;
            let cy = RF::RobustSqrtExpr::sqrt_expr_evaluator_pss4(&cA[0..], &cB[0..]);
            c_event.cell_set_y_xf(cy / denom);
        }

        if recompute_c_x || recompute_lower_x {
            cA[0] = (&dx * &dx + &dy * &dy)*&a[1] - (&dx * &a[1] + &dy * &b[1])*&ix;
            cA[1] = (&dx * &dx + &dy * &dy)*&a[0] - (&dx * &a[0] + &dy * &b[0])*&ix;
            cA[2] = ix * &sign;

            if recompute_c_x {
                let cx = RF::RobustSqrtExpr::sqrt_expr_evaluator_pss4(&cA, &cB);
                c_event.cell_set_x_xf(cx / denom);
            }

            if recompute_lower_x {
                cA[3] = orientation
                    * (&dx * &dx + &dy * &dy)
                    * EI::ExtendedInt::from(if temp.is_neg() { -1_i32 } else { 1 });
                let lower_x = RF::RobustSqrtExpr::sqrt_expr_evaluator_pss4(&cA, &cB);
                c_event.cell_set_lower_x_xf(lower_x / denom);
            }
        }
        #[cfg(feature = "console_debug")]
        {
            let c = c_event.0.get();
            tln!(
                "pss(x:{:.12}, y:{:.12}, lx:{:.12})",
                c.x(),
                c.y(),
                c.lower_x()
            );
            tln!(
                "recompute_c_x:{}, recompute_c_y:{}, recompute_lower_x:{}",
                recompute_c_x,
                recompute_c_y,
                recompute_lower_x
            );
        }
    }

    /// Recompute parameters of the segment, segment, segment circle event using high-precision library.
    #[allow(non_snake_case)]
    #[allow(clippy::many_single_char_names)]
    #[allow(clippy::suspicious_operation_groupings)]
    fn sss<I: InputType, F: OutputType>(
        site1: &VSE::SiteEvent<I, F>,
        site2: &VSE::SiteEvent<I, F>,
        site3: &VSE::SiteEvent<I, F>,
        c_event: &VC::CircleEventType,
        recompute_c_x: bool,
        recompute_c_y: bool,
        recompute_lower_x: bool,
    ) {
        tln!(">ExactCircleFormationFunctor:sss site1:{:?} site2:{:?}, site3:{:?}, recompute_c_x:{} recompute_c_y:{}, recompute_lower_x:{}",
            site1, site2, site3, recompute_c_x,recompute_c_y, recompute_lower_x);

        let i_to_bi = super::cast_i_to_xi;

        let mut cA: [EI::ExtendedInt; 4] = [
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
        ];
        let mut cB: [EI::ExtendedInt; 4] = [
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
        ];

        // cA - corresponds to the cross product.
        // cB - corresponds to the squared length.

        let a = [
            i_to_bi(site1.x1()) - i_to_bi(site1.x0()),
            i_to_bi(site2.x1()) - i_to_bi(site2.x0()),
            i_to_bi(site3.x1()) - i_to_bi(site3.x0()),
        ];
        let b = [
            i_to_bi(site1.y1()) - i_to_bi(site1.y0()),
            i_to_bi(site2.y1()) - i_to_bi(site2.y0()),
            i_to_bi(site3.y1()) - i_to_bi(site3.y0()),
        ];

        let c = [
            &i_to_bi(site1.x0()) * &i_to_bi(site1.y1())
                - &i_to_bi(site1.y0()) * &i_to_bi(site1.x1()),
            &i_to_bi(site2.x0()) * &i_to_bi(site2.y1())
                - &i_to_bi(site2.y0()) * &i_to_bi(site2.x1()),
            &i_to_bi(site3.x0()) * &i_to_bi(site3.y1())
                - &i_to_bi(site3.y0()) * &i_to_bi(site3.x1()),
        ];

        for (i, aa) in a.iter().enumerate().take(3) {
            cB[i] = aa.clone() * aa + &b[i] * &b[i];
        }
        for (i, cA_i) in cA.iter_mut().enumerate().take(3) {
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;
            *cA_i = &a[j] * &b[k] - &a[k] * &b[j];
        }
        let denom = RF::RobustSqrtExpr::eval3(&cA, &cB);

        if recompute_c_y {
            for (i, cA_i) in cA.iter_mut().enumerate().take(3) {
                let j = (i + 1) % 3;
                let k = (i + 2) % 3;
                *cA_i = &b[j] * &c[k] - &b[k] * &c[j];
            }
            let c_y = RF::RobustSqrtExpr::eval3(&cA, &cB);
            c_event.cell_set_y_xf(c_y / denom);
        }

        if recompute_c_x || recompute_lower_x {
            cA[3] = EI::ExtendedInt::zero();
            for i in 0..3 {
                let j = (i + 1) % 3;
                let k = (i + 2) % 3;
                cA[i] = &a[j] * &c[k] - &a[k] * &c[j];
                if recompute_lower_x {
                    cA[3] = &cA[3] + &(&cA[i] * &b[i]);
                }
            }

            if recompute_c_x {
                let c_x = RF::RobustSqrtExpr::eval3(&cA, &cB);
                c_event.cell_set_x_xf(c_x / denom);
            }

            if recompute_lower_x {
                cB[3] = EI::ExtendedInt::one();
                let lower_x = RF::RobustSqrtExpr::eval4(&cA, &cB);
                c_event.cell_set_lower_x_xf(lower_x / denom);
            }
        }
        #[cfg(feature = "console_debug")]
        {
            let c = c_event.0.get();
            tln!(
                "sss(x:{:.12}, y:{:.12}, lx:{:.12})",
                c.x(),
                c.y(),
                c.lower_x()
            );
        }
    }
}
