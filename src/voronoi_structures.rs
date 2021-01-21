// Boost.Polygon library voronoi_diagram.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::voronoi_beachline as VB;
use super::voronoi_diagram as VD;
use super::voronoi_predicate as VP;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

use super::{BigFloatType, BigIntType, BoostInputType, BoostOutputType};
use num::{NumCast, PrimInt};
use std::fmt;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Neg;
use std::rc::Rc;
use std::rc::Weak;

mod tests;

pub(crate) struct Bits;
impl Bits {
    pub const IS_INVERSE: VD::SourceCategoryType = 0x20; // 32
}

pub type GeometryCategoryType = usize;

/// Represents topology type of the voronoi site.
pub struct GeometryCategory(GeometryCategoryType);
impl GeometryCategory {
    pub const GEOMETRY_CATEGORY_POINT: GeometryCategoryType = 0x0;
    pub const GEOMETRY_CATEGORY_SEGMENT: GeometryCategoryType = 0x1;
}

/// todo! make debug_print_id & debug_print_bli_id into one function with generics
pub(crate) fn format_id(value: Option<usize>) -> String {
    if let Some(value) = value {
        value.to_string()
    } else {
        String::from("-")
    }
}

pub(crate) fn debug_print_bli_id(value: Option<VB::BeachLineIndex>) -> String {
    if let Some(value) = value {
        value.to_string()
    } else {
        String::from("-")
    }
}
/*
/// Cartesian 2D point data structure (float).
#[derive(Copy, Clone, Default)]
pub struct Point2d<O: BoostOutputType + Neg<Output = O>> {
    pub x: OrderedFloat<O>,
    pub y: OrderedFloat<O>,
}

impl<O> fmt::Debug for Point2d<O>
where
    O: BoostOutputType + Neg<Output = O>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?})", self.x, self.y,)
    }
}

impl<O: BoostOutputType + Neg<Output = O>> Point2d<O> {
    pub fn new(x: OrderedFloat<O>, y: OrderedFloat<O>) -> Point2d<O> {
        Self { x, y }
    }

    pub fn new_raw(x: O, y: O) -> Point2d<O> {
        Self {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
        }
    }

    // Todo! rename to set_x
    pub fn x(&mut self, x: OrderedFloat<O>) {
        self.x = x;
    }
    // Todo! rename to set_y
    pub fn y(&mut self, x: OrderedFloat<O>) {
        self.x = x;
    }

    pub fn set_x(&mut self, x: O) -> &mut Self {
        self.x = OrderedFloat(x);
        self
    }

    #[inline]
    pub fn get_x(&self) -> O {
        self.x.into_inner()
    }

    #[inline]
    pub fn get_ox(&self) -> OrderedFloat<O> {
        self.x
    }

    pub fn set_y(&mut self, y: O) -> &mut Self {
        self.y = OrderedFloat(y);
        self
    }

    #[inline]
    pub fn get_y(&self) -> O {
        self.y.into_inner()
    }

    #[inline]
    pub fn get_oy(&self) -> OrderedFloat<O> {
        self.y
    }

    pub fn deconvolve(&mut self, rvalue: &Self) -> &mut Self {
        self.set_x(self.get_x() - rvalue.get_x());
        self.set_y(self.get_y() - rvalue.get_y());
        self
    }
}

impl<O: BoostOutputType + Neg<Output = O>> Ord for Point2d<O> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x == other.x {
            return OrderedFloat::<O>::cmp(&self.y, &other.y);
        }
        OrderedFloat::<O>::cmp(&self.x, &other.x)
    }
}

impl<O: BoostOutputType + Neg<Output = O>> PartialOrd for Point2d<O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.x == other.x {
            OrderedFloat::<O>::cmp(&self.y, &other.y)
        } else {
            OrderedFloat::<O>::cmp(&self.x, &other.x)
        })
    }
}

impl<O: BoostOutputType + Neg<Output = O>> PartialEq for Point2d<O> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<O> Eq for Point2d<O> where O: BoostOutputType + Neg<Output = O> {}

impl<O: BoostOutputType + Neg<Output = O>> Hash for Point2d<O> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
*/
/*
/// Cartesian 2D point data structure (integer)
#[derive(Copy, Clone, Ord, Default, PartialOrd, Eq, PartialEq, Hash)]
pub struct Point2dI<InputType>
where
    InputType: BoostInputType + Neg<Output = InputType>,
{
    // TODO! rename make private
    pub x: InputType,
    pub y: InputType,
}

impl<I> fmt::Debug for Point2dI<I>
where
    I: BoostInputType + Neg<Output = I>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?},{:?})", self.x, self.y,)
    }
}

impl<InputType> Point2dI<InputType>
where
    InputType: Copy + BoostInputType + Neg<Output = InputType>,
{
    pub fn new(x: InputType, y: InputType) -> Point2dI<InputType> {
        Self { x, y }
    }
    pub fn new_i64(x: i64, y: i64) -> Point2dI<InputType> {
        Self {
            x: num::cast::<i64, InputType>(x).unwrap(),
            y: num::cast::<i64, InputType>(y).unwrap(),
        }
    }

    pub fn set_x(&mut self, x: InputType) -> &mut Point2dI<InputType> {
        self.x = x;
        self
    }

    #[inline]
    pub fn get_x(&self) -> InputType {
        self.x
    }

    pub fn set_y(&mut self, y: InputType) -> &mut Point2dI<InputType> {
        self.y = y;
        self
    }

    #[inline]
    pub fn get_y(&self) -> InputType {
        self.y
    }

    pub fn deconvolve(&mut self, rvalue: Self) -> &mut Self {
        self.set_x(self.get_x() - rvalue.get_x());
        self.set_y(self.get_y() - rvalue.get_y());
        self
    }
}

#[derive(Default, Clone)]
pub struct Segment2d<InputType>
where
    InputType: BoostInputType + Neg<Output = InputType>,
{
    pub a: Point2dI<InputType>,
    pub b: Point2dI<InputType>,
}

impl<I> fmt::Debug for Segment2d<I>
where
    I: BoostInputType + Neg<Output = I>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:?},{:?})({:?},{:?})",
            self.a.x, self.a.y, self.b.x, self.b.y
        )
    }
}

impl<InputType> Segment2d<InputType>
where
    InputType: BoostInputType + Neg<Output = InputType>,
{
    pub fn new(a: Point2dI<InputType>, b: Point2dI<InputType>) -> Self {
        Self { a, b }
    }

    /// Check if bounding boxes intersect. If one bounding box
    /// touches the other, they do intersect.
    pub fn do_bounding_boxes_intersect(&self, other: &Self) -> bool {
        self.a.x <= other.b.x
            && self.b.x >= other.a.x
            && self.a.y <= other.b.y
            && self.b.y >= other.a.y
    }

    /// Calculate the cross product of two points.
    #[inline(always)]
    pub fn cross_product(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
        ax * by - bx * ay
    }

    /* *
    * Checks if a Point is on a line
    * @param a line (interpreted as line, although given as line
    *                segment)
    * @param b point
    * @return <code>true</code> if point is on line, otherwise
    *         <code>false</code>
    https://martin-thoma.com/how-to-check-if-two-line-segments-intersect/
    * /
    pub fn is_point_on_line(&self, px:f64, py:f64) {
        // Move the image, so that a.first is on (0|0)
        LineSegment aTmp = new LineSegment(new Point(0, 0), new Point(
        a.second.x - a.first.x, a.second.y - a.first.y));
        Point bTmp = new Point(b.x - a.first.x, b.y - a.first.y);
        double r = crossProduct(aTmp.second, bTmp);
        return Math.abs(r) < EPSILON;
        }*/
    //}

    // todo! add real segment intersection tests
}

