#![allow(unused_imports)]
use super::RobustFpt;
use crate::extended_exp_fpt as EX;
use crate::extended_int as EI;
use crate::robust_fpt as RF;
use crate::TypeConverter2 as TC;
use num::{Float, Num, NumCast, Zero};

#[test]
/// Add and sub
/// Todo: add tests of re
fn sum_1() {
    for a_ in -10..10 {
        for b_ in (-10..10).rev() {
            let a_: f64 = NumCast::from(a_).unwrap();
            let b_: f64 = NumCast::from(b_).unwrap();

            let a = RobustFpt::new_1(a_);
            let b = RobustFpt::new_1(b_);
            let s = a + b;
            approx::assert_ulps_eq!(s.fpv(), a_ + b_);
            let s = a - b;
            approx::assert_ulps_eq!(s.fpv(), a_ - b_);
            let s = b - a;
            approx::assert_ulps_eq!(s.fpv(), b_ - a_);
        }
    }
}

#[test]
/// AddAssign and SubAssign
fn sum_2() {
    for a_ in -10..10 {
        for b_ in (-10..10).rev() {
            //for c_ in (-10..10).rev() {
            let a_: f64 = NumCast::from(a_).unwrap();
            let b_: f64 = NumCast::from(b_).unwrap();
            //let c_: f64 = NumCast::from(b_).unwrap();

            let mut a = RobustFpt::new_1(a_);
            let b = RobustFpt::new_1(b_);
            a += b;
            approx::assert_ulps_eq!(a.fpv(), a_ + b_);
            a -= b;
            approx::assert_ulps_eq!(a.fpv(), a_);
            a -= b;
            approx::assert_ulps_eq!(a.fpv(), a_ - b_);
            /* not implemented?
            a += c;
            assert_eq!(a.fpv(), a_ - b_ + c_);
            a -= c_;
            a -= c_;
            assert_eq!(a.fpv(), a_ - b_ - c_);*/
            //}
        }
    }
}

#[test]
fn sub_1() {
    let a_: f64 = 5.;
    let b_: f64 = 4.;
    let a = RobustFpt::new_1(a_);
    let b = RobustFpt::new_1(b_);
    let s = a - b;
    approx::assert_ulps_eq!(s.fpv(), a_ - b_);
    approx::assert_ulps_eq!(s.fpv(), 1.0)
}

#[test]
fn sub_2() {
    let mut a = RobustFpt::new_1(6.);
    let b = RobustFpt::new_1(2.);
    a -= b;
    approx::assert_ulps_eq!(a.fpv(), 4.0)
}

#[test]
fn sub_3() {
    let aa = 51302308860895380373504_f64;
    let a = EX::ExtendedExponentFpt::<f64>::from(aa);
    let bb = -5986866286194975689408512_f64;
    let b = EX::ExtendedExponentFpt::<f64>::from(bb);
    let c = a - b;
    println!("{:.0}", c.d());
    println!("{:.0}", aa - bb);
    approx::assert_ulps_eq!(c.d(), aa - bb)
}

#[test]
fn sub_4() {
    let a = EX::ExtendedExponentFpt::<f64>::from(-2429436843391029202764395141992491778048_f64);
    let b = EX::ExtendedExponentFpt::<f64>::from(4527715074734887233719567492889438060544_f64);
    let c = a - b;
    println!(
        "{:.0}",
        -2429436843391029202764395141992491778048_f64
            - 4527715074734887233719567492889438060544_f64
    );
    approx::assert_ulps_eq!(c.d(), -6957151918125916436483962634881929838592_f64)
}

#[test]
fn sub_5() {
    let aa = -6957151918125916436483962634881929838592_f64;
    let a = EX::ExtendedExponentFpt::<f64>::from(aa);
    approx::assert_ulps_eq!(a.d(), aa)
}

#[test]
/// mul and div
fn mul_1() {
    for a_ in -10..10 {
        for b_ in (-10..10).rev() {
            let a_: f64 = NumCast::from(a_).unwrap();
            let b_: f64 = NumCast::from(b_).unwrap();

            let a = RobustFpt::new_1(a_);
            let b = RobustFpt::new_1(b_);
            let s = a * b;
            approx::assert_ulps_eq!(s.fpv(), a_ * b_);
            if !b_.is_zero() {
                let s = a / b;
                approx::assert_ulps_eq!(s.fpv(), a_ / b_);
            }
        }
    }
}

#[test]
/// MulAssign and DivAssign
fn mul_2() {
    for a_ in -10..10 {
        for b_ in (-10..10).rev() {
            let a_: f64 = NumCast::from(a_).unwrap();
            let b_: f64 = NumCast::from(b_).unwrap();

            let mut a = RobustFpt::new_1(a_);
            let b = RobustFpt::new_1(b_);
            a *= b;
            approx::assert_ulps_eq!(a.fpv(), a_ * b_);
            if !b_.is_zero() {
                let mut a = RobustFpt::new_1(a_);
                let b = RobustFpt::new_1(b_);
                a /= b;
                approx::assert_ulps_eq!(a.fpv(), a_ / b_);
            }
        }
    }
}

#[test]
fn div_1() {
    let a = RobustFpt::new_1(12.);
    let b = RobustFpt::new_1(4.);
    let s = a / b;
    approx::assert_ulps_eq!(s.fpv(), 3.0)
}

#[test]
fn div_2() {
    let mut a = RobustFpt::new_1(6.);
    let b = RobustFpt::new_1(2.);
    a /= b;
    approx::assert_ulps_eq!(a.fpv(), 3.0)
}

