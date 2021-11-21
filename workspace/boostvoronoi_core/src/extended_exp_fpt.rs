// Boost.Polygon library detail/voronoi_structures.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code..

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! Utilities for extended float. Supports 63 bit mantissa with 32 bit exponent.
use crate::{extended_int as EI, OutputType};
use std::fmt;
use std::ops;

/// Floating point type wrapper. Allows to extend exponent boundaries to the
/// integer type range. This class does not handle division by zero, subnormal
/// numbers or NaNs.
/// Ported from the class extended_exponent_fpt in voronoi_ctypes.hpp
#[derive(Copy, Clone)]
pub struct ExtendedExponentFpt<F: OutputType> {
    val_: F,
    exp_: i32,
}

const MAX_SIGNIFICANT_EXP_DIF_F64: i32 = 54;

impl From<&EI::ExtendedInt> for ExtendedExponentFpt<f64> {
    #[inline]
    /// Converts to ExtendedExponentFpt::<f64> from &ExtendedInt
    /// ```
    /// # use boostvoronoi::extended_int::ExtendedInt;
    /// # use boostvoronoi::extended_exp_fpt::ExtendedExponentFpt;
    ///
    /// let aa = 41232131332_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let e = ExtendedExponentFpt::from(&a);
    /// approx::assert_ulps_eq!(e.d(), aa);
    /// ```
    fn from(that: &EI::ExtendedInt) -> Self {
        let p = that.p();
        Self::new(p.0, p.1)
    }
}

impl From<EI::ExtendedInt> for ExtendedExponentFpt<f64> {
    #[inline]
    /// Converts to `ExtendedExponentFpt::<f64>` from `ExtendedInt`
    /// ```
    /// # use boostvoronoi::extended_int::ExtendedInt;
    /// # use boostvoronoi::extended_exp_fpt::ExtendedExponentFpt;
    ///
    /// let aa = 41232131332_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let e = ExtendedExponentFpt::from(a);
    /// approx::assert_ulps_eq!(e.d(), aa);
    /// ```
    fn from(that: EI::ExtendedInt) -> Self {
        let p = that.p();
        Self::new(p.0, p.1)
    }
}

impl From<ExtendedExponentFpt<f64>> for f64 {
    #[inline]
    /// Converts from `ExtendedExponentFpt<f64>` to `f64`
    /// ```
    /// # use boostvoronoi::extended_exp_fpt::ExtendedExponentFpt;
    ///
    /// let f1 = 345345345453_f64;
    /// let e = ExtendedExponentFpt::from(f1);
    /// let f2 = f64::from(e);
    /// approx::assert_ulps_eq!(f1, f2);
    /// ```
    fn from(that: ExtendedExponentFpt<f64>) -> f64 {
        that.d()
    }
}

impl From<f64> for ExtendedExponentFpt<f64> {
    #[inline]
    /// Converts from `f64` to `ExtendedExponentFpt<f64>`
    /// ```
    /// # use boostvoronoi::extended_exp_fpt::ExtendedExponentFpt;
    ///
    /// let f1 = 345345345453_f64;
    /// let e = ExtendedExponentFpt::from(f1);
    /// let f2 = f64::from(e);
    /// approx::assert_ulps_eq!(f1, f2);
    /// ```
    fn from(that: f64) -> ExtendedExponentFpt<f64> {
        let rv = libm::frexp(that);
        Self::new(rv.0, rv.1)
    }
}

#[allow(dead_code)]
impl ExtendedExponentFpt<f64> {
    #[inline]
    /// Constructor with value and exponent as arguments.
    /// The value of this number is 'val_' * 2^ 'exp_'
    /// ```
    /// # use boostvoronoi::extended_exp_fpt::ExtendedExponentFpt;
    ///
    /// let a = ExtendedExponentFpt::<f64>::new(1.0, 12);
    /// approx::assert_ulps_eq!(a.d(), 4096.0);
    /// ```
    pub fn new(val: f64, exp: i32) -> Self {
        let fr = libm::frexp(val);
        Self {
            val_: fr.0,
            exp_: exp + fr.1,
        }
    }

