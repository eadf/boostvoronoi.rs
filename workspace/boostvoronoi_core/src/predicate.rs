// Boost.Polygon library detail/voronoi_predicates.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! Predicate utilities

mod circle_existence;
mod exact_circle_formation;
mod lazy_circle_formation;

#[cfg(test)]
mod tests;

use crate::{cast, geometry::Point, InputType, OutputType};
use std::fmt::Debug;

// TODO: how to make these generic?
//const ULPS: u64 = 64;
const ULPSX2: u64 = 64; // Todo: This is what c++ boost uses. Find a fix for this

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum SiteIndex {
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

#[inline(always)]
pub(crate) fn is_vertical<I: InputType, F: OutputType>(point1: Point<I>, point2: Point<I>) -> bool {
    point1.x == point2.x
}

/// Compute robust cross_product: a1 * b2 - b1 * a2.
/// It was mathematically proven that the result is correct
/// with epsilon relative error equal to 1EPS.
#[inline]
fn robust_cross_product<I: InputType, F: OutputType>(s_a1: I, s_b1: I, s_a2: I, s_b2: I) -> F {
    // Why can't *all* integers implement is_negative()? E.g u64 would just always return false.
    // It would make it easier to implement generic code
    let u_a1 = if s_a1 < I::zero() { -s_a1 } else { s_a1 };
    let u_b1 = if s_b1 < I::zero() { -s_b1 } else { s_b1 };
    let u_a2 = if s_a2 < I::zero() { -s_a2 } else { s_a2 };
    let u_b2 = if s_b2 < I::zero() { -s_b2 } else { s_b2 };

    let l = u_a1 * u_b2;
    let r = u_b1 * u_a2;

    if (s_a1 < I::zero()) ^ (s_b2 < I::zero()) {
        return if (s_a2 < I::zero()) ^ (s_b1 < I::zero()) {
            if l > r {
                -cast::<I, F>(l - r)
            } else {
                cast::<I, F>(r - l)
            }
        } else {
            -cast::<I, F>(l + r)
        };
    }
    if (s_a2 < I::zero()) ^ (s_b1 < I::zero()) {
        return cast::<I, F>(l + r);
    }
    if l < r {
        -cast::<I, F>(r - l)
    } else {
        cast::<I, F>(l - r)
    }
}

pub(crate) mod orientation_predicate {
    use crate::geometry::Point;
    use crate::predicate::robust_cross_product;
    use crate::{cast, InputType, OutputType};
    use num_traits::Zero;

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) enum Orientation {
        Right,     // = -1,
        Collinear, // = 0,
        Left,      // = 1
    }

    /// Value is a determinant of two vectors (e.g. x1 * y2 - x2 * y1).
    /// Return orientation based on the sign of the determinant.
    #[inline(always)]
    pub(crate) fn eval_f<I: InputType, F: OutputType>(value: f64) -> Orientation {
        if value.is_zero() {
            return Orientation::Collinear;
        }
        match value.is_sign_negative() {
            true => Orientation::Right,
            false => Orientation::Left,
        }
    }

    #[inline(always)]
    pub(crate) fn eval_p<I: InputType, F: OutputType>(
        point1: Point<I>,
        point2: Point<I>,
        point3: Point<I>,
    ) -> Orientation {
        let dx1: i64 = cast::<I, i64>(point1.x) - cast::<I, i64>(point2.x);
        let dx2: i64 = cast::<I, i64>(point2.x) - cast::<I, i64>(point3.x);
        let dy1: i64 = cast::<I, i64>(point1.y) - cast::<I, i64>(point2.y);
        let dy2: i64 = cast::<I, i64>(point2.y) - cast::<I, i64>(point3.y);
        let cp: f64 = robust_cross_product::<i64, f64>(dx1, dy1, dx2, dy2);
        eval_f::<I, F>(cp)
    }

    #[inline(always)]
    pub(crate) fn eval_i<I: InputType, F: OutputType>(
        dif_x1: i64,
        dif_y1: i64,
        dif_x2: i64,
        dif_y2: i64,
    ) -> Orientation {
        eval_f::<i64, f64>(robust_cross_product::<i64, f64>(
            dif_x1, dif_y1, dif_x2, dif_y2,
        ))
    }
}

