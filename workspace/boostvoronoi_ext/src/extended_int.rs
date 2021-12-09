// Boost.Polygon library detail/voronoi_structures.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code..

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! Utilities for big integers. Supports next set of arithmetic operations: +, -, *.

use crate::{cast, InputType};
#[allow(unused_imports)]
use crate::{t, tln};
use num::{One, ToPrimitive, Zero};
use std::cmp;
use std::fmt;
use std::num::Wrapping;
use std::ops;

/// the default size of the SmallVec inside ExtendedInt (in units of u32)
const EXTENDED_INT_VEC_SIZE: usize = 8;

/// Stack allocated big integer class.
/// Supports next set of arithmetic operations: +, -, *.
/// Ported from voronoi_ctypes.hpp
#[derive(Clone)]
pub struct ExtendedInt {
    chunks_: smallvec::SmallVec<[Wrapping<u32>; EXTENDED_INT_VEC_SIZE]>,
    count_: i32,
}

impl<I: InputType> From<I> for ExtendedInt {
    #[inline]
    ///```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
    ///
    /// let aa = 41231332_f64;
    /// let a = ExtendedInt::from(aa as i32);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// ```
    fn from(that: I) -> Self {
        let that = cast::<I, i64>(that);
        let mut rv = Self::zero();
        match that.cmp(&0) {
            cmp::Ordering::Greater => {
                let mut c = that as u64;
                rv.chunks_.push(Wrapping((c & 0xFFFFFFFF) as u32));
                c >>= 32;
                if c != 0 {
                    rv.chunks_.push(Wrapping(c as u32));
                    rv.count_ = 2
                } else {
                    rv.count_ = 1
                }
            }
            cmp::Ordering::Less => {
                let mut c: u64 = (-that) as u64;
                rv.chunks_.push(Wrapping((c & 0xFFFFFFFF) as u32));
                c >>= 32;
                if c != 0 {
                    rv.chunks_.push(Wrapping(c as u32));
                    rv.count_ = -2
                } else {
                    rv.count_ = -1
                }
            }
            _ => (),
        }
        rv
    }
}

impl One for ExtendedInt {
    #[inline]
    fn one() -> Self {
        Self::from(1_i32)
    }
}

impl Zero for ExtendedInt {
    #[inline]
    fn zero() -> Self {
        Self {
            chunks_: smallvec::SmallVec::<[Wrapping<u32>; EXTENDED_INT_VEC_SIZE]>::default(),
            count_: 0,
        }
    }
    #[inline]
    fn is_zero(&self) -> bool {
        unimplemented!()
    }
}

