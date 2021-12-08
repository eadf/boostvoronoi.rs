use super::super::beach_line as VB;
use super::super::geometry::Point;
use super::super::predicate as VP;
use super::super::site_event as VSE;
use super::{InputType, OutputType};

fn new_key<I: InputType, F: OutputType>(
    x1: I,
    y1: I,
    si1: usize,
    x2: I,
    y2: I,
    si2: usize,
) -> VB::BeachLineNodeKey<I, F> {
    let mut site1 = VSE::SiteEvent::<I, F>::new(VSE::Site::Point(Point { x: x1, y: y1 }), si1);
    site1.set_sorted_index(si1);
    let mut site2 = VSE::SiteEvent::<I, F>::new(VSE::Site::Point(Point { x: x2, y: y2 }), si2);
    site2.set_sorted_index(si2);
    VB::BeachLineNodeKey::<I, F>::new_2(site1, site2)
}

#[allow(clippy::too_many_arguments)]
fn node_test<I: InputType, F: OutputType>(
    a_key: &VB::BeachLineNodeKey<I, F>,
    x1: I,
    y1: I,
    si1: usize,
    x2: I,
    y2: I,
    si2: usize,
    expect: bool,
) -> bool {
    let test_node = new_key::<I, F>(x1, y1, si1, x2, y2, si2);

    let is_less = VP::node_comparison_predicate::node_comparison::<I, F>(a_key, &test_node);
    dbg!(&a_key, &test_node, is_less, expect);
    expect == is_less
}

#[test]
fn cross_product_1() {
    type I = i32;
    type F = f32;

    let a1 = -9;
    let a2 = 5;
    let b1 = -9;
    let b2 = 4;

    let x = VP::robust_cross_product::<I, F>(a1, b1, a2, b2);
    assert_eq!(x, 9.0);
}

#[test]
fn node_1() {
    type I = i32;
    type F = f32;

    // test data copy & pasted from c++ debug session
    let mykey = new_key::<I, F>(4, 13, 2, 4, 13, 2);
    assert!(node_test(&mykey, 1, 15, 0, 2, 14, 1, false));
    assert!(node_test(&mykey, 2, 14, 1, 1, 15, 0, true));
}

#[test]
fn node_2() {
    type I = i32;
    type F = f32;

    // test data copy & pasted from c++ debug session
    let mykey = new_key::<I, F>(9, 17, 5, 9, 17, 5);
    assert!(node_test::<I, F>(&mykey, 1, 15, 0, 2, 14, 1, false));
    assert!(node_test::<I, F>(&mykey, 1, 15, 0, 2, 14, 1, false));
    assert!(node_test::<I, F>(&mykey, 2, 14, 1, 4, 13, 2, false));
    assert!(node_test::<I, F>(&mykey, 4, 13, 2, 8, 9, 4, false));
    assert!(node_test::<I, F>(&mykey, 8, 9, 4, 4, 13, 4, false));
    assert!(node_test::<I, F>(&mykey, 4, 13, 2, 4, 16, 2, false));
    assert!(node_test::<I, F>(&mykey, 4, 16, 3, 1, 15, 3, true));
}

#[test]
fn node_3() {
    type I = i32;
    type F = f32;

    // test data copy & pasted from c++ debug session
    let mykey = new_key::<I, F>(4, 13, 2, 4, 13, 2);
    assert!(node_test::<I, F>(&mykey, 1, 15, 0, 2, 14, 1, false));
    assert!(node_test::<I, F>(&mykey, 2, 14, 1, 1, 15, 0, true));

    let mykey = new_key::<I, F>(4, 16, 3, 4, 16, 3);
    assert!(node_test::<I, F>(&mykey, 1, 15, 0, 2, 14, 1, false));
    assert!(node_test::<I, F>(&mykey, 2, 14, 1, 4, 13, 2, false));
    assert!(node_test::<I, F>(&mykey, 4, 13, 2, 2, 14, 1, false));
    assert!(node_test::<I, F>(&mykey, 2, 14, 1, 1, 15, 0, false));

    let mykey = new_key::<I, F>(8, 9, 4, 8, 9, 4);
    assert!(node_test::<I, F>(&mykey, 1, 15, 0, 2, 14, 1, false));
    assert!(node_test::<I, F>(&mykey, 2, 14, 1, 4, 13, 2, false));
    assert!(node_test::<I, F>(&mykey, 4, 13, 2, 4, 16, 3, true));
    assert!(node_test::<I, F>(&mykey, 4, 16, 3, 1, 15, 0, true));

    let mykey = new_key::<I, F>(9, 17, 5, 9, 17, 5);
    assert!(node_test::<I, F>(&mykey, 1, 15, 0, 2, 14, 1, false));
    assert!(node_test::<I, F>(&mykey, 2, 14, 1, 4, 13, 2, false));
    assert!(node_test::<I, F>(&mykey, 4, 13, 2, 8, 9, 4, false));
    assert!(node_test::<I, F>(&mykey, 8, 9, 4, 4, 13, 2, false));
    assert!(node_test::<I, F>(&mykey, 4, 13, 2, 4, 16, 3, false));
    assert!(node_test::<I, F>(&mykey, 4, 16, 3, 1, 15, 0, true));

    let mykey = new_key::<I, F>(10, 10, 6, 10, 10, 6);
    assert!(node_test::<I, F>(&mykey, 1, 15, 0, 2, 14, 1, false));
    assert!(node_test::<I, F>(&mykey, 2, 14, 1, 8, 9, 4, false));
    assert!(node_test::<I, F>(&mykey, 8, 9, 4, 4, 13, 2, true));
    assert!(node_test::<I, F>(&mykey, 4, 13, 2, 4, 16, 3, true));
    assert!(node_test::<I, F>(&mykey, 4, 16, 3, 9, 17, 5, true));
    assert!(node_test::<I, F>(&mykey, 9, 17, 5, 4, 16, 3, true));
    assert!(node_test::<I, F>(&mykey, 4, 16, 3, 1, 15, 0, true));

    let mykey = new_key::<I, F>(12, 11, 7, 12, 11, 7);
    assert!(node_test::<I, F>(&mykey, 1, 15, 0, 2, 14, 1, false));
    assert!(node_test::<I, F>(&mykey, 2, 14, 1, 8, 9, 4, false));
    assert!(node_test::<I, F>(&mykey, 8, 9, 4, 10, 10, 6, false));
    assert!(node_test::<I, F>(&mykey, 10, 10, 6, 9, 17, 5, true));
    assert!(node_test::<I, F>(&mykey, 9, 17, 5, 4, 16, 3, true));
    assert!(node_test::<I, F>(&mykey, 4, 16, 3, 1, 15, 0, true));
}

