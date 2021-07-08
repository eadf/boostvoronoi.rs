//          Copyright Eadf (github.com/eadf) 2021.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

//! Some basic geometry data structures together with From trait implementations.

use super::diagram::Vertex;
use super::InputType;
use crate::OutputType;
use std::cmp;
use std::fmt;
use std::hash;
use std::ops::Neg;

/// A really simple 2d coordinate container type - integer only
#[derive(Copy, Clone, cmp::PartialEq, cmp::Eq, Hash)]
pub struct Point<T: InputType> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: InputType + fmt::Display + hash::Hash,
{
    /// Got "conflicting implementations of trait `std::convert::From..."
    /// So i picked the name as_f64 for this conversion
    pub fn as_f64(&self) -> [f64; 2] {
        [
            num::cast::<T, f64>(self.x).unwrap(),
            num::cast::<T, f64>(self.y).unwrap(),
        ]
    }

    pub (crate) fn distance_to(&self, circle: &crate::circle_event::CircleEvent) -> f64 {
        let x = num::cast::<T, f64>(self.x).unwrap() - circle.x().0;
        let y = num::cast::<T, f64>(self.y).unwrap() - circle.y().0;
        (x*x+y*y).sqrt()
    }
}

/// Converts to [f64;2] from boostvoronoi::geometry::Point
/// ```
/// # use boostvoronoi::geometry::*;
/// let c1 = [1,2];
/// let p:Point<i32> = Point::from(c1);
/// let c2: [f64;2] = p.into();
/// println!("c1:{:?}, c2:{:?}", c1, c2);
/// assert_eq!(c1[0] as f64, c2[0]);
/// assert_eq!(c1[1] as f64, c2[1]);
/// ```
impl<T: InputType> From<Point<T>> for [f64; 2] {
    fn from(coordinate: Point<T>) -> [f64; 2] {
        coordinate.as_f64()
    }
}

impl<T> fmt::Debug for Point<T>
where
    T: InputType + fmt::Display + hash::Hash,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.12},{:.12})", self.x, self.y,)
    }
}

/// Converts to boostvoronoi::geometry::Point from \[T;2\]
/// ```
/// # use boostvoronoi::geometry::*;
/// let c = [1,2];
/// let p:Point<i32> = Point::from(c);
/// assert_eq!(p.x,c[0]);
/// assert_eq!(p.y,c[1]);
/// ```
impl<T: InputType> From<[T; 2]> for Point<T> {
    fn from(coordinate: [T; 2]) -> Point<T> {
        Point {
            x: coordinate[0],
            y: coordinate[1],
        }
    }
}

/// Converts to boostvoronoi::geometry::Point from &\[T;2\]
/// ```
/// # use boostvoronoi::geometry::*;
/// let c = [1,2];
/// let p:Point<i32> = Point::from(&c);
/// assert_eq!(p.x,c[0]);
/// assert_eq!(p.y,c[1]);
/// ```
impl<T: InputType> From<&[T; 2]> for Point<T> {
    fn from(coordinate: &[T; 2]) -> Point<T> {
        Point {
            x: coordinate[0],
            y: coordinate[1],
        }
    }
}

#[cfg(feature = "geo")]
/// Converts to boostvoronoi::geometry::Point from geo::Coordinate
/// ```
/// # use boostvoronoi::geometry::*;
/// let c1 = geo::Coordinate{x:1,y:2};
/// let p:Point<i32> = Point::from(c1);
/// assert_eq!(p.x,c1.x);
/// assert_eq!(p.y,c1.y);
/// ```
impl<T: InputType> From<geo::Coordinate<T>> for Point<T> {
    fn from(coordinate: geo::Coordinate<T>) -> Point<T> {
        Point {
            x: coordinate.x,
            y: coordinate.y,
        }
    }
}

#[cfg(feature = "geo")]
/// Converts to geo::Coordinate from boostvoronoi::geometry::Point
/// ```
/// # use boostvoronoi::geometry::*;
/// let p = Point{x:1,y:2};
/// let c = geo::Coordinate::<i32>::from(p);
/// assert_eq!(p.x,c.x);
/// assert_eq!(p.y,c.y);
/// ```
impl<T: InputType + geo::CoordNum> From<Point<T>> for geo::Coordinate<T> {
    fn from(coordinate: Point<T>) -> geo::Coordinate<T> {
        geo::Coordinate {
            x: coordinate.x,
            y: coordinate.y,
        }
    }
}