impl ExtendedInt {
    /// Return the mantissa and exponent components of this integer.
    /// `value` ≈ `mantissa` * 2^`exponent`
    pub fn p(&self) -> (f64, i32) {
        let sep = 0x100000000_u64 as f64;
        let mut rv = (0.0, 0);
        match self.size() {
            0 => return rv,
            1 => {
                rv.0 = self.chunks_.get(0).unwrap().0.to_f64().unwrap();
            }
            2 => {
                rv.0 = self.chunks_.get(1).unwrap().0.to_f64().unwrap() * sep
                    + self.chunks_.get(0).unwrap().0.to_f64().unwrap();
            }
            _ => {
                for v in self.chunks_.iter().rev().take(3) {
                    rv.0 *= sep;
                    rv.0 += v.0.to_f64().unwrap();
                }
                rv.1 = ((self.size() - 3) << 5).to_i32().unwrap();
            }
        }
        if self.count_ < 0 {
            rv.0 = -rv.0;
        }
        rv
    }

    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        self.count_ > 0
    }

    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        self.count_ < 0
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.count_ == 0
    }

    #[inline(always)]
    /// converts to f64
    pub fn d(&self) -> f64 {
        let p = self.p();
        libm::ldexp(p.0, p.1)
    }

    #[inline(always)]
    /// return the number of words in 'self.count'
    pub fn size(&self) -> usize {
        self.chunks_.len()
    }

    /// this method assumes self is an empty object
    fn add_others(&mut self, e1: &Self, e2: &Self) {
        if e1.count_ == 0 {
            self.count_ = e2.count_;
            self.chunks_ = e2.chunks_.clone();
            return;
        }
        if e2.count_ == 0 {
            self.count_ = e1.count_;
            self.chunks_ = e1.chunks_.clone();
            return;
        }
        if (e1.count_ > 0) ^ (e2.count_ > 0) {
            self.dif_slice(&e1.chunks_, e1.size(), &e2.chunks_, e2.size(), false);
        } else {
            self.add_slice(&e1.chunks_, e1.size(), &e2.chunks_, e2.size());
        }
        if e1.count_ < 0 {
            self.count_ = -self.count_;
        }
    }

    fn add_slice(&mut self, c1: &[Wrapping<u32>], sz1: usize, c2: &[Wrapping<u32>], sz2: usize) {
        if sz1 < sz2 {
            self.add_slice(c2, sz2, c1, sz1);
            return;
        }
        self.count_ = sz1 as i32;
        let mut temp = 0_u64;

        for _i in self.chunks_.len()..sz1 {
            self.chunks_.push(Wrapping(0));
        }
        for i in 0..sz2 {
            temp += (c1[i].0 as u64) + (c2[i].0 as u64);
            self.chunks_[i] = Wrapping(temp as u32);
            temp >>= 32;
        }
        for (i, c1_i) in c1.iter().enumerate().take(sz1).skip(sz2) {
            temp += c1_i.0 as u64;
            self.chunks_[i] = Wrapping(temp as u32);
            temp >>= 32;
        }
        if temp != 0 {
            if self.chunks_.len() <= self.count_ as usize {
                self.chunks_.push(Wrapping(temp as u32));
            } else {
                self.chunks_[self.count_ as usize] = Wrapping(temp as u32);
            }
            self.count_ += 1;
        }
    }

    /// this method assumes self is an empty object
    fn dif_other(&mut self, e1: &Self, e2: &Self) {
        if e1.count_ == 0 {
            self.count_ = e2.count_;
            self.chunks_ = e2.chunks_.clone();
            self.count_ = -self.count_;
            return;
        }
        if e2.count_ == 0 {
            self.count_ = e1.count_;
            self.chunks_ = e1.chunks_.clone();
            return;
        }
        if (e1.count_ > 0) ^ (e2.count_ > 0) {
            self.add_slice(&e1.chunks_, e1.size(), &e2.chunks_, e2.size());
        } else {
            self.dif_slice(&e1.chunks_, e1.size(), &e2.chunks_, e2.size(), false);
        }
        if e1.count_ < 0 {
            self.count_ = -self.count_;
        }
    }

    fn dif_slice(
        &mut self,
        c1: &[Wrapping<u32>],
        sz1: usize,
        c2: &[Wrapping<u32>],
        sz2: usize,
        rec: bool,
    ) {
        let mut sz2 = sz2;
        let mut sz1 = sz1;
        if sz1 < sz2 {
            self.dif_slice(c2, sz2, c1, sz1, true);
            self.count_ = -self.count_;
            return;
        } else if (sz1 == sz2) && !rec {
            loop {
                sz1 -= 1;
                match c1[sz1].cmp(&c2[sz1]) {
                    cmp::Ordering::Less => {
                        sz1 += 1;
                        self.dif_slice(c2, sz1, c1, sz1, true);
                        self.count_ = -self.count_;
                        return;
                    }
                    cmp::Ordering::Greater => {
                        sz1 += 1;
                        break;
                    }
                    _ => (),
                }
                if sz1 == 0 {
                    break;
                }
            }
            if sz1 == 0 {
                self.count_ = 0;
                return;
            }
            sz2 = sz1;
        }
        self.count_ = (sz1 - 1) as i32;
        let mut flag = false;

        for _i in self.chunks_.len()..sz1 {
            self.chunks_.push(Wrapping(0));
        }

        for i in 0..sz2 {
            self.chunks_[i] = c1[i] - c2[i] - if flag { Wrapping(1) } else { Wrapping(0) };
            flag = (c1[i] < c2[i]) || ((c1[i] == c2[i]) && flag);
        }
        for (i, c1_i) in c1.iter().enumerate().take(sz1).skip(sz2) {
            self.chunks_[i] = c1_i - if flag { Wrapping(1) } else { Wrapping(0) };
            flag = (c1_i.0 == 0) && flag;
        }
        if self.chunks_[self.count_ as usize].0 != 0 {
            self.count_ += 1;
            if (self.count_ as usize) > self.chunks_.len() {
                self.chunks_.push(Wrapping(0));
            }
        }
        if (self.count_ as usize) < self.chunks_.len() {
            let _ = self.chunks_.pop();
        }
    }

    fn mul_other(&mut self, e1: &Self, e2: &Self) {
        if e1.count_ == 0 || e2.count_ == 0 {
            self.count_ = 0;
            return;
        }
        self.mul_slice(&e1.chunks_, e1.size(), &e2.chunks_, e2.size());
        if (e1.count_ > 0) ^ (e2.count_ > 0) {
            self.count_ = -self.count_;
        }
    }

    fn mul_slice(&mut self, c1: &[Wrapping<u32>], sz1: usize, c2: &[Wrapping<u32>], sz2: usize) {
        let mut cur: u64 = 0;
        let mut nxt: u64;
        let mut tmp: u64;

        self.count_ = (sz1 + sz2 - 1_usize) as i32;

        for _i in self.chunks_.len()..(self.count_ as usize) {
            self.chunks_.push(Wrapping(0));
        }

        for shift in 0..(self.count_ as usize) {
            nxt = 0;
            for (first, c1_first) in c1.iter().enumerate().take(shift + 1) {
                if first >= sz1 {
                    break;
                }
                let second = shift - first;
                if second >= sz2 {
                    continue;
                }

                tmp = (c1_first.0 as u64) * (c2[second].0 as u64);
                cur += tmp & 0xFFFF_FFFF;
                nxt += tmp >> 32;
            }

            self.chunks_[shift] = Wrapping((cur & 0xFFFF_FFFF) as u32);
            cur = nxt + (cur >> 32);
        }
        if cur != 0 {
            self.chunks_.push(Wrapping(cur as u32));
            self.count_ += 1;
        }
    }
}