#[test]
fn node_4() {
    type I = i32;
    type F = f32;

    // test data copy & pasted from c++ debug session
    let mykey = new_key::<I, F>(10, 18, 2, 10, 18, 2);
    let _ = node_test::<I, F>(&mykey, 4, 21, 0, 8, 62, 1, true);
    let _ = node_test::<I, F>(&mykey, 8, 62, 1, 4, 21, 0, true);

    let mykey = new_key::<I, F>(12, 3, 3, 12, 3, 3);
    let _ = node_test::<I, F>(&mykey, 4, 21, 0, 10, 18, 2, true);
    let _ = node_test::<I, F>(&mykey, 10, 18, 2, 4, 21, 0, true);
    let _ = node_test::<I, F>(&mykey, 4, 21, 0, 8, 62, 1, true);
    let _ = node_test::<I, F>(&mykey, 8, 62, 1, 4, 21, 0, true);
}

#[test]
fn distance_predicate_pp() {
    type I = i32;
    type F = f32;
    let pp = super::distance_predicate::pp::<I, F>;
    let new_site = VSE::SiteEvent::<I, F>::new_7;
    let new_point = |x, y| Point::<I> { x, y };

    // test data copy & pasted from c++ debug session
    assert_eq!(
        pp(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(2, 2, 2, 2, 2, 2, 1),
            new_point(3, 1)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        pp(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 1, 3, 1, 3, 4, 2),
            new_point(5, 4)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        pp(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 1, 3, 1, 3, 4, 2),
            new_point(5, 6)
        ),
        true
    ); //distance_predicate
}

#[test]
fn distance_predicate_ps_32() {
    type I = i32;
    type F = f32;
    let ps = super::distance_predicate::ps::<I, F>;
    let new_site = VSE::SiteEvent::<I, F>::new_7;
    let new_point = |x, y| Point::<I> { x, y };

    // test data copy & pasted from c++ debug session
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            new_point(2, 2),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(3, 4),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(5, 4),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(5, 6),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(2, 2, 2, 2, 2, 2, 1),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            new_point(3, 1),
            false
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(4, 3),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 4),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 6),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(5, 4),
            false
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(5, 6),
            false
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(5, 4, 2, 2, 2, 3, 40),
            new_point(5, 4),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(5, 4, 2, 2, 2, 3, 40),
            new_point(5, 6),
            true
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 4),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 4),
            true
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 6),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 6),
            true
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(200, 400, 200, 400, 0, 2, 2),
            &new_site(400, 400, 200, 400, 1, 4, 40),
            new_point(400, 400),
            true
        ),
        false
    ); //distance_predicate
}

#[test]
fn distance_predicate_ps_64() {
    type I = i32;
    type F = f64;
    let ps = super::distance_predicate::ps::<I, F>;
    let new_site = VSE::SiteEvent::<I, F>::new_7;
    let new_point = Point::<I>::new;

    // test data copy & pasted from c++ debug session
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            new_point(2, 2),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(3, 4),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(5, 4),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(1, 2, 1, 2, 1, 0, 1),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(5, 6),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(2, 2, 2, 2, 2, 2, 1),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            new_point(3, 1),
            false
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(4, 3),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 4),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 1, 3, 1, 3, 4, 2),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 6),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(5, 4),
            false
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(5, 6),
            false
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(5, 4, 2, 2, 2, 3, 40),
            new_point(5, 4),
            true
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(3, 4, 3, 4, 1, 5, 2),
            &new_site(5, 4, 2, 2, 2, 3, 40),
            new_point(5, 6),
            true
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 4),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 4),
            true
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 6),
            false
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(4, 3, 4, 3, 0, 7, 0),
            &new_site(3, 1, 5, 6, 3, 6, 9),
            new_point(5, 6),
            true
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ps(
            &new_site(200, 400, 200, 400, 0, 2, 2),
            &new_site(400, 400, 200, 400, 1, 4, 40),
            new_point(400, 400),
            true
        ),
        false
    ); //distance_predicate
}

