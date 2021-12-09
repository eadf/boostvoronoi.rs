// Boost.Polygon library detail/voronoi_predicates.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

use crate::circle_event::CircleEvent;
use crate::predicate::orientation_predicate::{self, Orientation};
use crate::predicate::{exact_circle_formation, robust_cross_product, ULPSX2};
use boostvoronoi_ext::robust_fpt as RF;
use crate::site_event as VSE;
use crate::{cast, geometry::Point, predicate::SiteIndex, t, tln, InputType, OutputType};

/// Lazy evaluation of point, point, point circle events
pub(crate) fn ppp<I: InputType, F: OutputType>(
    point1: Point<I>,
    point2: Point<I>,
    point3: Point<I>,
    mut c_event: CircleEvent,
) -> Option<CircleEvent> {
    let dif_x1 = cast::<I, f64>(point1.x) - cast::<I, f64>(point2.x);
    let dif_x2 = cast::<I, f64>(point2.x) - cast::<I, f64>(point3.x);
    let dif_y1 = cast::<I, f64>(point1.y) - cast::<I, f64>(point2.y);
    let dif_y2 = cast::<I, f64>(point2.y) - cast::<I, f64>(point3.y);
    let orientation = robust_cross_product::<i64, f64>(
        cast::<I, i64>(point1.x) - cast::<I, i64>(point2.x),
        cast::<I, i64>(point2.x) - cast::<I, i64>(point3.x),
        cast::<I, i64>(point1.y) - cast::<I, i64>(point2.y),
        cast::<I, i64>(point2.y) - cast::<I, i64>(point3.y),
    );
    let inv_orientation: RF::RobustFpt = RF::RobustFpt::new(
        cast::<f32, f64>(0.5f32) / orientation,
        cast::<f32, f64>(2.0f32),
    );
    let sum_x1: f64 = cast::<I, f64>(point1.x) + cast::<I, f64>(point2.x);
    let sum_x2: f64 = cast::<I, f64>(point2.x) + cast::<I, f64>(point3.x);
    let sum_y1: f64 = cast::<I, f64>(point1.y) + cast::<I, f64>(point2.y);
    let sum_y2: f64 = cast::<I, f64>(point2.y) + cast::<I, f64>(point3.y);
    let dif_x3: f64 = cast::<I, f64>(point1.x) - cast::<I, f64>(point3.x);
    let dif_y3: f64 = cast::<I, f64>(point1.y) - cast::<I, f64>(point3.y);
    let mut c_x = RF::RobustDif::default();
    let mut c_y = RF::RobustDif::default();
    let error = 2_f64;
    c_x += RF::RobustFpt::new(dif_x1 * sum_x1 * dif_y2, error);
    c_x += RF::RobustFpt::new(dif_y1 * sum_y1 * dif_y2, error);
    c_x -= RF::RobustFpt::new(dif_x2 * sum_x2 * dif_y1, error);
    c_x -= RF::RobustFpt::new(dif_y2 * sum_y2 * dif_y1, error);
    c_y += RF::RobustFpt::new(dif_x2 * sum_x2 * dif_x1, error);
    c_y += RF::RobustFpt::new(dif_y2 * sum_y2 * dif_x1, error);
    c_y -= RF::RobustFpt::new(dif_x1 * sum_x1 * dif_x2, error);
    c_y -= RF::RobustFpt::new(dif_y1 * sum_y1 * dif_x2, error);
    let mut lower_x = c_x;
    lower_x -= RF::RobustFpt::new(
        ((dif_x1 * dif_x1 + dif_y1 * dif_y1)
            * (dif_x2 * dif_x2 + dif_y2 * dif_y2)
            * (dif_x3 * dif_x3 + dif_y3 * dif_y3))
            .sqrt(),
        cast::<f32, f64>(5.0f32),
    );

    c_event.set_3(
        c_x.dif().fpv() * inv_orientation.fpv(),
        c_y.dif().fpv() * inv_orientation.fpv(),
        lower_x.dif().fpv() * inv_orientation.fpv(),
    );
    let ulps = ULPSX2 as f64;
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
        exact_circle_formation::ppp::<I, F>(
            point1,
            point2,
            point3,
            &mut c_event,
            recompute_c_x,
            recompute_c_y,
            recompute_lower_x,
        );
    }
    Some(c_event)
}

