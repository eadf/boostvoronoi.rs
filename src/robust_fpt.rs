// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! Module containing robust floating points utilities.

#[cfg(test)]
mod extendedint_tests;
#[cfg(test)]
mod robustdif_tests;
#[cfg(test)]
mod robustfpt_tests;

use super::extended_exp_fpt as EX;
use super::extended_int as EI;
#[allow(unused_imports)]
use crate::{t, tln};
use num::{Float, Zero};
use ordered_float::OrderedFloat;
use std::fmt;
use std::marker::PhantomData;
use std::ops;

/// Rounding error is at most 1 EPS.
pub const ROUNDING_ERROR: f64 = 1_f64;

/// Is positive method.
/// IMPORTANT!!!!! in c++ boost voronoi implementation zero values can't be positive.
#[inline(always)]
fn is_pos_f(fpv: f64) -> bool {
    fpv > 0_f64
}

/// Is negative method.
/// IMPORTANT!!!!! in c++ boost voronoi implementation zero values can't be negative.
#[inline(always)]
fn is_neg_f(fpv: f64) -> bool {
    fpv < 0_f64
}

/// Geometry predicates with floating-point variables usually require
/// high-precision predicates to retrieve the correct result.
/// Epsilon robust predicates give the result within some epsilon relative
/// error, but are a lot faster than high-precision predicates.
/// To make algorithm robust and efficient epsilon robust predicates are
/// used at the first step. In case of the undefined result high-precision
/// arithmetic is used to produce required robustness. This approach
/// requires exact computation of epsilon intervals within which epsilon
/// robust predicates have undefined value.
/// There are two ways to measure an error of floating-point calculations:
/// relative error and ULPs (units in the last place).
/// Let EPS be machine epsilon, then next inequalities have place:
/// 1 EPS <= 1 ULP <= 2 EPS (1), 0.5 ULP <= 1 EPS <= 1 ULP (2).
/// ULPs are good for measuring rounding errors and comparing values.
/// Relative errors are good for computation of general relative
/// error of formulas or expressions. So to calculate epsilon
/// interval within which epsilon robust predicates have undefined result
/// next schema is used:
///     1) Compute rounding errors of initial variables using ULPs;
///     2) Transform ULPs to epsilons using upper bound of the (1);
///     3) Compute relative error of the formula using epsilon arithmetic;
///     4) Transform epsilon to ULPs using upper bound of the (2);
/// In case two values are inside undefined ULP range use high-precision
/// arithmetic to produce the correct result, else output the result.
/// Look at almost_equal function to see how two floating-point variables
/// are checked to fit in the ULP range.
/// If A has relative error of r(A) and B has relative error of r(B) then:
///     1) r(A + B) <= max(r(A), r(B)), for A * B >= 0;
///     2) r(A - B) <= B*r(A)+A*r(B)/(A-B), for A * B >= 0;
///     2) r(A * B) <= r(A) + r(B);
///     3) r(A / B) <= r(A) + r(B);
/// In addition rounding error should be added, that is always equal to
/// 0.5 ULP or at most 1 epsilon. As you might see from the above formulas
/// subtraction relative error may be extremely large, that's why
/// epsilon robust comparator class is used to store floating point values
/// and compute subtraction as the final step of the evaluation.
/// For further information about relative errors and ULPs try this link:
/// <http://docs.sun.com/source/806-3568/ncg_goldberg.html>
///
#[derive(Copy, Clone)]
pub struct RobustFpt {
    fpv_: f64,
    re_: OrderedFloat<f64>,
}

impl Default for RobustFpt {
    fn default() -> Self {
        Self {
            fpv_: 0_f64,
            re_: OrderedFloat(0_f64),
        }
    }
}

impl RobustFpt {
    pub fn new_1(fpv: f64) -> Self {
        Self {
            fpv_: fpv,
            re_: OrderedFloat(0_f64),
        }
    }

