#![allow(unused_imports)]
use super::super::beach_line as VB;
use super::super::builder::Builder;
use super::super::diagram as VD;
use super::super::geometry::{Line, Point};
use super::super::predicate as VP;
use super::super::site_event as VSE;
use super::super::BvError;
use super::{BeachLine, BeachLineIndex, BeachLineNodeData, BeachLineNodeKey};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

//#[ignore]
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
    type I = i32;
    type F = f32;
    let coord = |x, y| Point { x, y };

    // Co-linear sites
    let _v = vec![coord(10, 10), coord(1, 1), coord(1, 6)];

    let mut vb = Builder::<I, F>::default();
    vb.with_vertices(_v.iter()).unwrap();
    let _output = vb.build();
    //assert!(false);
}

//#[ignore]
#[test]
fn beachline_2() -> Result<(), BvError> {
    type I = i32;
    type F = f32;

    {
        let coord = |x, y| Point { x, y };

        let _v = vec![coord(10, 18), coord(12, 3), coord(4, 21), coord(8, 62)];
        let mut output: VD::Diagram<F> = VD::Diagram::<F>::default();

        let mut b = Builder::<I, F>::default();
        b.with_vertices(_v.iter()).unwrap();
        let mut site_event_iterator_: VSE::SiteEventIndexType = b.init_sites_queue();
        println!("site_event_iterator_:{:?}", site_event_iterator_);
        b.init_beach_line(&mut site_event_iterator_, &mut output)?;
        {
            println!("all: size:{}", b.beach_line_.beach_line_.borrow().len());
            assert_eq!(b.beach_line_.beach_line_.borrow().len(), 2);
            for n in b.beach_line_.beach_line_.borrow().iter() {
                println!("{:?}", n);
            }
            let site_event = &b.site_events_[2];
            dbg!(&site_event);
            let new_key = VB::BeachLineNodeKey::<I, F>::new_1(*site_event);
            dbg!(&new_key);
            let lb = b.beach_line_.lower_bound(new_key)?;
            dbg!(&lb); // lb should be : right_it:L(4,21#0) R(8,62#1)
            assert!(lb.is_ok()?);

            println!("experiment all done");
            println!();
        }
        println!("site_event_iterator_:{:?}", site_event_iterator_);
    }
    Ok(())
}

//#[ignore]
#[test]
fn beachline_3() -> Result<(), BvError> {
    type I = i32;
    type F = f32;

    {
        let _s = vec![Line::new(Point { x: 10, y: 10 }, Point { x: 50, y: 50 })];
        let mut output: VD::Diagram<F> = VD::Diagram::<F>::default();

        let mut b = Builder::<I, F>::default();
        b.with_segments(_s.iter()).unwrap();
        let mut site_event_iterator_: VSE::SiteEventIndexType = b.init_sites_queue();
        println!("site_event_iterator_:{:?}", site_event_iterator_);
        b.init_beach_line(&mut site_event_iterator_, &mut output)?;
        {
            println!("all: size:{}", b.beach_line_.beach_line_.borrow().len());
            assert_eq!(b.beach_line_.beach_line_.borrow().len(), 3);
            for n in b.beach_line_.beach_line_.borrow().iter() {
                println!("{:?}", n);
            }
            let site_event = &b.site_events_[2];
            dbg!(&site_event);
            let new_key = VB::BeachLineNodeKey::<I, F>::new_1(*site_event);
            dbg!(&new_key);
            let lb = b.beach_line_.lower_bound(new_key)?;
            dbg!(&lb); // lb should be : right_it:L(4,21#0) R(8,62#1)
            assert!(lb.is_ok()?);

            println!("experiment all done");
            println!();
        }
        println!("site_event_iterator_:{:?}", site_event_iterator_);
    }
    Ok(())
}

