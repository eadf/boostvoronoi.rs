// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.75.0 to Rust in 2020 by Eadf (github.com/eadf)

#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(unused_imports)]
#![allow(renamed_and_removed_lints)]
#![allow(clippy::unknown_clippy_lints)]
#![feature(map_first_last)]

use core::fmt::Debug;
use num::{Float, NumCast, PrimInt, Zero};
use extended_int as EI;
use extended_exp_fpt as EX;
use std::cmp;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Neg;

mod beach_line;
pub mod builder;
mod circle_event;
mod ctypes;
pub mod diagram;
mod end_point;
pub mod predicate;
// only made pub because of the documentation tests
pub mod file_reader;
pub mod robust_fpt;
pub mod site_event;
pub mod visual_utils;
pub mod extended_int;
pub mod extended_exp_fpt;

/// Debug utility function, formats an id string
pub(crate) fn format_id(value: Option<usize>) -> String {
    if let Some(value) = value {
        value.to_string()
    } else {
        String::from("-")
    }
}

/// 2d coordinate type - integer only
#[derive(Copy, Clone, cmp::PartialEq, cmp::Eq, Hash)]
pub struct Point<T: InputType> {
    pub x: T,
    pub y: T,
}

impl<T> Debug for Point<T>
where
    T: InputType + Display + Hash,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rv = String::new();

        rv.push_str(format!("({:.12},{:.12})", self.x, self.y,).as_str());
        write!(f, "{}", rv)
    }
}

impl<T: InputType> From<[T; 2]> for Point<T> {
    fn from(coordinate: [T; 2]) -> Point<T> {
        Point {
            x: coordinate[0],
            y: coordinate[1],
        }
    }
}

impl<T: InputType> From<&[T; 2]> for Point<T> {
    fn from(coordinate: &[T; 2]) -> Point<T> {
        Point {
            x: coordinate[0],
            y: coordinate[1],
        }
    }
}

/// 2d line type - integer only
#[derive(Copy, Clone, cmp::PartialEq, cmp::Eq, Hash, Debug)]
pub struct Line<T: InputType> {
    pub start: Point<T>,
    pub end: Point<T>,
}

impl<T, IT> From<[IT; 2]> for Line<T>
where
    T: InputType,
    IT: Copy + Into<Point<T>>,
{
    fn from(coordinate: [IT; 2]) -> Line<T> {
        Line::<T> {
            start: coordinate[0].into(),
            end: coordinate[1].into(),
        }
    }
}

impl<T: InputType> Line<T> {
    pub fn new(start: Point<T>, end: Point<T>) -> Line<T> {
        Line::<T> { start, end }
    }
}

impl<T> From<[T; 4]> for Line<T>
where
    T: InputType,
{
    fn from(coordinate: [T; 4]) -> Line<T> {
        Line {
            start: Point {
                x: coordinate[0],
                y: coordinate[1],
            },
            end: Point {
                x: coordinate[2],
                y: coordinate[3],
            },
        }
    }
}