pub(crate) mod point_comparison {
    use crate::geometry::Point;
    use crate::InputType;
    /// returns true if lhs.x < rhs.x, if lhs.x==rhs.x it returns lhs.y < rhs.y
    #[inline(always)]
    pub(crate) fn point_comparison<I: InputType>(lhs: Point<I>, rhs: Point<I>) -> bool {
        if lhs.x == rhs.x {
            lhs.y < rhs.y
        } else {
            lhs.x < rhs.x
        }
    }
}

pub(crate) mod event_comparison_predicate {
    use crate::ctypes::ulp_comparison;
    use crate::predicate::{is_vertical, orientation_predicate, ULPSX2};
    use crate::{
        cast, circle_event::CircleEvent, site_event::SiteEvent, tln, InputType, OutputType,
    };
    use std::cmp;

    /// boolean predicate between two sites (bool int int)
    #[allow(dead_code)]
    pub(crate) fn event_comparison_bii<I: InputType, F: OutputType>(
        lhs: &SiteEvent<I, F>,
        rhs: &SiteEvent<I, F>,
    ) -> bool {
        if lhs.x0() != rhs.x0() {
            return lhs.x0() < rhs.x0();
        }
        if !lhs.is_segment() {
            if !rhs.is_segment() {
                return lhs.y0() < rhs.y0();
            }
            if rhs.is_vertical() {
                return lhs.y0() <= rhs.y0();
            }
            true
        } else {
            if is_vertical::<I, F>(rhs.point0(), rhs.point1()) {
                if is_vertical::<I, F>(lhs.point0(), lhs.point1()) {
                    return lhs.y0() < rhs.y0();
                }
                return false;
            }
            if is_vertical::<I, F>(lhs.point0(), lhs.point1()) {
                return true;
            }
            if lhs.y0() != rhs.y0() {
                return lhs.y0() < rhs.y0();
            }
            orientation_predicate::eval_p::<I, F>(lhs.point1(), lhs.point0(), rhs.point1())
                == orientation_predicate::Orientation::Left
        }
    }

