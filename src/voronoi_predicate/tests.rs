#![allow(unused_imports)]
use super::super::voronoi_beachline as VB;
use super::super::voronoi_diagram as VD;
use super::super::voronoi_diagram::VoronoiDiagram;
use super::super::voronoi_error::BVError;
use super::super::voronoi_predicate as VP;
use super::super::voronoi_siteevent as VSE;
use super::{BigFloatType, BigIntType, InputType, OutputType};
use geo::{Coordinate, Line};
use std::ops::Neg;

use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

#[allow(dead_code)]
fn new_key<I1, F1, I2, F2>(
    x1: I1,
    y1: I1,
    si1: usize,
    x2: I1,
    y2: I1,
    si2: usize,
) -> VB::BeachLineNodeKey<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: InputType + Neg<Output = I2>,
    F2: OutputType + Neg<Output = F2>,
{
    let mut site1 = VSE::SiteEvent::<I1, F1, I2, F2>::new_2(Coordinate { x: x1, y: y1 }, si1);
    site1.sorted_index_ = si1;
    let mut site2 = VSE::SiteEvent::<I1, F1, I2, F2>::new_2(Coordinate { x: x2, y: y2 }, si2);
    site2.sorted_index_ = si2;
    VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site1, site2)
}

#[allow(clippy::too_many_arguments)]
#[allow(dead_code)] // this function is used 44 times in this file, still rustc don't think so
fn node_test<I1, F1, I2, F2>(
    a_key: &VB::BeachLineNodeKey<I1, F1, I2, F2>,
    x1: I1,
    y1: I1,
    si1: usize,
    x2: I1,
    y2: I1,
    si2: usize,
    expect: bool,
) -> bool
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: InputType + Neg<Output = I2>,
    F2: OutputType + Neg<Output = F2>,
{
    let test_node = new_key::<I1, F1, I2, F2>(x1, y1, si1, x2, y2, si2);

    let is_less =
        VP::NodeComparisonPredicate::<I1, F1, I2, F2>::node_comparison_predicate(a_key, &test_node);
    dbg!(&a_key, &test_node, is_less, expect);
    expect == is_less
}

#[test]
fn cross_product_1() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    let a1 = -9;
    let a2 = 5;
    let b1 = -9;
    let b2 = 4;

    let x = VP::VoronoiPredicates::<I1, F1, I2, F2>::robust_cross_product_2i(a1, b1, a2, b2);
    assert_eq!(x, 9.0);
}

#[test]
fn node_1() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    // test data copy & pasted from c++ debug session
    let mykey = new_key::<I1, F1, I2, F2>(4, 13, 2, 4, 13, 2);
    assert!(node_test(&mykey, 1, 15, 0, 2, 14, 1, false));
    assert!(node_test(&mykey, 2, 14, 1, 1, 15, 0, true));
}

#[test]
fn node_2() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    // test data copy & pasted from c++ debug session
    let mykey = new_key::<I1, F1, I2, F2>(9, 17, 5, 9, 17, 5);
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 1, 15, 0, 2, 14, 1, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 1, 15, 0, 2, 14, 1, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 2, 14, 1, 4, 13, 2, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 13, 2, 8, 9, 4, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 8, 9, 4, 4, 13, 4, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 13, 2, 4, 16, 2, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 16, 3, 1, 15, 3, true
    ));
}

#[test]
fn node_3() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    // test data copy & pasted from c++ debug session
    let mykey = new_key::<I1, F1, I2, F2>(4, 13, 2, 4, 13, 2);
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 1, 15, 0, 2, 14, 1, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 2, 14, 1, 1, 15, 0, true
    ));

    let mykey = new_key::<I1, F1, I2, F2>(4, 16, 3, 4, 16, 3);
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 1, 15, 0, 2, 14, 1, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 2, 14, 1, 4, 13, 2, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 13, 2, 2, 14, 1, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 2, 14, 1, 1, 15, 0, false
    ));

    let mykey = new_key::<I1, F1, I2, F2>(8, 9, 4, 8, 9, 4);
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 1, 15, 0, 2, 14, 1, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 2, 14, 1, 4, 13, 2, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 13, 2, 4, 16, 3, true
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 16, 3, 1, 15, 0, true
    ));

    let mykey = new_key::<I1, F1, I2, F2>(9, 17, 5, 9, 17, 5);
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 1, 15, 0, 2, 14, 1, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 2, 14, 1, 4, 13, 2, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 13, 2, 8, 9, 4, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 8, 9, 4, 4, 13, 2, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 13, 2, 4, 16, 3, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 16, 3, 1, 15, 0, true
    ));

    let mykey = new_key::<I1, F1, I2, F2>(10, 10, 6, 10, 10, 6);
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 1, 15, 0, 2, 14, 1, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 2, 14, 1, 8, 9, 4, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(&mykey, 8, 9, 4, 4, 13, 2, true));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 13, 2, 4, 16, 3, true
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 16, 3, 9, 17, 5, true
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 9, 17, 5, 4, 16, 3, true
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 16, 3, 1, 15, 0, true
    ));

    let mykey = new_key::<I1, F1, I2, F2>(12, 11, 7, 12, 11, 7);
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 1, 15, 0, 2, 14, 1, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 2, 14, 1, 8, 9, 4, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 8, 9, 4, 10, 10, 6, false
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 10, 10, 6, 9, 17, 5, true
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 9, 17, 5, 4, 16, 3, true
    ));
    assert!(node_test::<I1, F1, I2, F2>(
        &mykey, 4, 16, 3, 1, 15, 0, true
    ));
}

