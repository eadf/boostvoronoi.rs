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
use extended_int as EI;
use num::{Float, Integer, NumCast, PrimInt, Signed, Zero};
use std::fmt;
use std::hash::Hash;
mod beach_line;
pub mod builder;
mod circle_event;
mod ctypes;
pub mod diagram;
mod end_point;
// I'd prefer if this module could be pub(crate), but then the documentation examples would not work.
pub mod extended_exp_fpt;
// I'd prefer if this module could be pub(crate), but then the documentation examples would not work.
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

/// This is the integer input type of the algorithm. i32 or i64.
pub trait InputType:
    fmt::Display + Hash + Integer + PrimInt + Debug + Default + Unpin + Signed
{
}

impl InputType for i64 {}
impl InputType for i32 {}

/// This is the floating point output type of the algorithm. f32 or f64.
pub trait OutputType:
    Float + Debug + Zero + Unpin + fmt::Display + std::ops::MulAssign + Default
{
}

impl OutputType for f32 {}
impl OutputType for f64 {}

#[inline(always)]
/// Convert from the input integer type to an extended int
pub(crate) fn cast_i_to_xi<I: InputType>(input: I) -> EI::ExtendedInt {
    EI::ExtendedInt::from(num::cast::<I, i64>(input).unwrap())
}

#[inline(always)]
/// Convert from one numeric type to another.
/// # Panics
/// panics if the conversion fails
pub fn cast<T: NumCast, U: NumCast>(n: T) -> U {
    NumCast::from(n).unwrap()
}

#[inline(always)]
/// Try to convert from one numeric type to another
/// # Errors
/// Will return an BvError::NumberConversion if the conversion fails
pub fn try_cast<T: NumCast + Debug + Copy, U: NumCast>(n: T) -> Result<U, BvError> {
    NumCast::from(n).ok_or_else(|| {
        BvError::NumberConversion(format!(
            "Could not convert {:?} to {}",
            n,
            std::any::type_name::<U>()
        ))
    })
}

pub(crate) type VobU32 = vob::Vob<u32>;

pub(crate) trait GrowingVob {
    /// Will create a new Vob and fill it with `false`
    fn fill(initial_size: usize) -> Self;
    /// Conditionally grow to fit required size, set ´bit´ to ´state´ value
    fn set_grow(&mut self, bit: usize, state: bool);
    /// get() with default value `false`
    fn get_f(&self, bit: usize) -> bool;
}

impl<T: PrimInt + Debug> GrowingVob for vob::Vob<T> {
    #[inline]
    fn fill(initial_size: usize) -> Self {
        let mut v = Self::new_with_storage_type(0);
        v.resize(initial_size, false);
        v
    }

    #[inline]
    fn set_grow(&mut self, bit: usize, state: bool) {
        if bit >= self.len() {
            self.resize(bit + std::mem::size_of::<T>(), false);
        }
        let _ = self.set(bit, state);
    }

    #[inline]
    fn get_f(&self, bit: usize) -> bool {
        self.get(bit).unwrap_or(false)
    }
}
