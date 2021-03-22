// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.75.0 to Rust in 2020 by Eadf (github.com/eadf)

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
//!
mod extendedint_tests;
mod robustdif_tests;
mod robustfpt_tests;

use super::extended_exp_fpt as EX;
use super::extended_int as EI;
use super::OutputType;
use num::{Float, NumCast};
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
    fn i_to_f(that: &EI::ExtendedInt) -> EX::ExtendedExponentFpt<f64> {
        EX::ExtendedExponentFpt::<f64>::from(that)
    }

    /// Evaluates expression (re = 4 EPS):
    /// A[0] * sqrt(B[0]).
    pub fn eval1(
        &self,
        a: &[EI::ExtendedInt],
        b: &[EI::ExtendedInt],
    ) -> EX::ExtendedExponentFpt<f64> {
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
    pub fn eval2(
        &self,
        a: &[EI::ExtendedInt],
        b: &[EI::ExtendedInt],
    ) -> EX::ExtendedExponentFpt<f64> {
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
    pub fn eval3(
        &self,
        a: &[EI::ExtendedInt],
        b: &[EI::ExtendedInt],
    ) -> EX::ExtendedExponentFpt<f64> {
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
        let mut ta = [EI::ExtendedInt::zero(), EI::ExtendedInt::zero()];
        let mut tb = [EI::ExtendedInt::zero(), EI::ExtendedInt::zero()];

        ta[0] = &a[0] * &a[0] * &b[0] + &a[1] * &a[1] * &b[1] - &a[2] * &a[2] * &b[2];
        tb[0] = EI::ExtendedInt::from(1);
        ta[1] = &a[0] * &a[1] * &EI::ExtendedInt::from(2_i32);
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
    pub fn eval4(
        &self,
        a: &[EI::ExtendedInt],
        b: &[EI::ExtendedInt],
    ) -> EX::ExtendedExponentFpt<f64> {
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
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
        ];
        let mut tb = [
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
            EI::ExtendedInt::zero(),
        ];

        ta[0] = &a[0] * &a[0] * &b[0] + &a[1] * &a[1] * &b[1]
            - &a[2] * &a[2] * &b[2]
            - &a[3] * &a[3] * &b[3];
        tb[0] = EI::ExtendedInt::from(1_i32);
        ta[1] = &a[0] * &a[1] * &EI::ExtendedInt::from(2_i32);
        tb[1] = &b[0] * &b[1];
        ta[2] = &a[2] * &a[3] * &EI::ExtendedInt::from(-2_i32);
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
        A: &[EI::ExtendedInt],
        B: &[EI::ExtendedInt],
    ) -> EX::ExtendedExponentFpt<f64> {
        let mut cA: [EI::ExtendedInt; 2] = [EI::ExtendedInt::zero(), EI::ExtendedInt::zero()];
        let mut cB: [EI::ExtendedInt; 2] = [EI::ExtendedInt::zero(), EI::ExtendedInt::zero()];

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
        cB[0] = EI::ExtendedInt::from(1);
        cA[1] = (&A[0] * &A[1] - &A[2] * &A[3]) * &EI::ExtendedInt::from(2_i32);
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
        A: &[EI::ExtendedInt],
        B: &[EI::ExtendedInt],
    ) -> EX::ExtendedExponentFpt<f64> {
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
        if A[3].is_zero() {
            let lh = self.eval2(A, B);
            cA[0] = EI::ExtendedInt::from(1);
            cB[0] = &B[0] * &B[1];
            cA[1] = B[2].clone();
            cB[1] = EI::ExtendedInt::from(1);
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
            cB[0] = EI::ExtendedInt::from(1_i32);
            cA[1] = &A[0] * &A[1] * &EI::ExtendedInt::from(2_i32) - &A[2] * &A[2] * &B[3];
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
        cA[0] = EI::ExtendedInt::from(1);
        cB[0] = &B[0] * &B[1];
        cA[1] = B[2].clone();
        cB[1] = EI::ExtendedInt::from(1);
        let rh = self.eval1(&A[2..], &B[3..]) * (self.eval2(&cA, &cB).sqrt());
        cA[0] = A[0].clone();
        cB[0] = B[0].clone();
        cA[1] = A[1].clone();
        cB[1] = B[1].clone();
        cA[2] = A[3].clone();
        cB[2] = EI::ExtendedInt::from(1);
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
        cA[0] = &A[3] * &A[0] * &EI::ExtendedInt::from(2_i32);
        cA[1] = &A[3] * &A[1] * &EI::ExtendedInt::from(2_i32);
        cA[2] = &A[0] * &A[0] * &B[0] + &A[1] * &A[1] * &B[1] + &A[3] * &A[3]
            - &A[2] * &A[2] * &B[2] * &B[3];
        cA[3] = &A[0] * &A[1] * &EI::ExtendedInt::from(2_i32) - &A[2] * &A[2] * &B[3];
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
