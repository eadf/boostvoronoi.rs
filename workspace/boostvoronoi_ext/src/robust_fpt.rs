// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! Module containing robust floating points utilities.

#[cfg(test)]
mod robustdif_tests;
#[cfg(test)]
mod robustfpt_tests;

use num_traits::Zero;
use ordered_float::OrderedFloat;
use std::fmt;
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
    re_: f64,
}

impl Default for RobustFpt {
    #[inline(always)]
    /// Creates a new RobustFpt with value 0.0
    /// ```
    /// # use boostvoronoi_ext::robust_fpt::*;
    /// let r = RobustFpt::default();
    /// assert_eq!(r.fpv(), 0.0);
    /// ```
    fn default() -> Self {
        Self::from(0_f64)
    }
}

impl From<f64> for RobustFpt {
    #[inline(always)]
    /// Creates a new RobustFpt from a `f64`
    /// ```
    /// # use boostvoronoi_ext::robust_fpt::*;
    /// let f = 1.0f64;
    /// let r = RobustFpt::from(f);
    /// assert_eq!(r.fpv(),f);
    /// ```
    fn from(value: f64) -> Self {
        Self {
            fpv_: value,
            re_: 0_f64,
        }
    }
}

impl RobustFpt {
    pub fn new(fpv: f64, error: f64) -> Self {
        Self {
            fpv_: fpv,
            re_: error,
        }
    }

    #[inline(always)]
    pub fn fpv(&self) -> f64 {
        self.fpv_
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn re(&self) -> f64 {
        self.re_
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
    /// # use boostvoronoi_ext::robust_fpt;
    /// println!("is_pos()");
    /// let aa:f64 = 0_f64;
    /// let a = robust_fpt::RobustFpt::from(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::RobustFpt::from(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = f64::MIN_POSITIVE;
    /// let a = robust_fpt::RobustFpt::from(aa);
    /// assert_eq!(a.is_pos(), aa.is_sign_positive());
    /// ```
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        is_pos_f(self.fpv_)
    }

    /// Is negative method.
    /// IMPORTANT!!!!! in c++ boost voronoi implementation zero values can't be negative.
    /// ```
    /// # use boostvoronoi_ext::robust_fpt;
    ///
    /// println!("is_neg()");
    /// let aa:f64 = 0_f64;
    /// let a = robust_fpt::RobustFpt::from(aa);
    /// assert_eq!(a.is_neg(), aa.is_sign_negative());
    ///
    /// let aa:f64 = -0_f64;
    /// let a = robust_fpt::RobustFpt::from(aa);
    /// assert_eq!(a.is_neg(), false);
    /// ```
    #[inline(always)]
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
            re_: self.re_ * 0.5f64 + ROUNDING_ERROR,
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

    fn add(self, rhs: RobustFpt) -> Self {
        let fpv: f64 = self.fpv_ + rhs.fpv_;
        let re = if (!self.is_neg() && !rhs.is_neg()) || (!self.is_pos() && !rhs.is_pos()) {
            std::cmp::max(OrderedFloat(self.re_), OrderedFloat(rhs.re_)).into_inner()
                + ROUNDING_ERROR
        } else {
            let mut temp = (self.fpv_ * self.re_ - rhs.fpv_ * rhs.re_) / fpv;
            if is_neg_f(temp) {
                temp = -temp;
            } else if temp.is_nan() {
                temp = f64::INFINITY;
            }

            temp + ROUNDING_ERROR
        };
        #[cfg(feature = "console_debug")]
        {
            if !fpv.is_finite() {
                tln!("!fpv.is_finite() self:{:?}, rhs:{:?}", self, rhs);
            }
            if re.is_nan() {
                tln!("re.is_nan() self:{:?}, rhs:{:?}", self, rhs);
            }
        }
        debug_assert!(fpv.is_finite());
        debug_assert!(!re.is_nan());
        Self { fpv_: fpv, re_: re }
    }
}

impl ops::AddAssign<RobustFpt> for RobustFpt {
    fn add_assign(&mut self, rhs: RobustFpt) {
        debug_assert!(self.fpv_.is_finite());
        debug_assert!(rhs.fpv_.is_finite());

        let fpv: f64 = self.fpv_ + rhs.fpv_;
        let re = if (!self.is_neg() && !rhs.is_neg()) || (!self.is_pos() && !rhs.is_pos()) {
            std::cmp::max(OrderedFloat(self.re_), OrderedFloat(rhs.re_)).into_inner()
                + ROUNDING_ERROR
        } else {
            let mut temp = (self.fpv_ * self.re_ - rhs.fpv_ * rhs.re_) / fpv;
            if is_neg_f(temp) {
                temp = -temp;
            } else if temp.is_nan() {
                temp = f64::INFINITY;
            }
            temp + ROUNDING_ERROR
        };
        self.fpv_ = fpv;
        self.re_ = re;
        debug_assert!(self.fpv_.is_finite());
    }
}

impl ops::Mul<f64> for RobustFpt {
    type Output = RobustFpt;
    // Todo make this more efficient
    fn mul(self, rhs: f64) -> Self {
        self * RobustFpt::from(rhs)
    }
}

impl ops::Mul<RobustFpt> for RobustFpt {
    type Output = RobustFpt;

