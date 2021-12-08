// Boost.Polygon library detail/voronoi_predicates.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

use crate::predicate::orientation_predicate::{self, Orientation};
use crate::site_event as VSE;
use crate::{geometry::Point, predicate::SiteIndex, InputType, OutputType};

#[inline(always)]
pub(crate) fn ppp<I: InputType, F: OutputType>(
    point1: Point<I>,
    point2: Point<I>,
    point3: Point<I>,
) -> bool {
    orientation_predicate::eval_p::<I, F>(point1, point2, point3) == Orientation::Right
}

#[cfg(all(feature = "geo", feature = "ce_corruption_check"))]
#[inline(always)]
pub(crate) fn validate_circle_formation<I: InputType, F: OutputType>(
    site1: &VSE::SiteEvent<I, F>,
    site2: &VSE::SiteEvent<I, F>,
    site3: &VSE::SiteEvent<I, F>,
    c_event: &crate::circle_event::CircleEvent,
) {
    use approx::AbsDiffEq;

    let c = geo::Coordinate {
        x: c_event.x() as f64,
        y: c_event.y() as f64,
    };
    let d1 = site1.distance_to_point(c.x, c.y);
    let d2 = site2.distance_to_point(c.x, c.y);
    let d3 = site3.distance_to_point(c.x, c.y);

    let equidistant = if d1 <= d2 && d1 <= d3 {
        if d2 <= d3 {
            d1.abs_diff_ne(&d2, 0.00001)
        } else {
            d1.abs_diff_ne(&d3, 0.00001)
        }
    } else if d2 <= d1 && d2 <= d3 {
        if d1 <= d3 {
            d2.abs_diff_ne(&d1, 0.00001)
        } else {
            d2.abs_diff_ne(&d3, 0.00001)
        }
    } else if d1 <= d2 {
        d3.abs_diff_ne(&d1, 0.00001)
    } else {
        d3.abs_diff_ne(&d2, 0.00001)
    };

    // accept circle events that are in the middle of *two* sites,
    // as long as they are the closest two
    if equidistant {
        println!(
            "\nvalidate CE x={} y:{} xl:{}",
            c_event.x(),
            c_event.y(),
            c_event.lower_x()
        );

        println!("circle_formation_predicate should return false but doesn't");
        println!("c={:?} lx:{}", c, c_event.lower_x());
        println!("site1:{:?} distance={:.12}", site1, d1);
        println!("site2:{:?} distance={:.12}", site2, d2);
        println!("site3:{:?}, distance={:.12}", site3, d3);
        println!("there were no two point vertex!");
    }
}

#[inline(always)]
pub(crate) fn pps<I: InputType, F: OutputType>(
    point1: Point<I>,
    point2: Point<I>,
    site3: &VSE::SiteEvent<I, F>,
    segment_index: SiteIndex,
) -> bool {
    #[allow(clippy::suspicious_operation_groupings)]
    if segment_index != SiteIndex::Two {
        let orient1 = orientation_predicate::eval_p::<I, F>(point1, point2, site3.point0());
        let orient2 = orientation_predicate::eval_p::<I, F>(point1, point2, site3.point1());
        if segment_index == SiteIndex::One && point1.x >= point2.x {
            if orient1 != Orientation::Right {
                return false;
            }
        } else if segment_index == SiteIndex::Three && point2.x >= point1.x {
            if orient2 != Orientation::Right {
                return false;
            }
        } else if orient1 != Orientation::Right && orient2 != Orientation::Right {
            return false;
        }
    } else {
        return (site3.point0() != point1) || (site3.point1() != point2);
    }
    true
}

#[inline(always)]
pub(crate) fn pss<I: InputType, F: OutputType>(
    point1: Point<I>,
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
            && orientation_predicate::eval_p::<I, F>(site2.point0(), point1, site3.point1())
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
    (site1.sorted_index() != site2.sorted_index()) && (site2.sorted_index() != site3.sorted_index())
}
