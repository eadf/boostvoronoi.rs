// Boost.Polygon library voronoi_graphic_utils.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! Graphical utilities.

use super::{InputType, OutputType};
use crate::BvError;

use super::{Line, Point};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Neg;

/// Utilities class, that contains set of routines handful for visualization.
pub struct VoronoiVisualUtils<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    #[doc(hidden)]
    pdi_: PhantomData<I>,
    #[doc(hidden)]
    pdo_: PhantomData<F>,
}

impl<I, F> VoronoiVisualUtils<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    /// Discretize parabolic Voronoi edge.
    /// Parabolic Voronoi edges are always formed by one point and one segment
    /// from the initial input set.
    ///
    /// Args:
    ///   point: input point in diagram coordinate system
    ///   segment: input segment in diagram coordinate system
    ///   max_dist: maximum discretization distance in output coordinate system,
    ///   affine: an affine transform converting from diagram coordinate system to output coordinate system,
    ///   discretization: point discretization of the given Voronoi edge in output coordinate system,
    ///
    /// Template arguments:
    ///   InCT: coordinate type of the input geometries (usually integer).
    ///   Point: point type, should model point concept.
    ///   Segment: segment type, should model segment concept.
    ///
    /// Important:
    ///   discretization should contain both edge endpoints initially.
    pub fn discretize(
        point: &Point<I>,
        segment: &Line<I>,
        max_dist: F,
        affine: &SimpleAffine<I, F>,
        discretization: &mut Vec<[F; 2]>,
    ) {
        // no need to discretize infinitely small distances
        if discretization[0][0] == discretization[1][0]
            && discretization[0][1] == discretization[1][1]
        {
            return;
        }
        // Apply the linear transformation to move start point of the segment to
        // the point with coordinates (0, 0) and the direction of the segment to
        // coincide the positive direction of the x-axis.
        let segm_vec_x: F =
            affine.transform_ix(segment.end.x) - affine.transform_ix(segment.start.x);
        let segm_vec_y: F =
            affine.transform_iy(segment.end.y) - affine.transform_iy(segment.start.y);
        let sqr_segment_length = segm_vec_x * segm_vec_x + segm_vec_y * segm_vec_y;

        // Compute x-coordinates of the endpoints of the edge
        // in the transformed space.
        let projection_start =
            sqr_segment_length * Self::get_point_projection(affine, &discretization[0], segment);
        let projection_end =
            sqr_segment_length * Self::get_point_projection(affine, &discretization[1], segment);

        // Compute parabola parameters in the transformed space.
        // Parabola has next representation:
        // f(x) = ((x-rot_x)^2 + rot_y^2) / (2.0*rot_y).
        let point_vec_x = affine.transform_ix(point.x) - affine.transform_ix(segment.start.x);
        let point_vec_y = affine.transform_iy(point.y) - affine.transform_iy(segment.start.y);
        let rot_x = segm_vec_x * point_vec_x + segm_vec_y * point_vec_y;
        let rot_y = segm_vec_x * point_vec_y - segm_vec_y * point_vec_x;

        // Save the last point.
        let last_point = (*discretization)[1];
        let _ = discretization.pop();

        // Use stack to avoid recursion.
        let mut point_stack = vec![projection_end];
        let mut cur_x = projection_start;
        let mut cur_y = Self::parabola_y(cur_x, rot_x, rot_y);

        // Adjust max_dist parameter in the transformed space.
        let max_dist_transformed = max_dist * max_dist * sqr_segment_length;
        while !point_stack.is_empty() {
            let new_x = point_stack[point_stack.len() - 1]; // was .top();
            let new_y = Self::parabola_y(new_x, rot_x, rot_y);

            // Compute coordinates of the point of the parabola that is
            // furthest from the current line segment.
            let mid_x = (new_y - cur_y) / (new_x - cur_x) * rot_y + rot_x;
            let mid_y = Self::parabola_y(mid_x, rot_x, rot_y);

            // Compute maximum distance between the given parabolic arc
            // and line segment that discretize it.
            let mut dist = (new_y - cur_y) * (mid_x - cur_x) - (new_x - cur_x) * (mid_y - cur_y);
            #[allow(clippy::suspicious_operation_groupings)]
            {
                dist = dist * dist
                    / ((new_y - cur_y) * (new_y - cur_y) + (new_x - cur_x) * (new_x - cur_x));
            }
            if dist.is_nan() {
                break;
            }
            if dist <= max_dist_transformed {
                // Distance between parabola and line segment is less than max_dist.
                let _ = point_stack.pop();
                let inter_x = (segm_vec_x * new_x - segm_vec_y * new_y) / sqr_segment_length
                    + affine.transform_ix(segment.start.x);
                let inter_y = (segm_vec_x * new_y + segm_vec_y * new_x) / sqr_segment_length
                    + affine.transform_iy(segment.start.y);
                discretization.push([inter_x, inter_y]);
                cur_x = new_x;
                cur_y = new_y;
            } else {
                point_stack.push(mid_x);
            }
        }
        // Update last point.
        let discretization_len = discretization.len();
        discretization[discretization_len - 1] = last_point;
    }

    /// Compute y(x) = ((x - a) * (x - a) + b * b) / (2 * b).
    #[inline(always)]
    #[allow(clippy::suspicious_operation_groupings)]
    fn parabola_y(x: F, a: F, b: F) -> F {
        ((x - a) * (x - a) + b * b) / (b + b)
    }

    // Get normalized length of the distance between:
    //   1) point projection onto the segment
    //   2) start point of the segment
    // Return this length divided by the segment length. This is made to avoid
    // sqrt computation during transformation from the initial space to the
    // transformed one and vice versa. The assumption is made that projection of
    // the point lies between the start-point and endpoint of the segment.
    pub fn get_point_projection(
        affine: &SimpleAffine<I, F>,
        point: &[F; 2],
        segment: &Line<I>,
    ) -> F {
        let segment_vec_x: F =
            affine.transform_ix(segment.end.x) - affine.transform_ix(segment.start.x);
        let segment_vec_y: F =
            affine.transform_iy(segment.end.y) - affine.transform_iy(segment.start.y);
        let point_vec_x = point[0] - affine.transform_ix(segment.start.x);
        let point_vec_y = point[1] - affine.transform_iy(segment.start.y);
        let sqr_segment_length = segment_vec_x * segment_vec_x + segment_vec_y * segment_vec_y;
        let vec_dot = segment_vec_x * point_vec_x + segment_vec_y * point_vec_y;
        vec_dot / sqr_segment_length
    }

    #[inline(always)]
    pub fn cast_io(value: I) -> F {
        super::TypeConverter2::<I, F>::i_to_f(value)
    }
}