    pub fn new_2(fpv: f64, error: f64) -> Self {
        Self {
            fpv_: fpv,
            re_: OrderedFloat(error),
        }
    }

    #[inline(always)]
    pub fn fpv(&self) -> f64 {
        self.fpv_
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn re(&self) -> f64 {
        self.re_.into_inner()
    }

    #[inline(always)]
    pub fn ulp(&self) -> f64 {
        self.re()
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
    /// let a = robust_fpt::RobustFpt::new_1(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::RobustFpt::new_1(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = f64::MIN_POSITIVE;
    /// let a = robust_fpt::RobustFpt::new_1(aa);
    /// assert_eq!(a.is_pos(), aa.is_sign_positive());
    /// ```
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        is_pos_f(self.fpv_)
    }

    /// Is negative method.
    /// IMPORTANT!!!!! in c++ boost voronoi implementation zero values can't be negative.
    /// ```
    /// # use boostvoronoi::robust_fpt;
    ///
    /// println!("is_neg()");
    /// let aa:f64 = 0_f64;
    /// let a = robust_fpt::RobustFpt::new_1(aa);
    /// assert_eq!(a.is_neg(), aa.is_sign_negative());
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::RobustFpt::new_1(aa);
    /// assert_eq!(a.is_neg(), false);
    /// ```
    //#[inline(always)]
    pub fn is_neg(&self) -> bool {
        is_neg_f(self.fpv_)
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn is_zero(&self) -> bool {
        self.fpv_.is_zero()
    }

    pub fn sqrt(&self) -> RobustFpt {
        Self {
            //fpv_: Self::get_sqrt(self.fpv_),
            fpv_: self.fpv_.sqrt(),
            // self.re_ * 0.5 + ROUNDING_ERROR
            re_: self.re_ * OrderedFloat(0.5f64) + OrderedFloat(ROUNDING_ERROR),
        }
    }
}

impl fmt::Debug for RobustFpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:.12}[{:.12}]", self.fpv_, self.re_))
    }
}

impl ops::Add<RobustFpt> for RobustFpt {
    type Output = RobustFpt;

    fn add(self, _rhs: RobustFpt) -> Self {
        let fpv: f64 = self.fpv_ + _rhs.fpv_;
        let re = if (!self.is_neg() && !_rhs.is_neg()) || (!self.is_pos() && !_rhs.is_pos()) {
            std::cmp::max(self.re_, _rhs.re_) + ROUNDING_ERROR
        } else {
            let mut temp =
                (self.fpv_ * self.re_.into_inner() - _rhs.fpv_ * _rhs.re_.into_inner()) / fpv;
            if is_neg_f(temp) {
                temp = -temp;
            } else if temp.is_nan() {
                temp = f64::INFINITY;
            }

            OrderedFloat(temp + ROUNDING_ERROR)
        };
        #[cfg(feature = "console_debug")]
        {
            if !fpv.is_finite() {
                tln!("!fpv.is_finite() self:{:?}, _rhs:{:?}", self, _rhs);
            }
            if re.is_nan() {
                tln!("re.is_nan() self:{:?}, _rhs:{:?}", self, _rhs);
            }
            assert!(fpv.is_finite());
            assert!(!re.is_nan());
        }
        Self { fpv_: fpv, re_: re }
    }
}

impl ops::AddAssign<RobustFpt> for RobustFpt {
    fn add_assign(&mut self, _rhs: RobustFpt) {
        #[cfg(feature = "console_debug")]
        {
            assert!(self.fpv_.is_finite());
            assert!(_rhs.fpv_.is_finite());
        }

        let fpv: f64 = self.fpv_ + _rhs.fpv_;
        let re = if (!self.is_neg() && !_rhs.is_neg()) || (!self.is_pos() && !_rhs.is_pos()) {
            std::cmp::max(self.re_, _rhs.re_) + OrderedFloat(ROUNDING_ERROR)
        } else {
            let mut temp =
                (self.fpv_ * self.re_.into_inner() - _rhs.fpv_ * _rhs.re_.into_inner()) / fpv;
            if is_neg_f(temp) {
                temp = -temp;
            } else if temp.is_nan() {
                temp = f64::INFINITY;
            }
            OrderedFloat(temp + ROUNDING_ERROR)
        };
        self.fpv_ = fpv;
        self.re_ = re;
        #[cfg(feature = "console_debug")]
        assert!(self.fpv_.is_finite());
    }
}

