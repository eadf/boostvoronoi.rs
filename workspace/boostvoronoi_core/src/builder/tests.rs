#![allow(unused_imports)]
use crate::builder::Builder;
use crate::diagram::Diagram;
use crate::geometry::{Line, Point};
use crate::BvError;

#[test]
fn sort_1() {
    type I = i32;
    type F = f32;
    {
        let coord = |x, y| Point { x, y };

        let _v = vec![coord(10, 11), coord(0, 100), coord(10, 11), coord(0, 100)];

        let vb = Builder::<I, F>::default();
        assert!(vb.site_events_.is_empty());
        let mut vb = vb.with_vertices(_v.iter()).expect("sort_1");
        assert_eq!(vb.site_events_.len(), 4);
        {
            // emulating construct()

            let site_event_iterator_ = vb.init_sites_queue();
            assert_eq!(site_event_iterator_, 0);
            assert_eq!(vb.site_events_.len(), 2);
            let s0 = vb.site_events_.get(0).expect("sort_1");
            let s1 = vb.site_events_.get(1).expect("sort_1");
            assert!(s0.point0().x == 0 && s0.point0().y == 100);
            assert!(s0.point1().x == 0 && s0.point1().y == 100);
            assert!(s1.point0().x == 10 && s1.point0().y == 11);
            assert!(s1.point1().x == 10 && s1.point1().y == 11);
            //vb.init_beach_line(&mut site_event_iterator_, &mut output);
        }
    }
}
