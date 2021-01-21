use boostvoronoi::voronoi_builder as VB;
use boostvoronoi::voronoi_diagram as VD;
use boostvoronoi::voronoi_diagram::VoronoiEdgeIndex;
use boostvoronoi::voronoi_error::BVError;
use boostvoronoi::voronoi_predicate as VP;
use boostvoronoi::voronoi_siteevent as VSE;
use boostvoronoi::voronoi_structures as VS;
use boostvoronoi::voronoi_structures::{Point2dI, Segment2d};
use boostvoronoi::voronoi_visual_utils as VV;
use boostvoronoi::TypeConverter;
use boostvoronoi::{BigFloatType, BigIntType, BoostInputType, BoostOutputType};
use num::NumCast;
use ordered_float::OrderedFloat;
use std::marker::PhantomData;

use boostvoronoi::voronoi_builder::VoronoiBuilder;
use boostvoronoi::voronoi_diagram::VoronoiDiagram;

fn main() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;
    let ps = VP::DistancePredicate::<I1, F1, I2, F2>::ps;
    let new_site = VSE::SiteEvent::<I1, F1, I2, F2>::new_7;
    let new_point = VS::Point2dI::<I1>::new;
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_point(5, 4),
            true
        ),
        true
    ); //distance_predicate

    // test data copy & pasted from c++ debug session
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            &new_point(2, 2),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            &new_point(3, 4),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            &new_point(5, 4),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            &new_point(5, 6),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(2, 2, 2, 2, 2, 2, 1),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            &new_point(3, 1),
            false
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_point(4, 3),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_point(5, 4),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_point(5, 6),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            &new_point(5, 4),
            false
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            &new_point(5, 6),
            false
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_point(5, 4),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_point(5, 6),
            true
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_point(5, 4),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_point(5, 4),
            true
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_point(5, 6),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_point(5, 6),
            true
        ),
        true
    ); //distance_predicate
    println!("completed test");
}