impl ops::Mul<f64> for RobustFpt {
    type Output = RobustFpt;
    // Todo make this more efficient
    fn mul(self, _rhs: f64) -> Self {
        let _rhs = RobustFpt::new_1(_rhs);
        self * _rhs
    }
}

impl ops::Mul<RobustFpt> for RobustFpt {
    type Output = RobustFpt;

    fn mul(self, _rhs: RobustFpt) -> Self {
        let fpv: f64 = self.fpv_ * _rhs.fpv_;
        let re: OrderedFloat<f64> = self.re_ + _rhs.re_ + OrderedFloat(ROUNDING_ERROR);

        Self { fpv_: fpv, re_: re }
    }
}

impl ops::MulAssign<RobustFpt> for RobustFpt {
    fn mul_assign(&mut self, _rhs: RobustFpt) {
        self.re_ = self.re_ + _rhs.re_ + OrderedFloat(ROUNDING_ERROR);
        self.fpv_ = self.fpv_ * _rhs.fpv_;

        #[cfg(feature = "console_debug")]
        {
            assert!(self.fpv_.is_finite());
            assert!(!self.re_.is_nan());
        }
    }
}

impl ops::Sub<RobustFpt> for RobustFpt {
    type Output = RobustFpt;

    fn sub(self, _rhs: RobustFpt) -> Self {
        #[cfg(feature = "console_debug")]
        let old_self = self;

        let fpv: f64 = self.fpv_ - _rhs.fpv_;
        let re = if (!self.is_neg() && !_rhs.is_pos()) || (!self.is_pos() && !_rhs.is_neg()) {
            std::cmp::max(self.re_, _rhs.re_) + OrderedFloat(ROUNDING_ERROR)
        } else {
            let mut temp =
                (self.fpv_ * self.re_.into_inner() + _rhs.fpv_ * _rhs.re_.into_inner()) / fpv;
            if is_neg_f(temp) {
                temp = -temp;
            } else if temp.is_nan() {
                temp = f64::INFINITY;
            }
            OrderedFloat(temp) + OrderedFloat(ROUNDING_ERROR)
        };
        #[cfg(feature = "console_debug")]
        {
            if !self.fpv_.is_finite() {
                tln!(
                    "!self.fpv.is_finite() self:{:?}, _rhs:{:?} old_self:{:?}",
                    self,
                    _rhs,
                    old_self
                );
            }
            if self.re_.is_nan() {
                tln!(
                    "self.re.is_nan() self:{:?}, _rhs:{:?} old_self:{:?}",
                    self,
                    _rhs,
                    old_self
                );
            }
            assert!(self.fpv_.is_finite());
            assert!(!self.re_.is_nan());
        }
        Self { fpv_: fpv, re_: re }
    }
}

impl ops::SubAssign<RobustFpt> for RobustFpt {
    fn sub_assign(&mut self, _rhs: RobustFpt) {
        #[cfg(feature = "console_debug")]
        let old_self = *self;

        let fpv = self.fpv_ - _rhs.fpv_;
        if (!self.is_neg() && !_rhs.is_pos()) || (!self.is_pos() && !_rhs.is_neg()) {
            self.re_ = std::cmp::max(self.re_, _rhs.re_) + OrderedFloat(ROUNDING_ERROR);
        } else {
            let mut temp: f64 =
                (self.fpv_ * self.re_.into_inner() + _rhs.fpv_ * _rhs.re_.into_inner()) / fpv;
            if is_neg_f(temp) {
                temp = -temp;
            } else if temp.is_nan() {
                temp = f64::INFINITY;
            }
            self.re_ = OrderedFloat(temp) + OrderedFloat(ROUNDING_ERROR);
        }
        self.fpv_ = fpv;
        #[cfg(feature = "console_debug")]
        {
            if !self.fpv_.is_finite() {
                tln!(
                    "!self.fpv.is_finite() self:{:?}, _rhs:{:?} old_self:{:?}",
                    self,
                    _rhs,
                    old_self
                );
            }
            if self.re_.is_nan() {
                tln!(
                    "self.re.is_nan() self:{:?}, _rhs:{:?} old_self:{:?}",
                    self,
                    _rhs,
                    old_self
                );
            }
            assert!(self.fpv_.is_finite());
            assert!(!self.re_.is_nan());
        }
    }
}