    /// Is positive method.
    /// IMPORTANT!!!!! in the c++ boost voronoi implementation zero values can't be positive.
    /// ```
    /// # use boostvoronoi::extended_exp_fpt::ExtendedExponentFpt;
    ///
    /// let aa:f64 = 0_f64;
    /// let a = ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = -0_f64;
    /// let a = ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_pos(), false);
    ///
    /// let aa:f64 = f64::MIN_POSITIVE;
    /// let a = ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_pos(), aa.is_sign_positive());
    /// ```
    #[inline]
    pub fn is_pos(&self) -> bool {
        self.val_ > 0.0
    }

    /// Is negative method.
    /// IMPORTANT!!!!! in the c++ boost voronoi implementation zero values can't be negative.
    /// ```
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let aa:f64 = 0_f64;
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_neg(), aa.is_sign_negative());
    ///
    /// let aa:f64 = -0_f64;
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_neg(), false);
    /// ```
    #[inline]
    pub fn is_neg(&self) -> bool {
        self.val_ < 0.0
    }

    /// Is zero method.
    /// ```
    /// # use boostvoronoi::extended_exp_fpt;
    /// # use num_traits::identities::Zero;
    ///
    /// let aa:f64 = 0_f64;
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    ///
    /// let aa:f64 = -0_f64;
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    ///
    /// let aa:f64 = f64::MIN_POSITIVE;
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    ///
    /// let aa:f64 = -f64::MIN_POSITIVE;
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// assert_eq!(a.is_zero(), aa.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.val_ == 0.0
    }

    /// Square root method.
    /// ```
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let aa:f64 = f64::MAX;
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// let a = a.sqrt();
    /// approx::assert_ulps_eq!(a.d(), aa.sqrt());
    /// ```
    #[inline]
    pub fn sqrt(&self) -> Self {
        let mut val = self.val_;
        let mut exp = self.exp_;
        if (exp & 1) != 0 {
            val *= 2.0;
            exp -= 1;
        }

        Self::new(val.sqrt(), exp >> 1)
    }

    /// A to-float operation.
    /// ```
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let aa:f64 = 1000000000.0;
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(aa);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(-aa);
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
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// let c = -a;
    /// approx::assert_ulps_eq!(c.d(), -1_f64);
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
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
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// let c = a + b;
    /// approx::assert_ulps_eq!(c.d(), 3_f64);
    /// let c = c + b;
    /// approx::assert_ulps_eq!(c.d(), 5_f64);
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
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
            Self::new(val, that.exp_)
        } else {
            let exp_dif = that.exp_ - self.exp_;
            let val = libm::ldexp(that.val_, exp_dif) + self.val_;
            Self::new(val, self.exp_)
        }
    }
}

impl ops::Sub for ExtendedExponentFpt<f64> {
    type Output = Self;
    /// ```
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(-2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), -2_f64);
    /// let c = a - b;
    /// approx::assert_ulps_eq!(c.d(), 3_f64);
    /// let c = c - b;
    /// approx::assert_ulps_eq!(c.d(), 5_f64);
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(-3000000000_f64);
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), -3000000000_f64);
    /// let c = a - b;
    /// approx::assert_ulps_eq!(c.d(), 1000000000_f64-(-3000000000.0));
    /// ```
    fn sub(self, that: Self) -> Self {
        if self.val_ == 0.0 || that.exp_ > self.exp_ + MAX_SIGNIFICANT_EXP_DIF_F64 {
            return Self::new(-that.val_, that.exp_);
        }
        if that.val_ == 0.0 || self.exp_ > that.exp_ + MAX_SIGNIFICANT_EXP_DIF_F64 {
            return self;
        }
        if self.exp_ >= that.exp_ {
            let exp_dif = self.exp_ - that.exp_;
            let val = libm::ldexp(self.val_, exp_dif) - that.val_;
            Self::new(val, that.exp_)
        } else {
            let exp_dif = that.exp_ - self.exp_;
            let val = libm::ldexp(-that.val_, exp_dif) + self.val_;
            Self::new(val, self.exp_)
        }
    }
}