/// Lazy evaluation of point, point, segment circle events
pub(crate) fn pps<I: InputType, F: OutputType>(
    point1: Point<I>,
    point2: Point<I>,
    site3: &VSE::SiteEvent<I, F>,
    segment_index: SiteIndex,
    mut c_event: CircleEvent,
) -> Option<CircleEvent> {
    tln!(
        "->LazyCircleFormationFunctor::pps(site1:{:?}, site2:{:?}, site3:{:?}, segment_index:{:?})",
        point1,
        point2,
        site3,
        segment_index
    );

    // (line_a,line_b) it the perpendicular vector of site3-point0 -> site3-point1
    let line_a = cast::<I, f64>(site3.y1()) - cast::<I, f64>(site3.y0());
    let line_b = cast::<I, f64>(site3.x0()) - cast::<I, f64>(site3.x1());
    // (vec_x,vec_y) it the perpendicular vector of site1->site2
    // t*(vec_x,vec_y) + midpoint(site1->site2) is our circle event position
    let vec_x = cast::<I, f64>(point2.y) - cast::<I, f64>(point1.y);
    let vec_y = cast::<I, f64>(point1.x) - cast::<I, f64>(point2.x);

    let teta = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
            cast::<I, i64>(site3.y1()) - cast::<I, i64>(site3.y0()),
            cast::<I, i64>(site3.x0()) - cast::<I, i64>(site3.x1()),
            cast::<I, i64>(point2.x) - cast::<I, i64>(point1.x),
            cast::<I, i64>(point2.y) - cast::<I, i64>(point1.y),
        ),
        1_f64,
    );
    let a = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
            cast::<I, i64>(site3.y0()) - cast::<I, i64>(site3.y1()),
            cast::<I, i64>(site3.x0()) - cast::<I, i64>(site3.x1()),
            cast::<I, i64>(site3.y1()) - cast::<I, i64>(point1.y),
            cast::<I, i64>(site3.x1()) - cast::<I, i64>(point1.x),
        ),
        1_f64,
    );
    let b = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
            cast::<I, i64>(site3.y0()) - cast::<I, i64>(site3.y1()),
            cast::<I, i64>(site3.x0()) - cast::<I, i64>(site3.x1()),
            cast::<I, i64>(site3.y1()) - cast::<I, i64>(point2.y),
            cast::<I, i64>(site3.x1()) - cast::<I, i64>(point2.x),
        ),
        1_f64,
    );
    let denom = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
            cast::<I, i64>(point1.y) - cast::<I, i64>(point2.y),
            cast::<I, i64>(point1.x) - cast::<I, i64>(point2.x),
            cast::<I, i64>(site3.y1()) - cast::<I, i64>(site3.y0()),
            cast::<I, i64>(site3.x1()) - cast::<I, i64>(site3.x0()),
        ),
        1_f64,
    );
    let inv_segm_len =
        RF::RobustFpt::new(1_f64 / (line_a * line_a + line_b * line_b).sqrt(), 3_f64);
    let mut t = RF::RobustDif::default();
    //tln!("0t:{:?}", t);
    if orientation_predicate::eval_f::<I, F>(denom.fpv()) == Orientation::Collinear {
        t += teta / (RF::RobustFpt::from(8_f64) * a);
        //tln!("1t:{:?}", t);
        t -= a / (RF::RobustFpt::from(2_f64) * teta);
        //tln!("2t:{:?}", t);
    } else {
        let det = ((teta * teta + denom * denom) * a * b).sqrt();
        //tln!("det:{:?}", det);
        if segment_index == SiteIndex::Two {
            //tln!("3 det:{:?}", det);
            //tln!("3 denom:{:?}", denom);
            //tln!("3 det/denom:{:?}", det / (denom * denom));
            t -= det / (denom * denom);
            //tln!("3t:{:?}", t);
        } else {
            t += det / (denom * denom);
            //tln!("4t:{:?}", t);
        }
        //tln!("5teta:{:?}", teta);
        //tln!("A:{:?}", a);
        //tln!("B:{:?}", b);
        t += teta * (a + b) / (RF::RobustFpt::from(2_f64) * denom * denom);
        //tln!("5t:{:?}", t);
    }
    //tln!("6t:{:?}", t);
    let mut c_x = RF::RobustDif::default();
    tln!("0: c_x:{:?}", c_x);
    let mut c_y = RF::RobustDif::default();
    c_x += RF::RobustFpt::from(0.5 * (cast::<I, f64>(point1.x) + cast::<I, f64>(point2.x)));
    tln!("1: c_x:{:?}", c_x);
    c_x += t * RF::RobustFpt::from(vec_x);
    tln!("2: c_x:{:?}", c_x);
    c_y += RF::RobustFpt::from(0.5 * (cast::<I, f64>(point1.y) + cast::<I, f64>(point2.y)));
    c_y += t * RF::RobustFpt::from(vec_y);

    let mut r = RF::RobustDif::default();
    let mut lower_x = c_x;
    r -= RF::RobustFpt::from(line_a) * RF::RobustFpt::from(cast::<I, f64>(site3.x0()));
    r -= RF::RobustFpt::from(line_b) * RF::RobustFpt::from(cast::<I, f64>(site3.y0()));
    r += c_x * RF::RobustFpt::from(line_a);
    r += c_y * RF::RobustFpt::from(line_b);
    if r.positive().fpv() < r.negative().fpv() {
        r = -r;
    }
    lower_x += r * inv_segm_len;

    c_event.set_3(c_x.dif().fpv(), c_y.dif().fpv(), lower_x.dif().fpv());

    tln!("  c_x:{:?}, c_y:{:?}, l_x:{:?}", c_x, c_y, lower_x);

    let ulps = ULPSX2 as f64;
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
        exact_circle_formation::pps::<I, F>(
            point1,
            point2,
            site3,
            segment_index,
            &mut c_event,
            recompute_c_x,
            recompute_c_y,
            recompute_lower_x,
        );
    }
    // All sites must be unique, or the dot calculation will be invalid
    let unique_endpoints = !(
        point1 == point2
            || site3.point0() == point1
            || site3.point0() == point2
            || site3.point1() == point1
            || site3.point1() == point2
        //|| site3.point0() == site3.point1() this can never happen
    );
    tln!("pps unique_endpoints:{}", unique_endpoints);

    if unique_endpoints {
        // site3.point0 -> c
        let v_3_c = (
            c_event.x() - cast::<I, f64>(site3.point0().x),
            c_event.y() - cast::<I, f64>(site3.point0().y),
        );
        // site3.point0 -> site3.point1
        let v_3 = (
            cast::<I, f64>(site3.point1().x) - cast::<I, f64>(site3.point0().x),
            cast::<I, f64>(site3.point1().y) - cast::<I, f64>(site3.point0().y),
        );
        #[allow(clippy::suspicious_operation_groupings)]
        let dot = (v_3_c.0 * v_3.0 + v_3_c.1 * v_3.1) / (v_3.0 * v_3.0 + v_3.1 * v_3.1);
        tln!("pps dot:{:.12}", dot);

        // allow the dot to be [0..1] + some ULP fuzz
        let rv =
            (-0.0..=1.0).contains(&dot) || approx::ulps_eq!(0.0, dot) || approx::ulps_eq!(1.0, dot);

        #[cfg(feature = "ce_corruption_check")]
        if !rv {
            println!("\n->LazyCircleFormationFunctor::pps(site1:{:?}, site2:{:?}, site3:{:?}, segment_index:{:?})", point1, point2, site3, segment_index);

            println!("let site1=[{},{}];", point1.x, point1.y);
            println!("let site2=[{},{}];", point2.x, point2.y);
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

            println!(
                "site1->c distance:{:-12}",
                point1.distance_to_point(c_event.x(), c_event.y())
            );
            println!(
                "site2->c distance:{:-12}",
                point2.distance_to_point(c_event.x(), c_event.y())
            );
            println!(
                "site3->c distance:{:-12}",
                site3.distance_to_point(c_event.x(), c_event.y())
            );

            println!("v_a_c:{:?}, v3:{:?}", v_3_c, v_3);
            println!("dot:{:?}", dot);
            println!("ignoring this CE\n");
        }
        return rv.then(|| c_event);
    };
    Some(c_event)
}

