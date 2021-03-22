// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.75.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::extended_exp_fpt as EX;
use std::cmp;
use std::fmt;
use std::num::Wrapping;
use std::ops;

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
    /// # use boostvoronoi::extended_int::ExtendedInt;
    ///
    /// let aa = 42_f64;
    /// let a = ExtendedInt::from(aa as i32);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// ```
    fn from(that: i32) -> Self {
        let mut rv = Self::zero();
        match that.cmp(&0) {
            cmp::Ordering::Greater => {
                rv.chunks.push(Wrapping(that as u32));
                rv.count = 1;
            }
            cmp::Ordering::Less => {
                rv.chunks.push(Wrapping((-that) as u32));
                rv.count = -1;
            }
            _ => (),
        }
        rv
    }
}

impl From<i64> for ExtendedInt {
    #[inline]
    ///```
    /// # use boostvoronoi::extended_int::ExtendedInt;
    ///
    /// let aa = 41232131332_f64;
    /// let a = ExtendedInt::from(aa as i64);
    /// approx::assert_ulps_eq!(a.d(), aa);
    /// ```
    fn from(that: i64) -> Self {
        let mut rv = Self::zero();
        match that.cmp(&0) {
            cmp::Ordering::Greater => {
                let mut c = that as u64;
                rv.chunks.push(Wrapping((c & 0xFFFFFFFF) as u32));
                c >>= 32;
                if c != 0 {
                    rv.chunks.push(Wrapping(c as u32));
                    rv.count = 2
                } else {
                    rv.count = 1
                }
            }
            cmp::Ordering::Less => {
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
            _ => (),
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
    ///
    /// let aa = 41232131332_f64;
    /// let mut a = ExtendedInt::from(aa as i64);
    /// a.negate();
    /// approx::assert_ulps_eq!(a.d(), -aa);
    /// ```
    pub fn negate(&mut self) {
        //assert_eq!(self.chunks.len(), self.size());
        self.count = -self.count;
    }

    /// converts to f64
    pub fn d(&self) -> f64 {
        let p = self.p();
        libm::ldexp(p.0, p.1)
    }

    /// converts to EX::ExtendedExponentFpt::<f64>
    /// ```
    /// # use boostvoronoi::extended_int::ExtendedInt;
    ///
    /// let aa = 41232131332_f64;
    /// let mut a = ExtendedInt::from(aa as i64);
    /// let e = a.e();
    /// approx::assert_ulps_eq!(e.d(), aa);
    /// ```
    pub fn e(&self) -> EX::ExtendedExponentFpt<f64> {
        let p = self.p();
        EX::ExtendedExponentFpt::<f64>::new2(p.0, p.1)
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
        for (i, c1_i) in c1.iter().enumerate().take(sz1).skip(sz2) {
            temp += c1_i.0 as u64;
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
        //assert!(self.count >= 0);
        //assert_eq!(self.chunks.len(), self.count as usize);
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
                match c1[sz1].cmp(&c2[sz1]) {
                    cmp::Ordering::Less => {
                        sz1 += 1;
                        self.dif_slice(c2, sz1, c1, sz1, true);
                        self.count = -self.count;
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
        for (i, c1_i) in c1.iter().enumerate().take(sz1).skip(sz2) {
            self.chunks[i] = c1_i - if flag { Wrapping(1) } else { Wrapping(0) };
            flag = (c1_i.0 == 0) && flag;
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
        //assert!(self.count >= 0);
        //assert_eq!(self.chunks.len(), self.count as usize);
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
            for (first, c1_first) in c1.iter().enumerate().take(shift + 1) {
                if first >= sz1 {
                    //println!("mul_slice brk {:?}", self);
                    break;
                }
                let second = shift - first;
                if second >= sz2 {
                    //println!("mul_slice cnt {:?}", self);
                    continue;
                }

                tmp = (c1_first.0 as u64) * (c2[second].0 as u64);
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
            //assert_eq!(self.count as usize, self.chunks.len());
            self.chunks.push(Wrapping(cur as u32));
            //self.chunks[self.count as usize] = Wrapping(cur as u32);
            self.count += 1;
        }
        // Todo: remove these asserts when stable
        //assert!(self.count >= 0);
        //assert_eq!(self.chunks.len(), self.count as usize);
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
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
    /// # use boostvoronoi::extended_int::ExtendedInt;
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
