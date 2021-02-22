#![allow(unused_imports)]
use super::super::diagram::VoronoiDiagram;
use super::super::siteevent as VSE;
use super::super::BvError;
use super::super::{Line, Point};

#[test]
fn inverse_test_1() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    let mut a_site =
        VSE::SiteEvent::<I1, F1, I2, F2>::new_3(Point { x: 10, y: 10 }, Point { x: 50, y: 50 }, 1);
    assert_eq!(a_site.is_inverse(), false);
    let _ = a_site.inverse();
    assert_eq!(a_site.is_inverse(), true);
}

#[test]
fn inverse_test_2() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    let mut a_site =
        VSE::SiteEvent::<I1, F1, I2, F2>::new_3(Point { x: 10, y: 11 }, Point { x: 12, y: 13 }, 1);
    assert_eq!(a_site.is_inverse(), false);
    assert_eq!(a_site.x0(), 10);
    assert_eq!(a_site.y0(), 11);
    assert_eq!(a_site.x1(), 12);
    assert_eq!(a_site.y1(), 13);

    let _ = a_site.inverse();
    assert_eq!(a_site.is_inverse(), true);
    assert_eq!(a_site.x0(), 12);
    assert_eq!(a_site.y0(), 13);
    assert_eq!(a_site.x1(), 10);
    assert_eq!(a_site.y1(), 11);

    let _ = a_site.inverse();
    assert_eq!(a_site.is_inverse(), false);
    assert_eq!(a_site.x0(), 10);
    assert_eq!(a_site.y0(), 11);
    assert_eq!(a_site.x1(), 12);
    assert_eq!(a_site.y1(), 13);
}