impl Default for ExtendedInt {
    fn default() -> Self {
        Self::from(0_i32)
    }
}

impl ops::Add for ExtendedInt {
    type Output = Self;
    /// Adds `self` to `that` returning a new object containing the result
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
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
    /// Adds `self` to `that` returning a new object containing the result
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
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
        rv.add_others(self, that);
        rv
    }
}

impl<'b> ops::Add<&'b ExtendedInt> for ExtendedInt {
    type Output = ExtendedInt;
    /// Adds `self` to `that` returning a new object containing the result
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
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
    /// Subtracts `that` from `self` returning a new object containing the result
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
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
    /// Subtracts `that` from `self` returning a new object containing the result
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
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
        rv.dif_other(self, that);
        rv
    }
}

impl<'b> ops::Sub<&'b ExtendedInt> for ExtendedInt {
    type Output = ExtendedInt;
    /// Subtracts `that` from `self` returning a new object containing the result
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
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
        rv.dif_other(&self, that);
        rv
    }
}

impl ops::Mul for ExtendedInt {
    type Output = Self;
    /// Multiplies `self` with `that` returning a new object containing the result
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
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
    /// Multiplies `self` with `that` returning a new object containing the result
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
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
        rv.mul_other(self, that);
        rv
    }
}

impl<'b> ops::Mul<&'b ExtendedInt> for ExtendedInt {
    type Output = ExtendedInt;
    /// Multiplies `self` with `that` returning a new object containing the result
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
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
        rv.mul_other(&self, that);
        rv
    }
}

impl<'b> ops::Mul<i32> for ExtendedInt {
    type Output = ExtendedInt;
    /// Multiplies `self` with `that` returning a new object containing the result
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
    ///
    /// let aa = 4727377593577731_f64;
    /// let bb = 759935_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// let b = bb as i32;
    /// let c = a*b;
    /// approx::assert_ulps_eq!(c.d(), aa*bb);
    ///```
    fn mul(self, that: i32) -> ExtendedInt {
        let mut rv = ExtendedInt::default();
        let that = ExtendedInt::from(that);
        rv.mul_other(&self, &that);
        rv
    }
}

impl ops::Neg for ExtendedInt {
    type Output = Self;
    /// Negates the value of `self`
    /// ```
    /// # use boostvoronoi_core::extended_int::ExtendedInt;
    ///
    /// let aa = 4727377593577731_f64;
    /// let a = -ExtendedInt::from(aa as i64);
    /// approx::assert_ulps_eq!(a.d(), -aa);
    ///```
    fn neg(mut self) -> Self {
        self.count_ = -self.count_;
        self
    }
}

impl fmt::Debug for ExtendedInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0}", self.d())
    }
}