impl ops::Mul for ExtendedExponentFpt<f64> {
    type Output = Self;
    /// ```
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// let c = a * b;
    /// approx::assert_ulps_eq!(c.d(), 2_f64);
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
    /// approx::assert_ulps_eq!(a.d(), 1000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), 2000000000_f64);
    /// let c = a * b;
    /// approx::assert_ulps_eq!(c.d(), 1000000000_f64*2000000000_f64);
    /// ```
    fn mul(self, that: Self) -> Self {
        let val = self.val_ * that.val_;
        let exp = self.exp_ + that.exp_;
        Self::new(val, exp)
    }
}

impl ops::Mul<f64> for ExtendedExponentFpt<f64> {
    type Output = Self;
    /// ```
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(7_f64);
    /// let b = 2_f64;
    ///
    /// approx::assert_ulps_eq!(a.d(), 7_f64);
    /// approx::assert_ulps_eq!(b, 2_f64);
    /// let c = a * b;
    /// approx::assert_ulps_eq!(c.d(), 7_f64*2_f64);
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1234567890_f64);
    /// let b = 2000000000_f64;
    /// approx::assert_ulps_eq!(a.d(), 1234567890_f64);
    /// approx::assert_ulps_eq!(b, 2000000000_f64);
    /// let c = a * b;
    /// approx::assert_ulps_eq!(c.d(), 1234567890_f64*2000000000_f64);
    /// ```
    fn mul(self, that: f64) -> Self {
        let that = Self::from(that);
        self * that
    }
}

impl ops::Div for ExtendedExponentFpt<f64> {
    type Output = Self;
    /// ```
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// let c = a / b;
    /// approx::assert_ulps_eq!(c.d(), 1.0/2.0);
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(-2000000000_f64);
    /// approx::assert_ulps_eq!(a.d(),  2000000000_f64);
    /// approx::assert_ulps_eq!(b.d(), -2000000000_f64);
    /// let c = a / b;
    /// approx::assert_ulps_eq!(c.d(), -1f64);
    /// ```
    fn div(self, that: Self) -> Self {
        let val = self.val_ / that.val_;
        let exp = self.exp_ - that.exp_;
        Self::new(val, exp)
    }
}

impl ops::Div<f64> for ExtendedExponentFpt<f64> {
    type Output = Self;
    /// ```
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = 2_f64;
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// let c = a / b;
    /// approx::assert_ulps_eq!(c.d(), 1.0/2.0);
    /// let a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
    /// let b = -2000000000_f64;
    /// approx::assert_ulps_eq!(a.d(),  2000000000_f64);
    /// let c = a / b;
    /// approx::assert_ulps_eq!(c.d(), -1f64);
    /// ```
    fn div(self, that: f64) -> Self {
        let that = Self::from(that);
        let val = self.val_ / that.val_;
        let exp = self.exp_ - that.exp_;
        Self::new(val, exp)
    }
}

impl ops::AddAssign for ExtendedExponentFpt<f64> {
    /// ```
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let mut a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// a += b;
    /// approx::assert_ulps_eq!(a.d(), 3_f64);
    /// a += b;
    /// approx::assert_ulps_eq!(a.d(), 5_f64);
    /// let mut a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
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
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let mut a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(-2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), -2_f64);
    /// a -= b;
    /// approx::assert_ulps_eq!(a.d(), 3_f64);
    /// a -= b;
    /// approx::assert_ulps_eq!(a.d(), 5_f64);
    /// let mut a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(-3000000000_f64);
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
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let mut a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// a *= b;
    /// approx::assert_ulps_eq!(a.d(), 2_f64);
    /// let mut a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1000000000_f64);
    /// let     b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
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
    /// # use boostvoronoi::extended_exp_fpt;
    ///
    /// let mut a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(1_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2_f64);
    ///
    /// approx::assert_ulps_eq!(a.d(), 1_f64);
    /// approx::assert_ulps_eq!(b.d(), 2_f64);
    /// a /= b;
    /// approx::assert_ulps_eq!(a.d(), 1.0/2.0);
    /// let mut a = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(2000000000_f64);
    /// let b = extended_exp_fpt::ExtendedExponentFpt::<f64>::from(-2000000000_f64);
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