#[test]
fn distance_predicate_ss_32() {
    type I = i32;
    type F = f32;
    let ss = super::distance_predicate::ss::<I, F>;
    let new_site = VSE::SiteEvent::<I, F>::new_7;
    let new_point = |x, y| Point::<I> { x, y };

    // test data copy & pasted from c++ debug session
    assert_eq!(
        ss(
            &new_site(1, 2, 3, 4, 1, 1, 8),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(2, 2)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(2, 2, 5, 4, 2, 3, 8),
            &new_site(5, 4, 2, 2, 2, 3, 40),
            new_point(4, 3)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_site(5, 6, 3, 1, 3, 6, 41),
            new_point(4, 3)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_site(5, 6, 3, 1, 3, 6, 41),
            new_point(5, 4)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            new_point(3, 1)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            new_point(3, 4)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            new_point(5, 4)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            new_point(5, 6)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(367, 107, 529, 242, 4, 6, 9),
            &new_site(529, 242, 367, 107, 4, 6, 41),
            new_point(400, 200)
        ),
        true
    ); //distance_predicate
}

#[test]
fn distance_predicate_ss_64() {
    type I = i32;
    type F = f64;
    let ss = super::distance_predicate::ss::<I, F>;
    let new_site = VSE::SiteEvent::<I, F>::new_7;
    let new_point = |x, y| Point::<I> { x, y };

    // test data copy & pasted from c++ debug session
    assert_eq!(
        ss(
            &new_site(1, 2, 3, 4, 1, 1, 8),
            &new_site(3, 4, 1, 2, 1, 1, 40),
            new_point(2, 2)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(2, 2, 5, 4, 2, 3, 8),
            &new_site(5, 4, 2, 2, 2, 3, 40),
            new_point(4, 3)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_site(5, 6, 3, 1, 3, 6, 41),
            new_point(4, 3)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(3, 1, 5, 6, 3, 6, 9),
            &new_site(5, 6, 3, 1, 3, 6, 41),
            new_point(5, 4)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            new_point(3, 1)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 4, 2, 2, 2, 3, 40),
            &new_site(1, 2, 3, 4, 1, 1, 8),
            new_point(3, 4)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            new_point(5, 4)
        ),
        false
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(5, 6, 3, 1, 3, 6, 41),
            &new_site(2, 2, 5, 4, 2, 3, 8),
            new_point(5, 6)
        ),
        true
    ); //distance_predicate
    assert_eq!(
        ss(
            &new_site(367, 107, 529, 242, 4, 6, 9),
            &new_site(529, 242, 367, 107, 4, 6, 41),
            new_point(400, 200)
        ),
        true
    ); //distance_predicate
}