#[cfg(feature = "geo")]
/// Converts to geo::Coordinate from &boostvoronoi::geometry::Point
/// ```
/// # use boostvoronoi::geometry::*;
/// let p = Point{x:1,y:2};
/// let c = geo::Coordinate::<i32>::from(&p);
/// assert_eq!(p.x,c.x);
/// assert_eq!(p.y,c.y);
/// ```
impl<T: InputType + geo::CoordNum> From<&Point<T>> for geo::Coordinate<T> {
    fn from(coordinate: &Point<T>) -> geo::Coordinate<T> {
        geo::Coordinate {
            x: coordinate.x,
            y: coordinate.y,
        }
    }
}

#[cfg(feature = "geo")]
/// Converts to geo::Coordinate from &boostvoronoi::diagram::Vertex
/// ```
/// # use boostvoronoi::geometry::*;
/// # use boostvoronoi::diagram::Vertex;
/// # use boostvoronoi::diagram::VertexIndex;
///
/// let v = Vertex::<i32,f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
/// let c = geo::Coordinate::<f32>::from(&v);
/// assert_eq!(v.x(),c.x);
/// assert_eq!(v.y(),c.y);
/// ```
impl<I, F> From<&Vertex<I, F>> for geo::Coordinate<F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F> + geo::CoordFloat,
{
    fn from(vertex: &Vertex<I, F>) -> geo::Coordinate<F> {
        geo::Coordinate {
            x: vertex.x(),
            y: vertex.y(),
        }
    }
}

#[cfg(feature = "geo")]
/// Converts to geo::Line from boostvoronoi::geometry::Line
/// ```
/// # use boostvoronoi::geometry::*;
/// let bl = Line::from([0,1,2,3]);
/// let gl = geo::Line::<i32>::from(bl);
/// assert_eq!(bl.start.x,gl.start.x);
/// assert_eq!(bl.start.y,gl.start.y);
/// assert_eq!(bl.end.x,gl.end.x);
/// assert_eq!(bl.end.y,gl.end.y);
/// ```
impl<T: InputType + geo::CoordNum> From<Line<T>> for geo::Line<T> {
    fn from(line: Line<T>) -> geo::Line<T> {
        geo::Line {
            start: geo::Coordinate::from(line.start),
            end: geo::Coordinate::from(line.end),
        }
    }
}

#[cfg(feature = "geo")]
/// Converts to Line from geo::Line
/// ```
/// # use boostvoronoi::geometry::*;
/// let gl = geo::Line::from([(0,1),(2,3)]);
/// let bl = Line::<i32>::from(gl);
/// assert_eq!(bl.start.x,gl.start.x);
/// assert_eq!(bl.start.y,gl.start.y);
/// assert_eq!(bl.end.x,gl.end.x);
/// assert_eq!(bl.end.y,gl.end.y);
/// ```
impl<T: InputType + geo::CoordNum> From<geo::Line<T>> for Line<T> {
    fn from(line: geo::Line<T>) -> Line<T> {
        Line {
            start: Point::from(line.start),
            end: Point::from(line.end),
        }
    }
}

#[cfg(feature = "cgmath")]
/// Converts to boostvoronoi::geometry::Point from cgmath::Point2
/// ```
/// # use boostvoronoi::geometry::*;
/// let c1 = cgmath::Point2{x:1,y:2};
/// let p:Point<i32> = Point::from(c1);
/// assert_eq!(p.x,c1.x);
/// assert_eq!(p.y,c1.y);
/// ```
impl<T: InputType + cgmath::BaseNum> From<cgmath::Point2<T>> for Point<T> {
    fn from(coordinate: cgmath::Point2<T>) -> Point<T> {
        Point {
            x: coordinate.x,
            y: coordinate.y,
        }
    }
}

#[cfg(feature = "cgmath")]
/// Converts to geo::Coordinate from boostvoronoi::geometry::Point
/// ```
/// # use boostvoronoi::geometry::*;
/// let p = Point{x:1,y:2};
/// let c = cgmath::Point2::<i32>::from(p);
/// assert_eq!(p.x,c.x);
/// assert_eq!(p.y,c.y);
/// ```
impl<T: InputType + cgmath::BaseNum> From<Point<T>> for cgmath::Point2<T> {
    fn from(coordinate: Point<T>) -> cgmath::Point2<T> {
        cgmath::Point2 {
            x: coordinate.x,
            y: coordinate.y,
        }
    }
}