/// A simple 2d AABB
/// If min_max is none no data has not been assigned.
#[derive(PartialEq, Eq, Clone, fmt::Debug)]
pub struct Aabb2<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    min_max_: Option<([F; 2], [F; 2])>,
    #[doc(hidden)]
    pdi_: PhantomData<I>,
}

impl<I, F> Default for Aabb2<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    #[inline]
    fn default() -> Self {
        Self {
            min_max_: None,
            pdi_: PhantomData,
        }
    }
}

impl<I, F> Aabb2<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    /// Creates a new AABB with the limits defined by 'p1' & 'p2'
    pub fn new(p1: &Point<I>, p2: &Point<I>) -> Self {
        let mut rv = Self::default();
        rv.update_point(p1);
        rv.update_point(p2);
        rv
    }

    /// Creates a new AABB with i32 coordinates
    pub fn new_from_i32(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        let mut rv = Self::default();
        rv.update_coordinate(x1, y1);
        rv.update_coordinate(x2, y2);
        rv
    }

    #[inline(always)]
    pub fn update_point(&mut self, point: &Point<I>) {
        let x = super::TypeConverter2::i_to_f(point.x);
        let y = super::TypeConverter2::i_to_f(point.y);
        self.update_vertex(x, y);
    }

    #[inline(always)]
    pub fn update_coordinate(&mut self, x: i32, y: i32) {
        let x = super::TypeConverter2::<I, F>::i32_to_f(x);
        let y = super::TypeConverter2::<I, F>::i32_to_f(y);
        self.update_vertex(x, y);
    }

    #[inline]
    pub fn update_vertex(&mut self, x: F, y: F) {
        if self.min_max_.is_none() {
            self.min_max_ = Some(([x, y], [x, y]));
            return;
        }
        let (mut aabb_min, mut aabb_max) = self.min_max_.take().unwrap();

        if x < aabb_min[0] {
            aabb_min[0] = x;
        }
        if y < aabb_min[1] {
            aabb_min[1] = y;
        }
        if x > aabb_max[0] {
            aabb_max[0] = x;
        }
        if y > aabb_max[1] {
            aabb_max[1] = y;
        }
        self.min_max_ = Some((aabb_min, aabb_max));
    }

    #[inline(always)]
    pub fn update_line(&mut self, line: &Line<I>) {
        self.update_point(&line.start);
        self.update_point(&line.end);
    }

    #[inline(always)]
    pub fn get_high(&self) -> Option<[F; 2]> {
        if let Some((_, high)) = self.min_max_ {
            return Some(high);
        }
        None
    }

    #[inline(always)]
    pub fn get_low(&self) -> Option<[F; 2]> {
        if let Some((low, _)) = self.min_max_ {
            return Some(low);
        }
        None
    }

    /// grows the aabb uniformly by some percent.
    /// method does nothing if not initialized
    pub fn grow_percent(&mut self, percent: i32) {
        if self.min_max_.is_some() {
            let size_x = self.get_high().unwrap()[0] - self.get_low().unwrap()[0];
            let size_y = self.get_high().unwrap()[1] - self.get_low().unwrap()[1];
            let size = if size_x > size_y { size_x } else { size_y };

            let delta = size * super::TypeConverter2::<I, F>::f32_to_f((percent as f32) / 100.0);

            let mut p = self.get_high().unwrap();
            p[0] = p[0] + delta;
            p[1] = p[1] + delta;
            self.update_vertex(p[0], p[1]);
            let mut p = self.get_low().unwrap();
            p[0] = p[0] - delta;
            p[1] = p[1] - delta;
            self.update_vertex(p[0], p[1]);
        }
    }

    /// returns Some(true) if the aabb contains the point (inclusive)
    /// returns None if the aabb is uninitialized
    ///```
    /// # use boostvoronoi::Point;
    /// # use boostvoronoi::visual_utils::Aabb2;
    /// let p0 = Point::from([0,0]);
    /// let p1 = Point::from([1,1]);
    ///
    /// let aabb = Aabb2::<i32,f32>::new(&p0,&p1);
    /// assert!(aabb.contains_point(&Point::from([1,1])).unwrap_or(false));
    /// assert!(!aabb.contains_point(&Point::from([2,1])).unwrap_or(true));
    /// ```
    #[inline]
    pub fn contains_point(&self, point: &Point<I>) -> Option<bool> {
        if let Some(min_max) = self.min_max_ {
            let x = super::TypeConverter2::<I, F>::i_to_f(point.x);
            let y = super::TypeConverter2::<I, F>::i_to_f(point.y);

            Some(x >= min_max.0[0] && x <= min_max.1[0] && y >= min_max.0[1] && y <= min_max.1[1])
        } else {
            None
        }
    }

    /// returns Some(true) if the aabb contains the line (inclusive)
    /// returns None if the aabb is uninitialized
    /// ```
    /// # use boostvoronoi::{Line,Point};
    /// # use boostvoronoi::visual_utils::Aabb2;
    /// let p0 = Point::from([0,0]);
    /// let p1 = Point::from([10,10]);
    ///
    /// let aabb = Aabb2::<i32,f32>::new(&p0,&p1);
    /// assert!( aabb.contains_line(&Line::from([1,1,10,10])).unwrap_or(false));
    /// assert!(!aabb.contains_line(&Line::from([1,-1,10,10])).unwrap_or(true));
    /// ```
    #[inline]
    pub fn contains_line(&self, line: &Line<I>) -> Option<bool> {
        if self.min_max_.is_some() {
            // unwrap is safe now
            Some(
                self.contains_point(&line.start).unwrap()
                    && self.contains_point(&line.end).unwrap(),
            )
        } else {
            None
        }
    }
}

