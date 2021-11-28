// Boost.Polygon library detail/voronoi_structures.hpp header file
//
//          Copyright Eadf (github.com/eadf) 2021.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

//! Some basic geometry data structures together with From trait implementations.

use crate::{cast, diagram::Vertex, InputType, OutputType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp;
use std::fmt;

/// A really simple 2d coordinate container type - integer only
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, cmp::PartialEq, cmp::PartialOrd, cmp::Eq, Hash)]
pub struct Point<T: InputType> {
    pub x: T,
    pub y: T,
}

impl<T: InputType> Point<T> {
    /// Create a new `Point`
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Got "conflicting implementations of trait `std::convert::From...`"
    /// So i picked the name `as_f64` for this conversion
    pub fn as_f64(&self) -> [f64; 2] {
        [cast::<T, f64>(self.x), cast::<T, f64>(self.y)]
    }

    #[cfg(feature = "ce_corruption_check")]
    #[allow(dead_code)]
    pub(crate) fn distance_to(&self, circle: &crate::circle_event::CircleEvent) -> f64 {
        let x = cast::<T, f64>(self.x) - circle.x().0;
        let y = cast::<T, f64>(self.y) - circle.y().0;
        (x * x + y * y).sqrt()
    }

    #[cfg(all(feature = "ce_corruption_check", feature = "geo"))]
    #[inline(always)]
    pub fn distance_to_point(&self, x: f64, y: f64) -> f64 {
        use geo::algorithm::euclidean_distance::*;
        geo::Coordinate { x, y }.euclidean_distance(&geo::Coordinate::from(self.as_f64()))
    }

    /// Cast a `Point<T>` to ´Point<T2>¨
    pub fn cast<T2: InputType>(self) -> Point<T2> {
        Point::<T2> {
            x: cast::<T, T2>(self.x),
            y: cast::<T, T2>(self.y),
        }
    }
}

impl<T: InputType> From<Point<T>> for [f64; 2] {
    #[inline]
    /// Converts to `[f64;2]` from `boostvoronoi::geometry::Point<T>`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let c1 = [1,2];
    /// let p:Point<i32> = Point::from(c1);
    /// let c2: [f64;2] = p.into();
    /// println!("c1:{:?}, c2:{:?}", c1, c2);
    /// assert_eq!(c1[0] as f64, c2[0]);
    /// assert_eq!(c1[1] as f64, c2[1]);
    /// ```
    fn from(coordinate: Point<T>) -> Self {
        coordinate.as_f64()
    }
}

impl<T: InputType> fmt::Debug for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.12},{:.12})", self.x, self.y,)
    }
}

impl<T: InputType> From<&Point<T>> for Point<T> {
    #[inline]
    /// A copy conversion from `&boostvoronoi::geometry::Point` to `boostvoronoi::geometry::Point`
    /// This makes it possible to accept an `Iter<Into<Point>>` and `Iter<&Point>` in the same method.
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let c = [1,2];
    /// let p1:Point<i32> = Point::from(c);
    /// let p2:Point<i32> = Point::from(&p1);
    ///
    /// assert_eq!(p2.x,c[0]);
    /// assert_eq!(p2.y,c[1]);
    /// ```
    fn from(point: &Self) -> Self {
        *point
    }
}

impl<T: InputType> From<&Line<T>> for Line<T> {
    #[inline]
    /// A copy conversion from `&boostvoronoi::geometry::Line` to `boostvoronoi::geometry::Line`
    /// This makes it possible to accept an `Iter<Into<Line>>` and `Iter<&Line>` in the same method.
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let c = [1,2,3,4];
    /// let p1:Line<i32> = Line::from(c);
    /// let p2:Line<i32> = Line::from(&p1);
    ///
    /// assert_eq!(p2.start.x,c[0]);
    /// assert_eq!(p2.start.y,c[1]);
    /// assert_eq!(p2.end.x,c[2]);
    /// assert_eq!(p2.end.y,c[3]);
    /// ```
    fn from(point: &Self) -> Self {
        *point
    }
}

impl<T: InputType> From<[T; 2]> for Point<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point` from `[T;2]`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let c = [1,2];
    /// let p:Point<i32> = Point::from(c);
    /// assert_eq!(p.x,c[0]);
    /// assert_eq!(p.y,c[1]);
    /// ```
    fn from(p: [T; 2]) -> Self {
        Self { x: p[0], y: p[1] }
    }
}