#[cfg(feature = "cgmath")]
/// Converts to cgmath::Point2 from &boostvoronoi::diagram::Vertex
/// ```
/// # use boostvoronoi::geometry::*;
/// # use boostvoronoi::diagram::Vertex;
/// # use boostvoronoi::diagram::VertexIndex;
///
/// let v = Vertex::<i32,f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
/// let p = cgmath::Point2::<f32>::from(&v);
/// assert_eq!(v.x(),p.x);
/// assert_eq!(v.y(),p.y);
/// ```
impl<I, F> From<&Vertex<I, F>> for cgmath::Point2<F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F> + cgmath::BaseNum,
{
    fn from(vertex: &Vertex<I, F>) -> cgmath::Point2<F> {
        cgmath::Point2 {
            x: vertex.x(),
            y: vertex.y(),
        }
    }
}

/// A really simple 2d line type - integer only
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

/// Converts to Line from \[T;4\]
/// ```
/// # use boostvoronoi::geometry::Line;
/// let a = [0,1,2,3];
/// let bl = Line::<i32>::from(a);
/// assert_eq!(bl.start.x,a[0]);
/// assert_eq!(bl.start.y,a[1]);
/// assert_eq!(bl.end.x,a[2]);
/// assert_eq!(bl.end.y,a[3]);
/// ```
impl<T: InputType> From<[T; 4]> for Line<T> {
    fn from(line: [T; 4]) -> Line<T> {
        Line {
            start: Point {
                x: line[0],
                y: line[1],
            },
            end: Point {
                x: line[2],
                y: line[3],
            },
        }
    }
}

/// Converts to \[T;4\] from boostvoronoi::geometry::Line
/// ```
/// # use boostvoronoi::geometry::*;
/// let l = Line::from([0,1,2,3]);
/// let a = <[i32;4]>::from(l);
/// assert_eq!(l.start.x,a[0]);
/// assert_eq!(l.start.y,a[1]);
/// assert_eq!(l.end.x,a[2]);
/// assert_eq!(l.end.y,a[3]);
/// ```
impl<T: InputType> From<Line<T>> for [T; 4] {
    fn from(line: Line<T>) -> [T; 4] {
        [line.start.x, line.start.y, line.end.x, line.end.y]
    }
}

/// Converts to \[T;4\] from &boostvoronoi::geometry::Line
/// ```
/// # use boostvoronoi::geometry::*;
/// let l = Line::from([0,1,2,3]);
/// let a = <[i32;4]>::from(&l);
/// assert_eq!(l.start.x,a[0]);
/// assert_eq!(l.start.y,a[1]);
/// assert_eq!(l.end.x,a[2]);
/// assert_eq!(l.end.y,a[3]);
/// ```
impl<T: InputType> From<&Line<T>> for [T; 4] {
    fn from(line: &Line<T>) -> [T; 4] {
        [line.start.x, line.start.y, line.end.x, line.end.y]
    }
}

/// Converts to Line from &\[T;4\]
/// ```
/// # use boostvoronoi::geometry::Line;
/// let a = [0,1,2,3];
/// let bl = Line::<i32>::from(&a);
/// assert_eq!(bl.start.x,a[0]);
/// assert_eq!(bl.start.y,a[1]);
/// assert_eq!(bl.end.x,a[2]);
/// assert_eq!(bl.end.y,a[3]);
/// ```
impl<T: InputType> From<&[T; 4]> for Line<T> {
    fn from(line: &[T; 4]) -> Line<T> {
        Line {
            start: Point {
                x: line[0],
                y: line[1],
            },
            end: Point {
                x: line[2],
                y: line[3],
            },
        }
    }
}

/// Converts to \[T;2\] from &boostvoronoi::diagram::Vertex
/// ```
/// # use boostvoronoi::geometry::*;
/// # use boostvoronoi::diagram::Vertex;
/// # use boostvoronoi::diagram::VertexIndex;
///
/// let v = Vertex::<i32,f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
/// let a = <[f32;2]>::from(&v);
/// assert_eq!(v.x(),a[0]);
/// assert_eq!(v.y(),a[1]);
/// ```
impl<I, F> From<&Vertex<I, F>> for [F; 2]
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    fn from(vertex: &Vertex<I, F>) -> [F; 2] {
        [vertex.x(), vertex.y()]
    }
}
