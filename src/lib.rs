#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(unused_imports)]
// I use allow clippy::upper_case_acronyms
#![allow(renamed_and_removed_lints)]
#![allow(clippy::unknown_clippy_lints)]

use core::fmt::Debug;
use num::bigint::BigInt;
use num::ToPrimitive;
use num::{Float, NumCast, PrimInt, Zero};
use std::cmp;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Neg;

mod beachline;
pub mod builder;
mod circleevent;
mod ctypes;
pub mod diagram;
mod endpoint;
pub mod predicate;
mod robust_fpt;
pub mod siteevent;
pub mod visual_utils;

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
    #[error("error: given value for the radius is less than 0.0.")]
    RadiusLessThanZero,
    #[error("error: vertices should be added before segments")]
    VerticesGoesFirst { txt: String },
    #[error("error: Some error")]
    SomeError { txt: String },
    #[error("Suspected self-intersecting input data")]
    SelfIntersecting { txt: String },
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

pub trait BigIntType:
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

impl<I2> BigIntType for I2 where
    I2: Display
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

pub trait BigFloatType:
    Float + PartialOrd + PartialEq + NumCast + Copy + Clone + Display + Default + Debug + Zero
{
}

impl<F2> BigFloatType for F2 where
    F2: Float
        + PartialOrd
        + PartialEq
        + NumCast
        + Copy
        + Clone
        + Display
        + Default
        + Debug
        + Zero
        + Neg<Output = F2>
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

    // TODO: this is stupid: why can't I1 just use an int literal?
    #[inline(always)]
    pub fn one() -> I1 {
        num::cast::<i8, I1>(1i8).unwrap()
    }

    #[inline(always)]
    pub fn two() -> I1 {
        num::cast::<i8, I1>(2i8).unwrap()
    }
}

#[derive(Default)]
pub struct TypeConverter<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: InputType + Neg<Output = I2>,
    F2: OutputType + Neg<Output = F2>,
{
    _pdo: PhantomData<F1>,
    _pdi: PhantomData<I1>,
    _pdbi: PhantomData<I2>,
    _pdbf: PhantomData<F2>,
}

impl<I1, F1, I2, F2> TypeConverter<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: InputType + Neg<Output = I2>,
    F2: OutputType + Neg<Output = F2>,
{
    #[inline(always)]
    pub fn i1_to_f1(input: I1) -> F1 {
        num::cast::<I1, F1>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_f2(input: I1) -> F2 {
        num::cast::<I1, F2>(input).unwrap()
    }

    // todo! is there no way to solve this more efficiently?
    #[inline(always)]
    pub fn i1_to_bi(input: I1) -> BigInt {
        let stupid_generics = num::cast::<I1, i128>(input).unwrap();
        BigInt::from(stupid_generics)
    }

    #[inline(always)]
    pub fn i1_to_i128(input: I1) -> i128 {
        num::cast::<I1, i128>(input).unwrap()
    }

    // todo! is there no way to solve this more efficiently?
    #[inline(always)]
    pub fn i2_to_bi(input: I2) -> BigInt {
        let stupid_generics = num::cast::<I2, i128>(input).unwrap();
        BigInt::from(stupid_generics)
    }

    #[inline(always)]
    pub fn i2_to_f1(input: I2) -> F1 {
        num::cast::<I2, F1>(input).unwrap()
    }

    #[inline(always)]
    pub fn i2_to_f2(input: I2) -> F2 {
        num::cast::<I2, F2>(input).unwrap()
    }

    #[inline(always)]
    pub fn bi_to_f2(input: &BigInt) -> F2 {
        // why can't I1 just use num::cast::<BigInt, F2> ???
        //let rv = num::cast::<BigInt, F2>(input).unwrap();
        let stupid_generics = input.to_f64().unwrap();
        num::cast::<f64, F2>(stupid_generics).unwrap()
    }

    #[inline(always)]
    pub fn f2_to_f1(input: F2) -> F1 {
        num::cast::<F2, F1>(input).unwrap()
    }

    #[inline(always)]
    pub fn f1_to_f2(input: F1) -> F2 {
        num::cast::<F1, F2>(input).unwrap()
    }

    #[inline(always)]
    pub fn f1_to_f64(input: F1) -> f64 {
        num::cast::<F1, f64>(input).unwrap()
    }

    #[inline(always)]
    pub fn f2_to_f64(input: F2) -> f64 {
        num::cast::<F2, f64>(input).unwrap()
    }

    #[inline(always)]
    pub fn f1_to_i1(input: F1) -> I1 {
        num::cast::<F1, I1>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_i2(input: I1) -> I2 {
        num::cast::<I1, I2>(input).unwrap()
    }

    #[inline(always)]
    pub fn f32_to_f1(input: f32) -> F1 {
        num::cast::<f32, F1>(input).unwrap()
    }

    #[inline(always)]
    pub fn f32_to_f2(input: f32) -> F2 {
        num::cast::<f32, F2>(input).unwrap()
    }

    #[inline(always)]
    pub fn i32_to_i1(input: i32) -> I1 {
        num::cast::<i32, I1>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_i32(input: I1) -> i32 {
        num::cast::<I1, i32>(input).unwrap()
    }

    #[inline(always)]
    pub fn i1_to_f64(input: I1) -> f64 {
        NumCast::from(input).unwrap()
    }

    #[inline(always)]
    pub fn u64_to_f2(input: u64) -> F2 {
        num::cast::<u64, F2>(input).unwrap()
    }
}
