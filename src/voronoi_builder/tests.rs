#![allow(unused_imports)]
use super::super::voronoi_diagram::VoronoiDiagram;
use super::super::voronoi_error::BVError;
use super::VoronoiBuilder;
use geo::{Coordinate, Line};

#[test]
fn sort_1() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;

    {
        let coord = |x, y| Coordinate { x, y };

        let _v = vec![coord(10, 11), coord(0, 100), coord(10, 11), coord(0, 100)];

        let mut vb = VoronoiBuilder::<I1, F1, I2, F2>::new();
        assert!(vb.site_events_.is_empty());
        vb.with_vertices(_v.iter()).expect("sort_1");
        assert_eq!(vb.site_events_.len(), 4);
        {
            // emulating construct()

            let site_event_iterator_ = vb.init_sites_queue();
            assert_eq!(site_event_iterator_, 0);
            assert_eq!(vb.site_events_.len(), 2);
            let s0 = vb.site_events_.get(0).expect("sort_1");
            let s1 = vb.site_events_.get(1).expect("sort_1");
            assert!(s0.point0_.x == 0 && s0.point0_.y == 100);
            assert!(s0.point1_.x == 0 && s0.point1_.y == 100);
            assert!(s1.point0_.x == 10 && s1.point0_.y == 11);
            assert!(s1.point1_.x == 10 && s1.point1_.y == 11);
            //vb.init_beach_line(&mut site_event_iterator_, &mut output);
        }
    }
}
