use super::RobustFpt;
use crate::voronoi_robust_fpt as VR;
use crate::TypeConverter as TCC;
use num::{BigInt, Float, Num, NumCast, Zero};

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
            assert_eq!(s.fpv(), a_ + b_);
            let s = a - b;
            assert_eq!(s.fpv(), a_ - b_);
            let s = b - a;
            assert_eq!(s.fpv(), b_ - a_);
        }
    }
}

#[test]
/// AddAssign and SubAssign
fn sum_2() {
    for a_ in -10..10 {
        for b_ in (-10..10).rev() {
            for c_ in (-10..10).rev() {
                let a_: f64 = NumCast::from(a_).unwrap();
                let b_: f64 = NumCast::from(b_).unwrap();
                let c_: f64 = NumCast::from(b_).unwrap();

                let mut a = RobustFpt::new_1(a_);
                let b = RobustFpt::new_1(b_);
                a += b;
                assert_eq!(a.fpv(), a_ + b_);
                a -= b;
                assert_eq!(a.fpv(), a_);
                a -= b;
                assert_eq!(a.fpv(), a_ - b_);
                /* not implemented?
                a += c;
                assert_eq!(a.fpv(), a_ - b_ + c_);
                a -= c_;
                a -= c_;
                assert_eq!(a.fpv(), a_ - b_ - c_);*/
            }
        }
    }
}

#[test]
fn sub_1() {
    let a_: f32 = 5.;
    let b_: f32 = 4.;
    let a = RobustFpt::new_1(a_);
    let b = RobustFpt::new_1(b_);
    let s = a - b;
    assert_eq!(s.fpv(), a_ - b_);
    assert_eq!(s.fpv(), 1.0)
}

#[test]
fn sub_2() {
    let mut a = RobustFpt::new_1(6.);
    let b = RobustFpt::new_1(2.);
    a -= b;
    assert_eq!(a.fpv(), 4.0)
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
            assert_eq!(s.fpv(), a_ * b_);
            if !b_.is_zero() {
                let s = a / b;
                assert_eq!(s.fpv(), a_ / b_);
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
            assert_eq!(a.fpv(), a_ * b_);
            if !b_.is_zero() {
                let mut a = RobustFpt::new_1(a_);
                let b = RobustFpt::new_1(b_);
                a /= b;
                assert_eq!(a.fpv(), a_ / b_);
            }
        }
    }
}

#[test]
fn div_1() {
    let a = RobustFpt::new_1(12.);
    let b = RobustFpt::new_1(4.);
    let s = a / b;
    assert_eq!(s.fpv(), 3.0)
}

#[test]
fn div_2() {
    let mut a = RobustFpt::new_1(6.);
    let b = RobustFpt::new_1(2.);
    a /= b;
    assert_eq!(a.fpv(), 3.0)
}

#[test]
fn sqrt_1() {
    let a = RobustFpt::new_1(9.0f32);
    let b = a.sqrt();
    assert_eq!(b.fpv(), 3.0, "a.fpv fail");
    //assert_eq!(b.re(), 1.0 / 2.0 + 1.0, "a.re fail");
    let c = b * b;
    assert_eq!(c.fpv(), 9.0, "b.fpv fail");
    //assert_eq!(b.re(), (1.0 / 2.0 + 1.0) * 2.0, "b.re fail");
}

#[test]
fn sqrt_2() {
    type I1 = i32;
    type O = f32;
    type I2 = i64;
    type F2 = f64;
    type TC = TCC<I1, O, I2, F2>;
    let sqrte = VR::robust_sqrt_expr::<O>::new();

    let mut ca: [BigInt; 5] = [
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
    ];
    let mut cb: [BigInt; 5] = [
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
    ];

    // Evaluates expression (re = 4 EPS):
    // A[0] * sqrt(B[0]).
    ca[0] = BigInt::from(2);
    cb[0] = BigInt::from(9);

    let a = sqrte.eval1(&ca[..], &cb[..]);

    assert_eq!(a.fpv(), 2.0 * 3.0, "a.fpv fail");
    //assert_eq!(a.re(), 4.0, "a.re fail");
}