//#[ignore]
#[test]
fn beachline_4() {
    type I = i32;
    type F = f32;

    let coord = |x, y| Point { x, y };

    let mut a_site = VSE::SiteEvent::<I, F>::new_3(coord(10, 10), coord(50, 50), 1);
    assert!(!a_site.is_inverse());
    let _ = a_site.inverse();
    assert!(a_site.is_inverse());

    //new_key_(50,50)#2, ((50,50)#2
    let mykey = {
        let mut site1 = VSE::SiteEvent::<I, F>::new_2(coord(50, 50), 2);
        site1.set_sorted_index(2);
        VB::BeachLineNodeKey::<I, F>::new_1(site1)
    };

    // (10,10)#0, ((10,10)-(50,50)#1
    let node1 = {
        let mut site1 = VSE::SiteEvent::<I, F>::new_2(coord(10, 10), 0);
        site1.set_sorted_index(0);
        let mut site2 = VSE::SiteEvent::<I, F>::new_3(coord(10, 10), coord(50, 50), 1);
        site2.set_sorted_index(1);
        VB::BeachLineNodeKey::<I, F>::new_2(site1, site2)
    };

    //(50,50)¿(10,10)#1, ((10,10)#0
    let node2 = {
        let mut site1 = VSE::SiteEvent::<I, F>::new_3(coord(10, 10), coord(50, 50), 1);
        let _ = site1.inverse();
        site1.set_sorted_index(1);
        dbg!(site1);
        let mut site2 = VSE::SiteEvent::<I, F>::new_2(coord(10, 10), 0);
        site2.set_sorted_index(0);

        //dbg!(site1, site2);
        VB::BeachLineNodeKey::<I, F>::new_2(site1, site2)
    };

    dbg!(mykey, node1);

    let is_less = VP::NodeComparisonPredicate::node_comparison_predicate::<I, F>(&node1, &mykey);
    dbg!(is_less);
    assert_eq!(is_less, true);
    let cmp = mykey.cmp(&node1);
    dbg!(cmp);
    assert_eq!(cmp, Ordering::Greater);
    let cmp = node1.cmp(&mykey);
    dbg!(cmp);
    assert_eq!(cmp, Ordering::Less);

    println!();
    let is_less = VP::NodeComparisonPredicate::node_comparison_predicate::<I, F>(&node2, &mykey);
    dbg!(mykey, node2, is_less);
    assert_eq!(is_less, false);
    let cmp = mykey.cmp(&node2);
    dbg!(cmp);
    assert_eq!(cmp, Ordering::Less);
    let cmp = node2.cmp(&mykey);
    dbg!(cmp);
    assert_eq!(cmp, Ordering::Greater);
}

//#[ignore]
#[test]
fn beachline_5() {
    type I = i32;
    type F = f64;

    let coord = |x, y| Point::<I> { x, y };

    let node1 = {
        let mut site1 = VSE::SiteEvent::<I, F>::new_3(coord(367, 107), coord(529, 242), 4);
        site1.set_sorted_index(6);
        site1.set_flags(9);

        let mut site2 = VSE::SiteEvent::<I, F>::new_3(coord(367, 107), coord(529, 242), 4);
        let _ = site2.inverse();
        site2.set_sorted_index(6);
        site2.set_flags(41);
        VB::BeachLineNodeKey::<I, F>::new_2(site1, site2)
    };
    println!("L:#6(367,107)-(529,242),ii:4,f:9,R:#6(529,242)¿(367,107),ii:4,f:41 ");
    println!("{:?}", node1);
    println!();

    let node2 = {
        let mut site1 = VSE::SiteEvent::<I, F>::new_3(coord(367, 107), coord(529, 242), 4);
        let _ = site1.inverse();
        site1.set_sorted_index(6);
        site1.set_flags(41);
        let mut site2 = VSE::SiteEvent::<I, F>::new_3(coord(400, 200), coord(400, 200), 2);
        site2.set_sorted_index(7);
        site2.set_flags(2);
        VB::BeachLineNodeKey::<I, F>::new_2(site1, site2)
    };

    println!("L:#6(529,242)¿(367,107),ii:4,f:41,R:#7(400,200),ii:2,f:2 -> CircleEvent=-");
    println!("{:?}", node2);
    println!();

    //let is_less =
    //    VP::NodeComparisonPredicate::<I, F>::node_comparison_predicate(&node1, &node2);
    let is_less = node2.cmp(&node1);
    dbg!(is_less);
    assert_eq!(is_less, Ordering::Greater);
}