    fn mul(self, rhs: RobustFpt) -> Self {
        let fpv: f64 = self.fpv_ * rhs.fpv_;
        let re = self.re_ + rhs.re_ + ROUNDING_ERROR;

        Self { fpv_: fpv, re_: re }
    }
}

impl ops::MulAssign<RobustFpt> for RobustFpt {
    fn mul_assign(&mut self, rhs: RobustFpt) {
        self.re_ = self.re_ + rhs.re_ + ROUNDING_ERROR;
        self.fpv_ = self.fpv_ * rhs.fpv_;

        debug_assert!(self.fpv_.is_finite());
        debug_assert!(!self.re_.is_nan());
    }
}

impl ops::Sub<RobustFpt> for RobustFpt {
    type Output = RobustFpt;

    fn sub(self, rhs: RobustFpt) -> Self {
        #[cfg(feature = "console_debug")]
        let old_self = self;

        let fpv: f64 = self.fpv_ - rhs.fpv_;
        let re = if (!self.is_neg() && !rhs.is_pos()) || (!self.is_pos() && !rhs.is_neg()) {
            std::cmp::max(OrderedFloat(self.re_), OrderedFloat(rhs.re_)).into_inner()
                + ROUNDING_ERROR
        } else {
            let mut temp = (self.fpv_ * self.re_ + rhs.fpv_ * rhs.re_) / fpv;
            if is_neg_f(temp) {
                temp = -temp;
            } else if temp.is_nan() {
                temp = f64::INFINITY;
            }
            temp + ROUNDING_ERROR
        };
        #[cfg(feature = "console_debug")]
        {
            if !self.fpv_.is_finite() {
                tln!(
                    "!self.fpv.is_finite() self:{:?}, rhs:{:?} old_self:{:?}",
                    self,
                    rhs,
                    old_self
                );
            }
            if self.re_.is_nan() {
                tln!(
                    "self.re.is_nan() self:{:?}, rhs:{:?} old_self:{:?}",
                    self,
                    rhs,
                    old_self
                );
            }
            debug_assert!(self.fpv_.is_finite());
            debug_assert!(!self.re_.is_nan());
        }
        Self { fpv_: fpv, re_: re }
    }
}

impl ops::SubAssign<RobustFpt> for RobustFpt {
    fn sub_assign(&mut self, rhs: RobustFpt) {
        #[cfg(feature = "console_debug")]
        let old_self = *self;

        let fpv = self.fpv_ - rhs.fpv_;
        if (!self.is_neg() && !rhs.is_pos()) || (!self.is_pos() && !rhs.is_neg()) {
            self.re_ = std::cmp::max(OrderedFloat(self.re_), OrderedFloat(rhs.re_)).into_inner()
                + ROUNDING_ERROR;
        } else {
            let mut temp: f64 = (self.fpv_ * self.re_ + rhs.fpv_ * rhs.re_) / fpv;
            if is_neg_f(temp) {
                temp = -temp;
            } else if temp.is_nan() {
                temp = f64::INFINITY;
            }
            self.re_ = temp + ROUNDING_ERROR;
        }
        self.fpv_ = fpv;
        #[cfg(feature = "console_debug")]
        {
            if !self.fpv_.is_finite() {
                tln!(
                    "!self.fpv.is_finite() self:{:?}, rhs:{:?} old_self:{:?}",
                    self,
                    rhs,
                    old_self
                );
            }
            if self.re_.is_nan() {
                tln!(
                    "self.re.is_nan() self:{:?}, rhs:{:?} old_self:{:?}",
                    self,
                    rhs,
                    old_self
                );
            }
            debug_assert!(self.fpv_.is_finite());
            debug_assert!(!self.re_.is_nan());
        }
    }
}

impl ops::Div<f64> for RobustFpt {
    type Output = RobustFpt;

