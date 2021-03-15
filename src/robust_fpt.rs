// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

//! Geometry predicates with floating-point variables usually require
//! high-precision predicates to retrieve the correct result.
//! Epsilon robust predicates give the result within some epsilon relative
//! error, but are a lot faster than high-precision predicates.
//! To make algorithm robust and efficient epsilon robust predicates are
//! used at the first step. In case of the undefined result high-precision
//! arithmetic is used to produce required robustness. This approach
//! requires exact computation of epsilon intervals within which epsilon
//! robust predicates have undefined value.
//! There are two ways to measure an error of floating-point calculations:
//! relative error and ULPs (units in the last place).
//! Let EPS be machine epsilon, then next inequalities have place:
//! 1 EPS <= 1 ULP <= 2 EPS (1), 0.5 ULP <= 1 EPS <= 1 ULP (2).
//! ULPs are good for measuring rounding errors and comparing values.
//! Relative errors are good for computation of general relative
//! error of formulas or expressions. So to calculate epsilon
//! interval within which epsilon robust predicates have undefined result
//! next schema is used:
//!     1) Compute rounding errors of initial variables using ULPs;
//!     2) Transform ULPs to epsilons using upper bound of the (1);
//!     3) Compute relative error of the formula using epsilon arithmetic;
//!     4) Transform epsilon to ULPs using upper bound of the (2);
//! In case two values are inside undefined ULP range use high-precision
//! arithmetic to produce the correct result, else output the result.
//! Look at almost_equal function to see how two floating-point variables
//! are checked to fit in the ULP range.
//! If A has relative error of r(A) and B has relative error of r(B) then:
//!     1) r(A + B) <= max(r(A), r(B)), for A * B >= 0;
//!     2) r(A - B) <= B*r(A)+A*r(B)/(A-B), for A * B >= 0;
//!     2) r(A * B) <= r(A) + r(B);
//!     3) r(A / B) <= r(A) + r(B);
//! In addition rounding error should be added, that is always equal to
//! 0.5 ULP or at most 1 epsilon. As you might see from the above formulas
//! subtraction relative error may be extremely large, that's why
//! epsilon robust comparator class is used to store floating point values
//! and compute subtraction as the final step of the evaluation.
//! For further information about relative errors and ULPs try this link:
//! http://docs.sun.com/source/806-3568/ncg_goldberg.html
mod robustdif_tests;
mod robustfpt_tests;

use super::OutputType;
use num::{BigInt, Float, NumCast, ToPrimitive, Zero};
use ordered_float::OrderedFloat;
use std::fmt;
use std::marker::PhantomData;
use std::ops;

/// Rounding error is at most 1 EPS.
pub const ROUNDING_ERROR: u8 = 1;

#[derive(Copy, Clone, fmt::Debug, Default)]
pub struct RobustFpt<F: OutputType + ops::Neg<Output = F>> {
    fpv_: F,
    re_: OrderedFloat<F>,
}