#[test]
fn sqrt_3() {
    type I1 = i32;
    type O = f32;
    type I2 = i64;
    type F2 = f64;
    type TC = TCC<I1, O, I2, F2>;
    let sqrte = VR::robust_sqrt_expr::<O>::new();

    let mut ca: [BigInt; 5] = [
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
    ];
    let mut cb: [BigInt; 5] = [
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
    ];

    // Evaluates expression (re = 7 EPS):
    // A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]).
    ca[0] = BigInt::from(3);
    cb[0] = BigInt::from(16);
    ca[1] = BigInt::from(2);
    cb[1] = BigInt::from(25);

    let a = sqrte.eval2(&ca[..], &cb[..]);

    assert_eq!(a.fpv(), 3.0 * 4.0 + 2.0 * 5.0, "a.fpv fail");
    //assert_eq!(a.re(), 7.0, "a.re fail");
}

#[test]
fn sqrt_4() {
    type I1 = i32;
    type O = f32;
    type I2 = i64;
    type F2 = f64;
    type TC = TCC<I1, O, I2, F2>;
    let sqrte = VR::robust_sqrt_expr::<O>::new();

    let mut ca: [BigInt; 5] = [
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
    ];
    let mut cb: [BigInt; 5] = [
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
        BigInt::zero(),
    ];

    // A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) + A[2] * sqrt(B[2]).

    ca[0] = BigInt::from(3);
    cb[0] = BigInt::from(16);
    ca[1] = BigInt::from(2);
    cb[1] = BigInt::from(25);
    ca[2] = BigInt::from(7);
    cb[2] = BigInt::from(49);

    let a = sqrte.eval3(&ca[..], &cb[..]);

    assert_eq!(a.fpv(), 3.0 * 4.0 + 2.0 * 5.0 + 7.0 * 7.0, "a.fpv fail");
    //assert_eq!(a.re(), 7.0, "a.re fail");
}

#[test]
fn sqrt_5() {
    type I2 = i64;
    type F2 = f64;
    let sqrte = VR::robust_sqrt_expr::<F2>::new();

    let ca: [BigInt; 5] = [
        BigInt::from(3),
        BigInt::from(2),
        BigInt::from(7),
        BigInt::from(8),
        BigInt::zero(),
    ];
    let cb: [BigInt; 5] = [
        BigInt::from(16),
        BigInt::from(25),
        BigInt::from(49),
        BigInt::from(64),
        BigInt::zero(),
    ];

    // A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) +
    // A[2] * sqrt(B[2]) + A[3] * sqrt(B[3]).
    let a = sqrte.eval4(&ca[..], &cb[..]);

    assert_eq!(
        a.fpv(),
        3.0 * 4.0 + 2.0 * 5.0 + 7.0 * 7.0 + 8.0 * 8.0,
        "a.fpv fail"
    );
}

#[test]
fn sqrt_6() {
    type I2 = i64;
    type F2 = f64;
    let sqrte = VR::robust_sqrt_expr::<F2>::new();

    let ca: [BigInt; 5] = [
        BigInt::from(20205600),
        BigInt::from(12),
        BigInt::from(1147151200i64),
        BigInt::from(-472),
        BigInt::zero(),
    ];
    let cb: [BigInt; 5] = [
        BigInt::from(1825),
        BigInt::from(6218073520360000i64),
        BigInt::from(1),
        BigInt::from(3407163572800i64),
        BigInt::zero(),
    ];

    let a = sqrte.eval4(&ca[..], &cb[..]);
    assert_eq!(a.fpv().floor(), 2085350584.0.floor(), "a.fpv fail");
}

#[test]
fn sqrt_7() {
    type I2 = i64;
    type F2 = f64;
    let sqrte = VR::robust_sqrt_expr::<F2>::new();

    let ca: [BigInt; 5] = [
        BigInt::from(74125000i64),
        BigInt::from(17),
        BigInt::from(370703125i64),
        BigInt::from(-450),
        BigInt::zero(),
    ];
    let cb: [BigInt; 5] = [
        BigInt::from(1825),
        BigInt::from(0),
        BigInt::from(1),
        BigInt::from(0),
        BigInt::zero(),
    ];

    let a = sqrte.eval4(&ca[..], &cb[..]);
    assert_eq!(a.fpv().floor(), 3537324513.0.floor(), "a.fpv fail");
}