#[test]
fn node_4() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    // test data copy & pasted from c++ debug session
    let mykey = new_key::<I1, F1, I2, F2>(10, 18, 2, 10, 18, 2);
    let _ = node_test::<I1, F1, I2, F2>(&mykey, 4, 21, 0, 8, 62, 1, true);
    let _ = node_test::<I1, F1, I2, F2>(&mykey, 8, 62, 1, 4, 21, 0, true);

    let mykey = new_key::<I1, F1, I2, F2>(12, 3, 3, 12, 3, 3);
    let _ = node_test::<I1, F1, I2, F2>(&mykey, 4, 21, 0, 10, 18, 2, true);
    let _ = node_test::<I1, F1, I2, F2>(&mykey, 10, 18, 2, 4, 21, 0, true);
    let _ = node_test::<I1, F1, I2, F2>(&mykey, 4, 21, 0, 8, 62, 1, true);
    let _ = node_test::<I1, F1, I2, F2>(&mykey, 8, 62, 1, 4, 21, 0, true);
}

#[test]
fn distance_predicate_pp() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;
    let pp = super::DistancePredicate::<I1, F1, I2, F2>::pp;
    let new_site = VSE::SiteEvent::<I1, F1, I2, F2>::new_7;
    let new_point = |x, y| Coordinate::<I1> { x, y };

    // test data copy & pasted from c++ debug session
    assert_eq!(
        pp(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(2, 2, 2, 2, 2, 2, 1),
            &new_point(3, 1)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        pp(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_point(5, 4)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        pp(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_point(5, 6)
        ),
        true
    ); //distance_predicate
}

#[test]
fn distance_predicate_ps_32() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;
    let ps = super::DistancePredicate::<I1, F1, I2, F2>::ps;
    let new_site = VSE::SiteEvent::<I1, F1, I2, F2>::new_7;
    let new_point = |x, y| Coordinate::<I1> { x, y };

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
    assert_eq!(
        ps(
            &new_site(200, 400, 200, 400, 0, 2, 2),
            &new_site(400, 400, 200, 400, 1, 4, 40),
            &new_point(400, 400),
            true
        ),
        false
    ); //distance_predicate
}

#[test]
fn distance_predicate_ps_64() {
    type I1 = i32;
    type F1 = f64;
    type I2 = i64;
    type F2 = f64;
    let ps = super::DistancePredicate::<I1, F1, I2, F2>::ps;
    let new_site = VSE::SiteEvent::<I1, F1, I2, F2>::new_7;
    let new_point = |x, y| Coordinate::<I1> { x, y };

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
    assert_eq!(
        ps(
            &new_site(200, 400, 200, 400, 0, 2, 2),
            &new_site(400, 400, 200, 400, 1, 4, 40),
            &new_point(400, 400),
            true
        ),
        false
    ); //distance_predicate
}

#[test]
fn distance_predicate_ss_32() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;
    let ss = super::DistancePredicate::<I1, F1, I2, F2>::ss;
    let new_site = VSE::SiteEvent::<I1, F1, I2, F2>::new_7;
    let new_point = |x, y| Coordinate::<I1> { x, y };

    // test data copy & pasted from c++ debug session
    assert_eq!(
        ss(
            &new_site(1, 2, 3, 4, 1, 1, 8),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            &new_point(2, 2)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(2, 2, 5, 4, 2, 3, 8),
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_point(4, 3)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_point(4, 3)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_point(5, 4)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            &new_point(3, 1)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            &new_point(3, 4)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            &new_point(5, 4)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            &new_point(5, 6)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(367, 107, 529, 242, 4, 6, 9),
            &new_site(529, 242, 367, 107, 4, 6, 41),
            &new_point(400, 200)
        ),
        true
    ); //distance_predicate
}
#[test]
fn distance_predicate_ss_64() {
    type I1 = i32;
    type F1 = f64;
    type I2 = i64;
    type F2 = f64;
    let ss = super::DistancePredicate::<I1, F1, I2, F2>::ss;
    let new_site = VSE::SiteEvent::<I1, F1, I2, F2>::new_7;
    let new_point = |x, y| Coordinate::<I1> { x, y };

    // test data copy & pasted from c++ debug session
    assert_eq!(
        ss(
            &new_site(1, 2, 3, 4, 1, 1, 8),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            &new_point(2, 2)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(2, 2, 5, 4, 2, 3, 8),
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_point(4, 3)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_point(4, 3)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_point(5, 4)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            &new_point(3, 1)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            &new_point(3, 4)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            &new_point(5, 4)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            &new_point(5, 6)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(367, 107, 529, 242, 4, 6, 9),
            &new_site(529, 242, 367, 107, 4, 6, 41),
            &new_point(400, 200)
        ),
        true
    ); //distance_predicate
}
