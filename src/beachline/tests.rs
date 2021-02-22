#![allow(unused_imports)]
use super::super::beachline as VB;
use super::super::builder::Builder;
use super::super::diagram as VD;
use super::super::predicate as VP;
use super::super::siteevent as VSE;
use super::super::BvError;
use super::super::{Line, Point};
use super::{BeachLineIndex, BeachLineNodeData, BeachLineNodeKey, Beachline};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

#[test]
fn beachline_0() {
    let mut map = BTreeMap::<i32, i32>::new();
    for i in 0..5 {
        let _ = map.insert(i, i);
    }

    // search next
    let j = *map.range((Excluded(3), Unbounded)).next().unwrap().0;
    assert_eq!(j, 4);

    // search next
    let j = map.range((Excluded(4), Unbounded)).next();
    assert!(j.is_none());

    // search next
    let j = map.range((Excluded(40), Unbounded)).next();
    assert!(j.is_none());

    // search prev
    let j = *map.range((Unbounded, Excluded(3))).next_back().unwrap().0;
    assert_eq!(j, 2);

    // search prev
    let j = map.range((Unbounded, Excluded(0))).next_back();
    assert!(j.is_none());
}

//#[ignore]
#[test]
fn beachline_1() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;
    let coord = |x, y| Point { x, y };

    // Co-linear sites
    let _v = vec![coord(10, 10), coord(1, 1), coord(1, 6)];

    let mut vb = Builder::<I1, F1, I2, F2>::new();
    vb.with_vertices(_v.iter()).unwrap();
    let _output = vb.construct();
    //assert!(false);
}

//#[ignore]
#[test]
fn beachline_2() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    {
        let coord = |x, y| Point { x, y };

        let _v = vec![coord(10, 18), coord(12, 3), coord(4, 21), coord(8, 62)];
        let mut output: VD::VoronoiDiagram<I1, F1, I2, F2> =
            VD::VoronoiDiagram::<I1, F1, I2, F2>::default();

        let mut b = Builder::<I1, F1, I2, F2>::new();
        b.with_vertices(_v.iter()).unwrap();
        let mut site_event_iterator_: VSE::SiteEventIndexType = b.init_sites_queue();
        println!("site_event_iterator_:{:?}", site_event_iterator_);
        b.init_beach_line(&mut site_event_iterator_, &mut output);
        {
            println!("all: size:{}", b.beach_line_.beach_line_.len());
            assert_eq!(b.beach_line_.beach_line_.len(), 2);
            for n in b.beach_line_.beach_line_.iter() {
                println!("{:?}", n);
            }
            let site_event = &b.site_events_[2];
            dbg!(&site_event);
            let new_key = VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_1(*site_event);
            dbg!(&new_key);
            let lb = b.beach_line_.lower_bound(new_key);
            dbg!(&lb); // lb should be : right_it:L(4,21#0) R(8,62#1)
            assert!(lb.is_some());

            println!("experiment all done");
            println!();
        }
        println!("site_event_iterator_:{:?}", site_event_iterator_);
    }
}

#[test]
fn beachline_3() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    {
        let _s = vec![Line::new(Point { x: 10, y: 10 }, Point { x: 50, y: 50 })];
        let mut output: VD::VoronoiDiagram<I1, F1, I2, F2> =
            VD::VoronoiDiagram::<I1, F1, I2, F2>::default();

        let mut b = Builder::<I1, F1, I2, F2>::new();
        b.with_segments(_s.iter()).unwrap();
        let mut site_event_iterator_: VSE::SiteEventIndexType = b.init_sites_queue();
        println!("site_event_iterator_:{:?}", site_event_iterator_);
        b.init_beach_line(&mut site_event_iterator_, &mut output);
        {
            println!("all: size:{}", b.beach_line_.beach_line_.len());
            assert_eq!(b.beach_line_.beach_line_.len(), 3);
            for n in b.beach_line_.beach_line_.iter() {
                println!("{:?}", n);
            }
            let site_event = &b.site_events_[2];
            dbg!(&site_event);
            let new_key = VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_1(*site_event);
            dbg!(&new_key);
            let lb = b.beach_line_.lower_bound(new_key);
            dbg!(&lb); // lb should be : right_it:L(4,21#0) R(8,62#1)
            assert!(lb.is_some());

            println!("experiment all done");
            println!();
            //panic!();
        }
        println!("site_event_iterator_:{:?}", site_event_iterator_);
    }
}

