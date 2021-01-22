// Boost.Polygon library voronoi_diagram.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::voronoi_beachline as VB;
use super::voronoi_circleevent as VC;
use super::voronoi_diagram as VD;
use super::voronoi_endpoint as VEP;
use super::voronoi_error::BVError;
use super::voronoi_predicate as VP;
use super::voronoi_siteevent as VSE;
use super::voronoi_structures as VS;

use geo::{Coordinate, Line};
use num::{NumCast, PrimInt};
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;
use std::hash::Hash;
use std::ops::Neg;
use std::rc::Rc;

use super::{BigFloatType, BigIntType, BoostInputType, BoostOutputType};
use crate::voronoi_beachline::BeachLineNodeData;

mod tests;

/// GENERAL INFO:
/// The sweepline algorithm implementation to compute Voronoi diagram of
/// points and non-intersecting segments (excluding endpoints).
/// Complexity - O(N*logN), memory usage - O(N), where N is the total number
/// of input geometries.
///
/// CONTRACT:
/// 1) Input geometries should have integral (e.g. int32, int64) coordinate type.
/// 2) Input geometries should not intersect except their endpoints.
///
/// IMPLEMENTATION DETAILS:
/// Each input point creates one input site. Each input segment creates three
/// input sites: two for its endpoints and one for the segment itself (this is
/// made to simplify output construction). All the site objects are constructed
/// and sorted at the algorithm initialization step. Priority queue is used to
/// dynamically hold circle events. At each step of the algorithm execution the
/// leftmost event is retrieved by comparing the current site event and the
/// topmost element from the circle event queue. STL map (red-black tree)
/// container was chosen to hold state of the beach line. The keys of the map
/// correspond to the neighboring sites that form a bisector and values map to
/// the corresponding Voronoi edges in the output data structure.

pub struct VoronoiBuilder<I1, F1, I2, F2>
where
    I1: BoostInputType + Neg<Output = I1>,
    F1: BoostOutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    pub site_events_: Vec<VSE::SiteEvent<I1, F1, I2, F2>>,
    circle_events_: VC::CircleEventQueue<F2>,
    end_points_: BinaryHeap<VEP::EndPointPair<I1>>,
    // todo! make pub(crate)
    pub beach_line_: VB::Beachline<I1, F1, I2, F2>,
    index_: usize,
    segments_added: bool, // make sure eventual vertices is added before segments
    debug_circle_counter: isize, // Just for debugging purposes
    debug_site_counter: isize, // Just for debugging purposes
}

