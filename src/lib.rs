// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.75.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(unused_imports)]
#![deny(bare_trait_objects)]
#![deny(ellipsis_inclusive_range_patterns)]
#![deny(elided_lifetimes_in_paths)]
#![feature(map_first_last)]
use core::fmt::Debug;
use extended_exp_fpt as EX;
use extended_int as EI;
use num::{Float, NumCast, PrimInt, Zero};
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
// I'd prefer if this module could be pub (crate), but then the documentation examples would not work.
pub mod extended_exp_fpt;
// I'd prefer if this module could be pub (crate), but then the documentation examples would not work.
pub mod extended_int;
pub mod file_reader;
pub(crate) mod predicate;
// I'd prefer if this module could be pub (crate), but then the documentation examples would not work.
pub mod robust_fpt;
mod site_event;
pub mod sync_diagram;
pub mod visual_utils;

/// Debug utility function, formats an id string
pub(crate) fn format_id(value: Option<usize>) -> String {
    if let Some(value) = value {
        value.to_string()
    } else {
        String::from("-")
    }
}

/// A feature gated print(), will only be active when the feature "console_debug" is selected.
#[macro_export]
macro_rules! t {
    ($($arg:tt)*) => ({
     #[cfg(feature = "console_debug")]
     print!($($arg)*)
    });
}

/// A feature gated println(), will only be active when the feature "console_debug" is selected.
#[macro_export]
macro_rules! tln {
    ($($arg:tt)*) => ({
     #[cfg(feature = "console_debug")]
     println!($($arg)*)
    });
}

/// A really simple 2d coordinate container type - integer only
#[derive(Copy, Clone, cmp::PartialEq, cmp::Eq, Hash)]
pub struct Point<T: InputType> {
    pub x: T,
    pub y: T,
}

impl<T> Debug for Point<T>
where
    T: InputType + Display + Hash,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// A really simple 2d line container type - integer only
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
    #[error("error: Some error with object id")]
    IdError { txt: String },
    #[error("error: Some error with a value")]
    ValueError { txt: String },
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
    #[error(transparent)]
    BvError(#[from] std::io::Error),
}

/// This is the integer input type of the algorithm. Typically i32 or i64.
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

impl<I> InputType for I where
    I: Display
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

/// This is the floating point output type of the algorithm. Typically f32 or f64.
pub trait OutputType:
    Float
    + PartialOrd
    + PartialEq
    + NumCast
    + Copy
    + Clone
    + Display
    + Default
    + Debug
    + Zero
    + std::ops::MulAssign
{
}

impl<F> OutputType for F where
    F: Float
        + PartialOrd
        + PartialEq
        + NumCast
        + Copy
        + Clone
        + Display
        + Default
        + Debug
        + Zero
        + std::ops::MulAssign
        + Neg<Output = F>
{
}

/// Functions for converting the integer input type to other types (i32 i64 etc.)
#[derive(Default)]
pub struct TypeConverter1<I>
where
    I: InputType + Neg<Output = I>,
{
    #[doc(hidden)]
    _pdi: PhantomData<I>,
}

impl<I> TypeConverter1<I>
where
    I: InputType + Neg<Output = I>,
{
    #[inline(always)]
    pub fn i1_to_xi(input: I) -> EI::ExtendedInt {
        EI::ExtendedInt::from(num::cast::<I, i64>(input).unwrap())
    }

    #[inline(always)]
    pub fn i32_to_i1(input: i32) -> I {
        num::cast::<i32, I>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_i32(input: I) -> i32 {
        num::cast::<I, i32>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_i64(input: I) -> i64 {
        num::cast::<I, i64>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_f32(input: I) -> f32 {
        num::cast::<I, f32>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_f64(input: I) -> f64 {
        NumCast::from(input).unwrap()
    }
}

/// Functions for converting the integer and float input type to other types.
#[derive(Default)]
pub struct TypeConverter2<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    #[doc(hidden)]
    _pdo: PhantomData<F>,
    #[doc(hidden)]
    _pdi: PhantomData<I>,
}

impl<I, F> TypeConverter2<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    #[inline(always)]
    pub fn i1_to_f1(input: I) -> F {
        num::cast::<I, F>(input).unwrap()
    }

    #[inline(always)]
    pub fn f1_to_i32(input: F) -> i32 {
        num::cast::<F, i32>(input).unwrap()
    }

    #[inline(always)]
    pub fn try_f1_to_i32(input: F) -> Result<i32, BvError> {
        num::cast::<F, i32>(input).ok_or(BvError::NumberConversion {
            txt: format!("Could not convert from float{:?} to int32", input),
        })
    }

    #[inline(always)]
    pub fn i1_to_xi(input: I) -> EI::ExtendedInt {
        EI::ExtendedInt::from(num::cast::<I, i64>(input).unwrap())
    }

    #[inline(always)]
    pub fn f1_to_i1(input: F) -> I {
        num::cast::<F, I>(input).unwrap()
    }

    #[inline(always)]
    pub fn try_f1_to_i1(input: F) -> Result<I, BvError> {
        num::cast::<F, I>(input).ok_or(BvError::NumberConversion {
            txt: format!("Could not convert from float:{:?} to int32", input),
        })
    }

    #[inline(always)]
    pub fn f1_to_f64(input: F) -> f64 {
        num::cast::<F, f64>(input).unwrap()
    }

    #[inline(always)]
    pub fn f1_to_f32(input: F) -> f32 {
        num::cast::<F, f32>(input).unwrap()
    }

    #[inline(always)]
    pub fn i32_to_f1(input: i32) -> F {
        num::cast::<i32, F>(input).unwrap()
    }

    #[inline(always)]
    pub fn f32_to_f1(input: f32) -> F {
        num::cast::<f32, F>(input).unwrap()
    }

    #[inline(always)]
    pub fn f64_to_f1(input: f64) -> F {
        num::cast::<f64, F>(input).unwrap()
    }

    #[inline(always)]
    pub fn xi_to_xf(input: &EI::ExtendedInt) -> EX::ExtendedExponentFpt<f64> {
        EX::ExtendedExponentFpt::from(input)
    }
}
