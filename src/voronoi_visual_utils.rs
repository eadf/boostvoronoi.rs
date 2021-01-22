// Boost.Polygon library voronoi_graphic_utils.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::voronoi_diagram as VD;
use super::voronoi_structures as VS;
use super::{BigFloatType, BigIntType, InputType, OutputType};

use geo::{Coordinate, Line};
use std::marker::PhantomData;
use std::ops::Neg;

/// Utilities class, that contains set of routines handful for visualization.
pub struct VoronoiVisualUtils<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    _pdi: PhantomData<I1>,
    _pdo: PhantomData<F1>,
    _pdbi: PhantomData<I2>,
    _pdbf: PhantomData<F2>,
}

impl<I1, F1, I2, F2> VoronoiVisualUtils<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    /// Discretize parabolic Voronoi edge.
    /// Parabolic Voronoi edges are always formed by one point and one segment
    /// from the initial input set.
    ///
    /// Args:
    ///   point: input point.
    ///   segment: input segment.
    ///   max_dist: maximum discretization distance.
    ///   discretization: point discretization of the given Voronoi edge.
    ///
    /// Template arguments:
    ///   InCT: coordinate type of the input geometries (usually integer).
    ///   Point: point type, should model point concept.
    ///   Segment: segment type, should model segment concept.
    ///
    /// Important:
    ///   discretization should contain both edge endpoints initially.
    pub fn discretize(
        point: &Coordinate<I1>,
        segment: &Line<I1>,
        max_dist: F1,
        discretization: &mut Vec<Coordinate<F1>>,
    ) {
        // Apply the linear transformation to move start point of the segment to
        // the point with coordinates (0, 0) and the direction of the segment to
        // coincide the positive direction of the x-axis.
        let segm_vec_x = Self::cast_io(segment.end.x) - Self::cast_io(segment.start.x);
        let segm_vec_y = Self::cast_io(segment.end.y) - Self::cast_io(segment.start.y);
        let sqr_segment_length = segm_vec_x * segm_vec_x + segm_vec_y * segm_vec_y;

        // Compute x-coordinates of the endpoints of the edge
        // in the transformed space.
        let projection_start =
            sqr_segment_length * Self::get_point_projection(&discretization[0], &segment);
        let projection_end =
            sqr_segment_length * Self::get_point_projection(&discretization[1], &segment);

        // Compute parabola parameters in the transformed space.
        // Parabola has next representation:
        // f(x) = ((x-rot_x)^2 + rot_y^2) / (2.0*rot_y).
        let point_vec_x = Self::cast_io(point.x) - Self::cast_io(segment.start.x);
        let point_vec_y = Self::cast_io(point.y) - Self::cast_io(segment.start.y);
        let rot_x = segm_vec_x * point_vec_x + segm_vec_y * point_vec_y;
        let rot_y = segm_vec_x * point_vec_y - segm_vec_y * point_vec_x;

        // Save the last point.
        let last_point = (*discretization)[1];
        let _ = discretization.pop();

        // Use stack to avoid recursion.
        let mut point_stack = Vec::<F1>::new();
        point_stack.push(projection_end);
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
            if dist <= max_dist_transformed {
                // Distance between parabola and line segment is less than max_dist.
                let _ = point_stack.pop();
                let inter_x = (segm_vec_x * new_x - segm_vec_y * new_y) / sqr_segment_length
                    + Self::cast_io(segment.start.x);
                let inter_y = (segm_vec_x * new_y + segm_vec_y * new_x) / sqr_segment_length
                    + Self::cast_io(segment.start.y);
                discretization.push(Coordinate {
                    x: inter_x,
                    y: inter_y,
                });
                cur_x = new_x;
                cur_y = new_y;
            } else {
                point_stack.push(mid_x);
            }
        }

        // Update last point.
        //discretization.back() = last_point;
        let discretization_len = discretization.len();
        discretization[discretization_len - 1] = last_point;
    }

    /// Compute y(x) = ((x - a) * (x - a) + b * b) / (2 * b).
    #[inline(always)]
    fn parabola_y(x: F1, a: F1, b: F1) -> F1 {
        #[allow(clippy::suspicious_operation_groupings)]
        {
            ((x - a) * (x - a) + b * b) / (b + b)
        }
    }

    // Get normalized length of the distance between:
    //   1) point projection onto the segment
    //   2) start point of the segment
    // Return this length divided by the segment length. This is made to avoid
    // sqrt computation during transformation from the initial space to the
    // transformed one and vice versa. The assumption is made that projection of
    // the point lies between the start-point and endpoint of the segment.
    pub fn get_point_projection(point: &Coordinate<F1>, segment: &Line<I1>) -> F1 {
        let segment_vec_x = Self::cast_io(segment.end.x) - Self::cast_io(segment.start.x);
        let segment_vec_y = Self::cast_io(segment.end.y) - Self::cast_io(segment.start.y);
        let point_vec_x = point.x - Self::cast_io(segment.start.x);
        let point_vec_y = point.y - Self::cast_io(segment.start.y);
        let sqr_segment_length = segment_vec_x * segment_vec_x + segment_vec_y * segment_vec_y;
        let vec_dot = segment_vec_x * point_vec_x + segment_vec_y * point_vec_y;
        vec_dot / sqr_segment_length
    }

    #[inline(always)]
    pub fn cast_io(value: I1) -> F1 {
        super::TypeConverter::<I1, F1, I2, F2>::i1_to_f1(value)
    }
}