impl<T> From<&[T; 4]> for Line<T>
where
    T: InputType,
{
    fn from(coordinate: &[T; 4]) -> Line<T> {
        Line {
            start: Point {
                x: coordinate[0],
                y: coordinate[1],
            },
            end: Point {
                x: coordinate[2],
                y: coordinate[3],
            },
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum BvError {
    #[error("error: Some error with the beach-line")]
    BeachLineError { txt: String },
    #[error("error: given value for the radius is less than 0.0.")]
    RadiusLessThanZero,
    #[error("error: vertices should be added before segments")]
    VerticesGoesFirst { txt: String },
    #[error("error: Some error")]
    SomeError { txt: String },
    #[error("Suspected self-intersecting input data")]
    SelfIntersecting { txt: String },
    #[error("Could not cast number")]
    NumberConversion { txt: String },
}

pub trait InputType:
    Display
    + Ord
    + PartialOrd
    + Eq
    + PartialEq
    + Hash
    + PrimInt
    + Copy
    + Clone
    + NumCast
    + Debug
    + Zero
    + Default
{
}

impl<I1> InputType for I1 where
    I1: Display
        + Ord
        + PartialOrd
        + Eq
        + PartialEq
        + Hash
        + PrimInt
        + Copy
        + Clone
        + NumCast
        + Debug
        + Zero
        + Default
{
}

pub trait OutputType:
    Float + PartialOrd + PartialEq + NumCast + Copy + Clone + Display + Default + Debug + Zero
{
}

impl<F1> OutputType for F1 where
    F1: Float
        + PartialOrd
        + PartialEq
        + NumCast
        + Copy
        + Clone
        + Display
        + Default
        + Debug
        + Zero
        + Neg<Output = F1>
{
}

/// Project wide checker for float
pub struct TypeCheckF<F>
where
    F: Default + Copy + Clone + Float + Zero + Neg<Output = F>,
{
    _pdbf: PhantomData<F>,
}

impl<F> TypeCheckF<F>
where
    F: Default + Copy + Clone + Float + Zero + Neg<Output = F>,
{
    //#[inline]
    // todo: remove!
    //pub fn is_zero(v: F) -> bool {
    //    v == F::zero()
    //}

    //#[inline]
    // todo: remove!
    //pub fn is_neg(v: F) -> bool {
    //    v < F::zero()
    //}

    //#[inline]
    // todo: remove!
    //pub fn is_pos(v: F) -> bool {
    //    v > F::zero()
    //}

    // TODO: this is stupid: why can't I1 just use a float literal?
    #[inline]
    pub fn half() -> F {
        num::cast::<f32, F>(1.0f32 / 2.0f32).unwrap()
    }

    #[inline]
    pub fn one() -> F {
        num::cast::<f32, F>(1.0f32).unwrap()
    }

    #[inline]
    pub fn two() -> F {
        num::cast::<f32, F>(2.0f32).unwrap()
    }
}

/// Project wide checker for integer
pub struct TypeCheckI<I1>
where
    I1: PrimInt + Default + Copy + Clone + Zero + Neg<Output = I1>,
{
    _pdbf: PhantomData<I1>,
}

impl<I1> TypeCheckI<I1>
where
    I1: Default + Copy + Clone + PrimInt + Zero + Neg<Output = I1>,
{
    // todo: remove!
    #[inline(always)]
    pub fn is_neg(v: I1) -> bool {
        v < I1::zero()
    }

    // todo: remove!
    #[inline(always)]
    pub fn is_pos(v: I1) -> bool {
        v >= I1::zero()
    }
}

#[derive(Default)]
pub struct TypeConverter<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    #[doc(hidden)]
    _pdo: PhantomData<F1>,
    #[doc(hidden)]
    _pdi: PhantomData<I1>,
}

impl<I1, F1> TypeConverter<I1, F1>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
{
    #[inline(always)]
    pub fn i1_to_f1(input: I1) -> F1 {
        num::cast::<I1, F1>(input).unwrap()
    }

    #[inline(always)]
    pub fn f1_to_i32(input: F1) -> i32 {
        num::cast::<F1, i32>(input).unwrap()
    }

    #[inline(always)]
    pub fn try_f1_to_i32(input: F1) -> Result<i32, BvError> {
        num::cast::<F1, i32>(input).ok_or(BvError::NumberConversion {
            txt: "Could not convert from float to int32".to_string(),
        })
    }

    // todo! is there no way to do this more efficiently?
    #[inline(always)]
    pub fn i1_to_xi(input: I1) -> EI::ExtendedInt {
        EI::ExtendedInt::from(num::cast::<I1, i64>(input).unwrap())
    }

    #[inline(always)]
    pub fn f1_to_i1(input: F1) -> I1 {
        num::cast::<F1, I1>(input).unwrap()
    }

    #[inline(always)]
    pub fn try_f1_to_i1(input: F1) -> Result<I1, BvError> {
        num::cast::<F1, I1>(input).ok_or(BvError::NumberConversion {
            txt: "Could not convert from float to int".to_string(),
        })
    }

    #[inline(always)]
    pub fn f1_to_f64(input: F1) -> f64 {
        num::cast::<F1, f64>(input).unwrap()
    }

    #[inline(always)]
    pub fn f1_to_f32(input: F1) -> f32 {
        num::cast::<F1, f32>(input).unwrap()
    }

    #[inline(always)]
    pub fn i32_to_i1(input: i32) -> I1 {
        num::cast::<i32, I1>(input).unwrap()
    }

    #[inline(always)]
    pub fn i32_to_f1(input: i32) -> F1 {
        num::cast::<i32, F1>(input).unwrap()
    }

    #[inline(always)]
    pub fn f32_to_f1(input: f32) -> F1 {
        num::cast::<f32, F1>(input).unwrap()
    }

    #[inline(always)]
    pub fn f64_to_f1(input: f64) -> F1 {
        num::cast::<f64, F1>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_i32(input: I1) -> i32 {
        num::cast::<I1, i32>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_i64(input: I1) -> i64 {
        num::cast::<I1, i64>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_f32(input: I1) -> f32 {
        num::cast::<I1, f32>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_f64(input: I1) -> f64 {
        NumCast::from(input).unwrap()
    }

    #[inline(always)]
    pub fn u64_to_f64(input: u64) -> f64 {
        num::cast::<u64, f64>(input).unwrap()
    }

    #[inline(always)]
    pub fn xi_to_xf(input: &EI::ExtendedInt) -> EX::ExtendedExponentFpt<f64> {
        EX::ExtendedExponentFpt::from(input)
    }
}