impl ops::Div<f64> for RobustFpt {
    type Output = RobustFpt;

    fn div(self, _rhs: f64) -> Self {
        let _rhs = RobustFpt::new_1(_rhs);
        self / _rhs
    }
}

impl ops::Div<RobustFpt> for RobustFpt {
    type Output = RobustFpt;

    fn div(self, _rhs: RobustFpt) -> Self {
        let fpv: f64 = self.fpv_ / _rhs.fpv_;
        let re = self.re_ + _rhs.re_ + OrderedFloat(ROUNDING_ERROR);
        #[cfg(feature = "console_debug")]
        {
            assert!(fpv.is_finite());
            assert!(!re.is_nan());
        }
        Self { fpv_: fpv, re_: re }
    }
}

impl ops::DivAssign<RobustFpt> for RobustFpt {
    fn div_assign(&mut self, _rhs: RobustFpt) {
        self.re_ = self.re_ + _rhs.re_ + OrderedFloat(ROUNDING_ERROR);
        self.fpv_ = self.fpv_ / _rhs.fpv_;
        #[cfg(feature = "console_debug")]
        {
            assert!(self.fpv_.is_finite());
            assert!(!self.re_.is_nan());
        }
    }
}

impl ops::Neg for RobustFpt {
    type Output = RobustFpt;

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

#[derive(Copy, Clone, Default)]
pub struct RobustDif {
    positive_sum_: RobustFpt,
    negative_sum_: RobustFpt,
}

impl RobustDif {
    pub fn new() -> Self {
        Self {
            positive_sum_: RobustFpt::default(),
            negative_sum_: RobustFpt::default(),
        }
    }

    // TODO take & reference to other
    pub fn new_from(other: RobustDif) -> Self {
        Self {
            positive_sum_: other.positive_sum_,
            negative_sum_: other.negative_sum_,
        }
    }

    #[allow(dead_code)]
    pub fn new_from_2(a: &RobustFpt, b: &RobustFpt) -> Self {
        Self {
            positive_sum_: *a,
            negative_sum_: *b,
        }
    }

    #[allow(dead_code)]
    pub fn new_1(value: f64) -> Self {
        if is_pos_f(value) {
            Self {
                positive_sum_: RobustFpt::new_1(value),
                negative_sum_: RobustFpt::default(),
            }
        } else {
            Self {
                positive_sum_: RobustFpt::default(),
                negative_sum_: RobustFpt::new_1(value),
            }
        }
    }

    #[allow(dead_code)]
    pub fn new_2(pos: f64, neg: f64) -> Self {
        #[cfg(feature = "console_debug")]
        {
            assert!(!pos.is_sign_negative());
            assert!(!neg.is_sign_negative());
        }
        Self {
            positive_sum_: RobustFpt::new_1(pos),
            negative_sum_: RobustFpt::new_1(neg),
        }
    }

    pub fn dif(&self) -> RobustFpt {
        self.positive_sum_ - self.negative_sum_
    }

    #[inline]
    pub fn positive(&self) -> RobustFpt {
        self.positive_sum_
    }

    #[inline]
    // neg() will collide with the trait RobustDif
    pub fn negative(&self) -> RobustFpt {
        self.negative_sum_
    }