#[derive(Default, Clone)]
pub struct SegmentF<OutputType>
where
    OutputType: BoostOutputType + Neg<Output = OutputType>,
{
    pub a: Point2d<OutputType>,
    pub b: Point2d<OutputType>,
}

impl<OutputType> SegmentF<OutputType>
where
    OutputType: BoostOutputType + Neg<Output = OutputType>,
{
    pub fn new(a: Point2d<OutputType>, b: Point2d<OutputType>) -> Self {
        Self { a, b }
    }

    pub fn low(&self) -> Point2d<OutputType> {
        // todo! should x be used as a secondary condition?
        //if self.b.y < self.a.y {
        //    return self.b;
        //}
        self.a
    }

    pub fn high(&self) -> Point2d<OutputType> {
        //if self.a.y > self.b.y {
        //    return self.a;
        //}
        self.b
    }
}
*/
/*
/// Bound Rectangle: a.x is always <= b.x && a.y is always <= b.y
#[derive(Default, Clone)]
pub struct BoundRect<OutputType>
where
    OutputType: BoostOutputType + Neg<Output = OutputType>,
{
    pub a: Point2d<OutputType>,
    pub b: Point2d<OutputType>,
}

impl<OutputType> BoundRect<OutputType>
where
    OutputType: BoostOutputType + Neg<Output = OutputType>,
{
    pub fn new(mut a: Point2d<OutputType>, mut b: Point2d<OutputType>) -> Self {
        if a.x > b.x {
            std::mem::swap(&mut a.x, &mut b.x);
        }
        if a.y > b.y {
            std::mem::swap(&mut a.y, &mut b.y);
        }
        Self { a, b }
    }

    /// enlarge rectangle to encompass the point
    pub fn encompass(&mut self, point: &Point2d<OutputType>) -> &mut Self {
        if point.x > self.b.x {
            self.b.x = point.x
        }
        if point.y > self.b.y {
            self.b.y = point.y
        }
        if point.x < self.a.x {
            self.a.x = point.x
        }
        if point.y < self.a.y {
            self.a.y = point.y
        }
        self
    }

    /// set exact size of the rectangle
    pub fn set_points(
        &mut self,
        low: &Point2d<OutputType>,
        high: &Point2d<OutputType>,
    ) -> &mut Self {
        assert!(high.x >= low.x);
        assert!(high.y >= low.y);
        self.b.x = high.x;
        self.b.y = high.y;
        self.a.x = low.x;
        self.a.y = low.y;
        self
    }

    pub fn get_center(&self) -> Point2d<OutputType> {
        let two = num::cast::<f32, OutputType>(2.0).unwrap();
        Point2d::<OutputType>::new(
            (self.b.x + self.a.x) / OrderedFloat(two),
            (self.b.y + self.a.y) / OrderedFloat(two),
        )
    }

    pub fn bloat(&mut self, size: OutputType) -> &mut Self {
        let size = OrderedFloat(size);

        self.a.x = self.a.x - size;
        self.a.y = self.a.y - size;
        self.b.x = self.b.x + size;
        self.b.y = self.b.y + size;
        self
    }

    pub fn xl(&self) -> OutputType {
        assert!(self.a.x <= self.b.x);
        self.a.x.into_inner()
    }

    pub fn xh(&self) -> OutputType {
        assert!(self.a.x <= self.b.x);
        self.b.x.into_inner()
    }

    pub fn yl(&self) -> OutputType {
        assert!(self.a.y <= self.b.y);
        self.a.y.into_inner()
    }

    pub fn yh(&self) -> OutputType {
        assert!(self.a.y <= self.b.y);
        self.b.y.into_inner()
    }
}
*/
