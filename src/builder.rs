// Boost.Polygon library voronoi_diagram.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::beachline as VB;
use super::circleevent as VC;
use super::diagram as VD;
use super::endpoint as VEP;
use super::predicate as VP;
use super::siteevent as VSE;
use super::BvError;

use super::{Line, Point};
use std::collections::BinaryHeap;
use std::ops::Neg;

use super::{BigFloatType, BigIntType, InputType, OutputType};
use crate::beachline::BeachLineNodeData;

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

pub struct Builder<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    pub(crate) site_events_: Vec<VSE::SiteEvent<I1, F1, I2, F2>>,
    circle_events_: VC::CircleEventQueue<F2>,
    end_points_: BinaryHeap<VEP::EndPointPair<I1>>,
    pub(crate) beach_line_: VB::Beachline<I1, F1, I2, F2>,
    index_: usize,
    segments_added: bool, // make sure eventual vertices are added before segments
}

impl<I1, F1, I2, F2> Builder<I1, F1, I2, F2>
where
    I1: InputType + Neg<Output = I1>,
    F1: OutputType + Neg<Output = F1>,
    I2: BigIntType + Neg<Output = I2>,
    F2: BigFloatType + Neg<Output = F2>,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Builder<I1, F1, I2, F2> {
        Self {
            /// key by SiteEventIndexType
            site_events_: Vec::new(),
            beach_line_: VB::Beachline::default(),
            index_: 0,
            end_points_: BinaryHeap::new(),
            circle_events_: VC::CircleEventQueue::<F2>::default(),
            segments_added: false,
        }
    }

    pub fn with_vertices<'a, T>(&mut self, vertices: T) -> Result<(), BvError>
    where
        I1: 'a,
        T: Iterator<Item = &'a Point<I1>>,
    {
        if self.segments_added {
            return Err(BvError::VerticesGoesFirst {
                txt: "Vertices should be added before segments".to_string(),
            });
        }
        for v in vertices {
            let mut s = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(*v, *v, self.index_);
            s.or_source_category(&VD::ColorBits::SINGLE_POINT);
            self.site_events_.push(s);
            self.index_ += 1;
        }
        Ok(())
    }

    pub fn with_segments<'a, T>(&mut self, segments: T) -> Result<(), BvError>
    where
        I1: 'a,
        T: Iterator<Item = &'a Line<I1>>,
    {
        type Cb = VD::ColorBits;
        for s in segments {
            let p1 = s.start;
            let p2 = s.end;
            let mut s1 = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(p1, p1, self.index_);
            s1.or_source_category(&Cb::SEGMENT_START_POINT);
            let mut s2 = VSE::SiteEvent::new_3(p2, p2, self.index_);
            s2.or_source_category(&Cb::SEGMENT_END_POINT);

            self.site_events_.push(s1);
            self.site_events_.push(s2);
            let s3 = if VP::PointComparisonPredicate::<I1>::point_comparison_predicate(&p1, &p2) {
                let mut s3 = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(p1, p2, self.index_);
                s3.or_source_category(&Cb::INITIAL_SEGMENT);
                s3
            } else {
                let mut s3 = VSE::SiteEvent::<I1, F1, I2, F2>::new_3(p2, p1, self.index_);
                s3.or_source_category(&Cb::REVERSE_SEGMENT);
                s3
            };
            self.site_events_.push(s3);
            self.index_ += 1;
        }
        self.segments_added = true;
        Ok(())
    }

    /// Run sweepline algorithm and fill output data structure.
    pub fn construct(&mut self) -> Result<VD::VoronoiDiagram<I1, F1, I2, F2>, BvError> {
        let mut output: VD::VoronoiDiagram<I1, F1, I2, F2> =
            VD::VoronoiDiagram::<I1, F1, I2, F2>::new(self.site_events_.len());

        let mut site_event_iterator_: VSE::SiteEventIndexType = self.init_sites_queue();

        self.init_beach_line(&mut site_event_iterator_, &mut output);

        // The algorithm stops when there are no events to process.
        while !self.circle_events_.is_empty() || (site_event_iterator_ != self.site_events_.len()) {
            if self.circle_events_.is_empty() {
                self.process_site_event(&mut site_event_iterator_, &mut output);
            } else if site_event_iterator_ == self.site_events_.len() {
                self.process_circle_event(&mut output);
            } else if VP::EventComparisonPredicate::<I1, F1, I2, F2>::event_comparison_predicate_bif(
                &self.site_events_[site_event_iterator_],
                &self.circle_events_.peek().unwrap().0.get(),
            ) {
                self.process_site_event(&mut site_event_iterator_, &mut output);
            } else {
                self.process_circle_event(&mut output);
            }

            self.circle_events_.pop_inactive_at_top();
        }

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
                && VP::Predicates::<I1, F1, I2, F2>::is_vertical_2(
                    self.site_events_[*site_event_iterator_].point0(),
                    self.site_events_[0].point0(),
                )
                && VP::Predicates::<I1, F1, I2, F2>::is_vertical_1(
                    &self.site_events_[*site_event_iterator_],
                )
            {
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
    }

    /// Init beach line with the two first sites.
    /// The first site is always a point.
    fn init_beach_line_default(
        &mut self,
        site_event_iterator_: &mut VSE::SiteEventIndexType,
        output: &mut VD::VoronoiDiagram<I1, F1, I2, F2>,
    ) {
        // Get the first and the second site event.
        let first = *site_event_iterator_ - 1;
        let first = self.site_events_[first];
        let second = *site_event_iterator_;
        let second = self.site_events_[second];

        let _ = self.insert_new_arc(first, first, second, output);

        // The second site was already processed. Move the iterator.
        *site_event_iterator_ += 1;
    }

    /// Init beach line with collinear sites.
    fn init_beach_line_collinear_sites(
        &mut self,
        site_event_iterator_: &VSE::SiteEventIndexType,
        output: &mut VD::VoronoiDiagram<I1, F1, I2, F2>,
    ) {
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
            let _ = self
                .beach_line_
                .insert(new_node_key, Some(VB::BeachLineNodeData::new_1(edge)));

            // Update iterators.
            it_first += 1;
            it_second += 1;
        }
    }

    fn deactivate_circle_event(&mut self, value: &Option<VB::BeachLineNodeKey<I1, F1, I2, F2>>) {
        if let Some(value) = value {
            let node_data = self.beach_line_.get_node(&value.get_index()).1;
            let node_cell = node_data.get();
            if let Some(node_cell) = node_cell {
                let cevent: Option<VC::CircleEventIndexType> = node_cell.get_circle_event_id();
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
        let (mut right_it, last_index) = {
            // Get next site event to process.
            let site_event = self.site_events_.get(*site_event_iterator_).unwrap();

            // Move site iterator.
            let mut last_index = *site_event_iterator_ + 1;

            // If a new site is an end point of some segment,
            // remove temporary nodes from the beach line data structure.
            if !site_event.is_segment() {
                while !self.end_points_.is_empty()
                    && &self.end_points_.peek().unwrap().first == site_event.point0()
                {
                    let b_it = self.end_points_.peek().unwrap();
                    let b_it = b_it.second;
                    let _ = self.end_points_.pop();

                    self.beach_line_.erase(b_it);
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

            let right_it = self.beach_line_.lower_bound(new_key);

            (right_it, last_index)
        };

        while *site_event_iterator_ != last_index {
            // site_event is a copy of the the event site_event_iterator_ is indexing
            let mut site_event = self.site_events_[*site_event_iterator_];

            let mut left_it = right_it;

            // Do further processing depending on the above node position.
            // For any two neighboring nodes the second site of the first node
            // is the same as the first site of the second node.
            if right_it.is_none() {
                // The above arc corresponds to the second arc of the last node.
                // Move the iterator to the last node.

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

                // If the site event is a segment, update its direction.
                if site_event.is_segment() {
                    let _ = site_event.inverse();
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

                let site_arc1 = *(left_it.unwrap().right_site());
                let site1 = *(left_it.unwrap().left_site());

                // Insert new nodes into the beach line. Update the output.
                let new_node_it = self
                    .insert_new_arc(site_arc1, site_arc2, site_event /*, right_it*/, output);

                // Add candidate circles to the circle event queue.
                // There could be up to two circle events formed by
                // a new bisector and the one on the left or right.
                self.activate_circle_event(site1, site_arc1, site_event, new_node_it);

                // If the site event is a segment, update its direction.
                if site_event.is_segment() {
                    let _ = site_event.inverse();
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

        // Get the C site.
        let site3 = it_first.0.right_site();

        // Get the half-edge corresponding to the second bisector - (B, C).
        let bisector2 = it_first.1.get().unwrap().edge_id();

        // Get the half-edge corresponding to the first bisector - (A, B).
        let it_first = self.beach_line_.get_left_neighbour(it_first.0).unwrap();
        let it_first = &self.beach_line_.get_node(&it_first.1);

        let bisector1 = it_first.1.get().unwrap().edge_id();

        // Get the A site.
        let site1 = *it_first.0.left_site();
        #[allow(clippy::suspicious_operation_groupings)]
        let site3 = if !site1.is_segment() && site3.is_segment() && site3.point1() == site1.point0()
        {
            *site3.clone().inverse()
        } else {
            *site3
        };

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

        // Insert the new bisector into the beach line.
        {
            let edge = output
                ._insert_new_edge_5(site1, site3, circle_event, bisector1, bisector2)
                .0;
            let data = if let Some(mut node) = it_first.1.get() {
                let _ = node.set_edge_id(edge);
                Some(node)
            } else {
                Some(BeachLineNodeData::new_1(edge))
            };
            it_first.1.set(data);
        }
        // Remove the (B, C) bisector node from the beach line.
        self.beach_line_.erase(it_last.0.get_index());
        let it_last = (it_first.0, it_first.0.get_index());

        // Pop the topmost circle event from the event queue.
        self.circle_events_.pop_and_destroy();

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

        let it_last = self
            .beach_line_
            .get_right_neighbour_by_id(it_last.0.get_index());

        if it_last.is_some() {
            let it_last = it_last.unwrap();
            let it_last_node = self.beach_line_.get_node(&it_last.get_index()).1;
            self.circle_events_
                .deactivate(it_last_node.get().and_then(|x| x.get_circle_event_id()));

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
        // Create two new bisectors with opposite directions.
        let new_left_node = VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site_arc1, site_event);
        let mut new_right_node =
            VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site_event, site_arc2);

        // Set correct orientation for the first site of the second node.
        if site_event.is_segment() {
            let _ = new_right_node.left_site_m().inverse();
        }

        // Update the output.
        let edges = output._insert_new_edge_2(site_arc2, site_event);

        let _ = self
            .beach_line_
            .insert(new_right_node, Some(VB::BeachLineNodeData::new_1(edges.1)));

        if site_event.is_segment() {
            // Update the beach line with temporary bisector, that will
            // disappear after processing site event corresponding to the
            // second endpoint of the segment site.
            let mut new_node =
                VB::BeachLineNodeKey::<I1, F1, I2, F2>::new_2(site_event, site_event);
            let _ = new_node.right_site_m().inverse();
            let new_node = self.beach_line_.insert(new_node, None);

            // Update the data structure that holds temporary bisectors.
            self.end_points_.push(VEP::EndPointPair::new_2(
                *site_event.point1(),
                new_node.get_index(),
            ));
        }
        let new_node_data = VB::BeachLineNodeData::new_1(edges.0);
        //let rv =
        self.beach_line_
            .insert(new_left_node, Some(new_node_data))
            .get_index()
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
        // Check if the three input sites create a circle event.

        let c_event = VC::CircleEvent::<F2>::new_1(bisector_node);
        let c_event = VC::CircleEventC::<F2>::new_1(c_event);

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
                    let _ = bd.set_circle_event_id(Some(e.0.get().get_index().unwrap())); // make sure it is_some()
                    b.1.set(Some(bd));
                } else {
                    panic!();
                }
            }
        }
    }
}