    #[inline]
    fn swap(&mut self) {
        std::mem::swap(&mut self.positive_sum_, &mut self.negative_sum_);
    }
}

impl ops::Neg for RobustDif {
    type Output = RobustDif;

    fn neg(self) -> Self {
        Self {
            positive_sum_: self.negative_sum_,
            negative_sum_: self.positive_sum_,
        }
    }
}

impl ops::Add<RobustDif> for RobustDif {
    type Output = RobustDif;

    fn add(self, _rhs: RobustDif) -> Self {
        Self {
            positive_sum_: self.positive_sum_ + _rhs.positive_sum_,
            negative_sum_: self.negative_sum_ + _rhs.negative_sum_,
        }
    }
}

impl ops::AddAssign<RobustDif> for RobustDif {
    fn add_assign(&mut self, _rhs: RobustDif) {
        self.positive_sum_ += _rhs.positive_sum_;
        self.negative_sum_ += _rhs.negative_sum_;
    }
}

impl ops::AddAssign<RobustFpt> for RobustDif {
    fn add_assign(&mut self, _rhs: RobustFpt) {
        if !_rhs.is_neg() {
            self.positive_sum_ += _rhs;
        } else {
            self.negative_sum_ -= _rhs;
        }
    }
}

impl ops::Sub<RobustDif> for RobustDif {
    type Output = RobustDif;

    fn sub(self, _rhs: RobustDif) -> Self {
        Self {
            positive_sum_: self.positive_sum_ + _rhs.negative_sum_,
            negative_sum_: self.negative_sum_ + _rhs.positive_sum_,
        }
    }
}

/// Converts to RobustDif from RobustFpt
/// ```
/// # use boostvoronoi::robust_fpt::*;
/// let s = RobustFpt::new_1(1.0);
/// let d = RobustDif::from(s);
/// assert_eq!(s.fpv(),d.dif().fpv());
/// ```
impl From<RobustFpt> for RobustDif {
    fn from(value: RobustFpt) -> RobustDif {
        if value.is_neg() {
            RobustDif {
                positive_sum_: RobustFpt::default(),
                negative_sum_: -value,
            }
        } else {
            RobustDif {
                positive_sum_: value,
                negative_sum_: RobustFpt::default(),
            }
        }
    }
}

impl ops::Sub<RobustFpt> for RobustDif {
    type Output = RobustDif;

    fn sub(self, rhs: RobustFpt) -> Self {
        let rhs = RobustDif::from(rhs);
        Self {
            positive_sum_: self.positive_sum_ + rhs.negative_sum_,
            negative_sum_: self.negative_sum_ + rhs.positive_sum_,
        }
    }
}

impl ops::SubAssign<RobustDif> for RobustDif {
    fn sub_assign(&mut self, _rhs: RobustDif) {
        self.positive_sum_ += _rhs.negative_sum_;
        self.negative_sum_ += _rhs.positive_sum_;
    }
}

impl ops::SubAssign<RobustFpt> for RobustDif {
    fn sub_assign(&mut self, _rhs: RobustFpt) {
        #[cfg(feature = "console_debug")]
        {
            assert!(self.dif().fpv().is_finite());
            assert!(_rhs.fpv().is_finite());
        }
        //dbg!(&self, &_rhs);
        if !_rhs.is_neg() {
            self.negative_sum_ += _rhs;
        } else {
            self.positive_sum_ -= _rhs;
        }
    }
}

impl ops::Mul<RobustDif> for RobustDif {
    type Output = RobustDif;

    fn mul(self, _rhs: RobustDif) -> Self {
        Self {
            positive_sum_: self.positive_sum_ * _rhs.positive_sum_,
            negative_sum_: self.negative_sum_ * _rhs.negative_sum_,
        }
    }
}

impl ops::Mul<f64> for RobustDif {
    type Output = RobustDif;

