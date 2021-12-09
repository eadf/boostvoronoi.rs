#![allow(unused_imports)]
use super::RobustFpt;
use crate::extended_exp_fpt as EX;
use crate::extended_int as EI;
use crate::robust_fpt as RF;
use num_traits::{Float, Num, NumCast, Zero};

#[test]
/// Add and sub
// Todo: add tests of re
fn sum_1() {
    for a_ in -10..10 {
        for b_ in (-10..10).rev() {
            let a_: f64 = NumCast::from(a_).unwrap();
            let b_: f64 = NumCast::from(b_).unwrap();

            let a = RobustFpt::from(a_);
            let b = RobustFpt::from(b_);
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

            let mut a = RobustFpt::from(a_);
            let b = RobustFpt::from(b_);
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
    let a = RobustFpt::from(a_);
    let b = RobustFpt::from(b_);
    let s = a - b;
    approx::assert_ulps_eq!(s.fpv(), a_ - b_);
    approx::assert_ulps_eq!(s.fpv(), 1.0)
}

#[test]
fn sub_2() {
    let mut a = RobustFpt::from(6.);
    let b = RobustFpt::from(2.);
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

            let a = RobustFpt::from(a_);
            let b = RobustFpt::from(b_);
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

            let mut a = RobustFpt::from(a_);
            let b = RobustFpt::from(b_);
            a *= b;
            approx::assert_ulps_eq!(a.fpv(), a_ * b_);
            if !b_.is_zero() {
                let mut a = RobustFpt::from(a_);
                let b = RobustFpt::from(b_);
                a /= b;
                approx::assert_ulps_eq!(a.fpv(), a_ / b_);
            }
        }
    }
}

#[test]
fn div_1() {
    let a = RobustFpt::from(12.);
    let b = RobustFpt::from(4.);
    let s = a / b;
    approx::assert_ulps_eq!(s.fpv(), 3.0)
}

#[test]
fn div_2() {
    let mut a = RobustFpt::from(6.);
    let b = RobustFpt::from(2.);
    a /= b;
    approx::assert_ulps_eq!(a.fpv(), 3.0)
}

