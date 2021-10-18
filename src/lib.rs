// Boost.Polygon library

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    nonstandard_style,
    unused,
    future_incompatible,
    non_camel_case_types,
    unused_parens,
    non_upper_case_globals,
    unused_qualifications,
    unused_results,
    unused_imports,
    unused_variables,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    elided_lifetimes_in_paths
)]
#![cfg_attr(feature = "map_first_last", feature(map_first_last))]

use core::fmt::Debug;
use extended_exp_fpt as EX;
use extended_int as EI;
use num::{Float, Integer, NumCast, PrimInt, Signed, Zero};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;

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
pub mod geometry;
pub(crate) mod predicate;
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

#[derive(thiserror::Error, Debug)]
pub enum BvError {
    #[error("error: Some error from cpp_map")]
    ListError {
        #[from]
        source: cpp_map::MapError,
    },
    #[error("error: Some error with object id")]
    IdError(String),
    #[error("error: Some error with a value")]
    ValueError(String),
    #[error("error: Some error with the beach-line")]
    BeachLineError(String),
    #[error("error: given value for the radius is less than 0.0.")]
    RadiusLessThanZero,
    #[error("error: vertices should be added before segments")]
    VerticesGoesFirst(String),
    #[error("error: Some error")]
    InternalError(String),
    #[error("Suspected self-intersecting input data")]
    SelfIntersecting(String),
    #[error("Could not cast number")]
    NumberConversion(String),
    #[error(transparent)]
    BvError(#[from] std::io::Error),
}

/// This is the integer input type of the algorithm. Typically i32 or i64.
pub trait InputType:
    fmt::Display + Hash + Integer + PrimInt + Debug + Default + Unpin + Signed
{
}

impl InputType for i64 {}
impl InputType for i32 {}

/// This is the floating point output type of the algorithm. Typically f32 or f64.
pub trait OutputType:
    Float + Debug + Zero + Unpin + fmt::Display + std::ops::MulAssign + Default
{
}

impl OutputType for f32 {}
impl OutputType for f64 {}

/// Functions for converting the integer input type to other types (i32 i64 etc.)
#[derive(Default)]
pub struct TypeConverter1<I>
where
    I: InputType,
{
    #[doc(hidden)]
    pdi_: PhantomData<I>,
}

impl<I> TypeConverter1<I>
where
    I: InputType,
{
    #[inline(always)]
    /// Convert from the input integer type to an extended int
    pub fn i_to_xi(input: I) -> EI::ExtendedInt {
        EI::ExtendedInt::from(num::cast::<I, i64>(input).unwrap())
    }

    #[inline(always)]
    /// Convert from i32 to the input integer type
    pub fn i32_to_i(input: i32) -> I {
        num::cast::<i32, I>(input).unwrap()
    }

    #[inline(always)]
    /// Convert from the input integer type to a i32
    pub fn i_to_i32(input: I) -> i32 {
        num::cast::<I, i32>(input).unwrap()
    }

    #[inline(always)]
    /// Convert from the input integer type to a i64
    pub fn i_to_i64(input: I) -> i64 {
        num::cast::<I, i64>(input).unwrap()
    }

    #[inline(always)]
    /// Convert from the input integer type to a f32
    pub fn i_to_f32(input: I) -> f32 {
        num::cast::<I, f32>(input).unwrap()
    }

    #[inline(always)]
    /// Convert from the input integer type to a f64
    pub fn i_to_f64(input: I) -> f64 {
        NumCast::from(input).unwrap()
    }
}

/// Functions for converting the integer and float input type to other types.
#[derive(Default)]
pub struct TypeConverter2<I, F>
where
    I: InputType,
    F: OutputType,
{
    #[doc(hidden)]
    pdf_: PhantomData<F>,
    #[doc(hidden)]
    pdi_: PhantomData<I>,
}

impl<I, F> TypeConverter2<I, F>
where
    I: InputType,
    F: OutputType,
{
    #[inline(always)]
    /// Convert from the input integer type to the output float type
    pub fn i_to_f(input: I) -> F {
        num::cast::<I, F>(input).unwrap()
    }

    #[inline(always)]
    /// Convert from the output float type to i32
    pub fn f_to_i32(input: F) -> i32 {
        num::cast::<F, i32>(input).unwrap()
    }

    #[inline(always)]
    /// Try to convert from the output float type to i32
    pub fn try_f_to_i32(input: F) -> Result<i32, BvError> {
        num::cast::<F, i32>(input).ok_or_else(|| {
            BvError::NumberConversion(format!("Could not convert {:?} to int32", input))
        })
    }

    #[inline(always)]
    pub fn f_to_i(input: F) -> I {
        num::cast::<F, I>(input).unwrap()
    }

    #[inline(always)]
    pub fn try_f_to_i(input: F) -> Result<I, BvError> {
        num::cast::<F, I>(input)
            .ok_or_else(|| BvError::NumberConversion(format!("Could not convert {:?} to I", input)))
    }

    #[inline(always)]
    pub fn f_to_f64(input: F) -> f64 {
        num::cast::<F, f64>(input).unwrap()
    }

    #[inline(always)]
    pub fn f_to_f32(input: F) -> f32 {
        num::cast::<F, f32>(input).unwrap()
    }

    #[inline(always)]
    pub fn i32_to_f(input: i32) -> F {
        num::cast::<i32, F>(input).unwrap()
    }

    #[inline(always)]
    pub fn f32_to_f(input: f32) -> F {
        num::cast::<f32, F>(input).unwrap()
    }

    #[inline(always)]
    pub fn f64_to_f(input: f64) -> F {
        num::cast::<f64, F>(input).unwrap()
    }

    #[inline(always)]
    pub fn xi_to_xf(input: &EI::ExtendedInt) -> EX::ExtendedExponentFpt<f64> {
        EX::ExtendedExponentFpt::from(input)
    }
}

pub(crate) type VobU32 = vob::Vob<u32>;

pub(crate) trait GrowingVob {
    /// Will create a new Vob and fill it with `false`
    fn fill(initial_size: usize) -> Self;
    /// Grow to fit new size, set ´bit´ to ´state´ value
    fn set_grow(&mut self, bit: usize, state: bool) -> bool;
    /// get() with default value `false`
    fn get_f(&self, bit: usize) -> bool;
}

impl GrowingVob for VobU32 {
    #[inline]
    fn fill(initial_size: usize) -> Self {
        let mut v = Self::new_with_storage_type(0);
        v.resize(initial_size, false);
        v
    }

    #[inline]
    fn set_grow(&mut self, bit: usize, state: bool) -> bool {
        if bit >= self.len() {
            self.resize(bit + 64, false);
        }
        self.set(bit, state)
    }

    #[inline]
    fn get_f(&self, bit: usize) -> bool {
        self.get(bit).unwrap_or(false)
    }
}
