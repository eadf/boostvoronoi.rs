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
mod extendedint_tests;
mod robustdif_tests;
mod robustfpt_tests;

use super::OutputType;
use num::{Float, NumCast};
use ordered_float::OrderedFloat;
use std::fmt;
use std::marker::PhantomData;
use std::num::Wrapping;
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

    /// Is positive method.
    /// IMPORTANT!!!!! in c++ boost voronoi implementation zero values can't be positive.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    /// println!("is_pos()");
    /// let aa:f64 = 0_f64;
    /// let a = robust_fpt::RobustFpt::<f64>::new_1(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::RobustFpt::<f64>::new_1(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = f64::MIN_POSITIVE;
    /// let a = robust_fpt::RobustFpt::<f64>::new_1(aa);
    /// assert_eq!(a.is_pos(), aa.is_sign_positive());
    /// ```
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        self.fpv_ > F::zero()
    }

    /// Is negative method.
    /// IMPORTANT!!!!! in c++ boost voronoi implementation zero values can't be negative.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// println!("is_neg()");
    /// let aa:f64 = 0_f64;
    /// let a = robust_fpt::RobustFpt::<f64>::new_1(aa);
    /// assert_eq!(a.is_neg(), aa.is_sign_negative());
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::RobustFpt::<f64>::new_1(aa);
    /// assert_eq!(a.is_neg(), false);
    /// ```
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        self.fpv_ < F::zero()
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
        if _rhs.is_pos() {
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
        if _rhs.is_pos() {
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
        if _rhs.is_neg() {
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
        if _rhs.is_neg() {
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
    fn i_to_f(that: &ExtendedInt) -> ExtendedExponentFpt<f64> {
        ExtendedExponentFpt::<f64>::from(that)
    }

    /// Evaluates expression (re = 4 EPS):
    /// A[0] * sqrt(B[0]).
    pub fn eval1(&self, a: &[ExtendedInt], b: &[ExtendedInt]) -> ExtendedExponentFpt<f64> {
        let a = Self::i_to_f(&a[0]);
        let b = Self::i_to_f(&b[0]);
        //println!("eval1:");
        //println!(" a:{:.0}", a.d());
        //println!(" b:{:.0}", b.d());
        #[cfg(feature = "console_debug_eval")]
        {
            let rv = a * (b.sqrt());
            println!("eval1: {:.0}", rv.d());
            rv
        }
        #[cfg(not(feature = "console_debug_eval"))]
        {
            a * (b.sqrt())
        }
    }

    // Evaluates expression (re = 7 EPS):
    // A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]).
    pub fn eval2(&self, a: &[ExtendedInt], b: &[ExtendedInt]) -> ExtendedExponentFpt<f64> {
        let ra = self.eval1(a, b);
        let rb = self.eval1(&a[1..], &b[1..]);
        #[cfg(feature = "console_debug_eval")]
        {
            println!("->eval2");
            println!(" a[0]:{:.0}", a[0].d());
            println!(" a[1]:{:.0}", a[1].d());
            println!(" b[0]:{:.0}", b[0].d());
            println!(" b[1]:{:.0}", b[1].d());
            println!(" ra:{:.0}", ra.d());
            println!(" rb:{:.0}", rb.d());
        }
        if ra.is_zero()
            || rb.is_zero()
            || (!ra.is_neg() && !rb.is_neg())
            || (!ra.is_pos() && !rb.is_pos())
        {
            return ra + rb;
        }

        let p = &a[0] * &a[0] * &b[0] - &a[1] * &a[1] * &b[1];
        let numer = Self::i_to_f(&p);
        let divisor = ra - rb;
        #[cfg(feature = "console_debug_eval")]
        {
            let rv = numer / divisor;
            println!(
                "<-eval2:\n numer:{:.0}\n divisor:{:.0}\n rv:{:.0}",
                numer.d(),
                divisor.d(),
                rv.d()
            );
            rv
        }
        #[cfg(not(feature = "console_debug_eval"))]
        {
            numer / divisor
        }
    }

    /// Evaluates expression (re = 16 EPS):
    /// A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) + A[2] * sqrt(B[2]).
    pub fn eval3(&self, a: &[ExtendedInt], b: &[ExtendedInt]) -> ExtendedExponentFpt<f64> {
        let ra = self.eval2(a, b);
        let rb = self.eval1(&a[2..], &b[2..]);
        #[cfg(feature = "console_debug_eval")]
        {
            println!("->eval3");
            println!(" a[0]:{:.0}", a[0].d());
            println!(" a[1]:{:.0}", a[1].d());
            println!(" a[2]:{:.0}", a[2].d());
            println!(" b[0]:{:.0}", b[0].d());
            println!(" b[1]:{:.0}", b[1].d());
            println!(" b[2]:{:.0}", b[2].d());
            println!(" ra:{:.0}", ra.d());
            println!(" rb:{:.0}", rb.d());
        }
        if ra.is_zero()
            || rb.is_zero()
            || (!ra.is_neg() && !rb.is_neg())
            || (!ra.is_pos() && !rb.is_pos())
        {
            return ra + rb;
        }
        let mut ta = [ExtendedInt::zero(), ExtendedInt::zero()];
        let mut tb = [ExtendedInt::zero(), ExtendedInt::zero()];

        ta[0] = &a[0] * &a[0] * &b[0] + &a[1] * &a[1] * &b[1] - &a[2] * &a[2] * &b[2];
        tb[0] = ExtendedInt::from(1);
        ta[1] = &a[0] * &a[1] * &ExtendedInt::from(2_i32);
        tb[1] = &b[0] * &b[1];
        #[cfg(feature = "console_debug_eval")]
        {
            println!("<-eval3");
            println!(" ta[0]:{:.0}", ta[0].d());
            println!(" ta[1]:{:.0}", ta[1].d());
            println!(" tb[0]:{:.0}", tb[0].d());
            println!(" tb[1]:{:.0}", tb[1].d());
            println!(" ra:{:.0} val:{:.12} exp:{:}", ra.d(), ra.val(), ra.exp());
            println!(" rb:{:.0} val:{:.12} exp:{:}", rb.d(), rb.val(), rb.exp());
            let rarb = ra - rb;
            println!(
                " ra-rb:{:.0} val:{:.12} exp:{:}",
                rarb.d(),
                rarb.val(),
                rarb.exp()
            );
        }
        let nom = self.eval2(&ta[..], &tb[..]);
        let div = ra - rb;
        #[cfg(feature = "console_debug_eval")]
        {
            let rv = nom / div;
            println!(" nom:{:.0}", nom.d());
            println!(" div:{:.0}", div.d());
            println!(" rv:{:.0}", rv.d());
            rv
        }
        #[cfg(not(feature = "console_debug_eval"))]
        {
            nom / div
        }
    }

    /// Evaluates expression (re = 25 EPS):
    /// A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) +
    /// A[2] * sqrt(B[2]) + A[3] * sqrt(B[3]).
    pub fn eval4(&self, a: &[ExtendedInt], b: &[ExtendedInt]) -> ExtendedExponentFpt<f64> {
        let ra = self.eval2(a, b);
        let rb = self.eval2(&a[2..], &b[2..]);

        if ra.is_zero()
            || rb.is_zero()
            || (!ra.is_neg() && !rb.is_neg())
            || (!ra.is_pos() && !rb.is_pos())
        {
            return ra + rb;
        }
        let mut ta = [
            ExtendedInt::zero(),
            ExtendedInt::zero(),
            ExtendedInt::zero(),
        ];
        let mut tb = [
            ExtendedInt::zero(),
            ExtendedInt::zero(),
            ExtendedInt::zero(),
        ];

        ta[0] = &a[0] * &a[0] * &b[0] + &a[1] * &a[1] * &b[1]
            - &a[2] * &a[2] * &b[2]
            - &a[3] * &a[3] * &b[3];
        tb[0] = ExtendedInt::from(1_i32);
        ta[1] = &a[0] * &a[1] * &ExtendedInt::from(2_i32);
        tb[1] = &b[0] * &b[1];
        ta[2] = &a[2] * &a[3] * &ExtendedInt::from(-2_i32);
        tb[2] = &b[2] * &b[3];
        #[cfg(feature = "console_debug_eval")]
        {
            let rv = self.eval3(&ta, &tb) / (ra - rb);
            println!("<-eval4:{}", rv.d());
            rv
        }
        #[cfg(not(feature = "console_debug_eval"))]
        {
            self.eval3(&ta, &tb) / (ra - rb)
        }
    }

    /// Evaluates A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) +
    ///           A[2] + A[3] * sqrt(B[0] * B[1]).
    /// B[3] = B[0] * B[1].
    #[allow(non_snake_case)]
    pub fn sqrt_expr_evaluator_pss3(
        &mut self,
        A: &[ExtendedInt],
        B: &[ExtendedInt],
    ) -> ExtendedExponentFpt<f64> {
        let mut cA: [ExtendedInt; 2] = [ExtendedInt::zero(), ExtendedInt::zero()];
        let mut cB: [ExtendedInt; 2] = [ExtendedInt::zero(), ExtendedInt::zero()];

        let lh = self.eval2(A, B);
        let rh = self.eval2(&A[2..], &B[2..]);
        #[cfg(feature = "console_debug_eval")]
        {
            println!(
                "sqrt_expr_evaluator_pss3\n lh={:.0}\n rh={:.0}",
                lh.d(),
                rh.d()
            );
        }
        if lh.is_zero()
            || rh.is_zero()
            || (!lh.is_neg() && !rh.is_neg())
            || (!lh.is_pos() && !rh.is_pos())
        {
            #[cfg(feature = "console_debug_eval")]
            {
                println!("<-sqrt_expr_evaluator_pss3 lh + rh");
            }
            return lh + rh;
        }
        cA[0] = &A[0] * &A[0] * &B[0] + &A[1] * &A[1] * &B[1]
            - &A[2] * &A[2]
            - &A[3] * &A[3] * &B[0] * &B[1];
        cB[0] = ExtendedInt::from(1);
        cA[1] = (&A[0] * &A[1] - &A[2] * &A[3]) * &ExtendedInt::from(2_i32);
        cB[1] = B[3].clone();
        let numer = self.eval2(&cA, &cB);
        let divisor = lh - rh;
        #[cfg(feature = "console_debug_eval")]
        {
            println!(
                "<-sqrt_expr_evaluator_pss3\n numer:{:.0}\n divisor:{:.0}",
                numer.d(),
                divisor.d()
            );
        }
        numer / divisor
    }

    /// Evaluates A[3] + A[0] * sqrt(B[0]) + A[1] * sqrt(B[1]) +
    ///           A[2] * sqrt(B[3] * (sqrt(B[0] * B[1]) + B[2])).
    #[allow(non_snake_case)]
    pub fn sqrt_expr_evaluator_pss4(
        &mut self,
        A: &[ExtendedInt],
        B: &[ExtendedInt],
    ) -> ExtendedExponentFpt<f64> {
        #[cfg(feature = "console_debug_eval")]
        {
            println!("->sqrt_expr_evaluator_pss4");
            println!(" A[0]={:?}", A[0]);
            println!(" A[1]={:?}", A[1]);
            println!(" A[2]={:?}", A[2]);
            println!(" A[3]={:?}", A[3]);
            println!(" B[0]={:?}", B[0]);
            println!(" B[1]={:?}", B[1]);
            println!(" B[2]={:?}", B[2]);
            println!(" B[3]={:?}", B[3]);
        }
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
        if A[3].is_zero() {
            let lh = self.eval2(A, B);
            cA[0] = ExtendedInt::from(1);
            cB[0] = &B[0] * &B[1];
            cA[1] = B[2].clone();
            cB[1] = ExtendedInt::from(1);
            let rh = self.eval1(&A[2..], &B[3..]) * self.eval2(&cA, &cB).sqrt();
            if lh.is_zero()
                || rh.is_zero()
                || (!lh.is_neg() && !rh.is_neg())
                || (!lh.is_pos() && !rh.is_pos())
            {
                #[cfg(feature = "console_debug_eval")]
                {
                    println!(
                        "<-sqrt_expr_evaluator_pss4 1\nlh:{:.0}\nrh:{:.0}",
                        lh.d(),
                        rh.d()
                    );
                }
                return lh + rh;
            }
            cA[0] = &A[0] * &A[0] * &B[0] + &A[1] * &A[1] * &B[1] - &A[2] * &A[2] * &B[3] * &B[2];
            cB[0] = ExtendedInt::from(1_i32);
            cA[1] = &A[0] * &A[1] * &ExtendedInt::from(2_i32) - &A[2] * &A[2] * &B[3];
            cB[1] = &B[0] * &B[1];
            let numer = self.eval2(&cA, &cB);
            #[cfg(feature = "console_debug_eval")]
            {
                println!(
                    "<-sqrt_expr_evaluator_pss4 2\nnumerator:{:.0}\nlh:{:.0}\nrh:{:.0}",
                    numer.d(),
                    lh.d(),
                    rh.d()
                );
            }

            return numer / (lh - rh);
        }
        cA[0] = ExtendedInt::from(1);
        cB[0] = &B[0] * &B[1];
        cA[1] = B[2].clone();
        cB[1] = ExtendedInt::from(1);
        let rh = self.eval1(&A[2..], &B[3..]) * (self.eval2(&cA, &cB).sqrt());
        cA[0] = A[0].clone();
        cB[0] = B[0].clone();
        cA[1] = A[1].clone();
        cB[1] = B[1].clone();
        cA[2] = A[3].clone();
        cB[2] = ExtendedInt::from(1);
        let lh = self.eval3(&cA, &cB);
        #[cfg(feature = "console_debug_eval")]
        {
            println!(
                "<-sqrt_expr_evaluator_pss4 2.5\nlh:{:.0}\nrh:{:.0}",
                lh.d(),
                rh.d()
            );
            println!("lh.is_neg():{} lh.is_pos():{}", lh.is_neg(), lh.is_pos());
            println!("rh.is_neg():{} rh.is_pos():{}", rh.is_neg(), rh.is_pos());
            println!(
                "lh.is_zero():{} rh.is_zero():{}",
                lh.is_zero(),
                rh.is_zero()
            );
        }
        if lh.is_zero()
            || rh.is_zero()
            || (!lh.is_neg() && !rh.is_neg())
            || (!lh.is_pos() && !rh.is_pos())
        {
            #[cfg(feature = "console_debug_eval")]
            {
                println!(
                    "<-sqrt_expr_evaluator_pss4 3\nlh:{:.0}\nrh:{:.0}",
                    lh.d(),
                    rh.d()
                );
            }
            return lh + rh;
        }
        cA[0] = &A[3] * &A[0] * &ExtendedInt::from(2_i32);
        cA[1] = &A[3] * &A[1] * &ExtendedInt::from(2_i32);
        cA[2] = &A[0] * &A[0] * &B[0] + &A[1] * &A[1] * &B[1] + &A[3] * &A[3]
            - &A[2] * &A[2] * &B[2] * &B[3];
        cA[3] = &A[0] * &A[1] * &ExtendedInt::from(2_i32) - &A[2] * &A[2] * &B[3];
        cB[3] = &B[0] * &B[1];
        let numer = self.sqrt_expr_evaluator_pss3(&cA, &cB);
        #[cfg(feature = "console_debug_eval")]
        {
            println!(
                "<-sqrt_expr_evaluator_pss4 4\nnumer:{:.0}\nlh:{:.0}\nrh:{:.0}",
                numer.d(),
                lh.d(),
                rh.d()
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

impl From<&ExtendedInt> for ExtendedExponentFpt<f64> {
    #[inline]
    /// converts to ExtendedExponentFpt::<f64>
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    /// # use boostvoronoi::robust_fpt::ExtendedExponentFpt;
    ///
    /// let aa = 41232131332_f64;
    /// let mut a = ExtendedInt::from(aa as i64);
    /// let e = ExtendedExponentFpt::from(&a);
    /// approx::assert_ulps_eq!(e.d(), aa);
    /// ```
    fn from(that: &ExtendedInt) -> Self {
        let p = that.p();
        Self::new2(p.0, p.1)
    }
}

impl From<&ExtendedExponentFpt<f64>> for f64 {
    #[inline]
    /// converts from ExtendedExponentFpt<f64> to f64
    /// ```
    /// # use boostvoronoi::robust_fpt::*;
    ///
    /// let f1 = 345345345453_f64;
    /// let e = ExtendedExponentFpt::from(f1);
    /// let f2 = f64::from(&e);
    /// approx::assert_ulps_eq!(f1, f2);
    /// ```
    fn from(that: &ExtendedExponentFpt<f64>) -> f64 {
        that.d()
    }
}

impl From<f64> for ExtendedExponentFpt<f64> {
    #[inline]
    /// converts from f64 to ExtendedExponentFpt<f64>
    /// ```
    /// # use boostvoronoi::robust_fpt::*;
    ///
    /// let f1 = 345345345453_f64;
    /// let e = ExtendedExponentFpt::from(f1);
    /// let f2 = f64::from(&e);
    /// approx::assert_ulps_eq!(f1, f2);
    /// ```
    fn from(that: f64) -> ExtendedExponentFpt<f64> {
        let rv = libm::frexp(that);
        Self::new2(rv.0, rv.1)
    }
}

#[allow(dead_code)]
impl ExtendedExponentFpt<f64> {
    /// Constructor with value and exponent as arguments.
    /// The value of this number is 'val_' * 2^ 'exp_'
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::new2(1.0, 12);
    /// approx::assert_ulps_eq!(a.d(), 4096.0);
    /// ```
    #[inline]
    pub fn new2(val: f64, exp: i32) -> Self {
        let fr = libm::frexp(val);
        Self {
            val_: fr.0,
            exp_: exp + fr.1,
        }
    }

    /// Is positive method.
    /// IMPORTANT!!!!! in the c++ boost voronoi implementation zero values can't be positive.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let aa:f64 = 0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = f64::MIN_POSITIVE;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_pos(), aa.is_sign_positive());
    /// ```
    #[inline]
    pub fn is_pos(&self) -> bool {
        self.val_ > 0.0
    }

    /// Is negative method.
    /// IMPORTANT!!!!! in the c++ boost voronoi implementation zero values can't be negative.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let aa:f64 = 0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_neg(), aa.is_sign_negative());
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
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
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    ///
    /// let aa:f64 = f64::MIN_POSITIVE;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    ///
    /// let aa:f64 = -f64::MIN_POSITIVE;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
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
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// let a = a.sqrt();
    /// approx::assert_ulps_eq!(a.d(), aa.sqrt());
    /// ```
    #[inline]
    pub fn sqrt(&self) -> Self {
        #[cfg(feature = "console_debug_eval")]
        {
            println!(
                "->sqrt:{:.12} val:{:.12} exp:{:}",
                self.d(),
                self.val(),
                self.exp()
            );
        }
        let mut val = self.val_;
        let mut exp = self.exp_;
        if (exp & 1) != 0 {
            val *= 2.0;
            exp -= 1;
        }
        #[cfg(feature = "console_debug_eval")]
        {
            let rv = Self::new2(val.sqrt(), exp >> 1);
            println!(
                "<-sqrt:{:.12} val:{:.12} exp:{:}",
                rv.d(),
                rv.val(),
                rv.exp()
            );
            rv
        }
        #[cfg(not(feature = "console_debug_eval"))]
        {
            Self::new2(val.sqrt(), exp >> 1)
        }
    }

    /// A to-float operation.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let aa:f64 = 1000000000.0;
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(-aa);
    /// approx::assert_ulps_eq!(a.d(), -aa);
    /// ```
    pub fn d(&self) -> f64 {
        libm::ldexp(self.val_, self.exp_)
    }

    pub fn val(&self) -> f64 {
        self.val_
    }

    pub fn exp(&self) -> i32 {
        self.exp_
    }
}

impl ops::Neg for ExtendedExponentFpt<f64> {
    type Output = Self;

    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// let c = -a;
    /// approx::assert_ulps_eq!(c.d(), -1_f64);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
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
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// let c = a + b;
    /// approx::assert_ulps_eq!(c.d(), 3_f64);
    /// let c = c + b;
    /// approx::assert_ulps_eq!(c.d(), 5_f64);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
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
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(-2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), -2_f64);
    /// let c = a - b;
    /// approx::assert_ulps_eq!(c.d(), 3_f64);
    /// let c = c - b;
    /// approx::assert_ulps_eq!(c.d(), 5_f64);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(-3000000000_f64);
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
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// let c = a * b;
    /// approx::assert_ulps_eq!(c.d(), 2_f64);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
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
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// let c = a / b;
    /// approx::assert_ulps_eq!(c.d(), 1.0/2.0);
    /// let a = robust_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(-2000000000_f64);
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
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// a += b;
    /// approx::assert_ulps_eq!(a.d(), 3_f64);
    /// a += b;
    /// approx::assert_ulps_eq!(a.d(), 5_f64);
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
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
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(-2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), -2_f64);
    /// a -= b;
    /// approx::assert_ulps_eq!(a.d(), 3_f64);
    /// a -= b;
    /// approx::assert_ulps_eq!(a.d(), 5_f64);
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(-3000000000_f64);
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
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// a *= b;
    /// approx::assert_ulps_eq!(a.d(), 2_f64);
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let     b = robust_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
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
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// a /= b;
    /// approx::assert_ulps_eq!(a.d(), 1.0/2.0);
    /// let mut a = robust_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
    /// let b = robust_fpt::ExtendedExponentFpt::<f64>::from(-2000000000_f64);
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

/// Stack allocated big integer class.
/// Supports next set of arithmetic operations: +, -, *.
/// Ported from voronoi_ctypes.hpp
#[derive(Clone)]
pub struct ExtendedInt {
    chunks: smallvec::SmallVec<[Wrapping<u32>; 4]>,
    count: i32,
}

impl From<i32> for ExtendedInt {
    #[inline]
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 42_f64;
    /// let a = ExtendedInt::from(aa as i32);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// ```
    fn from(that: i32) -> Self {
        let mut rv = Self::zero();
        if that > 0 {
            rv.chunks.push(Wrapping(that as u32));
            rv.count = 1;
        } else if that < 0 {
            rv.chunks.push(Wrapping((-that) as u32));
            rv.count = -1;
        }
        rv
    }
}

impl From<i64> for ExtendedInt {
    #[inline]
    ///```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 41232131332_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// ```
    fn from(that: i64) -> Self {
        let mut rv = Self::zero();
        if that > 0 {
            let mut c = that as u64;
            rv.chunks.push(Wrapping((c & 0xFFFFFFFF) as u32));
            c >>= 32;
            if c != 0 {
                rv.chunks.push(Wrapping(c as u32));
                rv.count = 2
            } else {
                rv.count = 1
            }
        } else if that < 0 {
            let mut c: u64 = (-that) as u64;
            rv.chunks.push(Wrapping((c & 0xFFFFFFFF) as u32));
            c >>= 32;
            if c != 0 {
                rv.chunks.push(Wrapping(c as u32));
                rv.count = -2
            } else {
                rv.count = -1
            }
        }
        rv
    }
}

impl ExtendedInt {
    /// todo implement num::Zero
    #[inline(always)]
    pub fn zero() -> Self {
        Self {
            chunks: smallvec::SmallVec::<[Wrapping<u32>; 4]>::default(),
            count: 0,
        }
    }

    /// Return the mantissa and exponent components of this integer.
    /// `value` ≈ `mantissa` * 2^`exponent`
    pub fn p(&self) -> (f64, i32) {
        let sep = num::cast::<u64, f64>(0x100000000).unwrap();
        let mut rv = (0.0, 0);
        match self.size() {
            0 => return rv,
            1 => {
                rv.0 = num::cast::<u32, f64>(self.chunks.get(0).unwrap().0).unwrap();
            }
            2 => {
                rv.0 = num::cast::<u32, f64>(self.chunks.get(1).unwrap().0).unwrap() * sep
                    + num::cast::<u32, f64>(self.chunks.get(0).unwrap().0).unwrap();
            }
            _ => {
                //println!("{:?}",self);
                //println!("->p()");
                // why does not self.chunks.len() match self.size()?
                //let skip = self.chunks.len()-self.size();
                for v in self.chunks.iter().rev().take(3) {
                    //println!("i={}",i);
                    rv.0 *= sep;
                    //println!("{}", rv.0);
                    rv.0 += num::cast::<u32, f64>(v.0).unwrap();
                    //println!("{}", rv.0);
                }
                rv.1 = ((self.size() - 3) << 5) as i32;
            }
        }
        if self.count < 0 {
            rv.0 = -rv.0;
        }
        rv
    }

    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        self.count > 0
    }

    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        self.count < 0
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.count == 0
    }

    /// negates value
    ///```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 41232131332_f64;
    /// let mut a = ExtendedInt::from(aa as i64);
    /// a.negate();
    /// approx::assert_ulps_eq!(a.d(), -aa);
    /// ```
    pub fn negate(&mut self) {
        assert_eq!(self.chunks.len(), self.size());
        self.count = -self.count;
    }

    /// converts to f64
    pub fn d(&self) -> f64 {
        let p = self.p();
        libm::ldexp(p.0, p.1)
    }

    /// converts to ExtendedExponentFpt::<f64>
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 41232131332_f64;
    /// let mut a = ExtendedInt::from(aa as i64);
    /// let e = a.e();
    /// approx::assert_ulps_eq!(e.d(), aa);
    /// ```
    pub fn e(&self) -> ExtendedExponentFpt<f64> {
        let p = self.p();
        ExtendedExponentFpt::<f64>::new2(p.0, p.1)
    }

    /// return the number of words in 'self.count'
    pub fn size(&self) -> usize {
        //let rv = self.count.abs() as usize;
        //assert_eq!(rv,self.chunks.len());
        //rv
        // TODO replace this with return self.chunks.len() when stable
        //assert_eq!(self.chunks.len(), self.count.abs()as usize);
        self.count.abs() as usize
    }

    /// this method assumes self is an empty object
    fn add_others(&mut self, e1: &Self, e2: &Self) {
        //println!("->add_others {:?} {:?} {:?}", self, e1, e2);
        if e1.count == 0 {
            self.count = e2.count;
            self.chunks = e2.chunks.clone();
            return;
        }
        if e2.count == 0 {
            self.count = e1.count;
            self.chunks = e1.chunks.clone();
            return;
        }
        if (e1.count > 0) ^ (e2.count > 0) {
            self.dif_slice(&e1.chunks, e1.size(), &e2.chunks, e2.size(), false);
        } else {
            self.add_slice(&e1.chunks, e1.size(), &e2.chunks, e2.size());
        }
        if e1.count < 0 {
            self.count = -self.count;
        }
    }

    fn add_slice(&mut self, c1: &[Wrapping<u32>], sz1: usize, c2: &[Wrapping<u32>], sz2: usize) {
        //println!("->add_slice {:?} {:?} {:?}", self, c1, c2);
        if sz1 < sz2 {
            self.add_slice(c2, sz2, c1, sz1);
            return;
        }
        self.count = sz1 as i32;
        let mut temp = 0_u64;

        for _i in self.chunks.len()..sz1 {
            self.chunks.push(Wrapping(0));
        }
        for i in 0..sz2 {
            temp += (c1[i].0 as u64) + (c2[i].0 as u64);
            self.chunks[i] = Wrapping(temp as u32);
            temp >>= 32;
        }
        for i in sz2..sz1 {
            temp += c1[i].0 as u64;
            self.chunks[i] = Wrapping(temp as u32);
            temp >>= 32;
        }
        if temp != 0 {
            if self.chunks.len() <= self.count as usize {
                self.chunks.push(Wrapping(temp as u32));
            } else {
                self.chunks[self.count as usize] = Wrapping(temp as u32);
            }
            self.count += 1;
        }
        // Todo: remove these asserts when stable
        assert!(self.count >= 0);
        assert_eq!(self.chunks.len(), self.count as usize);
    }

    /// this method assumes self is an empty object
    fn dif_other(&mut self, e1: &Self, e2: &Self) {
        //println!("->dif_other {:?} {:?} {:?}", self, e1, e2);
        if e1.count == 0 {
            self.count = e2.count;
            self.chunks = e2.chunks.clone();
            self.count = -self.count;
            //println!("<-dif_other#1 {:?}", self);
            return;
        }
        if e2.count == 0 {
            self.count = e1.count;
            self.chunks = e1.chunks.clone();
            //println!("<-dif_other#2 {:?}", self);
            return;
        }
        if (e1.count > 0) ^ (e2.count > 0) {
            self.add_slice(&e1.chunks, e1.size(), &e2.chunks, e2.size());
        } else {
            self.dif_slice(&e1.chunks, e1.size(), &e2.chunks, e2.size(), false);
        }
        if e1.count < 0 {
            self.count = -self.count;
        }
        //println!("<-dif_other#3 {:?}", self);
    }

    fn dif_slice(
        &mut self,
        c1: &[Wrapping<u32>],
        sz1: usize,
        c2: &[Wrapping<u32>],
        sz2: usize,
        rec: bool,
    ) {
        //println!("->dif_slice {:?} count:{} c1:{:?} sz1:{} c2:{:?} sz2:{} rec:{}", self, self.count, c1, sz1, c2, sz2, rec);
        let mut sz2 = sz2;
        let mut sz1 = sz1;
        if sz1 < sz2 {
            self.dif_slice(c2, sz2, c1, sz1, true);
            self.count = -self.count;
            return;
        } else if (sz1 == sz2) && !rec {
            loop {
                sz1 -= 1;
                if c1[sz1] < c2[sz1] {
                    sz1 += 1;
                    self.dif_slice(c2, sz1, c1, sz1, true);
                    self.count = -self.count;
                    return;
                } else if c1[sz1] > c2[sz1] {
                    sz1 += 1;
                    break;
                }
                if sz1 == 0 {
                    break;
                }
            }
            if sz1 == 0 {
                self.count = 0;
                return;
            }
            sz2 = sz1;
        }
        self.count = (sz1 - 1) as i32;
        let mut flag = false;

        for _i in self.chunks.len()..sz1 {
            self.chunks.push(Wrapping(0));
        }

        for i in 0..sz2 {
            self.chunks[i] = c1[i] - c2[i] - if flag { Wrapping(1) } else { Wrapping(0) };
            flag = (c1[i] < c2[i]) || ((c1[i] == c2[i]) && flag);
        }
        for i in sz2..sz1 {
            self.chunks[i] = c1[i] - if flag { Wrapping(1) } else { Wrapping(0) };
            flag = (c1[i].0 == 0) && flag;
        }
        if self.chunks[self.count as usize].0 != 0 {
            self.count += 1;
            if (self.count as usize) > self.chunks.len() {
                self.chunks.push(Wrapping(0));
            }
        }
        if (self.count as usize) < self.chunks.len() {
            let _ = self.chunks.pop();
        }
        // Todo: remove these asserts when stable
        assert!(self.count >= 0);
        assert_eq!(self.chunks.len(), self.count as usize);
        //println!("<-dif_slice#1 {:?}", self);
    }

    fn mul_other(&mut self, e1: &Self, e2: &Self) {
        //println!("->mul_other {:?} {:?} {:?}", self, e1, e2);

        if e1.count == 0 || e2.count == 0 {
            self.count = 0;
            //println!("<-mul_other#1 {:?}", self);
            return;
        }
        self.mul_slice(&e1.chunks, e1.size(), &e2.chunks, e2.size());
        if (e1.count > 0) ^ (e2.count > 0) {
            self.count = -self.count;
        }
        //println!("<-mul_other#2 {:?}", self);
    }

    #[allow(unused_assignments)]
    fn mul_slice(&mut self, c1: &[Wrapping<u32>], sz1: usize, c2: &[Wrapping<u32>], sz2: usize) {
        //println!("->mul_slice {:?} c1:{:?} sz1:{} c2:{:?} sz2:{}", self, c1, sz1, c2, sz2);

        let mut cur: u64 = 0;
        let mut nxt: u64 = 0;
        let mut tmp: u64 = 0;

        self.count = (sz1 + sz2 - 1_usize) as i32;

        for _i in self.chunks.len()..(self.count as usize) {
            self.chunks.push(Wrapping(0));
        }

        //dbg!(self.count);
        for shift in 0..(self.count as usize) {
            nxt = 0;
            for first in 0..shift + 1 {
                if first >= sz1 {
                    //println!("mul_slice brk {:?}", self);
                    break;
                }
                let second = shift - first;
                if second >= sz2 {
                    //println!("mul_slice cnt {:?}", self);
                    continue;
                }

                tmp = (c1[first].0 as u64) * (c2[second].0 as u64);
                cur += tmp & 0xFFFF_FFFF;
                nxt += tmp >> 32;

                //println!("shift:{} first:{}, second:{}",shift, first, second);
                //println!("cur:{:0>16X}", cur );
                //println!("nxt:{:0>16X}", nxt);
            }

            self.chunks[shift] = Wrapping((cur & 0xFFFF_FFFF) as u32);
            //println!("self.chunks[shift]:{:0>8X}", self.chunks[shift]);
            //println!("self.chunks[shift]:{:}\n", self.chunks[shift]);
            cur = nxt + (cur >> 32);
        }
        if cur != 0 {
            //&& (self.count != N)) {
            assert_eq!(self.count as usize, self.chunks.len());
            self.chunks.push(Wrapping(cur as u32));
            //self.chunks[self.count as usize] = Wrapping(cur as u32);
            self.count += 1;
        }
        // Todo: remove these asserts when stable

        assert!(self.count >= 0);
        assert_eq!(self.chunks.len(), self.count as usize);
        //println!("<-mul_slice {:?}", self);
    }
}

impl Default for ExtendedInt {
    fn default() -> Self {
        Self::from(0_i32)
    }
}

impl ops::Add for ExtendedInt {
    type Output = Self;
    /// Adds `self` to `that` returning a new object with the result
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 472_f64;
    /// let bb = 147_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let b = ExtendedInt::from(bb as i64);
    /// let c = a+b;
    /// approx::assert_ulps_eq!(c.d(), aa+bb);
    ///```
    fn add(self, that: Self) -> Self {
        let mut rv = ExtendedInt::default();
        rv.add_others(&self, &that);
        rv
    }
}

impl<'a, 'b> ops::Add<&'b ExtendedInt> for &'a ExtendedInt {
    type Output = ExtendedInt;
    /// Adds `self` to `that` returning a new object with the result
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 472_f64;
    /// let bb = 147_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let b = ExtendedInt::from(bb as i64);
    /// let c = &a+&b;
    /// approx::assert_ulps_eq!(c.d(), aa+bb);
    ///```
    fn add(self, that: &'b ExtendedInt) -> ExtendedInt {
        let mut rv = ExtendedInt::default();
        rv.add_others(&self, that);
        rv
    }
}

impl<'b> ops::Add<&'b ExtendedInt> for ExtendedInt {
    type Output = ExtendedInt;
    /// Adds `self` to `that` returning a new object with the result
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 472_f64;
    /// let bb = 147_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let b = ExtendedInt::from(bb as i64);
    /// let c = a+&b;
    /// approx::assert_ulps_eq!(c.d(), aa+bb);
    ///```
    fn add(self, that: &'b ExtendedInt) -> ExtendedInt {
        let mut rv = ExtendedInt::default();
        rv.add_others(&self, that);
        rv
    }
}

impl ops::Sub for ExtendedInt {
    type Output = Self;
    /// Subtracts `that` from `self` returning a new object with the result
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 4727377593577731_f64;
    /// let bb = 759935777381_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let b = ExtendedInt::from(bb as i64);
    /// let c = a-b;
    /// approx::assert_ulps_eq!(c.d(), aa-bb);
    ///```
    fn sub(self, that: Self) -> Self {
        let mut rv = ExtendedInt::default();
        rv.dif_other(&self, &that);
        rv
    }
}

impl<'a, 'b> ops::Sub<&'b ExtendedInt> for &'a ExtendedInt {
    type Output = ExtendedInt;
    /// Subtracts `that` from `self` returning a new object with the result
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 4727377593577731_f64;
    /// let bb = 759935777381_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let b = ExtendedInt::from(bb as i64);
    /// let c = &a-&b;
    /// approx::assert_ulps_eq!(c.d(), aa-bb);
    ///```
    fn sub(self, that: &'b ExtendedInt) -> ExtendedInt {
        let mut rv = ExtendedInt::default();
        rv.dif_other(&self, &that);
        rv
    }
}

impl<'b> ops::Sub<&'b ExtendedInt> for ExtendedInt {
    type Output = ExtendedInt;
    /// Subtracts `that` from `self` returning a new object with the result
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 4727377593577731_f64;
    /// let bb = 759935777381_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let b = ExtendedInt::from(bb as i64);
    /// let c = a-&b;
    /// approx::assert_ulps_eq!(c.d(), aa-bb);
    ///```
    fn sub(self, that: &'b ExtendedInt) -> ExtendedInt {
        let mut rv = ExtendedInt::default();
        rv.dif_other(&self, &that);
        rv
    }
}

impl ops::Mul for ExtendedInt {
    type Output = Self;
    /// Multiplies `self` with `self` returning a new object with the result
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 4727377593577731_f64;
    /// let bb = 759935777381_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let b = ExtendedInt::from(bb as i64);
    /// let c = a*b;
    /// approx::assert_ulps_eq!(c.d(), aa*bb);
    ///```
    fn mul(self, that: Self) -> Self {
        let mut rv = ExtendedInt::default();
        rv.mul_other(&self, &that);
        rv
    }
}

impl<'a, 'b> ops::Mul<&'b ExtendedInt> for &'a ExtendedInt {
    type Output = ExtendedInt;
    /// Multiplies `self` with `self` returning a new object with the result
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 4727377593577731_f64;
    /// let bb = 759935777381_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let b = ExtendedInt::from(bb as i64);
    /// let c = &a*&b;
    /// approx::assert_ulps_eq!(c.d(), aa*bb);
    ///```
    fn mul(self, that: &'b ExtendedInt) -> ExtendedInt {
        let mut rv = ExtendedInt::default();
        rv.mul_other(&self, &that);
        rv
    }
}

impl<'b> ops::Mul<&'b ExtendedInt> for ExtendedInt {
    type Output = ExtendedInt;
    /// Multiplies `self` with `self` returning a new object with the result
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 4727377593577731_f64;
    /// let bb = 759935777381_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let b = ExtendedInt::from(bb as i64);
    /// let c = a*&b;
    /// approx::assert_ulps_eq!(c.d(), aa*bb);
    ///```
    fn mul(self, that: &'b ExtendedInt) -> ExtendedInt {
        let mut rv = ExtendedInt::default();
        rv.mul_other(&self, &that);
        rv
    }
}

impl ops::Neg for ExtendedInt {
    type Output = Self;
    /// Negates value of `self` returning a self with the result
    /// ```
    /// # use boostvoronoi::robust_fpt::ExtendedInt;
    ///
    /// let aa = 4727377593577731_f64;
    /// let a = -ExtendedInt::from(aa as i64);
    /// approx::assert_ulps_eq!(a.d(), -aa);
    ///```
    fn neg(mut self) -> Self {
        //let mut rv = self.clone();
        self.count = -self.count;
        self
    }
}

impl fmt::Debug for ExtendedInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0}", self.d())
        /*if self.count == 0 {
            write!(f, "ExtendedInt:0x0")
        } else {
            write!(f, "ExtendedInt:0x")?;
            for i in self.chunks.iter().rev() {
                write!(f, "{:0>8X}_", *i)?;
            }
            write!(f, " count:{}", self.count)?;
            Ok(())
        }*/
    }
}