impl<I1, F1, I2, F2> VoronoiBuilder<I1, F1, I2, F2>
where
    I1: BoostInputType + Neg<Output = I1>,
    F1: BoostOutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> VoronoiBuilder<I1, F1, I2, F2> {
        Self {
            //vertexes_: Vec::new(),
            //segments_: Vec::new(),
            /// key by SiteEventIndexType
            site_events_: Vec::new(),
            beach_line_: VB::Beachline::default(),
            index_: 0,
            //site_event_iterator_: 0,
            //_pdo: PhantomData,
            end_points_: BinaryHeap::new(),
            circle_events_: VC::CircleEventQueue::<F2>::default(),
            debug_circle_counter: 0,
            debug_site_counter: 0,
            segments_added: false,
        }
    }

    pub fn with_vertices<'a, T>(&mut self, vertices: T) -> Result<(), BVError>
    where
        I1: 'a,
        T: Iterator<Item = &'a Coordinate<I1>>,
    {
        if self.segments_added {
            return Err(BVError::VerticesGoesFirst {
                txt: "Vertices should be added before segments".to_string(),
            });
        }
        for v in vertices {
            let mut s = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(*v, *v, self.index_);
            s.or_source_category(&VD::SourceCategory::SOURCE_CATEGORY_SINGLE_POINT);
            self.site_events_.push(s);
            self.index_ += 1;
        }
        Ok(())
    }

    pub fn with_segments<'a, T>(&mut self, segments: T) -> Result<(), BVError>
    where
        I1: 'a,
        T: Iterator<Item = &'a Line<I1>>,
    {
        type SC = VD::SourceCategory;
        for s in segments {
            let p1 = Coordinate {
                x: s.start.x,
                y: s.start.y,
            };
            let p2 = Coordinate {
                x: s.end.x,
                y: s.end.y,
            };
            let mut s1 = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(p1, p1, self.index_);
            s1.or_source_category(&SC::SOURCE_CATEGORY_SEGMENT_START_POINT);
            let mut s2 = VSE::SiteEvent::new_3(p2, p2, self.index_);
            s2.or_source_category(&SC::SOURCE_CATEGORY_SEGMENT_END_POINT);

            self.site_events_.push(s1);
            self.site_events_.push(s2);
            let s3 = if VP::PointComparisonPredicate::<I1>::point_comparison_predicate(&p1, &p2) {
                let mut s3 = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(p1, p2, self.index_);
                s3.or_source_category(&SC::SOURCE_CATEGORY_INITIAL_SEGMENT);
                s3
            } else {
                let mut s3 = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(p2, p1, self.index_);
                s3.or_source_category(&SC::SOURCE_CATEGORY_REVERSE_SEGMENT);
                s3
            };
            self.site_events_.push(s3);
            self.index_ += 1;
        }
        self.segments_added = true;
        Ok(())
        // TODO: fail at intersecting segments
    }

    // todo: remove this clippy exception
    #[allow(clippy::collapsible_if)]
    /// Run sweepline algorithm and fill output data structure.
    pub fn construct(&mut self) -> Result<VD::VoronoiDiagram<I1, F1, I2, F2>, BVError> {
        let mut output: VD::VoronoiDiagram<I1, F1, I2, F2> =
            VD::VoronoiDiagram::<I1, F1, I2, F2>::new(self.site_events_.len());

        /*
         */
        //let mut output = ::new(self.site_events_.len());

        let mut site_event_iterator_: VSE::SiteEventIndexType = self.init_sites_queue();
        /*        println!(
                    "********************************************************************************"
                );
                println!("->construct()");
                println!(
                    "********************************************************************************"
                );
                for s in self.site_events_.iter() {
                    println!("{}", s);
                }
                println!("#site_events={}", self.site_events_.len());
        dbg!(
            self.site_events_.len(),
            &self.site_events_,
            site_event_iterator_
        );
        */

        self.init_beach_line(&mut site_event_iterator_, &mut output);

        // The algorithm stops when there are no events to process.
        //event_comparison_predicate event_comparison; wtf is this?
        while !self.circle_events_.is_empty() || (site_event_iterator_ != self.site_events_.len()) {
            /*self.beach_line_.debug_print_all();
            println!("################################################");
            println!(
                "loop circle_events_:{}, num_vertices:{}, site_events:{}, beachline:{} index:{} debug_site_counter:{} debug_circle_counter:{}",
                self.circle_events_.len(),
                output.num_vertices(),
                self.site_events_.len(),
                self.beach_line_.len().0,
                self.index_,
                self.debug_site_counter,
                self.debug_circle_counter,
            );
            if self.debug_circle_counter >= 27 {
                // 30
                print!("");
            }
            */
            //if self.debug_site_counter >= 8 {
            //    print!("");
            //}

            if self.circle_events_.is_empty() {
                self.process_site_event(&mut site_event_iterator_, &mut output);
            } else if site_event_iterator_ == self.site_events_.len() {
                self.process_circle_event(&mut output);
            } else {
                if VP::EventComparisonPredicate::<I1, F1, I2, F2>::event_comparison_predicate_bif(
                    &self.site_events_[site_event_iterator_],
                    &self.circle_events_.peek().unwrap().0.get(),
                ) {
                    self.process_site_event(&mut site_event_iterator_, &mut output);
                } else {
                    self.process_circle_event(&mut output);
                }
            }

            //dbg!(self.circle_events_.len(), site_event_iterator_);
            self.circle_events_.pop_inactive_at_top();

            //println!("end loop");
            //dbg!(self.circle_events_.len(), site_event_iterator_);
        }
        //println!("<-construct()");
        ////dbg!(&self.beach_line_);
        self.beach_line_.clear();

        // Finish construction.
        output._build();
        Ok(output)
    }

    // todo! make pub (crate)
    pub fn init_sites_queue(&mut self) -> VSE::SiteEventIndexType {
        // Sort site events.
        self.site_events_
            .sort_by(VP::EventComparisonPredicate::<I1, F1, I2, F2>::event_comparison_predicate_ii);

        // Remove duplicates.
        self.site_events_.dedup();

        // Index sites.
        for (cur, mut s) in self.site_events_.iter_mut().enumerate() {
            s.sorted_index_ = cur;
        }
        // Init site iterator.
        let site_event_iterator_: VSE::SiteEventIndexType = 0;
        site_event_iterator_
    }

    pub fn init_beach_line(
        &mut self,
        site_event_iterator_: &mut VSE::SiteEventIndexType,
        output: &mut VD::VoronoiDiagram<I1, F1, I2, F2>,
    ) {
        //println!("-> init_beach_line()");
        ////dbg!(&site_event_iterator_);

        if self.site_events_.is_empty() {
            return;
        }
        if self.site_events_.len() == 1 {
            // Handle single site event case.
            let site = &self.site_events_[0];
            output._process_single_site(site); //site.sorted_index(), site.initial_index(), site.source_category());
            *site_event_iterator_ += 1;
        } else {
            let mut skip = 0;

            while *site_event_iterator_ < self.site_events_.len()
                && VP::VoronoiPredicates::<I1, F1, I2, F2>::is_vertical_2(
                    self.site_events_[*site_event_iterator_].point0(),
                    self.site_events_[0].point0(),
                )
                && VP::VoronoiPredicates::<I1, F1, I2, F2>::is_vertical_1(
                    &self.site_events_[*site_event_iterator_],
                )
            {
                ////dbg!(&self.site_events_[*site_event_iterator_].point0());
                ////dbg!(&self.site_events_[0].point0());
                *site_event_iterator_ += 1;
                skip += 1;
            }

            if skip == 1 {
                // Init beach line with the first two sites.
                self.init_beach_line_default(site_event_iterator_, output);
            } else {
                // Init beach line with collinear vertical sites.
                self.init_beach_line_collinear_sites(site_event_iterator_, output);
            }
        }
        //println!("<- init_beach_line()");
        ////dbg!(&site_event_iterator_);
    }

    /// Init beach line with the two first sites.
    /// The first site is always a point.
    fn init_beach_line_default(
        &mut self,
        site_event_iterator_: &mut VSE::SiteEventIndexType,
        output: &mut VD::VoronoiDiagram<I1, F1, I2, F2>,
    ) {
        //println!("->init_beach_line_default()");
        ////dbg!(&site_event_iterator_);
        // Get the first and the second site event.
        let first = *site_event_iterator_ - 1;
        let first = self.site_events_[first];
        let second = *site_event_iterator_;
        let second = self.site_events_[second];

        self.insert_new_arc(first, first, second, output);

        // The second site was already processed. Move the iterator.
        *site_event_iterator_ += 1;
    }

    /// Init beach line with collinear sites.
    fn init_beach_line_collinear_sites(
        &mut self,
        site_event_iterator_: &VSE::SiteEventIndexType,
        output: &mut VD::VoronoiDiagram<I1, F1, I2, F2>,
    ) {
        //println!("->init_beach_line_collinear_sites()");
        ////dbg!(&site_event_iterator_);

        let mut it_first: VSE::SiteEventIndexType = 0;
        let mut it_second: VSE::SiteEventIndexType = 1;
        while it_second != *site_event_iterator_ {
            let first = &self.site_events_[it_first];
            let second = &self.site_events_[it_second];

            // Create a new beach line node.
            let new_node_key = VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(*first, *second);

            // Update the output.
            let edge = output._insert_new_edge_2(*first, *second).0;

            // Insert a new bisector into the beach line.
            self.beach_line_
                .insert(new_node_key, Some(VB::BeachLineNodeData::new_1(edge)));

            //self.beach_line_.debug_print_all();

            // Update iterators.
            it_first += 1;
            it_second += 1;
        }
        ////dbg!(&self.beach_line_, &output.cells(), &output.edges());
    }

    fn deactivate_circle_event(&mut self, value: &Option<VB::BeachLineNodeKey<I1, F1, I2, F2>>) {
        //if self.debug_site_counter >= 6 {
        /*println!(
            "debug_site_counter:{}, circle_events:{}",
            self.debug_site_counter,
            self.circle_events_.len()
        );*/
        //self.beach_line_.debug_print_all();
        //}
        if let Some(value) = value {
            //dbg!(&value);
            let node_data = self.beach_line_.get_node(&value.get_index()).1;
            let node_cell = node_data.get();
            if let Some(node_cell) = node_cell {
                //dbg!(&node_cell);
                let cevent: Option<VC::CircleEventIndexType> = node_cell.get_circle_event_id();
                //dbg!(&cevent);
                self.circle_events_.deactivate(cevent);

                // TODO! should this be in here?
                // make sure there are no dangling references to deactivated circle events..
                //node_cell.set_circle_event_id(None);
                //node_data.set(Some(node_cell));
            }
        }
    }

    pub(crate) fn process_site_event(
        &mut self,
        site_event_iterator_: &mut VSE::SiteEventIndexType,
        output: &mut VD::VoronoiDiagram<I1, F1, I2, F2>,
    ) {
        //println!("->process_site_event");
        //if self.debug_site_counter >= 6 {
        /*println!(
            "debug_site_counter:{}, circle_events:{}",
            self.debug_site_counter,
            self.circle_events_.len()
        );*/
        //self.beach_line_.debug_print_all();
        //}
        self.debug_site_counter += 1;

        let (mut right_it, last_index) = {
            // Get next site event to process.
            let site_event = self.site_events_.get(*site_event_iterator_).unwrap();
            //dbg!(&site_event);

            // Move site iterator.
            let mut last_index = *site_event_iterator_ + 1;

            // If a new site is an end point of some segment,
            // remove temporary nodes from the beach line data structure.
            if !site_event.is_segment() {
                ////dbg!(self.end_points_.len());
                if !self.end_points_.is_empty() {
                    let peek = self.end_points_.peek().unwrap();
                    /*dbg!(
                        self.end_points_.len(),
                        site_event.point0(),
                        peek.first,
                        peek.second.0,
                        self.beach_line_.get_node(&peek.second).0,
                    );*/
                    let a = 0;
                }
                while !self.end_points_.is_empty()
                    && &self.end_points_.peek().unwrap().first == site_event.point0()
                {
                    //self.beach_line_.debug_print_all();
                    let b_it = self.end_points_.peek().unwrap();
                    //dbg!(b_it.first, b_it.second);
                    let b_it = b_it.second;
                    self.end_points_.pop();
                    /*println!(
                        "Erasing:{:?} == {:?}",
                        b_it,
                        self.beach_line_.get_node(&b_it)
                    );*/
                    self.beach_line_.erase(b_it);
                    //self.beach_line_.debug_print_all();
                }
            } else {
                let mut last = self.site_events_.get(last_index);
                while last_index < self.site_events_.len()
                    && last.is_some()
                    && last.unwrap().is_segment()
                    && last.unwrap().point0() == site_event.point0()
                {
                    last_index += 1;
                    last = self.site_events_.get(last_index);
                }
            }

            // Find the node in the binary search tree with left arc
            // lying above the new site point.
            let new_key = VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_1(*site_event);
            //if self.debug_site_counter >= 7 {
            //dbg!(&new_key);
            //    self.beach_line_.debug_cmp_all(new_key);
            //}
            let right_it = self.beach_line_.lower_bound(new_key);
            //if self.debug_site_counter >= 7 {
            //dbg!(&right_it);
            //    println!("debug_site_counter:{}", self.debug_site_counter);
            //self.beach_line_.debug_print_all();
            //}
            (right_it, last_index)
        };
        //if self.debug_site_counter >= 6 {
        //println!("debug_site_counter:{}", self.debug_site_counter);
        //self.beach_line_.debug_print_all();
        //}

        while *site_event_iterator_ != last_index {
            // site_event is a copy of the the event site_event_iterator_ is indexing
            let mut site_event = self.site_events_[*site_event_iterator_];

            //dbg!(&right_it, &site_event_iterator_, &site_event, &last_index);
            //if self.debug_site_counter >= 6 {
            /*println!(
                "debug_site_counter:{}, circle_events:{}",
                self.debug_site_counter,
                self.circle_events_.len()
            );
            self.beach_line_.debug_print_all();
            */
            //}

            let mut left_it = right_it;

            // Do further processing depending on the above node position.
            // For any two neighboring nodes the second site of the first node
            // is the same as the first site of the second node.
            if right_it.is_none() {
                // The above arc corresponds to the second arc of the last node.
                // Move the iterator to the last node.

                let old_left_it = left_it;
                ////dbg!(self.beach_line_.peek_last());
                ////dbg!(self.beach_line_.len());
                left_it = Some(self.beach_line_.peek_last().unwrap().0);

                // Get the second site of the last node
                let site_arc = *(left_it.unwrap().right_site());

                // Insert new nodes into the beach line. Update the output.
                let right_it_idx =
                    self.insert_new_arc(site_arc, site_arc, site_event, /*right_it*/ output);
                {
                    let right_it_complete = self.beach_line_.get_node(&right_it_idx);
                    right_it = Some(right_it_complete.0);
                }

                // Add a candidate circle to the circle event queue.
                // There could be only one new circle event formed by
                // a new bisector and the one on the left.
                self.activate_circle_event(
                    *(left_it.unwrap().left_site()),
                    *(left_it.unwrap().right_site()),
                    site_event,
                    right_it_idx,
                );
            } else if self.beach_line_.is_at_beginning(&right_it) {
                let right_it_some = right_it.unwrap();

                // The above arc corresponds to the first site of the first node.
                let site_arc = right_it_some.left_site();

                // Insert new nodes into the beach line. Update the output.
                left_it = {
                    let new_key = self.insert_new_arc(
                        *site_arc, *site_arc, site_event, /*right_it,*/ output,
                    );
                    Some(self.beach_line_.get_node(&new_key).0)
                };
                ////dbg!(&left_it);

                // If the site event is a segment, update its direction.
                if site_event.is_segment() {
                    site_event.inverse();
                    // Todo! should the original be updated?
                    //self.site_events_[*site_event_iterator_]= site_event;
                }

                // Add a candidate circle to the circle event queue.
                // There could be only one new circle event formed by
                // a new bisector and the one on the right.
                self.activate_circle_event(
                    site_event,
                    *(right_it_some.left_site()),
                    *(right_it_some.right_site()),
                    right_it_some.get_index(),
                );
                right_it = left_it;
            } else {
                //dbg!(&right_it);
                let (site_arc2, site3) = {
                    // The above arc corresponds neither to the first,
                    // nor to the last site in the beach line.
                    let right_it_some = right_it.unwrap();
                    (*right_it_some.left_site(), *right_it_some.right_site())
                };

                // Remove the candidate circle from the event queue.
                self.deactivate_circle_event(&right_it);

                left_it = self
                    .beach_line_
                    .get_left_neighbour(left_it.unwrap())
                    .map(|x| x.0); //--left_it;
                                   //self.beach_line_.debug_print_all();
                                   //dbg!(&left_it);
                let site_arc1 = *(left_it.unwrap().right_site());
                let site1 = *(left_it.unwrap().left_site());
                //left_it.map(|x| x.left_site().sorted_index()).unwrap();

                // Insert new nodes into the beach line. Update the output.
                let new_node_it = self
                    .insert_new_arc(site_arc1, site_arc2, site_event /*, right_it*/, output);
                //self.beach_line_.debug_print_all();

                // Add candidate circles to the circle event queue.
                // There could be up to two circle events formed by
                // a new bisector and the one on the left or right.
                self.activate_circle_event(site1, site_arc1, site_event, new_node_it);

                // If the site event is a segment, update its direction.
                if site_event.is_segment() {
                    site_event.inverse();
                }

                self.activate_circle_event(
                    site_event,
                    site_arc2,
                    site3,
                    right_it.unwrap().get_index(),
                );
                //right_it = new_node_it;
                right_it = Some(self.beach_line_.get_node_key(new_node_it));
            }
            *site_event_iterator_ += 1;
        }
    }

    /// In general case circle event is made of the three consecutive sites
    /// that form two bisectors in the beach line data structure.
    /// Let circle event sites be A, B, C, two bisectors that define
    /// circle event are (A, B), (B, C). During circle event processing
    /// we remove (A, B), (B, C) and insert (A, C). As beach line comparison
    /// works correctly only if one of the nodes is a new one we remove
    /// (B, C) bisector and change (A, B) bisector to the (A, C). That's
    /// why we use const_cast there and take all the responsibility that
    /// map data structure keeps correct ordering.
    #[allow(clippy::unnecessary_unwrap)]
    pub(crate) fn process_circle_event(&mut self, output: &mut VD::VoronoiDiagram<I1, F1, I2, F2>) {
        // Get the topmost circle event.
        let e = self.circle_events_.top().unwrap();
        let circle_event = e.0.get();
        if !self
            .circle_events_
            .is_active(circle_event.get_index().unwrap())
        {
            // todo: remove this panic
            panic!();
        }
        let it_first = &self.beach_line_.get_node(&e.1.unwrap());
        let it_last = it_first;

        //self.circle_events_.dbg();
        //dbg!(e);
        //dbg!(&it_first.0);
        //if self.debug_circle_counter >= 36 {
        //self.beach_line_.debug_print_all();
        //}
        //println!("->process_circle_event() {}", self.debug_circle_counter);

        // Get the C site.
        let site3 = it_first.0.right_site();

        // Get the half-edge corresponding to the second bisector - (B, C).
        let bisector2 = it_first.1.get().unwrap().edge_id();

        // Get the half-edge corresponding to the first bisector - (A, B).
        let it_first = self.beach_line_.get_left_neighbour(it_first.0).unwrap();
        //dbg!(&it_first);
        let it_first = &self.beach_line_.get_node(&it_first.1);
        //dbg!(&it_first.0);

        let bisector1 = it_first.1.get().unwrap().edge_id();
        //dbg!(&bisector1);
        //dbg!(&bisector2);

        // Get the A site.
        let site1 = *it_first.0.left_site();
        #[allow(clippy::suspicious_operation_groupings)]
        let site3 = if !site1.is_segment() && site3.is_segment() && site3.point1() == site1.point0()
        {
            *site3.clone().inverse()
        } else {
            *site3
        };

        /*if self.debug_circle_counter >= 27 {
            print!("");
            self.beach_line_.debug_print_all();
        }*/
        // Change the (A, B) bisector node to the (A, C) bisector node.
        let mut it_first = {
            let it_first_key_before = it_first.0;
            let it_first_key_after = {
                let mut tmp = it_first_key_before;
                tmp.set_right_site(&site3);
                tmp
            };
            self.beach_line_
                .replace_key(it_first_key_before, it_first_key_after)
        };
        /*
        if self.debug_circle_counter >= 27 {
            print!("");
            self.beach_line_.debug_print_all();
        }*/

        // Insert the new bisector into the beach line.
        {
            let edge = output
                ._insert_new_edge_5(site1, site3, circle_event, bisector1, bisector2)
                .0;
            let data = if let Some(mut node) = it_first.1.get() {
                node.set_edge_id(edge);
                Some(node)
            } else {
                Some(BeachLineNodeData::new_1(edge))
            };
            it_first.1.set(data);
        }
        //self.beach_line_.debug_print_all();
        // Remove the (B, C) bisector node from the beach line.
        self.beach_line_.erase(it_last.0.get_index());
        //self.beach_line_.debug_print_all();
        let it_last = (it_first.0, it_first.0.get_index());

        // Pop the topmost circle event from the event queue.
        self.circle_events_.pop_and_destroy();
        //dbg!(self.circle_events_.len());

        // Check new triplets formed by the neighboring arcs
        // to the left for potential circle events.
        if self.beach_line_.len().0 > 0 && it_first.0 != self.beach_line_.peek_first().unwrap().0 {
            self.circle_events_
                .deactivate(it_first.1.get().and_then(|x| x.get_circle_event_id()));
            //--it_first;
            it_first = {
                let id = self.beach_line_.get_left_neighbour(it_first.0).unwrap().1;
                self.beach_line_.get_node(&id)
            };

            let site_l1 = it_first.0.left_site();
            self.activate_circle_event(*site_l1, site1, site3, it_last.0.get_index());
        }

        // Check the new triplet formed by the neighboring arcs
        // to the right for potential circle events.
        //self.beach_line_.debug_print_all();
        //println!("pre++ ");
        //dbg!(&it_last.0);
        let it_last = self
            .beach_line_
            .get_right_neighbour_by_id(it_last.0.get_index());
        //println!("post++ ");
        //dbg!(&it_last);
        //if self.debug_circle_counter >= 7 {
        //    print!("");
        //}
        if it_last.is_some() {
            let it_last = it_last.unwrap();
            let it_last_node = self.beach_line_.get_node(&it_last.get_index()).1;
            self.circle_events_
                .deactivate(it_last_node.get().and_then(|x| x.get_circle_event_id()));

            ////dbg!(&it_last);
            let site_r1 = it_last.right_site();
            self.activate_circle_event(site1, site3, *site_r1, it_last.get_index());
        }
    }

    /// Insert new nodes into the beach line. Update the output.
    /// Todo: should the site events be references and only copied when inserted in beachline?
    fn insert_new_arc(
        &mut self,
        site_arc1: VSE::SiteEvent<I1, F1, I2, F2>,
        site_arc2: VSE::SiteEvent<I1, F1, I2, F2>,
        site_event: VSE::SiteEvent<I1, F1, I2, F2>,
        //position: BeachLineIteratorType,
        output: &mut VD::VoronoiDiagram<I1, F1, I2, F2>,
    ) -> VB::BeachLineIndex {
        //println!("->insert_new_arc()");
        //dbg!(&site_arc1, &site_arc2, &site_event);

        // Create two new bisectors with opposite directions.
        let new_left_node = VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site_arc1, site_event);
        let mut new_right_node =
            VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site_event, site_arc2);

        // Set correct orientation for the first site of the second node.
        if site_event.is_segment() {
            new_right_node.left_site_m().inverse();
        }

        // Update the output.
        let edges = output._insert_new_edge_2(site_arc2, site_event);

        self.beach_line_
            .insert(new_right_node, Some(VB::BeachLineNodeData::new_1(edges.1)));
        //self.beach_line_.debug_print_all();

        if site_event.is_segment() {
            // Update the beach line with temporary bisector, that will
            // disappear after processing site event corresponding to the
            // second endpoint of the segment site.
            let mut new_node =
                VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site_event, site_event);
            new_node.right_site_m().inverse();
            //dbg!(new_node);
            //self.beach_line_.debug_print_all();
            let new_node = self.beach_line_.insert(new_node, None);
            //dbg!(new_node);
            //self.beach_line_.debug_print_all();

            //dbg!(&site_event.point1(), new_node.get_index());
            //dbg!(self.beach_line_.get_node(&new_node.get_index()).0);

            // Update the data structure that holds temporary bisectors.
            self.end_points_.push(VEP::EndPointPair::new_2(
                *site_event.point1(),
                new_node.get_index(),
            ));
            //dbg!(&self.end_points_.peek().unwrap().first);
        }
        let new_node_data = VB::BeachLineNodeData::new_1(edges.0);
        //let rv =
        self.beach_line_
            .insert(new_left_node, Some(new_node_data))
            .get_index()
        //self.beach_line_.debug_print_all();
        //rv
    }

    /// Add a new circle event to the event queue.
    /// bisector_node corresponds to the (site2, site3) bisector.
    fn activate_circle_event(
        &mut self,
        site1: VSE::SiteEvent<I1, F1, I2, F2>,
        site2: VSE::SiteEvent<I1, F1, I2, F2>,
        site3: VSE::SiteEvent<I1, F1, I2, F2>,
        bisector_node: VB::BeachLineIndex,
    ) {
        //self.beach_line_.debug_print_all();
        /*
        println!("activate_circle_event: {}", self.debug_circle_counter);
        println!("Site1:{:?}", &site1);
        println!("Site2:{:?}", &site2);
        println!("Site3:{:?}", &site3);
        println!(
            "bi-node L:{:?}",
            &self.beach_line_.get_node(&bisector_node).0.left_site()
        );
        println!(
            "bi-node R:{:?}",
            &self.beach_line_.get_node(&bisector_node).0.right_site()
        );
        */
        //if self.debug_circle_counter >= 6 {
        //    self.beach_line_.debug_print_all();
        //    self.debug_circle_counter += 0;
        //}
        self.debug_circle_counter += 1;

        // Check if the three input sites create a circle event.

        let c_event = VC::CircleEvent::<F2>::new_1(bisector_node);
        let c_event = VC::CircleEventC::<F2>::new_1(c_event);
        //dbg!(&c_event);
        if VP::CircleFormationFunctor::<I1, F1, I2, F2>::circle_formation_predicate(
            &site1, &site2, &site3, &c_event,
        ) {
            // Add the new circle event to the circle events queue.
            // Update bisector's circle event iterator to point to the
            // new circle event in the circle event queue.

            let e = self.circle_events_.associate_and_push(c_event);
            {
                let b = self.beach_line_.get_node(&bisector_node);
                if let Some(mut bd) = b.1.get() {
                    bd.set_circle_event_id(Some(e.0.get().get_index().unwrap())); // make sure it is_some()
                    b.1.set(Some(bd));
                } else {
                    panic!();
                }
            }
        }
    }
}