#[test]
fn sqrt_1() {
    let a = RobustFpt::new_1(9.0f64);
    let b = a.sqrt();
    approx::assert_ulps_eq!(b.fpv(), 3.0);
    //assert_eq!(b.re(), 1.0 / 2.0 + 1.0, "a.re fail");
    let c = b * b;
    approx::assert_ulps_eq!(c.fpv(), 9.0);
    //assert_eq!(b.re(), (1.0 / 2.0 + 1.0) * 2.0, "b.re fail");
}

#[test]
fn sqrt_2() {
    type F1 = f32;
    let sqrte = RF::robust_sqrt_expr::<F1>::default();

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

    // Evaluates expression (re = 4 EPS):
    // A[0] * sqrt(B[0]).
    ca[0] = EI::ExtendedInt::from(2);
    cb[0] = EI::ExtendedInt::from(9);

    let a = sqrte.eval1(&ca[..], &cb[..]);

    approx::assert_ulps_eq!(a.d(), 2.0 * 3.0);
    //assert_eq!(a.re(), 4.0, "a.re fail");
}

#[test]
fn sqrt_3() {
    type F1 = f32;
    let sqrte = RF::robust_sqrt_expr::<F1>::default();

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

    // Evaluates expression (re = 7 EPS):
    // A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]).
    ca[0] = EI::ExtendedInt::from(3);
    cb[0] = EI::ExtendedInt::from(16);
    ca[1] = EI::ExtendedInt::from(2);
    cb[1] = EI::ExtendedInt::from(25);

    let a = sqrte.eval2(&ca[..], &cb[..]);

    approx::assert_ulps_eq!(a.d(), 3.0 * 4.0 + 2.0 * 5.0);
    //assert_eq!(a.re(), 7.0, "a.re fail");
}

#[test]
fn sqrt_4() {
    type F1 = f32;

    let sqrte = RF::robust_sqrt_expr::<F1>::default();

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

    // A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) + A[2] * sqrt(B[2]).

    ca[0] = EI::ExtendedInt::from(3);
    cb[0] = EI::ExtendedInt::from(16);
    ca[1] = EI::ExtendedInt::from(2);
    cb[1] = EI::ExtendedInt::from(25);
    ca[2] = EI::ExtendedInt::from(7);
    cb[2] = EI::ExtendedInt::from(49);

    let a = sqrte.eval3(&ca[..], &cb[..]);

    approx::assert_ulps_eq!(a.d(), 3.0 * 4.0 + 2.0 * 5.0 + 7.0 * 7.0);
    //assert_eq!(a.re(), 7.0, "a.re fail");
}

#[test]
fn sqrt_5() {
    type F2 = f64;
    let sqrte = RF::robust_sqrt_expr::<F2>::default();

    let ca: [EI::ExtendedInt; 5] = [
        EI::ExtendedInt::from(3),
        EI::ExtendedInt::from(2),
        EI::ExtendedInt::from(7),
        EI::ExtendedInt::from(8),
        EI::ExtendedInt::zero(),
    ];
    let cb: [EI::ExtendedInt; 5] = [
        EI::ExtendedInt::from(16),
        EI::ExtendedInt::from(25),
        EI::ExtendedInt::from(49),
        EI::ExtendedInt::from(64),
        EI::ExtendedInt::zero(),
    ];

    // A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) +
    // A[2] * sqrt(B[2]) + A[3] * sqrt(B[3]).
    let a = sqrte.eval4(&ca[..], &cb[..]);

    approx::assert_ulps_eq!(a.d(), 3.0 * 4.0 + 2.0 * 5.0 + 7.0 * 7.0 + 8.0 * 8.0);
}

#[test]
fn sqrt_6() {
    type F2 = f64;
    let sqrte = RF::robust_sqrt_expr::<F2>::default();

    let ca: [EI::ExtendedInt; 5] = [
        EI::ExtendedInt::from(20205600),
        EI::ExtendedInt::from(12),
        EI::ExtendedInt::from(1147151200i64),
        EI::ExtendedInt::from(-472),
        EI::ExtendedInt::zero(),
    ];
    let cb: [EI::ExtendedInt; 5] = [
        EI::ExtendedInt::from(1825),
        EI::ExtendedInt::from(6218073520360000i64),
        EI::ExtendedInt::from(1),
        EI::ExtendedInt::from(3407163572800i64),
        EI::ExtendedInt::zero(),
    ];

    let a = sqrte.eval4(&ca[..], &cb[..]);
    approx::assert_ulps_eq!(a.d().floor(), 2085350584.0.floor());
}

#[test]
fn sqrt_7() {
    type F2 = f64;
    let sqrte = RF::robust_sqrt_expr::<F2>::default();

    let ca: [EI::ExtendedInt; 5] = [
        EI::ExtendedInt::from(74125000i64),
        EI::ExtendedInt::from(17),
        EI::ExtendedInt::from(370703125i64),
        EI::ExtendedInt::from(-450),
        EI::ExtendedInt::zero(),
    ];
    let cb: [EI::ExtendedInt; 5] = [
        EI::ExtendedInt::from(1825),
        EI::ExtendedInt::from(0),
        EI::ExtendedInt::from(1),
        EI::ExtendedInt::from(0),
        EI::ExtendedInt::zero(),
    ];

    let a = sqrte.eval4(&ca[..], &cb[..]);
    approx::assert_ulps_eq!(a.d().floor(), 3537324513.0.floor());
}
