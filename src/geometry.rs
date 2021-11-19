// Boost.Polygon library detail/voronoi_structures.hpp header file
//
//          Copyright Eadf (github.com/eadf) 2021.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

//! Some basic geometry data structures together with From trait implementations.

use crate::{cast, diagram::Vertex, InputType, OutputType};
use std::cmp;
use std::fmt;

/// A really simple 2d coordinate container type - integer only
#[derive(Copy, Clone, cmp::PartialEq, cmp::PartialOrd, cmp::Eq, Hash)]
pub struct Point<T: InputType> {
    pub x: T,
    pub y: T,
}

impl<T: InputType> Point<T> {
    /// Got "conflicting implementations of trait `std::convert::From..."
    /// So i picked the name as_f64 for this conversion
    pub fn as_f64(&self) -> [f64; 2] {
        [cast::<T, f64>(self.x), cast::<T, f64>(self.y)]
    }

    #[cfg(feature = "ce_corruption_check")]
    pub(crate) fn distance_to(&self, circle: &crate::circle_event::CircleEvent) -> f64 {
        let x = cast::<T, f64>(self.x) - circle.x().0;
        let y = cast::<T, f64>(self.y) - circle.y().0;
        (x * x + y * y).sqrt()
    }
}

impl<T: InputType> From<Point<T>> for [f64; 2] {
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
    fn from(coordinate: Point<T>) -> [f64; 2] {
        coordinate.as_f64()
    }
}

impl<T: InputType> fmt::Debug for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.12},{:.12})", self.x, self.y,)
    }
}

impl<T: InputType> From<[T; 2]> for Point<T> {
    /// Converts to boostvoronoi::geometry::Point from \[T;2\]
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// let c = [1,2];
    /// let p:Point<i32> = Point::from(c);
    /// assert_eq!(p.x,c[0]);
    /// assert_eq!(p.y,c[1]);
    /// ```
    fn from(coordinate: [T; 2]) -> Point<T> {
        Point {
            x: coordinate[0],
            y: coordinate[1],
        }
    }
}