#[test]
fn event_comparison_ii() {
    use std::cmp::Ordering;

    #[rustfmt::skip]
    let d = [
        // 003
        ((631,128,631,128,0,0,2),(759,-242,759,-242,0,0,2),Ordering::Less),
        ((631,128,759,-242,0,0,9),(631,128,631,128,0,0,9),Ordering::Greater),
        ((631,128,759,-242,0,0,9),(759,-242,759,-242,0,0,9),Ordering::Less),
        ((189,-303,189,-303,1,0,1),(759,-242,759,-242,0,0,1),Ordering::Less),
        ((189,-303,189,-303,1,0,1),(631,128,759,-242,0,0,1),Ordering::Less),
        ((189,-303,189,-303,1,0,1),(631,128,631,128,0,0,1),Ordering::Less),
        ((843,693,843,693,1,0,2),(759,-242,759,-242,0,0,2),Ordering::Greater),
        ((189,-303,843,693,1,0,8),(843,693,843,693,1,0,8),Ordering::Less),
        ((189,-303,843,693,1,0,8),(759,-242,759,-242,0,0,8),Ordering::Less),
        ((189,-303,843,693,1,0,8),(631,128,759,-242,0,0,8),Ordering::Less),
        ((189,-303,843,693,1,0,8),(631,128,631,128,0,0,8),Ordering::Less),
        ((189,-303,843,693,1,0,8),(189,-303,189,-303,1,0,8),Ordering::Greater),
        ((-911,-920,-911,-920,2,0,1),(843,693,843,693,1,0,1),Ordering::Less),
        ((-911,-920,-911,-920,2,0,1),(759,-242,759,-242,0,0,1),Ordering::Less),
        ((-911,-920,-911,-920,2,0,1),(631,128,759,-242,0,0,1),Ordering::Less),
        ((-911,-920,-911,-920,2,0,1),(631,128,631,128,0,0,1),Ordering::Less),
        ((-911,-920,-911,-920,2,0,1),(189,-303,843,693,1,0,1),Ordering::Less),
        ((-911,-920,-911,-920,2,0,1),(189,-303,189,-303,1,0,1),Ordering::Less),
        ((921,853,921,853,2,0,2),(843,693,843,693,1,0,2),Ordering::Greater),
        ((-911,-920,921,853,2,0,8),(921,853,921,853,2,0,8),Ordering::Less),
        ((-911,-920,921,853,2,0,8),(843,693,843,693,1,0,8),Ordering::Less),
        ((-911,-920,921,853,2,0,8),(759,-242,759,-242,0,0,8),Ordering::Less),
        ((-911,-920,921,853,2,0,8),(631,128,759,-242,0,0,8),Ordering::Less),
        ((-911,-920,921,853,2,0,8),(631,128,631,128,0,0,8),Ordering::Less),
        ((-911,-920,921,853,2,0,8),(189,-303,843,693,1,0,8),Ordering::Less),
        ((-911,-920,921,853,2,0,8),(189,-303,189,-303,1,0,8),Ordering::Less),
        ((-911,-920,921,853,2,0,8),(-911,-920,-911,-920,2,0,8),Ordering::Greater),
        // 04
        ((552,-566,552,-566,0,0,2),(580,-833,580,-833,0,0,2),Ordering::Less),
        ((552,-566,580,-833,0,0,9),(552,-566,552,-566,0,0,9),Ordering::Greater),
        ((552,-566,580,-833,0,0,9),(580,-833,580,-833,0,0,9),Ordering::Less),
        ((-671,955,-671,955,1,0,1),(580,-833,580,-833,0,0,1),Ordering::Less),
        ((-671,955,-671,955,1,0,1),(552,-566,580,-833,0,0,1),Ordering::Less),
        ((-671,955,-671,955,1,0,1),(552,-566,552,-566,0,0,1),Ordering::Less),
        ((604,-936,604,-936,1,0,2),(580,-833,580,-833,0,0,2),Ordering::Greater),
        ((-671,955,604,-936,1,0,8),(604,-936,604,-936,1,0,8),Ordering::Less),
        ((-671,955,604,-936,1,0,8),(580,-833,580,-833,0,0,8),Ordering::Less),
        ((-671,955,604,-936,1,0,8),(552,-566,580,-833,0,0,8),Ordering::Less),
        ((-671,955,604,-936,1,0,8),(552,-566,552,-566,0,0,8),Ordering::Less),
        ((-671,955,604,-936,1,0,8),(-671,955,-671,955,1,0,8),Ordering::Greater),
        ((535,-110,535,-110,2,0,1),(604,-936,604,-936,1,0,1),Ordering::Less),
        ((535,-110,535,-110,2,0,1),(580,-833,580,-833,0,0,1),Ordering::Less),
        ((535,-110,535,-110,2,0,1),(552,-566,580,-833,0,0,1),Ordering::Less),
        ((535,-110,535,-110,2,0,1),(552,-566,552,-566,0,0,1),Ordering::Less),
        ((535,-110,535,-110,2,0,1),(-671,955,604,-936,1,0,1),Ordering::Greater),
        ((412,-549,412,-549,2,0,2),(604,-936,604,-936,1,0,2),Ordering::Less),
        ((412,-549,412,-549,2,0,2),(580,-833,580,-833,0,0,2),Ordering::Less),
        ((412,-549,412,-549,2,0,2),(552,-566,580,-833,0,0,2),Ordering::Less),
        ((412,-549,412,-549,2,0,2),(552,-566,552,-566,0,0,2),Ordering::Less),
        ((412,-549,412,-549,2,0,2),(535,-110,535,-110,2,0,2),Ordering::Less),
        ((412,-549,412,-549,2,0,2),(-671,955,604,-936,1,0,2),Ordering::Greater),
        ((412,-549,535,-110,2,0,9),(604,-936,604,-936,1,0,9),Ordering::Less),
        ((412,-549,535,-110,2,0,9),(580,-833,580,-833,0,0,9),Ordering::Less),
        ((412,-549,535,-110,2,0,9),(552,-566,580,-833,0,0,9),Ordering::Less),
        ((412,-549,535,-110,2,0,9),(552,-566,552,-566,0,0,9),Ordering::Less),
        ((412,-549,535,-110,2,0,9),(535,-110,535,-110,2,0,9),Ordering::Less),
        ((412,-549,535,-110,2,0,9),(412,-549,412,-549,2,0,9),Ordering::Greater),
        // 06
        ((955,703,955,703,0,0,2),(415,-54,415,-54,0,0,2),Ordering::Greater),
        ((415,-54,955,703,0,0,8),(955,703,955,703,0,0,8),Ordering::Less),
        ((415,-54,955,703,0,0,8),(415,-54,415,-54,0,0,8),Ordering::Greater),
        ((976,38,976,38,1,0,1),(955,703,955,703,0,0,1),Ordering::Greater),
        ((-916,-467,-916,-467,1,0,2),(976,38,976,38,1,0,2),Ordering::Less),
        ((-916,-467,-916,-467,1,0,2),(955,703,955,703,0,0,2),Ordering::Less),
        ((-916,-467,-916,-467,1,0,2),(415,-54,955,703,0,0,2),Ordering::Less),
        ((-916,-467,-916,-467,1,0,2),(415,-54,415,-54,0,0,2),Ordering::Less),
        ((-916,-467,976,38,1,0,9),(976,38,976,38,1,0,9),Ordering::Less),
        ((-916,-467,976,38,1,0,9),(955,703,955,703,0,0,9),Ordering::Less),
        ((-916,-467,976,38,1,0,9),(415,-54,955,703,0,0,9),Ordering::Less),
        ((-916,-467,976,38,1,0,9),(415,-54,415,-54,0,0,9),Ordering::Less),
        ((-916,-467,976,38,1,0,9),(-916,-467,-916,-467,1,0,9),Ordering::Greater),
        ((909,424,909,424,2,0,1),(976,38,976,38,1,0,1),Ordering::Less),
        ((909,424,909,424,2,0,1),(955,703,955,703,0,0,1),Ordering::Less),
        ((909,424,909,424,2,0,1),(415,-54,955,703,0,0,1),Ordering::Greater),
        ((962,401,962,401,2,0,2),(976,38,976,38,1,0,2),Ordering::Less),
        ((962,401,962,401,2,0,2),(955,703,955,703,0,0,2),Ordering::Greater),
        ((909,424,962,401,2,0,8),(976,38,976,38,1,0,8),Ordering::Less),
        ((909,424,962,401,2,0,8),(962,401,962,401,2,0,8),Ordering::Less),
        ((909,424,962,401,2,0,8),(955,703,955,703,0,0,8),Ordering::Less),
        ((909,424,962,401,2,0,8),(909,424,909,424,2,0,8),Ordering::Greater),
        // 07
        ((741,366,741,366,0,0,2),(365,113,365,113,0,0,2),Ordering::Greater),
        ((365,113,741,366,0,0,8),(741,366,741,366,0,0,8),Ordering::Less),
        ((365,113,741,366,0,0,8),(365,113,365,113,0,0,8),Ordering::Greater),
        ((768,-67,768,-67,1,0,1),(741,366,741,366,0,0,1),Ordering::Greater),
        ((601,187,601,187,1,0,2),(768,-67,768,-67,1,0,2),Ordering::Less),
        ((601,187,601,187,1,0,2),(741,366,741,366,0,0,2),Ordering::Less),
        ((601,187,601,187,1,0,2),(365,113,741,366,0,0,2),Ordering::Greater),
        ((601,187,768,-67,1,0,9),(768,-67,768,-67,1,0,9),Ordering::Less),
        ((601,187,768,-67,1,0,9),(741,366,741,366,0,0,9),Ordering::Less),
        ((601,187,768,-67,1,0,9),(601,187,601,187,1,0,9),Ordering::Greater),
        ((-814,662,-814,662,2,0,1),(768,-67,768,-67,1,0,1),Ordering::Less),
        ((-814,662,-814,662,2,0,1),(741,366,741,366,0,0,1),Ordering::Less),
        ((-814,662,-814,662,2,0,1),(601,187,768,-67,1,0,1),Ordering::Less),
        ((-814,662,-814,662,2,0,1),(601,187,601,187,1,0,1),Ordering::Less),
        ((-814,662,-814,662,2,0,1),(365,113,741,366,0,0,1),Ordering::Less),
        ((-814,662,-814,662,2,0,1),(365,113,365,113,0,0,1),Ordering::Less),
        ((817,-285,817,-285,2,0,2),(768,-67,768,-67,1,0,2),Ordering::Greater),
        ((-814,662,817,-285,2,0,8),(817,-285,817,-285,2,0,8),Ordering::Less),
        ((-814,662,817,-285,2,0,8),(768,-67,768,-67,1,0,8),Ordering::Less),
        ((-814,662,817,-285,2,0,8),(741,366,741,366,0,0,8),Ordering::Less),
        ((-814,662,817,-285,2,0,8),(601,187,768,-67,1,0,8),Ordering::Less),
        ((-814,662,817,-285,2,0,8),(601,187,601,187,1,0,8),Ordering::Less),
        ((-814,662,817,-285,2,0,8),(365,113,741,366,0,0,8),Ordering::Less),
        ((-814,662,817,-285,2,0,8),(365,113,365,113,0,0,8),Ordering::Less),
        ((-814,662,817,-285,2,0,8),(-814,662,-814,662,2,0,8),Ordering::Greater),
        // 009
        ((342,-158,342,-158,0,0,2),(519,-635,519,-635,0,0,2),Ordering::Less),
        ((342,-158,519,-635,0,0,9),(342,-158,342,-158,0,0,9),Ordering::Greater),
        ((342,-158,519,-635,0,0,9),(519,-635,519,-635,0,0,9),Ordering::Less),
        ((-440,707,-440,707,1,0,1),(519,-635,519,-635,0,0,1),Ordering::Less),
        ((-440,707,-440,707,1,0,1),(342,-158,519,-635,0,0,1),Ordering::Less),
        ((-440,707,-440,707,1,0,1),(342,-158,342,-158,0,0,1),Ordering::Less),
        ((661,-976,661,-976,1,0,2),(519,-635,519,-635,0,0,2),Ordering::Greater),
        ((-440,707,661,-976,1,0,8),(661,-976,661,-976,1,0,8),Ordering::Less),
        ((-440,707,661,-976,1,0,8),(519,-635,519,-635,0,0,8),Ordering::Less),
        ((-440,707,661,-976,1,0,8),(342,-158,519,-635,0,0,8),Ordering::Less),
        ((-440,707,661,-976,1,0,8),(342,-158,342,-158,0,0,8),Ordering::Less),
        ((-440,707,661,-976,1,0,8),(-440,707,-440,707,1,0,8),Ordering::Greater),
        ((269,-113,269,-113,2,0,1),(661,-976,661,-976,1,0,1),Ordering::Less),
        ((269,-113,269,-113,2,0,1),(519,-635,519,-635,0,0,1),Ordering::Less),
        ((269,-113,269,-113,2,0,1),(342,-158,519,-635,0,0,1),Ordering::Less),
        ((269,-113,269,-113,2,0,1),(342,-158,342,-158,0,0,1),Ordering::Less),
        ((269,-113,269,-113,2,0,1),(-440,707,661,-976,1,0,1),Ordering::Greater),
        ((507,-171,507,-171,2,0,2),(661,-976,661,-976,1,0,2),Ordering::Less),
        ((507,-171,507,-171,2,0,2),(519,-635,519,-635,0,0,2),Ordering::Less),
        ((507,-171,507,-171,2,0,2),(342,-158,519,-635,0,0,2),Ordering::Greater),
        ((269,-113,507,-171,2,0,8),(661,-976,661,-976,1,0,8),Ordering::Less),
        ((269,-113,507,-171,2,0,8),(519,-635,519,-635,0,0,8),Ordering::Less),
        ((269,-113,507,-171,2,0,8),(507,-171,507,-171,2,0,8),Ordering::Less),
        ((269,-113,507,-171,2,0,8),(342,-158,519,-635,0,0,8),Ordering::Less),
        ((269,-113,507,-171,2,0,8),(342,-158,342,-158,0,0,8),Ordering::Less),
        ((269,-113,507,-171,2,0,8),(269,-113,269,-113,2,0,8),Ordering::Greater),
        // polyghon_003
        ((0,8,0,8,0,0,2),(0,0,0,0,0,0,1),Ordering::Greater), //false,true
        ((0,0,0,8,0,0,8),(0,8,0,8,0,0,2),Ordering::Less), //true,false
        ((0,0,0,8,0,0,8),(0,0,0,0,0,0,1),Ordering::Greater), //false,true
        ((0,8,0,8,1,0,1),(0,8,0,8,0,0,2),Ordering::Equal), //false,false
        ((4,12,4,12,1,0,2),(0,8,0,8,1,0,1),Ordering::Greater), //false,true
        ((0,8,4,12,1,0,8),(4,12,4,12,1,0,2),Ordering::Less), //true,false
        ((0,8,4,12,1,0,8),(0,8,0,8,1,0,1),Ordering::Greater), //false,true
        ((4,12,4,12,2,0,1),(4,12,4,12,1,0,2),Ordering::Equal), //false,false
        ((9,13,9,13,2,0,2),(4,12,4,12,2,0,1),Ordering::Greater), //false,true
        ((4,12,9,13,2,0,8),(9,13,9,13,2,0,2),Ordering::Less), //true,false
        ((4,12,9,13,2,0,8),(4,12,4,12,2,0,1),Ordering::Greater), //false,true
        ((9,13,9,13,3,0,1),(9,13,9,13,2,0,2),Ordering::Equal), //false,false
        ((13,13,13,13,3,0,2),(9,13,9,13,3,0,1),Ordering::Greater), //false,true
        ((9,13,13,13,3,0,8),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((9,13,13,13,3,0,8),(9,13,9,13,3,0,1),Ordering::Greater), //false,true
        ((13,13,13,13,4,0,1),(13,13,13,13,3,0,2),Ordering::Equal), //false,false
        ((13,4,13,4,4,0,2),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((13,4,13,4,4,0,2),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((13,4,13,4,4,0,2),(9,13,13,13,3,0,8),Ordering::Greater), //false,true
        ((13,4,13,13,4,0,9),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((13,4,13,13,4,0,9),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((13,4,13,13,4,0,9),(13,4,13,4,4,0,2),Ordering::Greater), //false,true
        ((13,4,13,4,5,0,1),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((13,4,13,4,5,0,1),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((13,4,13,4,5,0,1),(13,4,13,13,4,0,9),Ordering::Less), //true,false
        ((13,4,13,4,5,0,1),(13,4,13,4,4,0,2),Ordering::Equal), //false,false
        ((10,0,10,0,5,0,2),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((10,0,10,0,5,0,2),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((10,0,10,0,5,0,2),(13,4,13,13,4,0,9),Ordering::Less), //true,false
        ((10,0,10,0,5,0,2),(13,4,13,4,5,0,1),Ordering::Less), //true,false
        ((10,0,10,0,5,0,2),(13,4,13,4,4,0,2),Ordering::Less), //true,false
        ((10,0,10,0,5,0,2),(9,13,13,13,3,0,8),Ordering::Greater), //false,true
        ((10,0,13,4,5,0,9),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((10,0,13,4,5,0,9),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((10,0,13,4,5,0,9),(13,4,13,13,4,0,9),Ordering::Less), //true,false
        ((10,0,13,4,5,0,9),(13,4,13,4,5,0,1),Ordering::Less), //true,false
        ((10,0,13,4,5,0,9),(13,4,13,4,4,0,2),Ordering::Less), //true,false
        ((10,0,13,4,5,0,9),(10,0,10,0,5,0,2),Ordering::Greater), //false,true
        ((10,0,10,0,6,0,1),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((10,0,10,0,6,0,1),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((10,0,10,0,6,0,1),(13,4,13,13,4,0,9),Ordering::Less), //true,false
        ((10,0,10,0,6,0,1),(13,4,13,4,5,0,1),Ordering::Less), //true,false
        ((10,0,10,0,6,0,1),(13,4,13,4,4,0,2),Ordering::Less), //true,false
        ((10,0,10,0,6,0,1),(10,0,13,4,5,0,9),Ordering::Less), //true,false
        ((10,0,10,0,6,0,1),(10,0,10,0,5,0,2),Ordering::Equal), //false,false
        ((5,-1,5,-1,6,0,2),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(13,4,13,13,4,0,9),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(13,4,13,4,5,0,1),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(13,4,13,4,4,0,2),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(10,0,13,4,5,0,9),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(10,0,10,0,6,0,1),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(10,0,10,0,5,0,2),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(9,13,13,13,3,0,8),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(9,13,9,13,3,0,1),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(9,13,9,13,2,0,2),Ordering::Less), //true,false
        ((5,-1,5,-1,6,0,2),(4,12,9,13,2,0,8),Ordering::Greater), //false,true
        ((5,-1,10,0,6,0,9),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(13,4,13,13,4,0,9),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(13,4,13,4,5,0,1),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(13,4,13,4,4,0,2),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(10,0,13,4,5,0,9),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(10,0,10,0,6,0,1),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(10,0,10,0,5,0,2),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(9,13,13,13,3,0,8),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(9,13,9,13,3,0,1),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(9,13,9,13,2,0,2),Ordering::Less), //true,false
        ((5,-1,10,0,6,0,9),(5,-1,5,-1,6,0,2),Ordering::Greater), //false,true
        ((5,-1,5,-1,7,0,1),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(13,4,13,13,4,0,9),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(13,4,13,4,5,0,1),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(13,4,13,4,4,0,2),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(10,0,13,4,5,0,9),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(10,0,10,0,6,0,1),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(10,0,10,0,5,0,2),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(9,13,13,13,3,0,8),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(9,13,9,13,3,0,1),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(9,13,9,13,2,0,2),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(5,-1,10,0,6,0,9),Ordering::Less), //true,false
        ((5,-1,5,-1,7,0,1),(5,-1,5,-1,6,0,2),Ordering::Equal), //false,false
        ((0,0,0,0,7,0,2),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(13,4,13,13,4,0,9),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(13,4,13,4,5,0,1),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(13,4,13,4,4,0,2),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(10,0,13,4,5,0,9),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(10,0,10,0,6,0,1),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(10,0,10,0,5,0,2),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(9,13,13,13,3,0,8),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(9,13,9,13,3,0,1),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(9,13,9,13,2,0,2),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(5,-1,10,0,6,0,9),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(5,-1,5,-1,7,0,1),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(5,-1,5,-1,6,0,2),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(4,12,9,13,2,0,8),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(4,12,4,12,2,0,1),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(4,12,4,12,1,0,2),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(0,8,4,12,1,0,8),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(0,8,0,8,1,0,1),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(0,8,0,8,0,0,2),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(0,0,0,8,0,0,8),Ordering::Less), //true,false
        ((0,0,0,0,7,0,2),(0,0,0,0,0,0,1),Ordering::Equal), //false,false
        ((0,0,5,-1,7,0,9),(13,13,13,13,4,0,1),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(13,13,13,13,3,0,2),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(13,4,13,13,4,0,9),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(13,4,13,4,5,0,1),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(13,4,13,4,4,0,2),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(10,0,13,4,5,0,9),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(10,0,10,0,6,0,1),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(10,0,10,0,5,0,2),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(9,13,13,13,3,0,8),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(9,13,9,13,3,0,1),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(9,13,9,13,2,0,2),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(5,-1,10,0,6,0,9),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(5,-1,5,-1,7,0,1),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(5,-1,5,-1,6,0,2),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(4,12,9,13,2,0,8),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(4,12,4,12,2,0,1),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(4,12,4,12,1,0,2),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(0,8,4,12,1,0,8),Ordering::Less), //true,false
        ((0,0,5,-1,7,0,9),(0,8,0,8,1,0,1),Ordering::Greater), //false,true
        // polygon 006
        ((0,10,0,10,0,0,2),(0,0,0,0,0,0,2),Ordering::Greater),
        ((0,0,0,10,0,0,8),(0,10,0,10,0,0,8),Ordering::Less),
        ((0,0,0,10,0,0,8),(0,0,0,0,0,0,8),Ordering::Greater),
        ((0,10,0,10,1,0,1),(0,10,0,10,0,0,1),Ordering::Equal),
        ((6,10,6,10,1,0,2),(0,10,0,10,1,0,2),Ordering::Greater),
        ((0,10,6,10,1,0,8),(6,10,6,10,1,0,8),Ordering::Less),
        ((0,10,6,10,1,0,8),(0,10,0,10,1,0,8),Ordering::Greater),
        ((6,10,6,10,2,0,1),(6,10,6,10,1,0,1),Ordering::Equal),
        ((10,7,10,7,2,0,2),(6,10,6,10,2,0,2),Ordering::Greater),
        ((6,10,10,7,2,0,8),(10,7,10,7,2,0,8),Ordering::Less),
        ((6,10,10,7,2,0,8),(6,10,6,10,2,0,8),Ordering::Greater),
        ((10,7,10,7,3,0,1),(10,7,10,7,2,0,1),Ordering::Equal),
        ((14,10,14,10,3,0,2),(10,7,10,7,3,0,2),Ordering::Greater),
        ((10,7,14,10,3,0,8),(14,10,14,10,3,0,8),Ordering::Less),
        ((10,7,14,10,3,0,8),(10,7,10,7,3,0,8),Ordering::Greater),
        ((14,10,14,10,4,0,1),(14,10,14,10,3,0,1),Ordering::Equal),
        ((20,10,20,10,4,0,2),(14,10,14,10,4,0,2),Ordering::Greater),
        ((14,10,20,10,4,0,8),(20,10,20,10,4,0,8),Ordering::Less),
        ((14,10,20,10,4,0,8),(14,10,14,10,4,0,8),Ordering::Greater),
        ((20,10,20,10,5,0,1),(20,10,20,10,4,0,1),Ordering::Equal),
        ((20,0,20,0,5,0,2),(20,10,20,10,5,0,2),Ordering::Less),
        ((20,0,20,0,5,0,2),(20,10,20,10,4,0,2),Ordering::Less),
        ((20,0,20,0,5,0,2),(14,10,20,10,4,0,2),Ordering::Greater),
        ((20,0,20,10,5,0,9),(20,10,20,10,5,0,9),Ordering::Less),
        ((20,0,20,10,5,0,9),(20,10,20,10,4,0,9),Ordering::Less),
        ((20,0,20,10,5,0,9),(20,0,20,0,5,0,9),Ordering::Greater),
        ((20,0,20,0,6,0,1),(20,10,20,10,5,0,1),Ordering::Less),
        ((20,0,20,0,6,0,1),(20,10,20,10,4,0,1),Ordering::Less),
        ((20,0,20,0,6,0,1),(20,0,20,10,5,0,1),Ordering::Less),
        ((20,0,20,0,6,0,1),(20,0,20,0,5,0,1),Ordering::Equal),
        ((0,0,0,0,6,0,2),(20,10,20,10,5,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(20,10,20,10,4,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(20,0,20,10,5,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(20,0,20,0,6,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(20,0,20,0,5,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(14,10,20,10,4,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(14,10,14,10,4,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(14,10,14,10,3,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(10,7,14,10,3,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(10,7,10,7,3,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(10,7,10,7,2,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(6,10,10,7,2,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(6,10,6,10,2,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(6,10,6,10,1,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(0,10,6,10,1,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(0,10,0,10,1,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(0,10,0,10,0,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(0,0,0,10,0,0,2),Ordering::Less),
        ((0,0,0,0,6,0,2),(0,0,0,0,0,0,2),Ordering::Equal),
        ((0,0,20,0,6,0,9),(20,10,20,10,5,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(20,10,20,10,4,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(20,0,20,10,5,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(20,0,20,0,6,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(20,0,20,0,5,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(14,10,20,10,4,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(14,10,14,10,4,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(14,10,14,10,3,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(10,7,14,10,3,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(10,7,10,7,3,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(10,7,10,7,2,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(6,10,10,7,2,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(6,10,6,10,2,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(6,10,6,10,1,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(0,10,6,10,1,0,9),Ordering::Less),
        ((0,0,20,0,6,0,9),(0,10,0,10,1,0,9),Ordering::Greater),
    ];

    for (s0, s1, cmp0) in d {
        let lhs = VSE::SiteEvent::<i64, f64>::new_7(s0.0, s0.1, s0.2, s0.3, s0.4, s0.5, s0.6);
        let rhs = VSE::SiteEvent::<i64, f64>::new_7(s1.0, s1.1, s1.2, s1.3, s1.4, s1.5, s1.6);
        let cmp1 = lhs.cmp(&rhs);
        match (cmp1, cmp0) {
            (Ordering::Greater, Ordering::Less) | (Ordering::Less, Ordering::Greater) => {
                assert_eq!(cmp1, cmp0, "{:?} cmp {:?}", lhs, rhs)
            }
            (Ordering::Equal, _) => {
                assert_eq!(cmp1, cmp0, "{:?} cmp {:?}", lhs, rhs)
            }
            _ => (),
        }
    }
}