    fn div(self, rhs: f64) -> Self {
        self / RobustFpt::from(rhs)
    }
}

impl ops::Div<RobustFpt> for RobustFpt {
    type Output = RobustFpt;

    fn div(self, rhs: RobustFpt) -> Self {
        let fpv: f64 = self.fpv_ / rhs.fpv_;
        let re = self.re_ + rhs.re_ + ROUNDING_ERROR;

        debug_assert!(fpv.is_finite());
        debug_assert!(!re.is_nan());

        Self { fpv_: fpv, re_: re }
    }
}

impl ops::DivAssign<RobustFpt> for RobustFpt {
    fn div_assign(&mut self, rhs: RobustFpt) {
        self.re_ = self.re_ + rhs.re_ + ROUNDING_ERROR;
        self.fpv_ = self.fpv_ / rhs.fpv_;

        debug_assert!(self.fpv_.is_finite());
        debug_assert!(!self.re_.is_nan());
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

impl From<f64> for RobustDif {
    #[inline(always)]
    /// Creates a new RobustDif from a `f64`
    /// ```
    /// # use boostvoronoi_ext::robust_fpt::*;
    /// let f = 1.234f64;
    /// let r = RobustDif::from(f);
    /// assert_eq!(r.dif().fpv(),f);
    /// ```
    fn from(value: f64) -> Self {
        if is_pos_f(value) {
            Self {
                positive_sum_: RobustFpt::from(value),
                negative_sum_: RobustFpt::default(),
            }
        } else {
            Self {
                positive_sum_: RobustFpt::default(),
                negative_sum_: RobustFpt::from(value),
            }
        }
    }
}

impl From<(RobustFpt, RobustFpt)> for RobustDif {
    #[inline(always)]
    /// Creates a new RobustDif from a `(RobustFpt,RobustFpt)`
    /// ```
    /// # use boostvoronoi_ext::robust_fpt::*;
    /// let p = 1.234f64;
    /// let n = 2.234f64;
    /// let r = RobustDif::from((RobustFpt::from(p), RobustFpt::from(n)));
    /// assert_eq!(r.dif().fpv(),p-n);
    /// ```
    fn from(value: (RobustFpt, RobustFpt)) -> Self {
        debug_assert!(!value.0.is_neg());
        debug_assert!(!value.1.is_neg());
        Self {
            positive_sum_: value.0,
            negative_sum_: value.1,
        }
    }
}

impl RobustDif {
    pub fn new(pos: f64, neg: f64) -> Self {
        debug_assert!(!pos.is_sign_negative());
        debug_assert!(!neg.is_sign_negative());
        Self {
            positive_sum_: RobustFpt::from(pos),
            negative_sum_: RobustFpt::from(neg),
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

    fn add(self, rhs: RobustDif) -> Self {
        Self {
            positive_sum_: self.positive_sum_ + rhs.positive_sum_,
            negative_sum_: self.negative_sum_ + rhs.negative_sum_,
        }
    }
}

impl ops::AddAssign<RobustDif> for RobustDif {
    fn add_assign(&mut self, rhs: RobustDif) {
        self.positive_sum_ += rhs.positive_sum_;
        self.negative_sum_ += rhs.negative_sum_;
    }
}

impl ops::AddAssign<RobustFpt> for RobustDif {
    fn add_assign(&mut self, rhs: RobustFpt) {
        if !rhs.is_neg() {
            self.positive_sum_ += rhs;
        } else {
            self.negative_sum_ -= rhs;
        }
    }
}

impl ops::Sub<RobustDif> for RobustDif {
    type Output = RobustDif;

    fn sub(self, rhs: RobustDif) -> Self {
        Self {
            positive_sum_: self.positive_sum_ + rhs.negative_sum_,
            negative_sum_: self.negative_sum_ + rhs.positive_sum_,
        }
    }
}

/// Converts to RobustDif from RobustFpt
/// ```
/// # use boostvoronoi_ext::robust_fpt::*;
/// let s = RobustFpt::from(1.0);
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
    fn sub_assign(&mut self, rhs: RobustDif) {
        self.positive_sum_ += rhs.negative_sum_;
        self.negative_sum_ += rhs.positive_sum_;
    }
}

impl ops::SubAssign<RobustFpt> for RobustDif {
    fn sub_assign(&mut self, rhs: RobustFpt) {
        #[cfg(feature = "console_debug")]
        {
            assert!(self.dif().fpv().is_finite());
            assert!(rhs.fpv().is_finite());
        }
        //dbg!(&self, &rhs);
        if !rhs.is_neg() {
            self.negative_sum_ += rhs;
        } else {
            self.positive_sum_ -= rhs;
        }
    }
}

impl ops::Mul<RobustDif> for RobustDif {
    type Output = RobustDif;

    fn mul(self, rhs: RobustDif) -> Self {
        Self {
            positive_sum_: self.positive_sum_ * rhs.positive_sum_,
            negative_sum_: self.negative_sum_ * rhs.negative_sum_,
        }
    }
}

impl ops::Mul<f64> for RobustDif {
    type Output = RobustDif;

    fn mul(self, rhs_: f64) -> Self {
        let rhs = RobustFpt::from(rhs_);
        if is_pos_f(rhs_) {
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

    fn mul(self, mut rhs: RobustFpt) -> Self {
        if !rhs.is_neg() {
            Self {
                positive_sum_: self.positive_sum_ * rhs,
                negative_sum_: self.negative_sum_ * rhs,
            }
        } else {
            rhs = -rhs;
            Self {
                positive_sum_: self.negative_sum_ * rhs,
                negative_sum_: self.positive_sum_ * rhs,
            }
        }
    }
}

impl ops::MulAssign<f64> for RobustDif {
    fn mul_assign(&mut self, mut rhs: f64) {
        if is_neg_f(rhs) {
            rhs = -rhs;
            self.swap();
        }
        self.positive_sum_ = self.positive_sum_ * rhs;
        self.negative_sum_ = self.negative_sum_ * rhs;
    }
}

impl ops::MulAssign<RobustFpt> for RobustDif {
    fn mul_assign(&mut self, mut rhs: RobustFpt) {
        if rhs.is_neg() {
            rhs = -rhs;
            self.swap();
        }
        self.positive_sum_ = self.positive_sum_ * rhs;
        self.negative_sum_ = self.negative_sum_ * rhs;
    }
}

impl ops::MulAssign<RobustDif> for RobustDif {
    fn mul_assign(&mut self, rhs: RobustDif) {
        self.positive_sum_ = self.positive_sum_ * rhs.positive_sum_;
        self.negative_sum_ = self.negative_sum_ * rhs.negative_sum_;
    }
}

impl fmt::Debug for RobustDif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "({:?},{:?})",
            self.positive_sum_, self.negative_sum_
        ))
    }
}

impl ops::DivAssign<RobustFpt> for RobustDif {
    fn div_assign(&mut self, rhs: RobustFpt) {
        self.positive_sum_ /= rhs;
        self.negative_sum_ /= rhs;
    }
}