/// This is a simple affine transformation object.
/// Inadvertently it also serves as a type converter F<->I<->i32
/// It can pan and zoom but not rotate.
#[derive(PartialEq, Clone, fmt::Debug)]
pub struct SimpleAffine<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    /// The offsets used to center the 'source' coordinate system. Typically the input geometry
    /// in this case.
    to_center_: [F; 2],
    /// A zoom scale
    pub scale: [F; 2],
    /// The offsets needed to center coordinates of interest on the 'dest' coordinate system.
    /// i.e. the screen coordinate system.
    pub to_offset: [F; 2],
    #[doc(hidden)]
    pdi_: PhantomData<I>,
}

impl<I, F> Default for SimpleAffine<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    #[inline]
    fn default() -> Self {
        Self {
            to_center_: [F::zero(), F::zero()],
            scale: [F::one(), F::one()],
            to_offset: [F::zero(), F::zero()],
            pdi_: PhantomData,
        }
    }
}

impl<I, F> SimpleAffine<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    pub fn new(source_aabb: &Aabb2<I, F>, dest_aabb: &Aabb2<I, F>) -> Result<Self, BvError> {
        let i32_to_f1 = super::TypeConverter2::<I, F>::i32_to_f;
        let min_dim = i32_to_f1(10);

        if let Some(s_low) = source_aabb.get_low() {
            if let Some(s_high) = source_aabb.get_high() {
                if let Some(d_low) = dest_aabb.get_low() {
                    if let Some(d_high) = dest_aabb.get_high() {
                        //println!("s_low:{:?},s_high:{:?},d_low:{:?},d_high:{:?}", s_low, s_high, d_low, d_high);

                        let source_aabb_center = [
                            -(s_low[0] + s_high[0]) / i32_to_f1(2_i32),
                            -(s_low[1] + s_high[1]) / i32_to_f1(2_i32),
                        ];
                        let source_aabb_size = [
                            (s_high[0] - s_low[0]).max(min_dim),
                            (s_high[1] - s_low[1]).max(min_dim),
                        ];

                        let dest_aabb_center = [
                            (d_low[0] + d_high[0]) / super::TypeConverter2::<I, F>::i32_to_f(2_i32),
                            (d_low[1] + d_high[1]) / super::TypeConverter2::<I, F>::i32_to_f(2_i32),
                        ];
                        let dest_aabb_size = [
                            (d_high[0] - d_low[0]).max(min_dim),
                            (d_high[1] - d_low[1]).max(min_dim),
                        ];

                        // make sure the larges dimension of source fits inside smallest of dest
                        let source_aabb_size = source_aabb_size[0].max(source_aabb_size[1]);
                        let dest_aabb_size = dest_aabb_size[0].min(dest_aabb_size[1]);
                        let scale = dest_aabb_size / source_aabb_size;

                        return Ok(Self {
                            to_center_: source_aabb_center,
                            scale: [scale, scale],
                            to_offset: dest_aabb_center,
                            pdi_: PhantomData,
                        });
                    }
                }
            }
        }
        Err(BvError::InternalError(format!(
            "could not get dimension of the AABB. {}:{}",
            file!(),
            line!()
        )))
    }

    /// transform from destination coordinate system to source coordinate system
    #[inline(always)]
    pub fn reverse_transform(&self, x: F, y: F) -> Result<[I; 2], BvError> {
        let x = self.reverse_transform_x(x)?;
        let y = self.reverse_transform_y(y)?;
        Ok([x, y])
    }

    /// transform from destination coordinate system to source coordinate system
    #[inline(always)]
    pub fn reverse_transform_x(&self, x: F) -> Result<I, BvError> {
        super::TypeConverter2::<I, F>::try_f_to_i(
            ((x - self.to_offset[0]) / self.scale[0] - self.to_center_[0]).round(),
        )
    }

    /// transform from dest coordinate system to source coordinate system
    #[inline(always)]
    pub fn reverse_transform_y(&self, y: F) -> Result<I, BvError> {
        super::TypeConverter2::<I, F>::try_f_to_i(
            ((y - self.to_offset[1]) / self.scale[1] - self.to_center_[1]).round(),
        )
    }

    /// transform from source coordinate system to dest coordinate system
    #[inline(always)]
    pub fn transform(&self, x: F, y: F) -> [F; 2] {
        [self.transform_x(x), self.transform_y(y)]
    }

    /// transform from source coordinate system to dest coordinate system
    /// float x coordinate
    #[inline(always)]
    pub fn transform_x(&self, x: F) -> F {
        (x + self.to_center_[0]) * self.scale[0] + self.to_offset[0]
    }

    /// transform from source coordinate system to dest coordinate system
    /// float y coordinate
    #[inline(always)]
    pub fn transform_y(&self, y: F) -> F {
        (y + self.to_center_[1]) * self.scale[1] + self.to_offset[1]
    }

    /// transform from source coordinate system to dest coordinate system
    #[inline(always)]
    pub fn transform_i(&self, point: [I; 2]) -> [F; 2] {
        [self.transform_ix(point[0]), self.transform_iy(point[1])]
    }

    /// transform from source coordinate system to dest coordinate system
    #[inline(always)]
    pub fn transform_p(&self, point: &Point<I>) -> [F; 2] {
        [self.transform_ix(point.x), self.transform_iy(point.y)]
    }

    /// transform from source coordinate system to dest coordinate system
    /// /// integer x coordinate
    #[inline(always)]
    pub fn transform_ix(&self, x: I) -> F {
        (super::TypeConverter2::<I, F>::i_to_f(x) + self.to_center_[0]) * self.scale[0]
            + self.to_offset[0]
    }

    /// transform from source coordinate system to dest coordinate system
    /// integer y coordinate
    #[inline(always)]
    pub fn transform_iy(&self, y: I) -> F {
        (super::TypeConverter2::<I, F>::i_to_f(y) + self.to_center_[1]) * self.scale[1]
            + self.to_offset[1]
    }
}