    fn mul(self, _rhs: f64) -> Self {
        let rhs = RobustFpt::new_1(_rhs);
        if is_pos_f(_rhs) {
            Self {
                positive_sum_: self.positive_sum_ * rhs,
                negative_sum_: self.negative_sum_ * rhs,
            }
        } else {
            Self {
                positive_sum_: self.negative_sum_ * rhs,
                negative_sum_: self.positive_sum_ * rhs,
            }
        }
    }
}

impl ops::Mul<RobustFpt> for RobustDif {
    type Output = RobustDif;

    fn mul(self, mut _rhs: RobustFpt) -> Self {
        if !_rhs.is_neg() {
            Self {
                positive_sum_: self.positive_sum_ * _rhs,
                negative_sum_: self.negative_sum_ * _rhs,
            }
        } else {
            _rhs = -_rhs;
            Self {
                positive_sum_: self.negative_sum_ * _rhs,
                negative_sum_: self.positive_sum_ * _rhs,
            }
        }
    }
}

impl ops::MulAssign<f64> for RobustDif {
    fn mul_assign(&mut self, mut _rhs: f64) {
        if is_neg_f(_rhs) {
            _rhs = -_rhs;
            self.swap();
        }
        self.positive_sum_ = self.positive_sum_ * _rhs;
        self.negative_sum_ = self.negative_sum_ * _rhs;
    }
}

impl ops::MulAssign<RobustFpt> for RobustDif {
    fn mul_assign(&mut self, mut _rhs: RobustFpt) {
        if _rhs.is_neg() {
            _rhs = -_rhs;
            self.swap();
        }
        self.positive_sum_ = self.positive_sum_ * _rhs;
        self.negative_sum_ = self.negative_sum_ * _rhs;
    }
}

impl ops::MulAssign<RobustDif> for RobustDif {
    fn mul_assign(&mut self, _rhs: RobustDif) {
        self.positive_sum_ = self.positive_sum_ * _rhs.positive_sum_;
        self.negative_sum_ = self.negative_sum_ * _rhs.negative_sum_;
    }
}

/*
TODO: This must be wrong, but also - it's not used
impl ops::DivAssign<f64> for RobustDif {
    fn div_assign(&mut self, _rhs: f64) {
        if is_neg_f(_rhs) {
            self.swap();
        }
        let rhs = RobustFpt::new_1(-_rhs);

        self.positive_sum_ /= rhs;
        self.negative_sum_ /= rhs;
    }
}*/

impl fmt::Debug for RobustDif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "({:?},{:?})",
            self.positive_sum_, self.negative_sum_
        ))
    }
}

impl ops::DivAssign<RobustFpt> for RobustDif {
    fn div_assign(&mut self, _rhs: RobustFpt) {
        self.positive_sum_ /= _rhs;
        self.negative_sum_ /= _rhs;
    }
}

/// Used to compute expressions that operate with sqrts with predefined
/// relative error. Evaluates expressions of the next type:
/// sum(i = 1 .. n)(A\[i\] * sqrt(B\[i\])), 1 <= n <= 4.
#[allow(non_camel_case_types)]
pub struct robust_sqrt_expr<_fpt: Float + fmt::Display + Default + fmt::Debug> {
    #[doc(hidden)]
    pdf_: PhantomData<_fpt>,
}

#[allow(non_camel_case_types)]
impl<_fpt: Float + fmt::Display + Default + fmt::Debug> Default for robust_sqrt_expr<_fpt> {
    fn default() -> Self {
        Self { pdf_: PhantomData }
    }
}