    #[inline]
    /// cmp::Ordering predicate between two sites (int int)
    pub(crate) fn event_comparison_ii<I: InputType, F: OutputType>(
        lhs: &SiteEvent<I, F>,
        rhs: &SiteEvent<I, F>,
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
        if event_comparison_bii(lhs, rhs) {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }

    #[inline]
    #[allow(dead_code)]
    /// cmp::Ordering predicate between two sites (int int)
    pub(crate) fn event_comparison_ii_wip<I: InputType, F: OutputType>(
        lhs: &SiteEvent<I, F>,
        rhs: &SiteEvent<I, F>,
    ) -> cmp::Ordering {
        match lhs.x0().cmp(&rhs.x0()) {
            cmp::Ordering::Greater => return cmp::Ordering::Greater,
            cmp::Ordering::Less => return cmp::Ordering::Less,
            cmp::Ordering::Equal => {
                if lhs.is_point() {
                    if rhs.is_point() {
                        // lhs & rhs: Point
                        match lhs.point0().y.cmp(&rhs.point0().y) {
                            cmp::Ordering::Greater => {
                                println!("ii_1");
                                return cmp::Ordering::Greater;
                            }
                            cmp::Ordering::Less => {
                                println!("ii_2");
                                return cmp::Ordering::Less;
                            }
                            cmp::Ordering::Equal => (),
                        }
                    } else {
                        // lhs:Point rhs: Segment
                        println!("ii_2.1");
                        return cmp::Ordering::Less;
                    }
                } else {
                    // lhs = Segment
                    if rhs.is_point() {
                        // rhs: Point
                        match lhs.y0().cmp(&rhs.y0()) {
                            cmp::Ordering::Greater => {
                                println!("ii_2.3");
                                return cmp::Ordering::Greater;
                            }
                            cmp::Ordering::Less => {
                                println!("ii_2.4");
                                return cmp::Ordering::Less;
                            }
                            cmp::Ordering::Equal => {
                                println!("ii_2.5");
                                return cmp::Ordering::Greater;
                            }
                        }
                    }
                    if rhs.is_vertical() {
                        if lhs.is_vertical() {
                            println!("ii_3");
                            return lhs.y0().cmp(&rhs.y0());
                        }
                        println!("ii_4");
                        return cmp::Ordering::Greater;
                    }
                    if lhs.is_vertical() {
                        println!("ii_5");
                        return cmp::Ordering::Less;
                    }
                    if lhs.y0() != rhs.y0() {
                        println!("ii_6");
                        return lhs.y0().cmp(&rhs.y0());
                    }
                    if orientation_predicate::eval_p::<I, F>(
                        lhs.point1(),
                        lhs.point0(),
                        rhs.point1(),
                    ) == orientation_predicate::Orientation::Left
                    {
                        println!("ii_7");
                        return cmp::Ordering::Less;
                    }
                }
            }
        }
        println!("ii_8");
        lhs.initial_index().cmp(&rhs.initial_index())
    }

    /// boolean predicate between site and circle (Bool Integer Float)
    #[allow(clippy::let_and_return)]
    pub(crate) fn event_comparison_bif<I: InputType, F: OutputType>(
        lhs: &SiteEvent<I, F>,
        rhs: &CircleEvent,
    ) -> bool {
        let lhs = cast::<I, f64>(lhs.x0());
        let rhs = rhs.lower_x();
        let rv = ulp_comparison(lhs, rhs, ULPSX2) == cmp::Ordering::Less;
        tln!(
            "event_comparison_predicate_bif lhs:{:.12} rhs:{:.12} -> {}",
            lhs,
            rhs,
            rv
        );
        rv
    }
}

pub(crate) mod distance_predicate {
    use crate::ctypes::ulp_comparison;
    use crate::geometry::Point;
    use crate::predicate::{orientation_predicate, robust_cross_product};
    use crate::{cast, site_event::SiteEvent, InputType, OutputType};
    use std::cmp;

    /// Represents the result of the epsilon robust predicate. If the
    /// result is undefined some further processing is usually required.
    #[allow(clippy::upper_case_acronyms)]
    #[derive(Debug, PartialEq, Eq)]
    enum KPredicateResult {
        LESS,      // = -1,
        UNDEFINED, // = 0,
        MORE,      // = 1
    }

    // todo: return Ordering
    pub(crate) fn distance_predicate<I: InputType, F: OutputType>(
        left_site: &SiteEvent<I, F>,
        right_site: &SiteEvent<I, F>,
        new_point: Point<I>,
    ) -> bool {
        if !left_site.is_segment() {
            if !right_site.is_segment() {
                pp(left_site, right_site, new_point)
            } else {
                ps(left_site, right_site, new_point, false)
            }
        } else if !right_site.is_segment() {
            ps(right_site, left_site, new_point, true)
        } else {
            ss(left_site, right_site, new_point)
        }
    }

    /// Robust predicate, avoids using high-precision libraries.
    /// Returns true if a horizontal line going through the new point site
    /// intersects right arc at first, else returns false. If horizontal line
    /// goes through intersection point of the given two arcs returns false.
    pub(crate) fn pp<I: InputType, F: OutputType>(
        left_site: &SiteEvent<I, F>,
        right_site: &SiteEvent<I, F>,
        new_point: Point<I>,
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

        let dist1 = distance_to_point_arc(left_site, new_point);
        let dist2 = distance_to_point_arc(right_site, new_point);

        // The undefined ulp range is equal to 3EPS + 3EPS <= 6ULP.
        dist1 < dist2
    }

    pub(crate) fn ps<I: InputType, F: OutputType>(
        left_site: &SiteEvent<I, F>,
        right_site: &SiteEvent<I, F>,
        new_point: Point<I>,
        reverse_order: bool,
    ) -> bool {
        let fast_res = fast_ps(left_site, right_site, new_point, reverse_order);
        if fast_res != KPredicateResult::UNDEFINED {
            return fast_res == KPredicateResult::LESS;
        }

        let dist1 = distance_to_point_arc(left_site, new_point);
        let dist2 = distance_to_segment_arc(right_site, new_point);

        // The undefined ulp range is equal to 3EPS + 7EPS <= 10ULP.
        reverse_order ^ (dist1 < dist2)
    }

