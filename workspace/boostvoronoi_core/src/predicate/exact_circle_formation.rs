// Boost.Polygon library detail/voronoi_predicates.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! Evaluation of circle events using high-precision library.

use crate::circle_event::CircleEvent;
use crate::robust_sqrt_expr as RF;
use crate::site_event as VSE;
use crate::{geometry::Point, predicate::SiteIndex, t, tln, InputType, OutputType};
use boostvoronoi_ext::extended_exp_fpt as EX;
use boostvoronoi_ext::extended_int::ExtendedInt;
use num_traits::{One, Zero};

/// Recompute parameters of the point, point, point circle event using high-precision library.
pub(crate) fn ppp<I: InputType, F: OutputType>(
    point1: Point<I>,
    point2: Point<I>,
    point3: Point<I>,
    circle: &mut CircleEvent,
    recompute_c_x: bool,
    recompute_c_y: bool,
    recompute_lower_x: bool,
) {
    let dif_x = [
        ExtendedInt::from(point1.x) - ExtendedInt::from(point2.x),
        ExtendedInt::from(point2.x) - ExtendedInt::from(point3.x),
        ExtendedInt::from(point1.x) - ExtendedInt::from(point3.x),
    ];

    let dif_y = [
        ExtendedInt::from(point1.y) - ExtendedInt::from(point2.y),
        ExtendedInt::from(point2.y) - ExtendedInt::from(point3.y),
        ExtendedInt::from(point1.y) - ExtendedInt::from(point3.y),
    ];

    let sum_x = [
        ExtendedInt::from(point1.x) + ExtendedInt::from(point2.x),
        ExtendedInt::from(point2.x) + ExtendedInt::from(point3.x),
    ];
    let sum_y = [
        ExtendedInt::from(point1.y) + ExtendedInt::from(point2.y),
        ExtendedInt::from(point2.y) + ExtendedInt::from(point3.y),
    ];

    let inv_denom = {
        let tmp = &dif_x[0] * &dif_y[1] - &dif_x[1] * &dif_y[0];
        EX::ExtendedExponentFpt::<f64>::from(0.5) / EX::ExtendedExponentFpt::from(tmp)
    };
    let numer1: ExtendedInt = &dif_x[0] * &sum_x[0] + &dif_y[0] * &sum_y[0];
    let numer2: ExtendedInt = &dif_x[1] * &sum_x[1] + &dif_y[1] * &sum_y[1];

    if recompute_c_x || recompute_lower_x {
        let c_x: ExtendedInt = &numer1 * &dif_y[1] - &numer2 * &dif_y[0];
        circle.set_x_xf(EX::ExtendedExponentFpt::from(&c_x) * inv_denom);

        if recompute_lower_x {
            // Evaluate radius of the circle.
            let sqr_r: ExtendedInt = (&dif_x[0] * &dif_x[0] + &dif_y[0] * &dif_y[0])
                * (&dif_x[1] * &dif_x[1] + &dif_y[1] * &dif_y[1])
                * (&dif_x[2] * &dif_x[2] + &dif_y[2] * &dif_y[2]);
            let r = EX::ExtendedExponentFpt::from(&sqr_r).sqrt();

            // If c_x >= 0 then lower_x = c_x + r,
            // else lower_x = (c_x * c_x - r * r) / (c_x - r).
            // To guarantee epsilon relative error.

            // this value will be invalid after call to set_lower_x()
            let tmp_circle_x = circle.x_as_xf();

            if !tmp_circle_x.is_neg() {
                if !inv_denom.is_neg() {
                    circle.set_lower_x_xf(tmp_circle_x + r * inv_denom);
                } else {
                    circle.set_lower_x_xf(tmp_circle_x - r * inv_denom);
                }
            } else {
                let numer: ExtendedInt = &c_x * &c_x - &sqr_r;
                let lower_x = EX::ExtendedExponentFpt::from(numer) * inv_denom
                    / (EX::ExtendedExponentFpt::from(c_x) + r);
                circle.set_lower_x_xf(lower_x);
            }
        }
    }

    if recompute_c_y {
        let c_y: ExtendedInt = &numer2 * &dif_x[0] - &numer1 * &dif_x[1];
        circle.set_y_xf(EX::ExtendedExponentFpt::from(c_y) * inv_denom);
    }
    #[cfg(feature = "console_debug")]
    {
        tln!(
            "ppp(x:{:.12}, y:{:.12}, lx:{:.12})",
            circle.x(),
            circle.y(),
            circle.lower_x()
        );
    }
}