#[allow(non_camel_case_types)]
impl<_fpt: Float + fmt::Display + Default + fmt::Debug + ops::Neg<Output = _fpt>>
    robust_sqrt_expr<_fpt>
{
    #[inline(always)]
    fn i_to_f(that: &EI::ExtendedInt) -> EX::ExtendedExponentFpt<f64> {
        EX::ExtendedExponentFpt::<f64>::from(that)
    }

    /// Evaluates expression (re = 4 EPS):
    /// A\[0\] * sqrt(B\[0\]).
    pub fn eval1(
        &self,
        a: &[EI::ExtendedInt],
        b: &[EI::ExtendedInt],
    ) -> EX::ExtendedExponentFpt<f64> {
        let a = Self::i_to_f(&a[0]);
        let b = Self::i_to_f(&b[0]);
        //tln!("eval1:");
        //tln!(" a:{:.0}", a.d());
        //tln!(" b:{:.0}", b.d());
        a * (b.sqrt())
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

        numer / divisor
    }

    /// Evaluates expression (re = 16 EPS):
    /// A\[0\] * sqrt(B\[0\]) + A\[1\] * sqrt(B\[1\]) + A\[2\] * sqrt(B\[2\]).
    pub fn eval3(
        &self,
        a: &[EI::ExtendedInt],
        b: &[EI::ExtendedInt],
    ) -> EX::ExtendedExponentFpt<f64> {
        let ra = self.eval2(a, b);
        let rb = self.eval1(&a[2..], &b[2..]);

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

        let nom = self.eval2(&ta[..], &tb[..]);
        let div = ra - rb;
        nom / div
    }

    /// Evaluates expression (re = 25 EPS):
    /// A\[0\] * sqrt(B\[0\]) + A\[1\] * sqrt(B\[1\]) +
    /// A\[2\] * sqrt(B\[2\]) + A\[3\] * sqrt(B\[3\]).
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
        self.eval3(&ta, &tb) / (ra - rb)
    }

    /// Evaluates A\[0] * sqrt(B\[0\]) + A\[1\] * sqrt(B\[1\]) +
    ///           A\[2] + A\[3\] * sqrt(B\[0\] * B\[1\]).
    /// B\[3\] = B\[0\] * B\[1\].
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

        if lh.is_zero()
            || rh.is_zero()
            || (!lh.is_neg() && !rh.is_neg())
            || (!lh.is_pos() && !rh.is_pos())
        {
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
        numer / divisor
    }

    /// Evaluates A\[3\] + A\[0\] * sqrt(B\[0\]) + A\[1\] * sqrt(B\[1\]) +
    ///           A\[2\] * sqrt(B\[3\] * (sqrt(B\[0\] * B\[1\]) + B\[2\])).
    #[allow(non_snake_case)]
    pub fn sqrt_expr_evaluator_pss4(
        &mut self,
        A: &[EI::ExtendedInt],
        B: &[EI::ExtendedInt],
    ) -> EX::ExtendedExponentFpt<f64> {
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
                return lh + rh;
            }
            cA[0] = &A[0] * &A[0] * &B[0] + &A[1] * &A[1] * &B[1] - &A[2] * &A[2] * &B[3] * &B[2];
            cB[0] = EI::ExtendedInt::from(1_i32);
            cA[1] = &A[0] * &A[1] * &EI::ExtendedInt::from(2_i32) - &A[2] * &A[2] * &B[3];
            cB[1] = &B[0] * &B[1];
            let numer = self.eval2(&cA, &cB);

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

        if lh.is_zero()
            || rh.is_zero()
            || (!lh.is_neg() && !rh.is_neg())
            || (!lh.is_pos() && !rh.is_pos())
        {
            return lh + rh;
        }
        cA[0] = &A[3] * &A[0] * &EI::ExtendedInt::from(2_i32);
        cA[1] = &A[3] * &A[1] * &EI::ExtendedInt::from(2_i32);
        cA[2] = &A[0] * &A[0] * &B[0] + &A[1] * &A[1] * &B[1] + &A[3] * &A[3]
            - &A[2] * &A[2] * &B[2] * &B[3];
        cA[3] = &A[0] * &A[1] * &EI::ExtendedInt::from(2_i32) - &A[2] * &A[2] * &B[3];
        cB[3] = &B[0] * &B[1];
        let numer = self.sqrt_expr_evaluator_pss3(&cA, &cB);

        numer / (lh - rh)
    }
}