    pub(crate) fn ss<I: InputType, F: OutputType>(
        left_site: &SiteEvent<I, F>,
        right_site: &SiteEvent<I, F>,
        new_point: Point<I>,
    ) -> bool {
        // Handle temporary segment sites.
        if left_site.sorted_index() == right_site.sorted_index() {
            return orientation_predicate::eval_p::<I, F>(
                left_site.point0(),
                left_site.point1(),
                new_point,
            ) == orientation_predicate::Orientation::Left;
        }

        let dist1 = distance_to_segment_arc(left_site, new_point);
        let dist2 = distance_to_segment_arc(right_site, new_point);

        // The undefined ulp range is equal to 7EPS + 7EPS <= 14ULP.
        dist1 < dist2
    }

    #[inline(always)]
    fn distance_to_point_arc<I: InputType, F: OutputType>(
        site: &SiteEvent<I, F>,
        point: Point<I>,
    ) -> f64 {
        let dx = cast::<I, f64>(site.x()) - cast::<I, f64>(point.x);
        let dy = cast::<I, f64>(site.y()) - cast::<I, f64>(point.y);
        // The relative error is at most 3EPS.
        (dx * dx + dy * dy) / (dx * 2_f64)
    }

    fn distance_to_segment_arc<I: InputType, F: OutputType>(
        site: &SiteEvent<I, F>,
        point: Point<I>,
    ) -> f64 {
        if site.is_vertical() {
            (cast::<I, f64>(site.x()) - cast::<I, f64>(point.x)) * 0.5_f64
        } else {
            let segment0 = site.point0();
            let segment1 = site.point1();
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
            k * robust_cross_product::<i64, f64>(
                cast::<I, i64>(segment1.x) - cast::<I, i64>(segment0.x),
                cast::<I, i64>(segment1.y) - cast::<I, i64>(segment0.y),
                cast::<I, i64>(point.x) - cast::<I, i64>(segment0.x),
                cast::<I, i64>(point.y) - cast::<I, i64>(segment0.y),
            )
        }
    }