/// Lazy evaluation of point, segment, segment circle events
#[allow(unused_parens)]
pub(crate) fn pss<I: InputType, F: OutputType>(
    point1: Point<I>,
    site2: &VSE::SiteEvent<I, F>,
    site3: &VSE::SiteEvent<I, F>,
    point_index: SiteIndex,
    mut c_event: CircleEvent,
) -> Option<CircleEvent> {
    let segm_start1 = site2.point1();
    let segm_end1 = site2.point0();
    let segm_start2 = site3.point0();
    let segm_end2 = site3.point1();
    tln!(
        "->LazyCircleFormationFunctor::pss(site1:{:?}, site2:{:?}, site3:{:?}, point_index:{:?})",
        point1,
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
    if (point1 == site2.point0() || point1 == site2.point1())
        && (point1 == site3.point0() || point1 == site3.point1())
    {
        c_event.set_is_site_point();
        let x = cast::<I, f64>(point1.x);
        let y = cast::<I, f64>(point1.y);
        c_event.set_3(x, y, x);
        tln!("<-LazyCircleFormationFunctor::pss shortcut");
        return Some(c_event);
    }

    let a1 = cast::<I, f64>(segm_end1.x) - cast::<I, f64>(segm_start1.x);
    let b1 = cast::<I, f64>(segm_end1.y) - cast::<I, f64>(segm_start1.y);
    let a2 = cast::<I, f64>(segm_end2.x) - cast::<I, f64>(segm_start2.x);
    let b2 = cast::<I, f64>(segm_end2.y) - cast::<I, f64>(segm_start2.y);
    let recompute_c_x: bool;
    let recompute_c_y: bool;
    let recompute_lower_x: bool;

    let orientation = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
            cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
            cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
            cast::<I, i64>(segm_end2.y) - cast::<I, i64>(segm_start2.y),
            cast::<I, i64>(segm_end2.x) - cast::<I, i64>(segm_start2.x),
        ),
        1_f64,
    );
    let is_collinear =
        orientation_predicate::eval_f::<I, F>(orientation.fpv()) == Orientation::Collinear;
    if is_collinear {
        tln!("  LazyCircleFormationFunctor::pss collinear");
        let a = RF::RobustFpt::new(a1 * a1 + b1 * b1, 2_f64);
        let c = RF::RobustFpt::new(
            robust_cross_product::<i64, f64>(
                cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                cast::<I, i64>(segm_start2.y) - cast::<I, i64>(segm_start1.y),
                cast::<I, i64>(segm_start2.x) - cast::<I, i64>(segm_start1.x),
            ),
            1_f64,
        );
        let det = RF::RobustFpt::new(
            robust_cross_product::<i64, f64>(
                cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                cast::<I, i64>(point1.x) - cast::<I, i64>(segm_start1.x),
                cast::<I, i64>(point1.y) - cast::<I, i64>(segm_start1.y),
            ) * robust_cross_product::<i64, f64>(
                cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                cast::<I, i64>(point1.y) - cast::<I, i64>(segm_start2.y),
                cast::<I, i64>(point1.x) - cast::<I, i64>(segm_start2.x),
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
        t -= RF::RobustFpt::from(a1)
            * RF::RobustFpt::from(
                (cast::<I, f64>(segm_start1.x) + cast::<I, f64>(segm_start2.x)) * 0.5
                    - cast::<I, f64>(point1.x),
            );
        t -= RF::RobustFpt::from(b1)
            * RF::RobustFpt::from(
                (cast::<I, f64>(segm_start1.y) + cast::<I, f64>(segm_start2.y)) * 0.5
                    - cast::<I, f64>(point1.y),
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
        c_x += RF::RobustFpt::from(
            0.5 * (cast::<I, f64>(segm_start1.x) + cast::<I, f64>(segm_start2.x)),
        );
        //tln!("ulps1: x:{:.12}, y:{:.12}", c_x.dif().fpv(), c_y.dif().fpv());
        //tln!("ulps1.5: 1:{:.12}, 2:{:.12}", RF::RobustFpt::from(a1).fpv(), t.dif().fpv());
        //tln!("ulps1.6: 1:{:.12}", (t*RF::RobustFpt::from(a1)).dif().fpv());
        c_x += t * RF::RobustFpt::from(a1);
        c_y += RF::RobustFpt::from(
            0.5 * (cast::<I, f64>(segm_start1.y) + cast::<I, f64>(segm_start2.y)),
        );
        //tln!("ulps2: x:{:.12}, y:{:.12}", c_x.dif().fpv(), c_y.dif().fpv());
        c_y += t * RF::RobustFpt::from(b1);
        //tln!("ulps3: x:{:.12}, y:{:.12}", c_x.dif().fpv(), c_y.dif().fpv());
        let mut lower_x = c_x;
        if c.is_neg() {
            lower_x -= RF::RobustFpt::from(0.5) * c / a.sqrt();
        } else {
            lower_x += RF::RobustFpt::from(0.5) * c / a.sqrt();
        }
        let ulps = ULPSX2 as f64;
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
        c_event.set_3(c_x.dif().fpv(), c_y.dif().fpv(), lower_x.dif().fpv());
    } else {
        tln!("  LazyCircleFormationFunctor::pss !collinear");
        let sqr_sum1 = RF::RobustFpt::new((a1 * a1 + b1 * b1).sqrt(), 2_f64);
        let sqr_sum2 = RF::RobustFpt::new((a2 * a2 + b2 * b2).sqrt(), 2_f64);
        let mut a = RF::RobustFpt::new(
            robust_cross_product::<i64, f64>(
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
        let or1 = RF::RobustFpt::new(
            robust_cross_product::<i64, f64>(
                cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                cast::<I, i64>(segm_end1.y) - cast::<I, i64>(point1.y),
                cast::<I, i64>(segm_end1.x) - cast::<I, i64>(point1.x),
            ),
            1_f64,
        );
        let or2 = RF::RobustFpt::new(
            robust_cross_product::<i64, f64>(
                cast::<I, i64>(segm_end2.x) - cast::<I, i64>(segm_start2.x),
                cast::<I, i64>(segm_end2.y) - cast::<I, i64>(segm_start2.y),
                cast::<I, i64>(segm_end2.x) - cast::<I, i64>(point1.x),
                cast::<I, i64>(segm_end2.y) - cast::<I, i64>(point1.y),
            ),
            1_f64,
        );
        let det = RF::RobustFpt::from(2_f64) * a * or1 * or2;
        let c1 = RF::RobustFpt::new(
            robust_cross_product::<i64, f64>(
                cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                cast::<I, i64>(segm_end1.y),
                cast::<I, i64>(segm_end1.x),
            ),
            1_f64,
        );
        let c2 = RF::RobustFpt::new(
            robust_cross_product::<i64, f64>(
                cast::<I, i64>(segm_end2.x) - cast::<I, i64>(segm_start2.x),
                cast::<I, i64>(segm_end2.y) - cast::<I, i64>(segm_start2.y),
                cast::<I, i64>(segm_end2.x),
                cast::<I, i64>(segm_end2.y),
            ),
            1_f64,
        );
        let inv_orientation = RF::RobustFpt::from(1_f64) / orientation;
        let mut t = RF::RobustDif::default();
        tln!("0: t:{:?}", t);
        let mut b = RF::RobustDif::default();
        tln!("0: b:{:?}", b);
        let mut ix = RF::RobustDif::default();
        let mut iy = RF::RobustDif::default();

        ix += RF::RobustFpt::from(a2) * c1 * inv_orientation;
        ix += RF::RobustFpt::from(a1) * c2 * inv_orientation;
        iy += RF::RobustFpt::from(b1) * c2 * inv_orientation;
        iy += RF::RobustFpt::from(b2) * c1 * inv_orientation;
        tln!("1: ix:{:?}", ix);
        tln!("1: s:{:?}", RF::RobustFpt::from(a1) * sqr_sum2);
        tln!("1: p:{:?}", ix * (RF::RobustFpt::from(a1) * sqr_sum2));
        b += ix * (RF::RobustFpt::from(a1) * sqr_sum2);
        tln!("1: b:{:?}", b);
        b += ix * (RF::RobustFpt::from(a2) * sqr_sum1);
        tln!("2: b:{:?}", b);
        b += iy * (RF::RobustFpt::from(b1) * sqr_sum2);
        tln!("3: b:{:?}", b);
        b += iy * (RF::RobustFpt::from(b2) * sqr_sum1);
        tln!("4: b:{:?}", b);
        b -= sqr_sum1
            * RF::RobustFpt::new(
                robust_cross_product::<i64, f64>(
                    cast::<I, i64>(segm_end2.x) - cast::<I, i64>(segm_start2.x),
                    cast::<I, i64>(segm_end2.y) - cast::<I, i64>(segm_start2.y),
                    -cast::<I, i64>(point1.y),
                    cast::<I, i64>(point1.x),
                ),
                1_f64,
            );
        tln!("5: b:{:?}", b);
        b -= sqr_sum2
            * RF::RobustFpt::new(
                robust_cross_product::<i64, f64>(
                    cast::<I, i64>(segm_end1.x) - cast::<I, i64>(segm_start1.x),
                    cast::<I, i64>(segm_end1.y) - cast::<I, i64>(segm_start1.y),
                    -cast::<I, i64>(point1.y),
                    cast::<I, i64>(point1.x),
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
        let mut c_x = ix;
        let mut c_y = iy;
        tln!("0: c_x:{:?}", c_x);
        tln!("0: t:{:?}", t);
        c_x += t * (RF::RobustFpt::from(a1) * sqr_sum2);
        tln!("1: c_x:{:?}", c_x);
        c_x += t * (RF::RobustFpt::from(a2) * sqr_sum1);
        tln!("2: c_x:{:?}", c_x);
        c_y += t * (RF::RobustFpt::from(b1) * sqr_sum2);
        c_y += t * (RF::RobustFpt::from(b2) * sqr_sum1);

        if t.positive().fpv() < t.negative().fpv() {
            t = -t;
        }
        let mut lower_x = c_x;
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

        let ulps = ULPSX2 as f64;
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
        c_event.set_3(c_x.dif().fpv(), c_y.dif().fpv(), lower_x.dif().fpv());
    }

    if recompute_c_x || recompute_c_y || recompute_lower_x {
        exact_circle_formation::pss(
            point1,
            site2,
            site3,
            point_index,
            &mut c_event,
            recompute_c_x,
            recompute_c_y,
            recompute_lower_x,
        );
    }
    Some(c_event)
}

/// Lazy evaluation of segment, segment, segment circle events
pub(crate) fn sss<I: InputType, F: OutputType>(
    site1: &VSE::SiteEvent<I, F>,
    site2: &VSE::SiteEvent<I, F>,
    site3: &VSE::SiteEvent<I, F>,
    mut c_event: CircleEvent,
) -> Option<CircleEvent> {
    let a1 = RF::RobustFpt::from(cast::<I, f64>(site1.x1()) - cast::<I, f64>(site1.x0()));
    let b1 = RF::RobustFpt::from(cast::<I, f64>(site1.y1()) - cast::<I, f64>(site1.y0()));
    let c1 = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
            cast::<I, i64>(site1.x0()),
            cast::<I, i64>(site1.y0()),
            cast::<I, i64>(site1.x1()),
            cast::<I, i64>(site1.y1()),
        ),
        1_f64,
    );

    let a2 = RF::RobustFpt::from(cast::<I, f64>(site2.x1()) - cast::<I, f64>(site2.x0()));
    let b2 = RF::RobustFpt::from(cast::<I, f64>(site2.y1()) - cast::<I, f64>(site2.y0()));
    let c2 = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
            cast::<I, i64>(site2.x0()),
            cast::<I, i64>(site2.y0()),
            cast::<I, i64>(site2.x1()),
            cast::<I, i64>(site2.y1()),
        ),
        1_f64,
    );

    let a3 = RF::RobustFpt::from(cast::<I, f64>(site3.x1()) - cast::<I, f64>(site3.x0()));
    let b3 = RF::RobustFpt::from(cast::<I, f64>(site3.y1()) - cast::<I, f64>(site3.y0()));
    let c3 = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
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
    let cross_12 = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
            cast::<I, i64>(site1.x1()) - cast::<I, i64>(site1.x0()),
            cast::<I, i64>(site1.y1()) - cast::<I, i64>(site1.y0()),
            cast::<I, i64>(site2.x1()) - cast::<I, i64>(site2.x0()),
            cast::<I, i64>(site2.y1()) - cast::<I, i64>(site2.y0()),
        ),
        1_f64,
    );
    let cross_23 = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
            cast::<I, i64>(site2.x1()) - cast::<I, i64>(site2.x0()),
            cast::<I, i64>(site2.y1()) - cast::<I, i64>(site2.y0()),
            cast::<I, i64>(site3.x1()) - cast::<I, i64>(site3.x0()),
            cast::<I, i64>(site3.y1()) - cast::<I, i64>(site3.y0()),
        ),
        1_f64,
    );
    let cross_31 = RF::RobustFpt::new(
        robust_cross_product::<i64, f64>(
            cast::<I, i64>(site3.x1()) - cast::<I, i64>(site3.x0()),
            cast::<I, i64>(site3.y1()) - cast::<I, i64>(site3.y0()),
            cast::<I, i64>(site1.x1()) - cast::<I, i64>(site1.x0()),
            cast::<I, i64>(site1.y1()) - cast::<I, i64>(site1.y0()),
        ),
        1_f64,
    );

    // denom = cross_12 * len3 + cross_23 * len1 + cross_31 * len2.
    let mut denom = RF::RobustDif::default();
    denom += cross_12 * len3;
    denom += cross_23 * len1;
    denom += cross_31 * len2;

    // denom * r = (b2 * c_x - a2 * c_y - c2 * denom) / len2.
    let mut r = RF::RobustDif::default();
    r -= cross_12 * c3;
    r -= cross_23 * c1;
    r -= cross_31 * c2;

    let mut c_x = RF::RobustDif::default();
    c_x += a1 * c2 * len3;
    c_x -= a2 * c1 * len3;
    c_x += a2 * c3 * len1;
    c_x -= a3 * c2 * len1;
    c_x += a3 * c1 * len2;
    c_x -= a1 * c3 * len2;

    let mut c_y = RF::RobustDif::default();
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

    let ulps = ULPSX2 as f64;
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
    c_event.set_3(c_x_dif.fpv(), c_y_dif.fpv(), lower_x_dif.fpv());

    if recompute_c_x || recompute_c_y || recompute_lower_x {
        exact_circle_formation::sss(
            site1,
            site2,
            site3,
            &mut c_event,
            recompute_c_x,
            recompute_c_y,
            recompute_lower_x,
        );
    }

    tln!("<-LazyCircleFormationFunctor::sss(");
    tln!("  site1:{:?}", site1);
    tln!("  site2:{:?}", site2);
    tln!("  site3:{:?}", site3);
    tln!("  c_event:{:?}", c_event);

    Some(c_event)
}