#[test]
fn beachline_4() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    let coord = |x, y| Point { x, y };

    let mut a_site = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(coord(10, 10), coord(50, 50), 1);
    assert!(!a_site.is_inverse());
    let _ = a_site.inverse();
    assert!(a_site.is_inverse());

    //new_key_(50,50)#2, ((50,50)#2
    let mykey = {
        let mut site1 = VSE::SiteEvent::<I1, F1, I2, F2>::new_2(coord(50, 50), 2);
        site1.set_sorted_index(2);
        VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_1(site1)
    };

    // (10,10)#0, ((10,10)-(50,50)#1
    let node1 = {
        let mut site1 = VSE::SiteEvent::<I1, F1, I2, F2>::new_2(coord(10, 10), 0);
        site1.set_sorted_index(0);
        let mut site2 = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(coord(10, 10), coord(50, 50), 1);
        site2.set_sorted_index(1);
        VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site1, site2)
    };

    //(50,50)¿(10,10)#1, ((10,10)#0
    let node2 = {
        let mut site1 = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(coord(10, 10), coord(50, 50), 1);
        let _ = site1.inverse();
        site1.set_sorted_index(1);
        dbg!(site1);
        let mut site2 = VSE::SiteEvent::<I1, F1, I2, F2>::new_2(coord(10, 10), 0);
        site2.set_sorted_index(0);

        //dbg!(site1, site2);
        VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site1, site2)
    };

    dbg!(mykey, node1);

    let is_less =
        VP::NodeComparisonPredicate::<I1, F1, I2, F2>::node_comparison_predicate(&node1, &mykey);
    dbg!(is_less);
    assert_eq!(is_less, true);

    println!();
    let is_less =
        VP::NodeComparisonPredicate::<I1, F1, I2, F2>::node_comparison_predicate(&node2, &mykey);
    dbg!(mykey, node2, is_less);
    assert_eq!(is_less, false);
}

#[test]
fn beachline_5() {
    type I1 = i32;
    type F1 = f64;
    type I2 = i64;
    type F2 = f64;

    let coord = |x, y| Point::<I1> { x, y };

    let node1 = {
        let mut site1 =
            VSE::SiteEvent::<I1, F1, I2, F2>::new_3(coord(367, 107), coord(529, 242), 4);
        site1.set_sorted_index(6);
        site1.set_flags(9);

        let mut site2 =
            VSE::SiteEvent::<I1, F1, I2, F2>::new_3(coord(367, 107), coord(529, 242), 4);
        let _ = site2.inverse();
        site2.set_sorted_index(6);
        site2.set_flags(41);
        VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site1, site2)
    };
    println!("L:#6(367,107)-(529,242),ii:4,f:9,R:#6(529,242)¿(367,107),ii:4,f:41 -> CircleEvent=-, Temporary bisector");
    println!("{:?}", node1);
    println!();

    let node2 = {
        let mut site1 =
            VSE::SiteEvent::<I1, F1, I2, F2>::new_3(coord(367, 107), coord(529, 242), 4);
        let _ = site1.inverse();
        site1.set_sorted_index(6);
        site1.set_flags(41);
        let mut site2 =
            VSE::SiteEvent::<I1, F1, I2, F2>::new_3(coord(400, 200), coord(400, 200), 2);
        site2.set_sorted_index(7);
        site2.set_flags(2);
        VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site1, site2)
    };

    println!("L:#6(529,242)¿(367,107),ii:4,f:41,R:#7(400,200),ii:2,f:2 -> CircleEvent=-");
    println!("{:?}", node2);
    println!();

    let node3 = {
        let mut site1 =
            VSE::SiteEvent::<I1, F1, I2, F2>::new_3(coord(200, 200), coord(400, 200), 3);
        site1.set_sorted_index(3);
        site1.set_flags(9);
        let mut site2 = VSE::SiteEvent::<I1, F1, I2, F2>::new_2(coord(400, 200), 2);
        site2.set_sorted_index(7);
        site2.set_flags(2);
        VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site1, site2)
    };

    println!("L:#3(200,200)-(400,200),ii:3,f:9,R:#7(400,200),ii:2,f:2 -> CircleEvent(x=4.00000e+02,y=1.71543e+02,lx=4.28457e+02)");
    println!("{:?}", node3);
    println!();

    //let is_less =
    //    VP::NodeComparisonPredicate::<I1, F1, I2, F2>::node_comparison_predicate(&node1, &node2);
    let is_less = node2.cmp(&node1);
    dbg!(is_less);
    assert_eq!(is_less, Ordering::Greater);

    //let is_less =
    //    VP::NodeComparisonPredicate::<I1, F1, I2, F2>::node_comparison_predicate(&node2, &node3);
    let is_less = node3.cmp(&node2);
    dbg!(is_less);
    assert_eq!(is_less, Ordering::Greater);
}