/// Recompute parameters of the point, point, segment circle event using high-precision library.
#[allow(clippy::too_many_arguments)]
pub(crate) fn pps<I: InputType, F: OutputType>(
    point1: Point<I>,
    point2: Point<I>,
    site3: &VSE::SiteEvent<I, F>,
    segment_index: SiteIndex,
    c_event: &mut CircleEvent,
    recompute_c_x: bool,
    recompute_c_y: bool,
    recompute_lower_x: bool,
) {
    tln!(
        "->pps site1:{:?} site2:{:?} site3:{:?}",
        point1,
        point2,
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

    // Todo: is 5 the correct size?
    let mut ca: [ExtendedInt; 5] = [
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
    ];
    let mut cb: [ExtendedInt; 5] = [
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
    ];
    let line_a = ExtendedInt::from(site3.y1()) - ExtendedInt::from(site3.y0());
    let line_b = ExtendedInt::from(site3.x0()) - ExtendedInt::from(site3.x1());
    let segm_len = &line_a * &line_a + &line_b * &line_b;
    let vec_x = ExtendedInt::from(point2.y) - ExtendedInt::from(point1.y);
    let vec_y = ExtendedInt::from(point1.x) - ExtendedInt::from(point2.x);
    let sum_x = ExtendedInt::from(point1.x) + ExtendedInt::from(point2.x);
    let sum_y = ExtendedInt::from(point1.y) + ExtendedInt::from(point2.y);
    let teta: ExtendedInt = &line_a * &vec_x + &line_b * &vec_y;
    let mut denom: ExtendedInt = &vec_x * &line_b - &vec_y * &line_a;

    let mut dif0 = ExtendedInt::from(site3.y1()) - ExtendedInt::from(point1.y);
    let mut dif1 = ExtendedInt::from(point1.x) - ExtendedInt::from(site3.x1());
    let a: ExtendedInt = &line_a * &dif1 - &line_b * &dif0;

    dif0 = ExtendedInt::from(site3.y1()) - ExtendedInt::from(point2.y);
    dif1 = ExtendedInt::from(point2.x) - ExtendedInt::from(site3.x1());
    let b = line_a * dif1 - line_b * dif0;
    let sum_ab = &a + &b;
    tln!("a:{:?} b:{:?} denom:{:?}", a, b, denom);

    if denom.is_zero() {
        let numer: ExtendedInt = &teta * &teta - &sum_ab * &sum_ab;
        denom = &teta * &sum_ab;
        ca[0] = &denom * &sum_x * 2 + &numer * &vec_x;
        cb[0] = segm_len.clone();
        ca[1] = &denom * &sum_ab * 2 + &numer * &teta;
        cb[1] = ExtendedInt::one();
        ca[2] = &denom * &sum_y * 2 + &numer * &vec_y;
        let inv_denom = EX::ExtendedExponentFpt::from(1f64) / EX::ExtendedExponentFpt::from(&denom);
        if recompute_c_x {
            c_event.set_x_xf(EX::ExtendedExponentFpt::from(&ca[0]) * inv_denom / 4_f64);
        }
        if recompute_c_y {
            c_event.set_y_xf(EX::ExtendedExponentFpt::from(&ca[2]) * inv_denom / 4_f64);
        }
        if recompute_lower_x {
            c_event.set_lower_x_xf(
                RF::eval2(&ca, &cb) * inv_denom * 0.25f64
                    / (EX::ExtendedExponentFpt::from(&segm_len).sqrt()),
            );
        }
        return;
    }
    let det: ExtendedInt = (&teta * &teta + &denom * &denom) * &a * &b * 4;
    let mut inv_denom_sqr =
        EX::ExtendedExponentFpt::from(1f64) / EX::ExtendedExponentFpt::from(&denom);
    inv_denom_sqr = inv_denom_sqr * inv_denom_sqr;
    tln!("det:{:?} inv_denom_sqr:{:.12}", det, inv_denom_sqr.d());

    if recompute_c_x || recompute_lower_x {
        ca[0] = sum_x * &denom * &denom + &teta * &sum_ab * &vec_x;
        cb[0] = ExtendedInt::from(1_i32);
        ca[1] = if segment_index == SiteIndex::Two {
            -vec_x
        } else {
            vec_x
        };
        cb[1] = det.clone();
        if recompute_c_x {
            c_event.set_x_xf(RF::eval2(&ca, &cb) * inv_denom_sqr * 0.5f64);
        }
    }

    if recompute_c_y || recompute_lower_x {
        ca[2] = sum_y * &denom * &denom + &teta * &sum_ab * &vec_y;
        cb[2] = ExtendedInt::one();
        ca[3] = if segment_index == SiteIndex::Two {
            -vec_y
        } else {
            vec_y
        };
        cb[3] = det.clone();
        if recompute_c_y {
            c_event.set_y_xf(RF::eval2(&ca[2..], &cb[2..]) * inv_denom_sqr * 0.5f64);
        }
    }

    if recompute_lower_x {
        cb[0] = &cb[0] * &segm_len;
        cb[1] = &cb[1] * &segm_len;
        ca[2] = sum_ab * (&denom * &denom + &teta * &teta);
        cb[2] = ExtendedInt::one();
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

        let eval4 = RF::eval4(&ca, &cb);
        tln!("eval4:{:.12}", eval4.d());

        c_event.set_lower_x_xf(eval4 * inv_denom_sqr * 0.5f64 / segm_len);
    }
    #[cfg(feature = "console_debug")]
    {
        tln!(
            "<-pps(x:{:.12}, y:{:.12}, lx:{:.12})",
            c_event.x(),
            c_event.y(),
            c_event.lower_x()
        );
    }
}

/// Recompute parameters of the point, segment, segment circle event using high-precision library.
#[allow(non_snake_case)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn pss<I: InputType, F: OutputType>(
    point1: Point<I>,
    site2: &VSE::SiteEvent<I, F>,
    site3: &VSE::SiteEvent<I, F>,
    point_index: SiteIndex,
    c_event: &mut CircleEvent,
    recompute_c_x: bool,
    recompute_c_y: bool,
    recompute_lower_x: bool,
) {
    let mut c: [ExtendedInt; 2] = [ExtendedInt::zero(), ExtendedInt::zero()];
    let mut cA: [ExtendedInt; 4] = [
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
    ];
    let mut cB: [ExtendedInt; 4] = [
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
    ];

    let segm_start1 = site2.point1();
    let segm_end1 = site2.point0();
    let segm_start2 = site3.point0();
    let segm_end2 = site3.point1();
    let a: [ExtendedInt; 2] = [
        ExtendedInt::from(segm_end1.x) - ExtendedInt::from(segm_start1.x),
        ExtendedInt::from(segm_end2.x) - ExtendedInt::from(segm_start2.x),
    ];

    let b: [ExtendedInt; 2] = [
        ExtendedInt::from(segm_end1.y) - ExtendedInt::from(segm_start1.y),
        ExtendedInt::from(segm_end2.y) - ExtendedInt::from(segm_start2.y),
    ];
    tln!("->ExactCircleFormationFunctor:pss");
    tln!(" a[0]={:?}", a[0]);
    tln!(" a[1]={:?}", a[1]);
    tln!(" b[0]={:?}", b[0]);
    tln!(" b[1]={:?}", b[1]);
    tln!(" recompute_c_x:{}", recompute_c_x);
    tln!(" recompute_c_y:{}", recompute_c_y);
    tln!(" recompute_lower_x:{}", recompute_lower_x);

    let orientation: ExtendedInt = &a[1] * &b[0] - &a[0] * &b[1];
    tln!(" orientation={:?}", orientation);

    if orientation.is_zero() {
        let denom = EX::ExtendedExponentFpt::from(
            ExtendedInt::from(2_i32) * (&a[0] * &a[0] + &b[0] * &b[0]),
        );

        c[0] = (ExtendedInt::from(segm_start2.x) - ExtendedInt::from(segm_start1.x)) * &b[0]
            - (ExtendedInt::from(segm_start2.y) - ExtendedInt::from(segm_start1.y)) * &a[0];
        let dx: ExtendedInt = (ExtendedInt::from(point1.y) - ExtendedInt::from(segm_start1.y))
            * &a[0]
            - (ExtendedInt::from(point1.x) - ExtendedInt::from(segm_start1.x)) * &b[0];
        let dy: ExtendedInt = (ExtendedInt::from(point1.x) - ExtendedInt::from(segm_start2.x))
            * &b[0]
            - (ExtendedInt::from(point1.y) - ExtendedInt::from(segm_start2.y)) * &a[0];
        cB[0] = dx * dy;
        cB[1] = ExtendedInt::one();

        if recompute_c_y {
            cA[0] = if point_index == SiteIndex::Two {
                ExtendedInt::from(2i32)
            } else {
                ExtendedInt::from(-2i32)
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
            cA[1] = (ExtendedInt::from(segm_start1.y) + ExtendedInt::from(segm_start2.y))
                * &a[0]
                * &a[0]
                - (ExtendedInt::from(segm_start1.x) + ExtendedInt::from(segm_start2.x)
                    - (ExtendedInt::from(point1.x) * ExtendedInt::from(2_i32)))
                    * &a[0]
                    * &b[0]
                + (ExtendedInt::from(point1.y) * ExtendedInt::from(2_i32)) * &b[0] * &b[0];
            tln!("cA[1]={:?}", cA[1]);
            let c_y = RF::eval2(&cA, &cB);
            tln!("c_y={:?}", c_y);
            tln!("denom={:?}", denom);
            c_event.set_y_xf(c_y / denom);
        }

        if recompute_c_x || recompute_lower_x {
            cA[0] = ExtendedInt::from(if point_index == SiteIndex::Two {
                2i32
            } else {
                -2i32
            }) * &a[0];
            cA[1] = (ExtendedInt::from(segm_start1.x) + ExtendedInt::from(segm_start2.x))
                * &b[0]
                * &b[0]
                - (ExtendedInt::from(segm_start1.y) + ExtendedInt::from(segm_start2.y)
                    - ExtendedInt::from(point1.y) * ExtendedInt::from(2_i32))
                    * &a[0]
                    * &b[0]
                + ExtendedInt::from(point1.x) * &a[0] * &a[0] * ExtendedInt::from(2_i32);
            tln!(" cA[0]={:.0}", cA[0].d());
            tln!(" cA[1]={:.0}", cA[1].d());

            if recompute_c_x {
                let c_x = RF::eval2(&cA, &cB);
                tln!(" c_x={:.0}", c_x.d());
                tln!(" denom={:.0}", denom.d());
                tln!(" c_x/denom={:.0}", (c_x / denom).d());

                c_event.set_x_xf(c_x / denom);
            }

            if recompute_lower_x {
                cA[2] = if c[0].is_neg() {
                    -(c[0].clone())
                } else {
                    c[0].clone()
                };
                cB[2] = &a[0] * &a[0] + &b[0] * &b[0];
                let lower_x = RF::eval3(&cA, &cB);
                c_event.set_lower_x_xf(lower_x / denom);
            }
        }
        return;
    }
    c[0] = ExtendedInt::from(segm_end1.x) * &b[0] - ExtendedInt::from(segm_end1.y) * &a[0];
    c[1] = ExtendedInt::from(segm_end2.y) * &a[1] - ExtendedInt::from(segm_end2.x) * &b[1];
    let ix: ExtendedInt = &a[0] * &c[1] + &a[1] * &c[0];
    let iy: ExtendedInt = &b[0] * &c[1] + &b[1] * &c[0];
    let dx: ExtendedInt = ix.clone() - ExtendedInt::from(point1.x) * &orientation;
    let dy: ExtendedInt = iy.clone() - ExtendedInt::from(point1.y) * &orientation;
    tln!(" ix={:?}", ix);
    tln!(" iy={:?}", iy);
    tln!(" dx={:?}", dx);
    tln!(" dy={:?}", dy);

    if dx.is_zero() && dy.is_zero() {
        let denom = EX::ExtendedExponentFpt::from(&orientation);
        let c_x = EX::ExtendedExponentFpt::from(&ix) / denom;
        let c_y = EX::ExtendedExponentFpt::from(&iy) / denom;
        c_event.set_3_ext(c_x, c_y, c_x);
        return;
    }

    let sign = ExtendedInt::from(
        if point_index == SiteIndex::Two { 1 } else { -1 }
            * if orientation.is_neg() { 1 } else { -1 },
    );
    tln!(" a[1]={:?}", &a[1]);
    tln!(" b[1]={:?}", &b[1]);
    tln!(" cA[0]={:?}", -(&a[1] * &dx));
    tln!(" cA[1]={:?}", -(&b[1] * &dy));

    cA[0] = (-(&a[1] * &dx)) - (&b[1] * &dy);
    cA[1] = (-(&a[0] * &dx)) - (&b[0] * &dy);
    cA[2] = sign.clone();
    cA[3] = ExtendedInt::zero();

    tln!(" cA[0]={:?}", cA[0]);
    tln!(" cA[1]={:?}", cA[1]);
    tln!(" cA[2]={:?}", cA[2]);
    tln!(" cA[3]={:?}", cA[3]);

    cB[0] = &a[0] * &a[0] + &b[0] * &b[0];
    cB[1] = &a[1] * &a[1] + &b[1] * &b[1];
    cB[2] = &a[0] * &a[1] + &b[0] * &b[1];
    cB[3] = ExtendedInt::from(-2_i32) * (&a[0] * &dy - &b[0] * &dx) * (&a[1] * &dy - &b[1] * &dx);
    let temp = RF::sqrt_expr_evaluator_pss4(&cA[0..], &cB[0..]);
    let denom = temp * EX::ExtendedExponentFpt::from(&orientation);

    if recompute_c_y {
        cA[0] = (&dx * &dx + &dy * &dy) * &b[1] - (&dx * &a[1] + &dy * &b[1]) * &iy;
        cA[1] = (&dx * &dx + &dy * &dy) * &b[0] - (&dx * &a[0] + &dy * &b[0]) * &iy;
        cA[2] = iy * &sign;
        let cy = RF::sqrt_expr_evaluator_pss4(&cA[0..], &cB[0..]);
        c_event.set_y_xf(cy / denom);
    }

    if recompute_c_x || recompute_lower_x {
        cA[0] = (&dx * &dx + &dy * &dy) * &a[1] - (&dx * &a[1] + &dy * &b[1]) * &ix;
        cA[1] = (&dx * &dx + &dy * &dy) * &a[0] - (&dx * &a[0] + &dy * &b[0]) * &ix;
        cA[2] = ix * &sign;

        if recompute_c_x {
            let cx = RF::sqrt_expr_evaluator_pss4(&cA, &cB);
            c_event.set_x_xf(cx / denom);
        }

        if recompute_lower_x {
            cA[3] = if temp.is_neg() {
                -orientation
            } else {
                orientation
            } * (&dx * &dx + &dy * &dy);
            let lower_x = RF::sqrt_expr_evaluator_pss4(&cA, &cB);
            c_event.set_lower_x_xf(lower_x / denom);
        }
    }
    #[cfg(feature = "console_debug")]
    {
        tln!(
            "pss(x:{:.12}, y:{:.12}, lx:{:.12})",
            c_event.x(),
            c_event.y(),
            c_event.lower_x()
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
pub(crate) fn sss<I: InputType, F: OutputType>(
    site1: &VSE::SiteEvent<I, F>,
    site2: &VSE::SiteEvent<I, F>,
    site3: &VSE::SiteEvent<I, F>,
    c_event: &mut CircleEvent,
    recompute_c_x: bool,
    recompute_c_y: bool,
    recompute_lower_x: bool,
) {
    tln!(">ExactCircleFormationFunctor:sss site1:{:?} site2:{:?}, site3:{:?}, recompute_c_x:{} recompute_c_y:{}, recompute_lower_x:{}",
            site1, site2, site3, recompute_c_x,recompute_c_y, recompute_lower_x);

    let mut cA: [ExtendedInt; 4] = [
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
    ];
    let mut cB: [ExtendedInt; 4] = [
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
        ExtendedInt::zero(),
    ];

    // cA - corresponds to the cross product.
    // cB - corresponds to the squared length.

    let a = [
        ExtendedInt::from(site1.x1()) - ExtendedInt::from(site1.x0()),
        ExtendedInt::from(site2.x1()) - ExtendedInt::from(site2.x0()),
        ExtendedInt::from(site3.x1()) - ExtendedInt::from(site3.x0()),
    ];
    let b = [
        ExtendedInt::from(site1.y1()) - ExtendedInt::from(site1.y0()),
        ExtendedInt::from(site2.y1()) - ExtendedInt::from(site2.y0()),
        ExtendedInt::from(site3.y1()) - ExtendedInt::from(site3.y0()),
    ];

    let c = [
        ExtendedInt::from(site1.x0()) * ExtendedInt::from(site1.y1())
            - ExtendedInt::from(site1.y0()) * ExtendedInt::from(site1.x1()),
        ExtendedInt::from(site2.x0()) * ExtendedInt::from(site2.y1())
            - ExtendedInt::from(site2.y0()) * ExtendedInt::from(site2.x1()),
        ExtendedInt::from(site3.x0()) * ExtendedInt::from(site3.y1())
            - ExtendedInt::from(site3.y0()) * ExtendedInt::from(site3.x1()),
    ];

    for (i, aa) in a.iter().enumerate().take(3) {
        cB[i] = aa.clone() * aa + &b[i] * &b[i];
    }
    for (i, cA_i) in cA.iter_mut().enumerate().take(3) {
        let j = (i + 1) % 3;
        let k = (i + 2) % 3;
        *cA_i = &a[j] * &b[k] - &a[k] * &b[j];
    }
    let denom = RF::eval3(&cA, &cB);

    if recompute_c_y {
        for (i, cA_i) in cA.iter_mut().enumerate().take(3) {
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;
            *cA_i = &b[j] * &c[k] - &b[k] * &c[j];
        }
        let c_y = RF::eval3(&cA, &cB);
        c_event.set_y_xf(c_y / denom);
    }

    if recompute_c_x || recompute_lower_x {
        cA[3] = ExtendedInt::zero();
        for i in 0..3 {
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;
            cA[i] = &a[j] * &c[k] - &a[k] * &c[j];
            if recompute_lower_x {
                cA[3] = &cA[3] + &(&cA[i] * &b[i]);
            }
        }

        if recompute_c_x {
            let c_x = RF::eval3(&cA, &cB);
            c_event.set_x_xf(c_x / denom);
        }

        if recompute_lower_x {
            cB[3] = ExtendedInt::one();
            let lower_x = RF::eval4(&cA, &cB);
            c_event.set_lower_x_xf(lower_x / denom);
        }
    }
    #[cfg(feature = "console_debug")]
    {
        tln!(
            "sss(x:{:.12}, y:{:.12}, lx:{:.12})",
            c_event.x(),
            c_event.y(),
            c_event.lower_x()
        );
    }
}