impl<F: OutputType + ops::Neg<Output = F>> RobustFpt<F> {
    pub fn new_1(fpv: F) -> Self {
        Self {
            fpv_: fpv,
            re_: OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap()),
        }
    }

    pub fn new_2(fpv: F, error: F) -> Self {
        Self {
            fpv_: fpv,
            re_: OrderedFloat(error),
        }
    }

    pub fn copy_from(other: &RobustFpt<F>) -> Self {
        Self {
            fpv_: other.fpv_,
            re_: OrderedFloat(F::zero()),
        }
    }

    #[inline(always)]
    pub fn fpv(&self) -> F {
        self.fpv_
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn re(&self) -> F {
        self.re_.into_inner()
    }

    #[inline(always)]
    pub fn ulp(&self) -> F {
        self.re_.into_inner()
    }

    #[allow(dead_code)]
    pub fn assign_from(&mut self, that: &Self) -> &mut Self {
        self.fpv_ = that.fpv_;
        self.re_ = that.re_;
        self
    }

    #[inline(always)]
    pub fn is_sign_positive(&self) -> bool {
        self.fpv_.is_sign_positive()
    }

    #[inline(always)]
    pub fn is_sign_negative(&self) -> bool {
        self.fpv_.is_sign_negative()
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn is_zero(&self) -> bool {
        self.fpv_.is_zero()
    }

    pub fn sqrt(&self) -> RobustFpt<F> {
        Self {
            //fpv_: Self::get_sqrt(self.fpv_),
            fpv_: self.fpv_.sqrt(),
            // self.re_ * 0.5 + ROUNDING_ERROR
            re_: self.re_ * OrderedFloat(num::cast::<f32, F>(0.5f32).unwrap())
                + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap()),
        }
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::Add<RobustFpt<F>> for RobustFpt<F> {
    type Output = RobustFpt<F>;

    fn add(self, _rhs: RobustFpt<F>) -> Self {
        let fpv: F = self.fpv_ + _rhs.fpv_;
        let re = if (!self.fpv_.is_sign_negative() && !_rhs.fpv_.is_sign_negative())
            || (!self.fpv_.is_sign_positive() && !_rhs.fpv_.is_sign_positive())
        {
            std::cmp::max(self.re_, _rhs.re_)
                + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap())
        } else {
            let mut temp = OrderedFloat(
                (self.fpv_ * *self.re_.as_ref() - _rhs.fpv_ * *_rhs.re_.as_ref()) / fpv,
            );
            if temp.is_sign_negative() {
                temp = -temp;
            }
            temp + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap())
        };
        Self { fpv_: fpv, re_: re }
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::AddAssign<RobustFpt<F>> for RobustFpt<F> {
    fn add_assign(&mut self, _rhs: RobustFpt<F>) {
        let fpv: F = self.fpv_ + _rhs.fpv_;
        let re = if (!self.fpv_.is_sign_negative() && !_rhs.fpv_.is_sign_negative())
            || (!self.fpv_.is_sign_positive() && !_rhs.fpv_.is_sign_positive())
        {
            std::cmp::max(self.re_, _rhs.re_)
                + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap())
        } else {
            let mut temp = OrderedFloat(
                (self.fpv_ * *self.re_.as_ref() - _rhs.fpv_ * *_rhs.re_.as_ref()) / fpv,
            );
            if temp.is_sign_negative() {
                temp = -temp;
            }
            temp + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap())
        };
        self.fpv_ = fpv;
        self.re_ = re;
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::Mul<F> for RobustFpt<F> {
    type Output = RobustFpt<F>;
    // Todo make this more efficient
    fn mul(self, _rhs: F) -> Self {
        let _rhs = RobustFpt::<F>::new_1(_rhs);
        self * _rhs
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::Mul<RobustFpt<F>> for RobustFpt<F> {
    type Output = RobustFpt<F>;

    fn mul(self, _rhs: RobustFpt<F>) -> Self {
        let fpv: F = self.fpv_ * _rhs.fpv_;
        let re: OrderedFloat<F> =
            self.re_ + _rhs.re_ + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap());

        Self { fpv_: fpv, re_: re }
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::MulAssign<RobustFpt<F>> for RobustFpt<F> {
    fn mul_assign(&mut self, _rhs: RobustFpt<F>) {
        self.re_ = self.re_ + _rhs.re_ + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap());
        self.fpv_ = self.fpv_ * _rhs.fpv_;
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::Sub<RobustFpt<F>> for RobustFpt<F> {
    type Output = RobustFpt<F>;

    fn sub(self, _rhs: RobustFpt<F>) -> Self {
        let fpv: F = self.fpv_ - _rhs.fpv_;
        let re = if (!self.fpv_.is_sign_negative() && !_rhs.fpv_.is_sign_positive())
            || (!self.fpv_.is_sign_positive() && !_rhs.fpv_.is_sign_negative())
        {
            std::cmp::max(self.re_, _rhs.re_)
                + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap())
        } else {
            let mut temp = (self.fpv_ * *self.re_ + _rhs.fpv_ * *_rhs.re_) / fpv;
            if temp.is_sign_negative() {
                temp = -temp;
            }
            OrderedFloat(temp) + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap())
        };
        Self { fpv_: fpv, re_: re }
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::SubAssign<RobustFpt<F>> for RobustFpt<F> {
    fn sub_assign(&mut self, _rhs: RobustFpt<F>) {
        let fpv = self.fpv_ - _rhs.fpv_;
        if (!self.fpv_.is_sign_negative() && !_rhs.fpv_.is_sign_positive())
            || (!self.fpv_.is_sign_positive() && !_rhs.fpv_.is_sign_negative())
        {
            self.re_ = std::cmp::max(self.re_, _rhs.re_)
                + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap());
        } else {
            let mut temp: F =
                (self.fpv_ * *self.re_.as_ref() + _rhs.fpv_ * *_rhs.re_.as_ref()) / fpv;
            if temp.is_sign_negative() {
                temp = -temp;
            }
            self.re_ =
                OrderedFloat(temp) + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap());
        }
        self.fpv_ = fpv;
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::Div<F> for RobustFpt<F> {
    type Output = RobustFpt<F>;

    // todo make efficient
    fn div(self, _rhs: F) -> Self {
        let _rhs = RobustFpt::<F>::new_1(_rhs);
        self / _rhs
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::Div<RobustFpt<F>> for RobustFpt<F> {
    type Output = RobustFpt<F>;

    fn div(self, _rhs: RobustFpt<F>) -> Self {
        let fpv: F = self.fpv_ / _rhs.fpv_;
        let re = self.re_ + _rhs.re_ + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap());
        Self { fpv_: fpv, re_: re }
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::DivAssign<RobustFpt<F>> for RobustFpt<F> {
    fn div_assign(&mut self, _rhs: RobustFpt<F>) {
        self.re_ = self.re_ + _rhs.re_ + OrderedFloat(num::cast::<u8, F>(ROUNDING_ERROR).unwrap());
        self.fpv_ = self.fpv_ / _rhs.fpv_;
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::Neg for RobustFpt<F> {
    type Output = RobustFpt<F>;

    fn neg(self) -> Self {
        Self {
            fpv_: -self.fpv_,
            re_: self.re_,
        }
    }
}

/// robust_dif consists of two not negative values: value1 and value2.
/// The resulting expression is equal to the value1 - value2.
/// Subtraction of a positive value is equivalent to the addition to value2
/// and subtraction of a negative value is equivalent to the addition to
/// value1. The structure implicitly avoids difference computation.

#[derive(Copy, Clone, fmt::Debug, Default)]
pub struct RobustDif<F: OutputType + ops::Neg<Output = F>> {
    positive_sum_: RobustFpt<F>,
    negative_sum_: RobustFpt<F>,
}

impl<F: OutputType + ops::Neg<Output = F>> RobustDif<F> {
    pub fn new() -> Self {
        Self {
            positive_sum_: RobustFpt::<F>::default(),
            negative_sum_: RobustFpt::<F>::default(),
        }
    }

    // TODO take & reference to other
    pub fn new_from(other: RobustDif<F>) -> Self {
        Self {
            positive_sum_: other.positive_sum_,
            negative_sum_: other.negative_sum_,
        }
    }

    #[allow(dead_code)]
    pub fn new_from_2(a: &RobustFpt<F>, b: &RobustFpt<F>) -> Self {
        Self {
            positive_sum_: *a,
            negative_sum_: *b,
        }
    }

    #[allow(dead_code)]
    pub fn new_1(value: F) -> Self {
        if value.is_sign_positive() {
            Self {
                positive_sum_: RobustFpt::<F>::new_1(value),
                negative_sum_: RobustFpt::<F>::default(),
            }
        } else {
            Self {
                positive_sum_: RobustFpt::<F>::default(),
                negative_sum_: RobustFpt::<F>::new_1(value),
            }
        }
    }

    #[allow(dead_code)]
    pub fn new_2(pos: F, neg: F) -> Self {
        assert!(!pos.is_sign_negative());
        assert!(!neg.is_sign_negative());
        Self {
            positive_sum_: RobustFpt::<F>::new_1(pos),
            negative_sum_: RobustFpt::<F>::new_1(neg),
        }
    }

    pub fn dif(&self) -> RobustFpt<F> {
        self.positive_sum_ - self.negative_sum_
    }

    #[inline]
    pub fn positive(&self) -> RobustFpt<F> {
        self.positive_sum_
    }

    #[inline]
    // neg() will collide with the trait RobustDif
    pub fn negative(&self) -> RobustFpt<F> {
        self.negative_sum_
    }

    #[inline]
    fn swap(&mut self) {
        std::mem::swap(&mut self.positive_sum_, &mut self.negative_sum_);
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::Neg for RobustDif<F> {
    type Output = RobustDif<F>;

    fn neg(self) -> Self {
        Self {
            positive_sum_: self.negative_sum_,
            negative_sum_: self.positive_sum_,
        }
    }
}
/*
    pub fn robust_dif<T>& operator+=(const T& val) {
    if (!is_neg(val))
      positive_sum_ += val;
    else
      negative_sum_ -= val;
    return *this;
  }

    pub fn robust_dif<T>& operator+=(const robust_dif<T>& that) {
    positive_sum_ += that.positive_sum_;
    negative_sum_ += that.negative_sum_;
    return *this;
  }
*/
impl<F: OutputType + ops::Neg<Output = F>> ops::Add<RobustDif<F>> for RobustDif<F> {
    type Output = RobustDif<F>;

    fn add(self, _rhs: RobustDif<F>) -> Self {
        Self {
            positive_sum_: self.positive_sum_ + _rhs.positive_sum_,
            negative_sum_: self.negative_sum_ + _rhs.negative_sum_,
        }
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::AddAssign<RobustDif<F>> for RobustDif<F> {
    fn add_assign(&mut self, _rhs: RobustDif<F>) {
        self.positive_sum_ += _rhs.positive_sum_;
        self.negative_sum_ += _rhs.negative_sum_;
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::AddAssign<RobustFpt<F>> for RobustDif<F> {
    fn add_assign(&mut self, _rhs: RobustFpt<F>) {
        if _rhs.is_sign_positive() {
            self.positive_sum_ += _rhs;
        } else {
            self.negative_sum_ -= _rhs;
        }
    }
}

/*
impl<F: BoostOutputType + ops::Neg<Output = F>> ops::AddAssign<RobustFpt<F>> for RobustDif<F> {
    fn add_assign(&mut self, _rhs: RobustFpt<F>) {
        let _rhs: F = _rhs.fpv();
        if is_pos(rhs) {
            self.positive_sum_ = self.positive_sum_ + _rhs;
        } else {
            self.negative_sum_ = self.negative_sum_ + _rhs;
        }
    }
}*/

/*
    pub fn robust_dif<T>& operator-=(const T& val) {
    if (!is_neg(val))
      negative_sum_ += val;
    else
      positive_sum_ -= val;
    return *this;
  }

    pub fn robust_dif<T>& operator-=(const robust_dif<T>& that) {
    positive_sum_ += that.negative_sum_;
    negative_sum_ += that.positive_sum_;
    return *this;
  }
*/
impl<F: OutputType + ops::Neg<Output = F>> ops::Sub<RobustDif<F>> for RobustDif<F> {
    type Output = RobustDif<F>;

    fn sub(self, _rhs: RobustDif<F>) -> Self {
        Self {
            positive_sum_: self.positive_sum_ + _rhs.negative_sum_,
            negative_sum_: self.negative_sum_ + _rhs.positive_sum_,
        }
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::SubAssign<RobustDif<F>> for RobustDif<F> {
    fn sub_assign(&mut self, _rhs: RobustDif<F>) {
        self.positive_sum_ += _rhs.negative_sum_;
        self.negative_sum_ += _rhs.positive_sum_;
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::SubAssign<RobustFpt<F>> for RobustDif<F> {
    fn sub_assign(&mut self, _rhs: RobustFpt<F>) {
        //dbg!(&self, &_rhs);
        if _rhs.is_sign_positive() {
            self.negative_sum_ += _rhs;
        } else {
            self.positive_sum_ -= _rhs;
        }
    }
}

/*
    pub fn robust_dif<T>& operator*=(const T& val) {
    if (!is_neg(val)) {
      positive_sum_ *= val;
      negative_sum_ *= val;
    } else {
      positive_sum_ *= -val;
      negative_sum_ *= -val;
      swap();
    }
    return *this;
  }

    pub fn robust_dif<T>& operator*=(const robust_dif<T>& that) {
    T positive_sum = this->positive_sum_ * that.positive_sum_ +
                     this->negative_sum_ * that.negative_sum_;
    T negative_sum = this->positive_sum_ * that.negative_sum_ +
                     this->negative_sum_ * that.positive_sum_;
    positive_sum_ = positive_sum;
    negative_sum_ = negative_sum;
    return *this;
  }
*/

impl<F: OutputType + ops::Neg<Output = F>> ops::Mul<RobustDif<F>> for RobustDif<F> {
    type Output = RobustDif<F>;

    fn mul(self, _rhs: RobustDif<F>) -> Self {
        Self {
            positive_sum_: self.positive_sum_ * _rhs.positive_sum_,
            negative_sum_: self.negative_sum_ * _rhs.negative_sum_,
        }
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::Mul<F> for RobustDif<F> {
    type Output = RobustDif<F>;

    fn mul(self, _rhs: F) -> Self {
        if _rhs.is_sign_positive() {
            let rhs = RobustFpt::<F>::new_1(_rhs);
            Self {
                positive_sum_: self.positive_sum_ * rhs,
                negative_sum_: self.negative_sum_ * rhs,
            }
        } else {
            let rhs = RobustFpt::<F>::new_1(_rhs);
            Self {
                positive_sum_: self.negative_sum_ * rhs,
                negative_sum_: self.positive_sum_ * rhs,
            }
        }
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::Mul<RobustFpt<F>> for RobustDif<F> {
    type Output = RobustDif<F>;

    fn mul(self, mut _rhs: RobustFpt<F>) -> Self {
        if _rhs.is_sign_negative() {
            _rhs = -_rhs;
            Self {
                positive_sum_: self.negative_sum_ * _rhs,
                negative_sum_: self.positive_sum_ * _rhs,
            }
        } else {
            Self {
                positive_sum_: self.positive_sum_ * _rhs,
                negative_sum_: self.negative_sum_ * _rhs,
            }
        }
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::MulAssign<F> for RobustDif<F> {
    fn mul_assign(&mut self, mut _rhs: F) {
        if _rhs.is_sign_negative() {
            _rhs = -_rhs;
            self.swap();
        }
        self.positive_sum_ = self.positive_sum_ * _rhs;
        self.negative_sum_ = self.negative_sum_ * _rhs;
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::MulAssign<RobustFpt<F>> for RobustDif<F> {
    fn mul_assign(&mut self, mut _rhs: RobustFpt<F>) {
        if _rhs.is_sign_negative() {
            _rhs = -_rhs;
            self.swap();
        }
        self.positive_sum_ = self.positive_sum_ * _rhs;
        self.negative_sum_ = self.negative_sum_ * _rhs;
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::MulAssign<RobustDif<F>> for RobustDif<F> {
    fn mul_assign(&mut self, _rhs: RobustDif<F>) {
        self.positive_sum_ = self.positive_sum_ * _rhs.positive_sum_;
        self.negative_sum_ = self.negative_sum_ * _rhs.negative_sum_;
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::DivAssign<F> for RobustDif<F> {
    fn div_assign(&mut self, _rhs: F) {
        let rhs = if _rhs.is_sign_negative() {
            self.swap();
            RobustFpt::<F>::new_1(-_rhs)
        } else {
            RobustFpt::<F>::new_1(-_rhs)
        };
        self.positive_sum_ /= rhs;
        self.negative_sum_ /= rhs;
    }
}

impl<F: OutputType + ops::Neg<Output = F>> ops::DivAssign<RobustFpt<F>> for RobustDif<F> {
    fn div_assign(&mut self, _rhs: RobustFpt<F>) {
        self.positive_sum_ /= _rhs;
        self.negative_sum_ /= _rhs;
    }
}

/// Used to compute expressions that operate with sqrts with predefined
/// relative error. Evaluates expressions of the next type:
/// sum(i = 1 .. n)(A[i] * sqrt(B[i])), 1 <= n <= 4.
#[allow(non_camel_case_types)]
pub struct robust_sqrt_expr<
    _fpt: NumCast + Float + fmt::Display + Default + fmt::Debug + ops::Neg<Output = _fpt>,
> {
    #[doc(hidden)]
    _pdf: PhantomData<_fpt>,
}

#[allow(non_camel_case_types)]
impl<
        _fpt: Clone + NumCast + Float + fmt::Display + Default + fmt::Debug + ops::Neg<Output = _fpt>,
    > Default for robust_sqrt_expr<_fpt>
{
    fn default() -> Self {
        Self { _pdf: PhantomData }
    }
}

#[allow(non_camel_case_types)]
impl<
        _fpt: Clone + NumCast + Float + fmt::Display + Default + fmt::Debug + ops::Neg<Output = _fpt>,
    > robust_sqrt_expr<_fpt>
{
    #[inline(always)]
    fn i_to_f(that: &BigInt) -> RobustFpt<_fpt> {
        let that = that.to_f64().unwrap();
        RobustFpt::<_fpt>::new_1(num::cast::<f64, _fpt>(that).unwrap())
    }

    /// Evaluates expression (re = 4 EPS):
    /// A[0] * sqrt(B[0]).
    pub fn eval1(&self, a: &[BigInt], b: &[BigInt]) -> RobustFpt<_fpt> {
        let a = Self::i_to_f(&a[0]);
        let b = Self::i_to_f(&b[0]);
        a * b.sqrt()
    }

    // Evaluates expression (re = 7 EPS):
    // A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]).
    pub fn eval2(&self, a: &[BigInt], b: &[BigInt]) -> RobustFpt<_fpt> {
        let ra = self.eval1(a, b);
        let rb = self.eval1(&a[1..], &b[1..]);

        if ra.is_zero()
            || rb.is_zero()
            || (!ra.is_sign_negative() && !rb.is_sign_negative())
            || (!ra.is_sign_positive() && !rb.is_sign_positive())
        {
            return ra + rb;
        }
        (Self::i_to_f(&a[0]) * Self::i_to_f(&a[0]) * Self::i_to_f(&b[0])
            - Self::i_to_f(&a[1]) * Self::i_to_f(&a[1]) * Self::i_to_f(&b[1]))
            / (ra - rb)
    }

    /// Evaluates expression (re = 16 EPS):
    /// A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) + A[2] * sqrt(B[2]).
    pub fn eval3(&self, a: &[BigInt], b: &[BigInt]) -> RobustFpt<_fpt> {
        let ra = self.eval2(a, b);
        let rb = self.eval1(&a[2..], &b[2..]);
        if ra.is_zero()
            || rb.is_zero()
            || (!ra.is_sign_negative() && !rb.is_sign_negative())
            || (!ra.is_sign_positive() && !rb.is_sign_positive())
        {
            return ra + rb;
        }
        let mut ta = [BigInt::zero(), BigInt::zero()];
        let mut tb = [BigInt::zero(), BigInt::zero()];

        ta[0] = a[0].clone() * &a[0] * &b[0] + &a[1] * &a[1] * &b[1] - &a[2] * &a[2] * &b[2];
        tb[0] = BigInt::from(1);
        ta[1] = a[0].clone() * &a[1] * 2;
        tb[1] = b[0].clone() * &b[1];

        let nom = self.eval2(&ta[..], &tb[..]);
        nom / (ra - rb)
    }

    /// Evaluates expression (re = 25 EPS):
    /// A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) +
    /// A[2] * sqrt(B[2]) + A[3] * sqrt(B[3]).
    pub fn eval4(&self, a: &[BigInt], b: &[BigInt]) -> RobustFpt<_fpt> {
        let ra = self.eval2(a, b);
        let rb = self.eval2(&a[2..], &b[2..]);

        if ra.is_zero()
            || rb.is_zero()
            || (!ra.is_sign_negative() && !rb.is_sign_negative())
            || (!ra.is_sign_positive() && !rb.is_sign_positive())
        {
            return ra + rb;
        }
        let mut ta = [BigInt::zero(), BigInt::zero(), BigInt::zero()];
        let mut tb = [BigInt::zero(), BigInt::zero(), BigInt::zero()];

        // todo remove all these clone()
        ta[0] = a[0].clone() * &a[0] * &b[0] + &a[1] * &a[1] * &b[1]
            - &a[2] * &a[2] * &b[2]
            - &a[3] * &a[3] * &b[3];
        tb[0] = BigInt::from(1);
        ta[1] = a[0].clone() * &a[1] * 2;
        tb[1] = b[0].clone() * &b[1];
        ta[2] = a[2].clone() * &a[3] * -2;
        tb[2] = b[2].clone() * &b[3];
        self.eval3(&ta, &tb) / (ra - rb)
    }

    /// Evaluates A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) +
    ///           A[2] + A[3] * sqrt(B[0] * B[1]).
    /// B[3] = B[0] * B[1].
    #[allow(non_snake_case)]
    pub fn sqrt_expr_evaluator_pss3(&mut self, A: &[BigInt], B: &[BigInt]) -> RobustFpt<_fpt> {
        let mut cA: [BigInt; 2] = [BigInt::zero(), BigInt::zero()];
        let mut cB: [BigInt; 2] = [BigInt::zero(), BigInt::zero()];

        let lh = self.eval2(A, B);
        let rh = self.eval2(&A[2..], &B[2..]);
        if lh.is_zero()
            || rh.is_zero()
            || (!lh.is_sign_negative() && !rh.is_sign_negative())
            || (!lh.is_sign_positive() && !rh.is_sign_positive())
        {
            return lh + rh;
        }
        cA[0] = A[0].clone() * &A[0] * &B[0] + &A[1] * &A[1] * &B[1]
            - &A[2] * &A[2]
            - &A[3] * &A[3] * &B[0] * &B[1];
        cB[0] = BigInt::from(1);
        cA[1] = (A[0].clone() * &A[1] - &A[2] * &A[3]) * 2;
        cB[1] = B[3].clone();
        self.eval2(&cA, &cB) / (lh - rh)
    }

    /// Evaluates A[3] + A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) +
    ///           A[2] * sqrt(B[3] * (sqrt(B[0] * B[1]) + B[2])).
    #[allow(non_snake_case)]
    pub fn sqrt_expr_evaluator_pss4(&mut self, A: &[BigInt], B: &[BigInt]) -> RobustFpt<_fpt> {
        #[cfg(feature = "console_debug")]
        {
            println!("->sqrt_expr_evaluator_pss4");
            println!(" A[0]={}", A[0]);
            println!(" A[1]={}", A[1]);
            println!(" A[2]={}", A[2]);
            println!(" A[3]={}", A[3]);
            println!(" B[0]={}", B[0]);
            println!(" B[1]={}", B[1]);
            println!(" B[2]={}", B[2]);
            println!(" B[3]={}", B[3]);
        }
        let mut cA: [BigInt; 4] = [
            BigInt::zero(),
            BigInt::zero(),
            BigInt::zero(),
            BigInt::zero(),
        ];
        let mut cB: [BigInt; 4] = [
            BigInt::zero(),
            BigInt::zero(),
            BigInt::zero(),
            BigInt::zero(),
        ];
        if A[3] == BigInt::zero() {
            let lh = self.eval2(A, B);
            cA[0] = BigInt::from(1);
            cB[0] = B[0].clone() * &B[1];
            cA[1] = B[2].clone();
            cB[1] = BigInt::from(1);
            let rh = self.eval1(&A[2..], &B[3..]) * self.eval2(&cA, &cB).sqrt();
            if lh.is_zero()
                || rh.is_zero()
                || (!lh.is_sign_negative() && !rh.is_sign_negative())
                || (!lh.is_sign_positive() && !rh.is_sign_positive())
            {
                #[cfg(feature = "console_debug")]
                {
                    println!(
                        "sqrt_expr_evaluator_pss4 1\nlh:{}\nrh:{}",
                        lh.fpv(),
                        rh.fpv()
                    );
                }
                return lh + rh;
            }
            cA[0] = A[0].clone() * &A[0] * &B[0] + &A[1] * &A[1] * &B[1]
                - &A[2] * &A[2] * &B[3] * &B[2];
            cB[0] = BigInt::from(1);
            cA[1] = A[0].clone() * &A[1] * 2 - &A[2] * &A[2] * &B[3];
            cB[1] = B[0].clone() * &B[1];
            let numer = self.eval2(&cA, &cB);
            #[cfg(feature = "console_debug")]
            {
                println!(
                    "sqrt_expr_evaluator_pss4 2\nnumerator:{}\nlh:{}\nrh:{}",
                    numer.fpv(),
                    lh.fpv(),
                    rh.fpv()
                );
            }

            return numer / (lh - rh);
        }
        cA[0] = BigInt::from(1);
        cB[0] = B[0].clone() * &B[1];
        cA[1] = B[2].clone();
        cB[1] = BigInt::from(1);
        let rh = self.eval1(&A[2..], &B[3..]) * (self.eval2(&cA, &cB).sqrt());
        cA[0] = A[0].clone();
        cB[0] = B[0].clone();
        cA[1] = A[1].clone();
        cB[1] = B[1].clone();
        cA[2] = A[3].clone();
        cB[2] = BigInt::from(1);
        let lh = self.eval3(&cA, &cB);
        #[cfg(feature = "console_debug")]
        {
            println!(
                "sqrt_expr_evaluator_pss4 ->3\nlh:{}\nrh:{}",
                lh.fpv(),
                rh.fpv()
            );
            println!(
                "lh.is_neg():{} lh.is_pos():{}",
                lh.is_sign_negative(),
                lh.is_sign_positive()
            );
            println!(
                "rh.is_neg():{} rh.is_pos():{}",
                rh.is_sign_negative(),
                rh.is_sign_positive()
            );
            println!(
                "lh.is_zero():{} rh.is_zero():{}",
                lh.is_zero(),
                rh.is_zero()
            );
        }
        if lh.is_zero()
            || rh.is_zero()
            || (!lh.is_sign_negative() && !rh.is_sign_negative())
            || (!lh.is_sign_positive() && !rh.is_sign_positive())
        {
            #[cfg(feature = "console_debug")]
            {
                println!(
                    "sqrt_expr_evaluator_pss4 3\nlh:{}\nrh:{}",
                    lh.fpv(),
                    rh.fpv()
                );
            }
            return lh + rh;
        }
        cA[0] = A[3].clone() * &A[0] * 2;
        cA[1] = A[3].clone() * &A[1] * 2;
        cA[2] = A[0].clone() * &A[0] * &B[0] + &A[1] * &A[1] * &B[1] + &A[3] * &A[3]
            - &A[2] * &A[2] * &B[2] * &B[3];
        cA[3] = A[0].clone() * &A[1] * 2 - &A[2] * &A[2] * &B[3];
        cB[3] = B[0].clone() * &B[1];
        let numer = self.sqrt_expr_evaluator_pss3(&cA, &cB);
        #[cfg(feature = "console_debug")]
        {
            println!(
                "sqrt_expr_evaluator_pss4 4\nnumerator:{}\nlh:{}\nrh:{}",
                numer.fpv(),
                lh.fpv(),
                rh.fpv()
            );
        }

        numer / (lh - rh)
    }
}

/// Floating point type wrapper. Allows to extend exponent boundaries to the
/// integer type range. This class does not handle division by zero, subnormal
/// numbers or NaNs.
/// Ported from the class extended_exponent_fpt in voronoi_ctypes.hpp
#[derive(Copy, Clone)]
pub struct ExtendedExponentFpt<F>
where
    F: NumCast + Float + fmt::Display + Copy + fmt::Debug + ops::Neg<Output = F>,
{
    val_: F,
    exp_: i32,
}
const MAX_SIGNIFICANT_EXP_DIF_F64: i32 = 54;

#[allow(dead_code)]
impl ExtendedExponentFpt<f64> {
    #[inline]
    pub fn new(v: f64) -> Self {
        let rv = libm::frexp(v);
        Self {
            val_: rv.0,
            exp_: rv.1,
        }
    }

    #[inline]
    pub fn new2(val: f64, exp: i32) -> Self {
        Self {
            val_: val,
            exp_: exp,
        }
    }

    /// Is positive method.
    /// IMPORTANT!!!!! in c++ boost voronoi implementation zero values can't be positive.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let aa:f64 = 0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = f64::MIN_POSITIVE;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// assert_eq!(a.is_pos(), aa.is_sign_positive());
    /// ```
    #[inline]
    pub fn is_pos(&self) -> bool {
        self.val_ > 0.0
    }

    /// Is negative method.
    /// IMPORTANT!!!!! in c++ boost voronoi implementation zero values can't be negative.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// println!("is_neg()");
    /// let aa:f64 = 0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// assert_eq!(a.is_neg(), aa.is_sign_negative());
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// assert_eq!(a.is_neg(), false);
    /// ```
    #[inline]
    pub fn is_neg(&self) -> bool {
        self.val_ < 0.0
    }

    /// Is zero method.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    /// # use num_traits::identities::Zero;
    ///
    /// let aa:f64 = 0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    ///
    /// let aa:f64 = f64::MIN_POSITIVE;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    ///
    /// let aa:f64 = -f64::MIN_POSITIVE;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.val_ == 0.0
    }

    /// Square root method.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let aa:f64 = f64::MAX;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// let a = a.sqrt();
    /// approx::assert_ulps_eq!(a.d(), aa.sqrt());
    /// ```
    pub fn sqrt(&self) -> Self {
        let mut val = self.val_;
        let mut exp = self.exp_;
        if (exp & 1) != 0 {
            val *= 2.0;
            exp -= 1;
        }
        Self::new2(val.sqrt(), exp >> 1)
    }

    /// A to-float operation.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let aa:f64 = 1000000000.0;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(aa);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(-aa);
    /// approx::assert_ulps_eq!(a.d(), -aa);
    /// ```
    pub fn d(&self) -> f64 {
        libm::ldexp(self.val_, self.exp_)
    }
}

impl ops::Neg for ExtendedExponentFpt<f64> {
    type Output = Self;

    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(1.0);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// let c = -a;
    /// approx::assert_ulps_eq!(c.d(), -1_f64);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(1000000000.0);
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64);
    /// let c = -a;
    /// approx::assert_ulps_eq!(c.d(), -1000000000_f64);
    /// ```
    fn neg(self) -> Self {
        Self {
            val_: -self.val_,
            exp_: self.exp_,
        }
    }
}

impl ops::Add for ExtendedExponentFpt<f64> {
    type Output = Self;

    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(1.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(2.0);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// let c = a + b;
    /// approx::assert_ulps_eq!(c.d(), 3_f64);
    /// let c = c + b;
    /// approx::assert_ulps_eq!(c.d(), 5_f64);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(1000000000.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(2000000000.0);
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), 2000000000_f64);
    /// let c = a + b;
    /// approx::assert_ulps_eq!(c.d(), 3000000000_f64);
    /// ```
    fn add(self, that: Self) -> Self {
        if self.val_ == 0.0 || that.exp_ > self.exp_ + MAX_SIGNIFICANT_EXP_DIF_F64 {
            return that;
        }
        if that.val_ == 0.0 || self.exp_ > that.exp_ + MAX_SIGNIFICANT_EXP_DIF_F64 {
            return self;
        }
        if self.exp_ >= that.exp_ {
            let exp_dif = self.exp_ - that.exp_;
            let val = libm::ldexp(self.val_, exp_dif) + that.val_;
            Self::new2(val, that.exp_)
        } else {
            let exp_dif = that.exp_ - self.exp_;
            let val = libm::ldexp(that.val_, exp_dif) + self.val_;
            Self::new2(val, self.exp_)
        }
    }
}

impl ops::Sub for ExtendedExponentFpt<f64> {
    type Output = Self;
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(1.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(-2.0);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), -2_f64);
    /// let c = a - b;
    /// approx::assert_ulps_eq!(c.d(), 3_f64);
    /// let c = c - b;
    /// approx::assert_ulps_eq!(c.d(), 5_f64);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(1000000000.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(-3000000000.0);
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), -3000000000_f64);
    /// let c = a - b;
    /// approx::assert_ulps_eq!(c.d(), 1000000000_f64-(-3000000000.0));
    /// ```
    fn sub(self, that: Self) -> Self {
        if self.val_ == 0.0 || that.exp_ > self.exp_ + MAX_SIGNIFICANT_EXP_DIF_F64 {
            return Self::new2(-that.val_, that.exp_);
        }
        if that.val_ == 0.0 || self.exp_ > that.exp_ + MAX_SIGNIFICANT_EXP_DIF_F64 {
            return self;
        }
        if self.exp_ >= that.exp_ {
            let exp_dif = self.exp_ - that.exp_;
            let val = libm::ldexp(self.val_, exp_dif) - that.val_;
            Self::new2(val, that.exp_)
        } else {
            let exp_dif = that.exp_ - self.exp_;
            let val = libm::ldexp(-that.val_, exp_dif) + self.val_;
            Self::new2(val, self.exp_)
        }
    }
}

impl ops::Mul for ExtendedExponentFpt<f64> {
    type Output = Self;
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(1.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(2.0);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// let c = a * b;
    /// approx::assert_ulps_eq!(c.d(), 2_f64);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(1000000000.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(2000000000.0);
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), 2000000000_f64);
    /// let c = a * b;
    /// approx::assert_ulps_eq!(c.d(), 1000000000_f64*2000000000_f64);
    /// ```
    fn mul(self, that: Self) -> Self {
        let val = self.val_ * that.val_;
        let exp = self.exp_ + that.exp_;
        Self::new2(val, exp)
    }
}

impl ops::Div for ExtendedExponentFpt<f64> {
    type Output = Self;
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(1.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(2.0);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// let c = a / b;
    /// approx::assert_ulps_eq!(c.d(), 1.0/2.0);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new(2000000000_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(-2000000000_f64);
    /// approx::assert_ulps_eq!(a.d(),  2000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), -2000000000_f64);
    /// let c = a / b;
    /// approx::assert_ulps_eq!(c.d(), -1f64);
    /// ```
    fn div(self, that: Self) -> Self {
        let val = self.val_ / that.val_;
        let exp = self.exp_ - that.exp_;
        Self::new2(val, exp)
    }
}

impl ops::AddAssign for ExtendedExponentFpt<f64> {
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::new(1.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(2.0);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// a += b;
    /// approx::assert_ulps_eq!(a.d(), 3_f64);
    /// a += b;
    /// approx::assert_ulps_eq!(a.d(), 5_f64);
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::new(1000000000.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(2000000000.0);
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), 2000000000_f64);
    /// a += b;
    /// approx::assert_ulps_eq!(a.d(), 3000000000_f64);
    /// ```
    fn add_assign(&mut self, that: Self) {
        if self.val_ == 0.0 || that.exp_ > self.exp_ + MAX_SIGNIFICANT_EXP_DIF_F64 {
            self.val_ = that.val_;
            self.exp_ = that.exp_;
        }
        if that.val_ == 0.0 || self.exp_ > that.exp_ + MAX_SIGNIFICANT_EXP_DIF_F64 {
            // do nothing
            return;
        }
        if self.exp_ >= that.exp_ {
            let exp_dif = self.exp_ - that.exp_;
            let val = libm::ldexp(self.val_, exp_dif) + that.val_;
            self.val_ = val;
            self.exp_ = that.exp_;
        } else {
            let exp_dif = that.exp_ - self.exp_;
            let val = libm::ldexp(that.val_, exp_dif) + self.val_;
            self.val_ = val;
            //self.exp_ = self.exp;
        }
    }
}

impl ops::SubAssign for ExtendedExponentFpt<f64> {
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::new(1.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(-2.0);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), -2_f64);
    /// a -= b;
    /// approx::assert_ulps_eq!(a.d(), 3_f64);
    /// a -= b;
    /// approx::assert_ulps_eq!(a.d(), 5_f64);
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::new(1000000000.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(-3000000000.0);
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), -3000000000_f64);
    /// a -= b;
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64-(-3000000000.0));
    /// ```
    fn sub_assign(&mut self, that: Self) {
        if self.val_ == 0.0 || that.exp_ > self.exp_ + MAX_SIGNIFICANT_EXP_DIF_F64 {
            self.val_ = -that.val_;
            self.exp_ = that.exp_;
        }
        if that.val_ == 0.0 || self.exp_ > that.exp_ + MAX_SIGNIFICANT_EXP_DIF_F64 {
            return;
        }
        if self.exp_ >= that.exp_ {
            let exp_dif = self.exp_ - that.exp_;
            let val = libm::ldexp(self.val_, exp_dif) - that.val_;
            self.val_ = val;
            self.exp_ = that.exp_;
        } else {
            let exp_dif = that.exp_ - self.exp_;
            let val = libm::ldexp(-that.val_, exp_dif) + self.val_;
            self.val_ = val;
            //self.exp_ = self.exp_;
        }
    }
}

impl ops::MulAssign for ExtendedExponentFpt<f64> {
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::new(1.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(2.0);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// a *= b;
    /// approx::assert_ulps_eq!(a.d(), 2_f64);
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::new(1000000000.0);
    /// let     b = robust_fpt::ExtendedExponentFpt::<f64>::new(2000000000.0);
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), 2000000000_f64);
    /// a *= b;
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64*2000000000_f64);
    /// ```
    fn mul_assign(&mut self, that: Self) {
        self.val_ *= that.val_;
        self.exp_ += that.exp_;
    }
}

impl ops::DivAssign for ExtendedExponentFpt<f64> {
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::new(1.0);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(2.0);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// a /= b;
    /// approx::assert_ulps_eq!(a.d(), 1.0/2.0);
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::new(2000000000_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::new(-2000000000_f64);
    /// approx::assert_ulps_eq!(a.d(),  2000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), -2000000000_f64);
    /// a /= b;
    /// approx::assert_ulps_eq!(a.d(), -1f64);
    /// ```
    fn div_assign(&mut self, that: Self) {
        self.val_ /= that.val_;
        self.exp_ -= that.exp_;
    }
}

impl fmt::Debug for ExtendedExponentFpt<f64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}^{}", self.val_, self.exp_)
    }
}