impl<T: InputType> From<&[T; 2]> for Point<T> {
    /// Converts to boostvoronoi::geometry::Point from &\[T;2\]
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// let c = [1,2];
    /// let p:Point<i32> = Point::from(&c);
    /// assert_eq!(p.x,c[0]);
    /// assert_eq!(p.y,c[1]);
    /// ```
    fn from(coordinate: &[T; 2]) -> Point<T> {
        Point {
            x: coordinate[0],
            y: coordinate[1],
        }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType> From<geo::Coordinate<T>> for Point<T> {
    /// Converts to boostvoronoi::geometry::Point from geo::Coordinate
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// let c1 = geo::Coordinate{x:1,y:2};
    /// let p:Point<i32> = Point::from(c1);
    /// assert_eq!(p.x,c1.x);
    /// assert_eq!(p.y,c1.y);
    /// ```
    fn from(coordinate: geo::Coordinate<T>) -> Point<T> {
        Point {
            x: coordinate.x,
            y: coordinate.y,
        }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<Point<T>> for geo::Coordinate<T> {
    /// Converts to geo::Coordinate from boostvoronoi::geometry::Point
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// let p = Point{x:1,y:2};
    /// let c = geo::Coordinate::<i32>::from(p);
    /// assert_eq!(p.x,c.x);
    /// assert_eq!(p.y,c.y);
    /// ```
    fn from(coordinate: Point<T>) -> geo::Coordinate<T> {
        geo::Coordinate {
            x: coordinate.x,
            y: coordinate.y,
        }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<&Point<T>> for geo::Coordinate<T> {
    /// Converts to geo::Coordinate from &boostvoronoi::geometry::Point
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// let p = Point{x:1,y:2};
    /// let c = geo::Coordinate::<i32>::from(&p);
    /// assert_eq!(p.x,c.x);
    /// assert_eq!(p.y,c.y);
    /// ```
    fn from(coordinate: &Point<T>) -> geo::Coordinate<T> {
        geo::Coordinate {
            x: coordinate.x,
            y: coordinate.y,
        }
    }
}

#[cfg(feature = "geo")]
impl<F: OutputType + geo::CoordFloat> From<&Vertex<F>> for geo::Coordinate<F> {
    /// Converts to geo::Coordinate from &boostvoronoi::diagram::Vertex
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// # use boostvoronoi::diagram::Vertex;
    /// # use boostvoronoi::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let c = geo::Coordinate::<f32>::from(&v);
    /// assert_eq!(v.x(),c.x);
    /// assert_eq!(v.y(),c.y);
    /// ```
    fn from(vertex: &Vertex<F>) -> geo::Coordinate<F> {
        geo::Coordinate {
            x: vertex.x(),
            y: vertex.y(),
        }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<Line<T>> for geo::Line<T> {
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
    fn from(line: Line<T>) -> geo::Line<T> {
        geo::Line {
            start: geo::Coordinate::from(line.start),
            end: geo::Coordinate::from(line.end),
        }
    }
}

#[cfg(feature = "geo")]
impl<T: InputType + geo::CoordNum> From<geo::Line<T>> for Line<T> {
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
    fn from(line: geo::Line<T>) -> Line<T> {
        Line {
            start: Point::from(line.start),
            end: Point::from(line.end),
        }
    }
}

#[cfg(feature = "cgmath")]
impl<T: InputType + cgmath::BaseNum> From<cgmath::Point2<T>> for Point<T> {
    /// Converts to boostvoronoi::geometry::Point from cgmath::Point2
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// let c1 = cgmath::Point2{x:1,y:2};
    /// let p:Point<i32> = Point::from(c1);
    /// assert_eq!(p.x,c1.x);
    /// assert_eq!(p.y,c1.y);
    /// ```
    fn from(coordinate: cgmath::Point2<T>) -> Point<T> {
        Point {
            x: coordinate.x,
            y: coordinate.y,
        }
    }
}

#[cfg(feature = "cgmath")]
impl<T: InputType + cgmath::BaseNum> From<Point<T>> for cgmath::Point2<T> {
    /// Converts to geo::Coordinate from boostvoronoi::geometry::Point
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// let p = Point{x:1,y:2};
    /// let c = cgmath::Point2::<i32>::from(p);
    /// assert_eq!(p.x,c.x);
    /// assert_eq!(p.y,c.y);
    /// ```
    fn from(coordinate: Point<T>) -> cgmath::Point2<T> {
        cgmath::Point2 {
            x: coordinate.x,
            y: coordinate.y,
        }
    }
}

#[cfg(feature = "cgmath")]
impl<F: OutputType + cgmath::BaseNum> From<&Vertex<F>> for cgmath::Point2<F> {
    /// Converts to cgmath::Point2 from &boostvoronoi::diagram::Vertex
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// # use boostvoronoi::diagram::Vertex;
    /// # use boostvoronoi::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = cgmath::Point2::<f32>::from(&v);
    /// assert_eq!(v.x(),p.x);
    /// assert_eq!(v.y(),p.y);
    /// ```
    fn from(vertex: &Vertex<F>) -> cgmath::Point2<F> {
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

impl<T: InputType, IT: Copy + Into<Point<T>>> From<[IT; 2]> for Line<T> {
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

impl<T: InputType> From<[T; 4]> for Line<T> {
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

impl<T: InputType> From<Line<T>> for [T; 4] {
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
    fn from(line: Line<T>) -> [T; 4] {
        [line.start.x, line.start.y, line.end.x, line.end.y]
    }
}

impl<T: InputType> From<&Line<T>> for [T; 4] {
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
    fn from(line: &Line<T>) -> [T; 4] {
        [line.start.x, line.start.y, line.end.x, line.end.y]
    }
}

impl<T: InputType> From<&[T; 4]> for Line<T> {
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

impl<F: OutputType> From<&Vertex<F>> for [F; 2] {
    /// Converts to \[T;2\] from &boostvoronoi::diagram::Vertex
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// # use boostvoronoi::diagram::Vertex;
    /// # use boostvoronoi::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let a = <[f32;2]>::from(&v);
    /// assert_eq!(v.x(),a[0]);
    /// assert_eq!(v.y(),a[1]);
    /// ```
    fn from(vertex: &Vertex<F>) -> [F; 2] {
        [vertex.x(), vertex.y()]
    }
}

#[cfg(feature = "mint")]
impl<I: InputType> mint::IntoMint for Point<I> {
    type MintType = mint::Point2<I>;
}

#[cfg(feature = "mint")]
impl<I: InputType> From<Point<I>> for mint::Point2<I> {
    /// Converts to mint::Point2 from boostvoronoi::geometry::Point
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// let p:Point<i32> = Point{x:1, y:2};
    /// let m = mint::Point2::from(p);
    ///
    /// assert_eq!(p.x,m.x);
    /// assert_eq!(p.y,m.y);
    /// ```
    fn from(p: Point<I>) -> mint::Point2<I> {
        mint::Point2::from([p.x, p.y])
    }
}

#[cfg(feature = "mint")]
impl<I: InputType> From<mint::Point2<I>> for Point<I> {
    /// Converts to boostvoronoi::geometry::Point from mint::Point2
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// let m = mint::Point2{x:1,y:2};
    /// let p:Point<i32> = Point::from(m);
    /// assert_eq!(p.x,m.x);
    /// assert_eq!(p.y,m.y);
    /// ```
    fn from(m: mint::Point2<I>) -> Point<I> {
        Point { x: m.x, y: m.y }
    }
}

#[cfg(feature = "mint")]
impl<F: OutputType> mint::IntoMint for &Vertex<F> {
    type MintType = mint::Point2<F>;
}

#[cfg(feature = "mint")]
impl<F: OutputType> From<&Vertex<F>> for mint::Point2<F> {
    /// Converts to mint::Point2 from &boostvoronoi::diagram::Vertex
    /// ```
    /// # use boostvoronoi::geometry::*;
    /// # use boostvoronoi::diagram::Vertex;
    /// # use boostvoronoi::diagram::VertexIndex;
    ///
    /// let v = Vertex::<f32>::new_3(VertexIndex(0),1.0,2.0,false).get();
    /// let p = mint::Point2::<f32>::from(&v);
    /// assert_eq!(v.x(),p.x);
    /// assert_eq!(v.y(),p.y);
    /// ```
    fn from(vertex: &Vertex<F>) -> mint::Point2<F> {
        mint::Point2 {
            x: vertex.x(),
            y: vertex.y(),
        }
    }
}