    fn fast_ps<I: InputType, F: OutputType>(
        left_site: &SiteEvent<I, F>,
        right_site: &SiteEvent<I, F>,
        new_point: Point<I>,
        reverse_order: bool,
    ) -> KPredicateResult {
        let site_point: Point<I> = left_site.point0();
        let segment_start: Point<I> = right_site.point0();
        let segment_end: Point<I> = right_site.point1();
        let eval = orientation_predicate::eval_p::<I, F>(segment_start, segment_end, new_point);
        if eval != orientation_predicate::Orientation::Right {
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

        if right_site.is_vertical() {
            if new_point.y < site_point.y && !reverse_order {
                return KPredicateResult::MORE;
            } else if new_point.y > site_point.y && reverse_order {
                return KPredicateResult::LESS;
            }
            return KPredicateResult::UNDEFINED;
        } else {
            let orientation = orientation_predicate::eval_i::<I, F>(
                cast::<I, i64>(segment_end.x) - cast::<I, i64>(segment_start.x),
                cast::<I, i64>(segment_end.y) - cast::<I, i64>(segment_start.y),
                cast::<I, i64>(new_point.x) - cast::<I, i64>(site_point.x),
                cast::<I, i64>(new_point.y) - cast::<I, i64>(site_point.y),
            );
            if orientation == orientation_predicate::Orientation::Left {
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

pub(crate) mod node_comparison_predicate {
    use crate::beach_line as VB;
    use crate::geometry::Point;
    use crate::predicate::{distance_predicate, point_comparison};
    use crate::{site_event::SiteEvent, InputType, OutputType};
    use std::cmp;

    /// Compares nodes in the balanced binary search tree. Nodes are
    /// compared based on the y coordinates of the arcs intersection points.
    /// Nodes with less y coordinate of the intersection point go first.
    /// Comparison is only called during the new site events processing.
    /// That's why one of the nodes will always lie on the sweepline and may
    /// be represented as a straight horizontal line.
    pub(crate) fn node_comparison<I: InputType, F: OutputType>(
        node1: &VB::BeachLineNodeKey<I, F>,
        node2: &VB::BeachLineNodeKey<I, F>,
    ) -> bool {
        // Get x coordinate of the rightmost site from both nodes.
        let site1 = comparison_site_(node1);
        let site2 = comparison_site_(node2);
        let point1 = comparison_point_(site1);
        let point2 = comparison_point_(site2);
        let rv = {
            match point1.x.cmp(&point2.x) {
                cmp::Ordering::Less => {
                    //tln!("point1.x < point2.x {}<{}", point1.x, point2.x);
                    // The second node contains a new site.
                    distance_predicate::distance_predicate(
                        node1.left_site(),
                        node1.right_site(),
                        point2,
                    )
                }
                cmp::Ordering::Greater => {
                    //tln!( "point1.x > point2.x");
                    // The first node contains a new site.
                    !distance_predicate::distance_predicate(
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
                            let y1 = comparison_y_(node1, true);
                            let y2 = comparison_y_(node2, true);
                            y1 < y2
                        }
                        cmp::Ordering::Less => {
                            let y1 = comparison_y_(node1, false);
                            let y2 = comparison_y_(node2, true);
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
                            let y1 = comparison_y_(node1, true);
                            let y2 = comparison_y_(node2, false);
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

    #[inline(always)]
    /// Get the newer site.
    fn comparison_site_<I: InputType, F: OutputType>(
        node: &VB::BeachLineNodeKey<I, F>,
    ) -> &SiteEvent<I, F> {
        if node.left_site().sorted_index() > node.right_site().sorted_index() {
            node.left_site()
        } else {
            node.right_site()
        }
    }

    #[inline(always)]
    /// returns the point with lowest x, or point with lowest y if x are equal
    fn comparison_point_<I: InputType, F: OutputType>(site: &SiteEvent<I, F>) -> Point<I> {
        if point_comparison::point_comparison(site.point0(), site.point1()) {
            site.point0()
        } else {
            site.point1()
        }
    }

    #[inline(always)]
    /// Get comparison pair: tuple of y coordinate and direction of the newer site.
    fn comparison_y_<I: InputType, F: OutputType>(
        node: &VB::BeachLineNodeKey<I, F>,
        is_new_node: bool,
    ) -> (I, i8) {
        if node.left_site().sorted_index() == node.right_site().sorted_index() {
            return (node.left_site().y0(), 0);
        }
        if node.left_site().sorted_index() > node.right_site().sorted_index() {
            if !is_new_node && node.left_site().is_segment() && node.left_site().is_vertical() {
                return (node.left_site().y0(), 1);
            }
            return (node.left_site().y1(), 1);
        }
        return (node.right_site().y0(), -1);
    }
}

pub(crate) mod circle_formation_predicate {
    use crate::beach_line as VB;
    use crate::ctypes::ulp_comparison;
    use crate::predicate::{circle_existence, lazy_circle_formation, SiteIndex};
    use crate::{cast, circle_event::CircleEvent, site_event::SiteEvent, InputType, OutputType};
    use std::cmp;

    pub(crate) fn lies_outside_vertical_segment<I: InputType, F: OutputType>(
        c: &CircleEvent,
        s: &SiteEvent<I, F>,
    ) -> bool {
        if !s.is_segment() || !s.is_vertical() {
            return false;
        }
        let y0 = cast::<I, f64>(if s.is_inverse() { s.y1() } else { s.y0() });
        let y1 = cast::<I, f64>(if s.is_inverse() { s.y0() } else { s.y1() });
        let cc_y = c.y();

        ulp_comparison(cc_y, y0, 64) == cmp::Ordering::Less
            || ulp_comparison(cc_y, y1, 64) == cmp::Ordering::Greater
    }

    /// Create a circle event from the given three sites.
    /// Returns true if the circle event exists, else false.
    /// If exists circle event is saved into the c_event variable.
    pub(crate) fn circle_formation<I: InputType, F: OutputType>(
        site1: &SiteEvent<I, F>,
        site2: &SiteEvent<I, F>,
        site3: &SiteEvent<I, F>,
        bisector_node: VB::BeachLineIndex,
    ) -> Option<CircleEvent> {
        let circle = if !site1.is_segment() {
            if !site2.is_segment() {
                if !site3.is_segment() {
                    // (point, point, point) sites.
                    if !circle_existence::ppp::<I, F>(
                        site1.point0(),
                        site2.point0(),
                        site3.point0(),
                    ) {
                        return None;
                    }
                    lazy_circle_formation::ppp::<I, F>(
                        site1.point0(),
                        site2.point0(),
                        site3.point0(),
                        CircleEvent::new(bisector_node),
                    )
                } else {
                    // (point, point, segment) sites.
                    if !circle_existence::pps::<I, F>(
                        site1.point0(),
                        site2.point0(),
                        site3,
                        SiteIndex::Three,
                    ) {
                        return None;
                    }
                    lazy_circle_formation::pps::<I, F>(
                        site1.point0(),
                        site2.point0(),
                        site3,
                        SiteIndex::Three,
                        CircleEvent::new(bisector_node),
                    )
                }
            } else if !site3.is_segment() {
                // (point, segment, point) sites.
                if !circle_existence::pps::<I, F>(
                    site1.point0(),
                    site3.point0(),
                    site2,
                    SiteIndex::Two,
                ) {
                    return None;
                }
                lazy_circle_formation::pps::<I, F>(
                    site1.point0(),
                    site3.point0(),
                    site2,
                    SiteIndex::Two,
                    CircleEvent::new(bisector_node),
                )
            } else {
                // (point, segment, segment) sites.
                if !circle_existence::pss::<I, F>(site1.point0(), site2, site3, SiteIndex::One) {
                    return None;
                }
                lazy_circle_formation::pss::<I, F>(
                    site1.point0(),
                    site2,
                    site3,
                    SiteIndex::One,
                    CircleEvent::new(bisector_node),
                )
            }
        } else if !site2.is_segment() {
            if !site3.is_segment() {
                // (segment, point, point) sites.
                if !circle_existence::pps::<I, F>(
                    site2.point0(),
                    site3.point0(),
                    site1,
                    SiteIndex::One,
                ) {
                    return None;
                }
                lazy_circle_formation::pps::<I, F>(
                    site2.point0(),
                    site3.point0(),
                    site1,
                    SiteIndex::One,
                    CircleEvent::new(bisector_node),
                )
            } else {
                // (segment, point, segment) sites.
                if !circle_existence::pss::<I, F>(site2.point0(), site1, site3, SiteIndex::Two) {
                    return None;
                }
                lazy_circle_formation::pss::<I, F>(
                    site2.point0(),
                    site1,
                    site3,
                    SiteIndex::Two,
                    CircleEvent::new(bisector_node),
                )
            }
        } else if !site3.is_segment() {
            // (segment, segment, point) sites.
            if !circle_existence::pss::<I, F>(site3.point0(), site1, site2, SiteIndex::Three) {
                return None;
            }
            lazy_circle_formation::pss::<I, F>(
                site3.point0(),
                site1,
                site2,
                SiteIndex::Three,
                CircleEvent::new(bisector_node),
            )
        } else {
            // (segment, segment, segment) sites.
            if !circle_existence::sss::<I, F>(site1, site2, site3) {
                return None;
            }
            lazy_circle_formation::sss::<I, F>(site1, site2, site3, CircleEvent::new(bisector_node))
        };

        if let Some(circle) = circle.as_ref() {
            if lies_outside_vertical_segment(circle, site1)
                || lies_outside_vertical_segment(circle, site2)
                || lies_outside_vertical_segment(circle, site3)
            {
                return None;
            }
        }
        #[cfg(all(feature = "geo", feature = "ce_corruption_check"))]
        if let Some(circle) = circle.as_ref() {
            circle_existence::validate_circle_formation::<I, F>(site1, site2, site3, circle);
        }
        circle
    }
}
