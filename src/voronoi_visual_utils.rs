// Boost.Polygon library voronoi_graphic_utils.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::voronoi_diagram as VD;
use super::voronoi_structures as VS;
use super::{BigFloatType, BigIntType, BoostInputType, BoostOutputType};

use geo::{Line, Point};
use std::marker::PhantomData;
use std::ops::Neg;

/// Utilities class, that contains set of routines handful for visualization.
pub struct VoronoiVisualUtils<I, O, BI, BF>
where
    I: BoostInputType + Neg<Output = I>,
    O: BoostOutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
    _pdi: PhantomData<I>,
    _pdo: PhantomData<O>,
    _pdbi: PhantomData<BI>,
    _pdbf: PhantomData<BF>,
}

impl<I, O, BI, BF> VoronoiVisualUtils<I, O, BI, BF>
where
    I: BoostInputType + Neg<Output = I>,
    O: BoostOutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
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
        point: &Point<I>,
        segment: &Line<I>,
        max_dist: O,
        discretization: &mut Vec<Point<O>>,
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
        let point_vec_x = Self::cast_io(point.x()) - Self::cast_io(segment.start.x);
        let point_vec_y = Self::cast_io(point.y()) - Self::cast_io(segment.start.y);
        let rot_x = segm_vec_x * point_vec_x + segm_vec_y * point_vec_y;
        let rot_y = segm_vec_x * point_vec_y - segm_vec_y * point_vec_x;

        // Save the last point.
        let last_point = (*discretization)[1];
        discretization.pop();

        // Use stack to avoid recursion.
        let mut point_stack = Vec::<O>::new();
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
                point_stack.pop();
                let inter_x = (segm_vec_x * new_x - segm_vec_y * new_y) / sqr_segment_length
                    + Self::cast_io(segment.start.x);
                let inter_y = (segm_vec_x * new_y + segm_vec_y * new_x) / sqr_segment_length
                    + Self::cast_io(segment.start.y);
                discretization.push(Point::<O>::new(inter_x, inter_y));
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
    fn parabola_y(x: O, a: O, b: O) -> O {
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
    pub fn get_point_projection(point: &Point<O>, segment: &Line<I>) -> O {
        let segment_vec_x = Self::cast_io(segment.end.x) - Self::cast_io(segment.start.x);
        let segment_vec_y = Self::cast_io(segment.end.y) - Self::cast_io(segment.start.y);
        let point_vec_x = point.x() - Self::cast_io(segment.start.x);
        let point_vec_y = point.y() - Self::cast_io(segment.start.y);
        let sqr_segment_length = segment_vec_x * segment_vec_x + segment_vec_y * segment_vec_y;
        let vec_dot = segment_vec_x * point_vec_x + segment_vec_y * point_vec_y;
        vec_dot / sqr_segment_length
    }

    #[inline(always)]
    pub fn cast_io(value: I) -> O {
        super::TypeConverter::<I, O, BI, BF>::i1_to_f1(value)
    }
}