impl<T: InputType> From<&[T; 2]> for Point<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point` from `&\[T;2\]`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let c = [1,2];
    /// let p:Point<i32> = Point::from(&c);
    /// assert_eq!(p.x,c[0]);
    /// assert_eq!(p.y,c[1]);
    /// ```
    fn from(c: &[T; 2]) -> Self {
        Self { x: c[0], y: c[1] }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<geo::Coordinate<T>> for Point<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point` from `geo::Coordinate`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let c = geo::Coordinate{x:1,y:2};
    /// let p:Point<i32> = Point::from(c);
    /// assert_eq!(p.x, c.x);
    /// assert_eq!(p.y, c.y);
    /// ```
    fn from(c: geo::Coordinate<T>) -> Self {
        Self { x: c.x, y: c.y }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<&geo::Coordinate<T>> for Point<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point` from `&geo::Coordinate`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let c = geo::Coordinate{x:1,y:2};
    /// let p:Point<i32> = Point::from(&c);
    /// assert_eq!(p.x, c.x);
    /// assert_eq!(p.y, c.y);
    /// ```
    fn from(c: &geo::Coordinate<T>) -> Self {
        Self { x: c.x, y: c.y }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<Point<T>> for geo::Coordinate<T> {
    #[inline]
    /// Converts to `geo::Coordinate` from `boostvoronoi::geometry::Point`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let p = Point{x:1,y:2};
    /// let c = geo::Coordinate::<i32>::from(p);
    /// assert_eq!(p.x, c.x);
    /// assert_eq!(p.y, c.y);
    /// ```
    fn from(p: Point<T>) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<&geo::Point<T>> for Point<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point` from `&geo::Point`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let g = geo::Point::new(1,2);
    /// let p:Point<i32> = Point::from(&g);
    /// assert_eq!(p.x, g.x());
    /// assert_eq!(p.y, g.y());
    /// ```
    fn from(p: &geo::Point<T>) -> Self {
        Self { x: p.x(), y: p.y() }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<geo::Point<T>> for Point<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point` from `geo::Point`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let g = geo::Point::new(1,2);
    /// let p:Point<i32> = Point::from(g.clone());
    /// assert_eq!(p.x, g.x());
    /// assert_eq!(p.y, g.y());
    /// ```
    fn from(p: geo::Point<T>) -> Self {
        Self { x: p.x(), y: p.y() }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<Point<T>> for geo::Point<T> {
    #[inline]
    /// Converts to `geo::Point` from `boostvoronoi::geometry::Point`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let p = Point{x:1,y:2};
    /// let g = geo::Point::<i32>::from(p);
    /// assert_eq!(p.x, g.x());
    /// assert_eq!(p.y, g.y());
    /// ```
    fn from(p: Point<T>) -> Self {
        Self::new(p.x, p.y)
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<&Point<T>> for geo::Coordinate<T> {
    #[inline]
    /// Converts to `geo::Coordinate` from `&boostvoronoi::geometry::Point`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let p = Point{x:1,y:2};
    /// let c = geo::Coordinate::<i32>::from(&p);
    /// assert_eq!(p.x,c.x);
    /// assert_eq!(p.y,c.y);
    /// ```
    fn from(p: &Point<T>) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[cfg(feature = "geo")]
impl<F: OutputType + geo::CoordFloat> From<&Vertex<F>> for geo::Coordinate<F> {
    #[inline]
    /// Converts to `geo::Coordinate` from `&boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let c = geo::Coordinate::<f32>::from(&v);
    /// assert_eq!(v.x(),c.x);
    /// assert_eq!(v.y(),c.y);
    /// ```
    fn from(v: &Vertex<F>) -> Self {
        Self { x: v.x(), y: v.y() }
    }
}

#[cfg(feature = "geo")]
impl<F: OutputType + geo::CoordFloat> From<Vertex<F>> for geo::Coordinate<F> {
    #[inline]
    /// Converts to `geo::Coordinate` from `boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let c = geo::Coordinate::<f32>::from(v.clone());
    /// assert_eq!(v.x(),c.x);
    /// assert_eq!(v.y(),c.y);
    /// ```
    fn from(v: Vertex<F>) -> Self {
        Self { x: v.x(), y: v.y() }
    }
}

#[cfg(feature = "geo")]
impl<F: OutputType + geo::CoordFloat> From<&Vertex<F>> for geo::Point<F> {
    #[inline]
    /// Converts to `geo::Point` from `&boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let c = geo::Point::<f32>::from(&v);
    /// assert_eq!(v.x(),c.x());
    /// assert_eq!(v.y(),c.y());
    /// ```
    fn from(v: &Vertex<F>) -> Self {
        Self::new(v.x(), v.y())
    }
}

#[cfg(feature = "geo")]
impl<F: OutputType + geo::CoordFloat> From<Vertex<F>> for geo::Point<F> {
    #[inline]
    /// Converts to `geo::Point` from `boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = geo::Point::<f32>::from(v.clone());
    /// assert_eq!(v.x(),p.x());
    /// assert_eq!(v.y(),p.y());
    /// ```
    fn from(v: Vertex<F>) -> Self {
        Self::new(v.x(), v.y())
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<Line<T>> for geo::Line<T> {
    #[inline]
    /// Converts to `geo::Line` from `boostvoronoi::geometry::Line`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let bl = Line::from([0,1,2,3]);
    /// let gl = geo::Line::<i32>::from(bl);
    /// assert_eq!(bl.start.x,gl.start.x);
    /// assert_eq!(bl.start.y,gl.start.y);
    /// assert_eq!(bl.end.x,gl.end.x);
    /// assert_eq!(bl.end.y,gl.end.y);
    /// ```
    fn from(line: Line<T>) -> Self {
        Self {
            start: geo::Coordinate::from(line.start),
            end: geo::Coordinate::from(line.end),
        }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<geo::Line<T>> for Line<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Line` from `geo::Line`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let gl = geo::Line::from([(0,1),(2,3)]);
    /// let bl = Line::<i32>::from(gl);
    /// assert_eq!(bl.start.x,gl.start.x);
    /// assert_eq!(bl.start.y,gl.start.y);
    /// assert_eq!(bl.end.x,gl.end.x);
    /// assert_eq!(bl.end.y,gl.end.y);
    /// ```
    fn from(line: geo::Line<T>) -> Self {
        Self {
            start: Point::from(line.start),
            end: Point::from(line.end),
        }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<&geo::Line<T>> for Line<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Line` from `&geo::Line`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let gl = geo::Line::from([(0,1),(2,3)]);
    /// let bl = Line::<i32>::from(&gl);
    /// assert_eq!(bl.start.x,gl.start.x);
    /// assert_eq!(bl.start.y,gl.start.y);
    /// assert_eq!(bl.end.x,gl.end.x);
    /// assert_eq!(bl.end.y,gl.end.y);
    /// ```
    fn from(line: &geo::Line<T>) -> Self {
        Self {
            start: Point::from(line.start),
            end: Point::from(line.end),
        }
    }
}

#[cfg(feature = "cgmath")]
impl<T: InputType + cgmath::BaseNum> From<cgmath::Point2<T>> for Point<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point` from `cgmath::Point2`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let c = cgmath::Point2{x:1,y:2};
    /// let p:Point<i32> = Point::from(c);
    /// assert_eq!(p.x,c.x);
    /// assert_eq!(p.y,c.y);
    /// ```
    fn from(p: cgmath::Point2<T>) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[cfg(feature = "cgmath")]
impl<T: InputType + cgmath::BaseNum> From<&cgmath::Point2<T>> for Point<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point` from `&cgmath::Point2`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let c = cgmath::Point2{x:1,y:2};
    /// let p:Point<i32> = Point::from(&c);
    /// assert_eq!(p.x,c.x);
    /// assert_eq!(p.y,c.y);
    /// ```
    fn from(p: &cgmath::Point2<T>) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[cfg(feature = "cgmath")]
impl<T: InputType + cgmath::BaseNum> From<Point<T>> for cgmath::Point2<T> {
    #[inline]
    /// Converts to `geo::Coordinate` from `boostvoronoi::geometry::Point`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let p = Point{x:1,y:2};
    /// let c = cgmath::Point2::<i32>::from(p);
    /// assert_eq!(p.x,c.x);
    /// assert_eq!(p.y,c.y);
    /// ```
    fn from(p: Point<T>) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[cfg(feature = "cgmath")]
impl<F: OutputType + cgmath::BaseFloat> From<&Vertex<F>> for cgmath::Point2<F> {
    #[inline]
    /// Converts to `cgmath::Point2` from `&boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = cgmath::Point2::<f32>::from(&v);
    /// assert_eq!(v.x(),p.x);
    /// assert_eq!(v.y(),p.y);
    /// ```
    fn from(v: &Vertex<F>) -> Self {
        Self { x: v.x(), y: v.y() }
    }
}

#[cfg(feature = "cgmath")]
impl<F: OutputType + cgmath::BaseFloat> From<Vertex<F>> for cgmath::Point2<F> {
    #[inline]
    /// Converts to `cgmath::Point2` from `boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = cgmath::Point2::<f32>::from(v.clone());
    /// assert_eq!(v.x(),p.x);
    /// assert_eq!(v.y(),p.y);
    /// ```
    fn from(v: Vertex<F>) -> Self {
        Self { x: v.x(), y: v.y() }
    }
}

/// A really simple 2d line type - integer only
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, cmp::PartialEq, cmp::Eq, Hash, Debug)]
pub struct Line<T: InputType> {
    pub start: Point<T>,
    pub end: Point<T>,
}

impl<T: InputType, IT: Copy + Into<Point<T>>> From<[IT; 2]> for Line<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Line<T>` from `[Into<Point<T>>;2]`
    fn from(l: [IT; 2]) -> Self {
        Self {
            start: l[0].into(),
            end: l[1].into(),
        }
    }
}

impl<T: InputType, IT: Copy + Into<Point<T>>> From<&[IT; 2]> for Line<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Line<T>` from `&[Into<Point<T>>;2]`
    fn from(l: &[IT; 2]) -> Self {
        Self {
            start: l[0].into(),
            end: l[1].into(),
        }
    }
}

impl<T: InputType> Line<T> {
    #[inline]
    /// Create a new Line
    pub fn new(start: Point<T>, end: Point<T>) -> Self {
        Self { start, end }
    }

    /// Cast a `Line<T>` to ´Line<T2>¨
    pub fn cast<T2: InputType>(self) -> Line<T2> {
        Line {
            start: self.start.cast::<T2>(),
            end: self.end.cast::<T2>(),
        }
    }
}

impl<T: InputType> From<[T; 4]> for Line<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Line` from `[T;4]`
    /// ```
    /// # use boostvoronoi_core::geometry::Line;
    /// let a = [0,1,2,3];
    /// let bl = Line::<i32>::from(a);
    /// assert_eq!(bl.start.x,a[0]);
    /// assert_eq!(bl.start.y,a[1]);
    /// assert_eq!(bl.end.x,a[2]);
    /// assert_eq!(bl.end.y,a[3]);
    /// ```
    fn from(l: [T; 4]) -> Self {
        Self {
            start: Point { x: l[0], y: l[1] },
            end: Point { x: l[2], y: l[3] },
        }
    }
}

impl<T: InputType> From<Line<T>> for [T; 4] {
    #[inline]
    /// Converts to `[T;4]` from `boostvoronoi::geometry::Line`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let l = Line::from([0,1,2,3]);
    /// let a = <[i32;4]>::from(l);
    /// assert_eq!(l.start.x,a[0]);
    /// assert_eq!(l.start.y,a[1]);
    /// assert_eq!(l.end.x,a[2]);
    /// assert_eq!(l.end.y,a[3]);
    /// ```
    fn from(l: Line<T>) -> Self {
        [l.start.x, l.start.y, l.end.x, l.end.y]
    }
}

impl<T: InputType> From<&Line<T>> for [T; 4] {
    #[inline]
    /// Converts to `[T;4]` from `&boostvoronoi::geometry::Line`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let l = Line::from([0,1,2,3]);
    /// let a = <[i32;4]>::from(&l);
    /// assert_eq!(l.start.x,a[0]);
    /// assert_eq!(l.start.y,a[1]);
    /// assert_eq!(l.end.x,a[2]);
    /// assert_eq!(l.end.y,a[3]);
    /// ```
    fn from(l: &Line<T>) -> Self {
        [l.start.x, l.start.y, l.end.x, l.end.y]
    }
}

impl<T: InputType> From<&[T; 4]> for Line<T> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Line` from `&[T;4]`
    /// ```
    /// # use boostvoronoi_core::geometry::Line;
    /// let a = [0,1,2,3];
    /// let bl = Line::<i32>::from(&a);
    /// assert_eq!(bl.start.x,a[0]);
    /// assert_eq!(bl.start.y,a[1]);
    /// assert_eq!(bl.end.x,a[2]);
    /// assert_eq!(bl.end.y,a[3]);
    /// ```
    fn from(l: &[T; 4]) -> Self {
        Self {
            start: Point { x: l[0], y: l[1] },
            end: Point { x: l[2], y: l[3] },
        }
    }
}

impl<F: OutputType> From<&Vertex<F>> for [F; 2] {
    #[inline]
    /// Converts to `[T;2]` from `&boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let a = <[f32;2]>::from(&v);
    /// assert_eq!(v.x(),a[0]);
    /// assert_eq!(v.y(),a[1]);
    /// ```
    fn from(v: &Vertex<F>) -> Self {
        [v.x(), v.y()]
    }
}

#[cfg(feature = "mint")]
impl<T: InputType> From<Line<T>> for [mint::Point2<T>; 2] {
    #[inline]
    /// Converts to `[mint::Point2<T>;2]` from `boostvoronoi::geometry::Line`
    /// ```
    /// # use boostvoronoi_core::geometry::Line;
    /// let bl = Line::<i32>::from([0,1,2,3]);
    /// let ml:[mint::Point2<i32>;2] = bl.into();
    /// assert_eq!(bl.start.x,ml[0].x);
    /// assert_eq!(bl.start.y,ml[0].y);
    /// assert_eq!(bl.end.x,ml[1].x);
    /// assert_eq!(bl.end.y,ml[1].y);
    ///
    /// let ml = [mint::Point2::<i32>::from([1,2]),mint::Point2::from([3,4])];
    /// let bl = Line::<i32>::from(ml);
    /// assert_eq!(bl.start.x,ml[0].x);
    /// assert_eq!(bl.start.y,ml[0].y);
    /// assert_eq!(bl.end.x,ml[1].x);
    /// assert_eq!(bl.end.y,ml[1].y);
    /// ```
    fn from(l: Line<T>) -> Self {
        [mint::Point2::from(l.start), mint::Point2::from(l.end)]
    }
}

#[cfg(feature = "mint")]
impl<I: InputType> mint::IntoMint for Line<I> {
    type MintType = [mint::Point2<I>; 2];
}

#[cfg(feature = "mint")]
impl<I: InputType> mint::IntoMint for Point<I> {
    type MintType = mint::Point2<I>;
}

#[cfg(feature = "mint")]
impl<I: InputType> From<Point<I>> for mint::Point2<I> {
    #[inline]
    /// Converts to `mint::Point2` from `boostvoronoi::geometry::Point`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let p:Point<i32> = Point{x:1, y:2};
    /// let m = mint::Point2::from(p);
    ///
    /// assert_eq!(p.x,m.x);
    /// assert_eq!(p.y,m.y);
    /// ```
    fn from(p: Point<I>) -> Self {
        Self::from([p.x, p.y])
    }
}

#[cfg(feature = "mint")]
impl<I: InputType> From<mint::Point2<I>> for Point<I> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point` from `mint::Point2`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let m = mint::Point2{x:1,y:2};
    /// let p:Point<i32> = Point::from(m);
    /// assert_eq!(p.x,m.x);
    /// assert_eq!(p.y,m.y);
    /// ```
    fn from(p: mint::Point2<I>) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[cfg(feature = "mint")]
impl<I: InputType> From<&mint::Point2<I>> for Point<I> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point` from `&mint::Point2`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let m = mint::Point2{x:1,y:2};
    /// let p:Point<i32> = Point::from(&m);
    /// assert_eq!(p.x,m.x);
    /// assert_eq!(p.y,m.y);
    /// ```
    fn from(p: &mint::Point2<I>) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[cfg(feature = "mint")]
impl<F: OutputType> mint::IntoMint for &Vertex<F> {
    type MintType = mint::Point2<F>;
}

#[cfg(feature = "mint")]
impl<F: OutputType> From<&Vertex<F>> for mint::Point2<F> {
    #[inline]
    /// Converts to `mint::Point2` from `&boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = mint::Point2::<f32>::from(&v);
    /// assert_eq!(v.x(),p.x);
    /// assert_eq!(v.y(),p.y);
    /// ```
    fn from(v: &Vertex<F>) -> Self {
        Self { x: v.x(), y: v.y() }
    }
}

#[cfg(feature = "mint")]
impl<F: OutputType> From<Vertex<F>> for mint::Point2<F> {
    #[inline]
    /// Converts to `mint::Point2` from `boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = mint::Point2::<f32>::from(v.clone());
    /// assert_eq!(v.x(),p.x);
    /// assert_eq!(v.y(),p.y);
    /// ```
    fn from(v: Vertex<F>) -> Self {
        Self { x: v.x(), y: v.y() }
    }
}

#[cfg(feature = "glam")]
impl From<&Vertex<f64>> for glam::DVec2 {
    #[inline]
    /// Converts to `glam::DVec2` from `&boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f64>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = glam::DVec2::from(&v);
    /// assert_eq!(v.x(),p.x);
    /// assert_eq!(v.y(),p.y);
    /// ```
    fn from(v: &Vertex<f64>) -> Self {
        Self::new(v.x(), v.y())
    }
}

#[cfg(feature = "glam")]
impl From<Vertex<f64>> for glam::DVec2 {
    #[inline]
    /// Converts to `glam::DVec2` from `boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f64>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = glam::DVec2::from(v.clone());
    /// assert_eq!(v.x(),p.x);
    /// assert_eq!(v.y(),p.y);
    /// ```
    fn from(v: Vertex<f64>) -> Self {
        Self::new(v.x(), v.y())
    }
}

#[cfg(feature = "glam")]
impl From<&Vertex<f32>> for glam::Vec2 {
    #[inline]
    /// Converts to `glam::Vec2` from `&boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = glam::Vec2::from(&v);
    /// assert_eq!(v.x(),p.x);
    /// assert_eq!(v.y(),p.y);
    /// ```
    fn from(v: &Vertex<f32>) -> Self {
        Self::new(v.x(), v.y())
    }
}

#[cfg(feature = "glam")]
impl From<Vertex<f32>> for glam::Vec2 {
    #[inline]
    /// Converts to `glam::Vec2` from `boostvoronoi::diagram::Vertex`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// # use boostvoronoi_core::diagram::Vertex;
    /// # use boostvoronoi_core::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = glam::Vec2::from(v.clone());
    /// assert_eq!(v.x(),p.x);
    /// assert_eq!(v.y(),p.y);
    /// ```
    fn from(v: Vertex<f32>) -> Self {
        Self::new(v.x(), v.y())
    }
}

#[cfg(feature = "glam")]
impl From<glam::IVec2> for Point<i32> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point<i32>` from `glam::IVec2`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let m = glam::IVec2::new(1,2);
    /// let p:Point<i32> = Point::from(m);
    /// assert_eq!(p.x,m.x);
    /// assert_eq!(p.y,m.y);
    /// ```
    fn from(p: glam::IVec2) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[cfg(feature = "glam")]
impl From<&glam::IVec2> for Point<i32> {
    #[inline]
    /// Converts to `boostvoronoi::geometry::Point<i32>` from `&glam::IVec2`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let m = glam::IVec2::new(1,2);
    /// let p:Point<i32> = Point::from(&m);
    /// assert_eq!(p.x,m.x);
    /// assert_eq!(p.y,m.y);
    /// ```
    fn from(p: &glam::IVec2) -> Self {
        Self { x: p.x, y: p.y }
    }
}

#[cfg(feature = "glam")]
impl From<Point<i32>> for glam::IVec2 {
    #[inline]
    /// Converts to `glam::IVec2` from `boostvoronoi::geometry::Point<i32>`
    /// ```
    /// # use boostvoronoi_core::geometry::*;
    /// let p = Point::<i32>{x:1,y:2};
    /// let c = glam::IVec2::from(p);
    /// assert_eq!(p.x,c.x);
    /// assert_eq!(p.y,c.y);
    /// ```
    fn from(p: Point<i32>) -> Self {
        Self::new(p.x, p.y)
    }
}

#[cfg(feature = "glam")]
impl From<Line<i32>> for [glam::IVec2; 2] {
    #[inline]
    /// Converts to `[glam::IVec2;2]` from `boostvoronoi::geometry::Line<i32>`
    /// ```
    /// # use boostvoronoi_core::geometry::Line;
    /// let bl = Line::<i32>::from([0,1,2,3]);
    /// let ml:[glam::IVec2;2] = bl.into();
    /// assert_eq!(bl.start.x,ml[0].x);
    /// assert_eq!(bl.start.y,ml[0].y);
    /// assert_eq!(bl.end.x,ml[1].x);
    /// assert_eq!(bl.end.y,ml[1].y);
    ///
    /// let ml = [glam::IVec2::new(1,2),glam::IVec2::new(3,4)];
    /// let bl = Line::from(ml);
    /// assert_eq!(bl.start.x,ml[0].x);
    /// assert_eq!(bl.start.y,ml[0].y);
    /// assert_eq!(bl.end.x,ml[1].x);
    /// assert_eq!(bl.end.y,ml[1].y);
    /// ```
    fn from(l: Line<i32>) -> Self {
        [glam::IVec2::from(l.start), glam::IVec2::from(l.end)]
    }
}
