use boostvoronoi::voronoi_beachline as VB;
#[allow(unused_imports)]
use boostvoronoi::voronoi_diagram as VD;
use boostvoronoi::voronoi_error::BVError;
#[allow(unused_imports)]
use boostvoronoi::voronoi_predicate as VP;
use boostvoronoi::voronoi_siteevent as VSE;
use boostvoronoi::voronoi_structures as VS;
use boostvoronoi::voronoi_structures::Point2dI;
use boostvoronoi::voronoi_structures::Segment2d;
use boostvoronoi::{BigFloatType, BigIntType, BoostInputType, BoostOutputType};
use std::ops::Bound::{Excluded, Included, Unbounded};

#[allow(dead_code)]
fn main() {
    type I1 = i32;
    type F1 = f32;
    type I2 = i64;
    type F2 = f64;
    type Point2D = Point2dI<I1>;
    type Segment2DI = Segment2d<I1>;

    let mut a_site = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(Point2dI::new(10, 10), Point2dI::new(50, 50), 1);
    assert!(!a_site.is_inverse());
    a_site.inverse();
    assert!(a_site.is_inverse());
    
     
    //new_key_(50,50)#2, ((50,50)#2
    let mykey = {
        let mut site1 = VSE::SiteEvent::<I1, F1, I2, F2>::new_2(Point2dI::new(50, 50), 2);
        site1.set_sorted_index(2);
        VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_1(site1)
    };
    
    // (10,10)#0, ((10,10)-(50,50)#1
    let node1 = {
        let mut site1 = VSE::SiteEvent::<I1, F1, I2, F2>::new_2(Point2dI::new(10, 10), 0);
        site1.set_sorted_index(0);
        let mut site2 =
            VSE::SiteEvent::<I1, F1, I2, F2>::new_3(Point2dI::new(10, 10), Point2dI::new(50, 50), 1);
        site2.set_sorted_index(1);
        VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site1, site2)
    };
    
    //(50,50)¿(10,10)#1, ((10,10)#0
    let node2 = {
        let mut site1 = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(Point2dI::new(50, 50), Point2dI::new(10, 10), 1);
        site1.inverse();
        site1.set_sorted_index(1);
        dbg!(site1);
        let mut site2 = VSE::SiteEvent::<I1, F1, I2, F2>::new_2(Point2dI::new(10, 10), 0);
        site2.set_sorted_index(0);
        
        //dbg!(site1, site2);
        VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site1, site2)
    };
    
    dbg!(mykey, node1);
    
    
    let is_less =
        VP::NodeComparisonPredicate::<I1, F1, I2, F2>::node_comparison_predicate(&node1, &mykey);
    dbg!(is_less);
    assert_eq!(is_less,true); 
    
    println!();println!();
    let is_less =
        VP::NodeComparisonPredicate::<I1, F1, I2, F2>::node_comparison_predicate(&node2, &mykey);
    dbg!(mykey);
    dbg!(node2);
    dbg!(is_less);
    assert_eq!(is_less, false); 
    
}
