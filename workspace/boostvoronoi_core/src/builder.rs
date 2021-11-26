// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code..

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! Contains the builder code.

use crate::beach_line as VB;
use crate::circle_event as VC;
use crate::diagram as VD;
use crate::end_point as VEP;
use crate::predicate as VP;
use crate::site_event as VSE;
#[cfg(feature = "console_debug")]
use crate::t;
use crate::{
    geometry::{Line, Point},
    tln, BvError, InputType, OutputType,
};

use cpp_map::PIterator;
use std::collections::BinaryHeap;
use std::rc::Rc;

#[cfg(test)]
mod tests;

/// The sweepline algorithm implementation to compute Voronoi diagram of
/// points and non-intersecting segments (excluding endpoints).
/// Complexity - O(N*logN), memory usage - O(N), where N is the total number
/// of input geometries.
///
/// CONTRACT:
/// 1) Input geometries should be of signed integer type (e.g. i32, i64).
/// 2) Input geometries should never intersect except at their endpoints.
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
/// ```
/// # use boostvoronoi_core::geometry::{Point,Line};
/// # use boostvoronoi_core::builder::Builder;
///
/// type I = i32; // this is the integer input type
/// type F = f64; // this is the float output type (circle event coordinates)
///
/// // Points should be unique. Points should not intersect lines
/// let p = vec!(Point{x:9_i32, y:10});
/// // Lines may only intersect at the endpoints.
/// let s = vec!(Line::new(Point{x:10_i32, y:11}, Point{x:12, y:13}));
/// let result = Builder::<I, F>::default()
///     // You will have to keep track of the input geometry. it will be referenced as
///     // input geometry indices in the output.
///     // `with_vertices()` and `with_segments()` accepts iterators of anything that implements
///     // `Into()` for `Point` and `Line`
///     .with_vertices(p.iter()).unwrap()
///     .with_segments(s.iter()).unwrap()
///     // this will generate a the list of cells, edges and circle events (aka vertices)
///     .build().unwrap();
/// ```
pub struct Builder<I: InputType, F: OutputType> {
    pub(crate) site_events_: Vec<VSE::SiteEvent<I, F>>,
    circle_events_: VC::CircleEventQueue,
    end_points_: BinaryHeap<VEP::EndPointPair<I>>,
    pub(crate) beach_line_: VB::BeachLine<I, F>,
    // The number of input sites if points and segments are counted as one.
    // (segments generates two site events so we can't use the length of the list)
    index_: usize,
    segments_added_: bool, // make sure eventual vertices are added before segments
    #[cfg(feature = "console_debug")]
    debug_circle_counter_: isize, // Just for debugging purposes
    #[cfg(feature = "console_debug")]
    debug_site_counter_: isize, // Just for debugging purposes
}

impl<I: InputType, F: OutputType> Default for Builder<I, F> {
    fn default() -> Self {
        Self {
            site_events_: Vec::new(),
            beach_line_: VB::BeachLine::default(),
            index_: 0,
            end_points_: BinaryHeap::new(),
            circle_events_: VC::CircleEventQueue::default(),
            #[cfg(feature = "console_debug")]
            debug_circle_counter_: 0,
            #[cfg(feature = "console_debug")]
            debug_site_counter_: 0,
            segments_added_: false,
        }
    }
}

impl<I: InputType, F: OutputType> Builder<I, F> {
    /// Inserts vertices.
    /// This should be done before inserting segments.
    /// This method accepts iterators of anything that implements `Into<boostvoronoi::geometry::Point>`
    pub fn with_vertices<T, IT>(mut self, vertices: T) -> Result<Self, BvError>
    where
        T: Iterator<Item = IT>,
        IT: Copy + Into<Point<I>>,
    {
        if self.segments_added_ {
            return Err(BvError::VerticesGoesFirst(
                "Vertices should be added before segments".to_string(),
            ));
        }
        for v in vertices {
            let mut s = VSE::SiteEvent::<I, F>::new(VSE::Site::Point(v.into()), self.index_);
            s.or_source_category(&VD::ColorBits::SINGLE_POINT__BIT);
            self.site_events_.push(s);
            self.index_ += 1;
        }
        Ok(self)
    }

    /// Inserts segments.
    /// This should be done after inserting vertices.
    /// This method accepts iterators of anything that implements `Into<boostvoronoi::geometry::Line>`
    pub fn with_segments<T, IT>(mut self, segments: T) -> Result<Self, BvError>
    where
        T: Iterator<Item = IT>,
        IT: Copy + Into<Line<I>>,
    {
        type Cb = VD::ColorBits;
        for s in segments {
            let line: Line<I> = s.into();

            let mut s1 = VSE::SiteEvent::<I, F>::new(VSE::Site::Point(line.start), self.index_);
            s1.or_source_category(&Cb::SEGMENT_START_POINT__BIT);

            let mut s2 = VSE::SiteEvent::<I, F>::new(VSE::Site::Point(line.end), self.index_);
            s2.or_source_category(&Cb::SEGMENT_END_POINT__BIT);

            self.site_events_.push(s1);
            self.site_events_.push(s2);
            let site = VSE::Site::from(line);
            let s3 = if VP::PointComparisonPredicate::<I>::point_comparison(line.start, line.end) {
                let mut s3 = VSE::SiteEvent::<I, F>::new(site, self.index_);
                s3.or_source_category(&Cb::INITIAL_SEGMENT);
                s3
            } else {
                let mut s3 = VSE::SiteEvent::<I, F>::new(site.reverse(), self.index_);
                s3.or_source_category(&Cb::REVERSE_SEGMENT);
                s3
            };
            self.site_events_.push(s3);
            self.index_ += 1;
        }
        self.segments_added_ = true;
        Ok(self)
    }

    /// Run sweep-line algorithm and fill output data structure.
    pub fn build(mut self) -> Result<VD::Diagram<F>, BvError> {
        let mut output: VD::Diagram<F> = VD::Diagram::<F>::new(self.site_events_.len());

        let mut site_event_iterator_: VSE::SiteEventIndexType = self.init_sites_queue();

        tln!("********************************************************************************");
        tln!("->build()");
        tln!("********************************************************************************");

        self.init_beach_line(&mut site_event_iterator_, &mut output)?;
        #[cfg(feature = "console_debug")]
        let mut i = 0;

        // The algorithm stops when there are no events to process.
        while !self.circle_events_.is_empty() || (site_event_iterator_ != self.site_events_.len()) {
            #[cfg(feature = "console_debug")]
            {
                tln!("################################################");
                tln!(
                    "loop:{} circle_events_:{} num_vertices:{} beach_line:{} debug_site_counter:{} debug_circle_counter:{}",
                    i,self.circle_events_.len(),
                    output.num_vertices(),
                    self.beach_line_.len(),
                    self.debug_site_counter_,
                    self.debug_circle_counter_,
                );
                tln!("################################################");
                if i >= 8 {
                    self.beach_line_.dbgpa_compat_(&self.circle_events_)?;
                    print!("");
                }
                i += 1;
            }
            if self.circle_events_.is_empty() {
                self.process_site_event(&mut site_event_iterator_, &mut output)?;
            } else if site_event_iterator_ == self.site_events_.len() {
                self.process_circle_event(&mut output)?;
            } else if VP::EventComparisonPredicate::event_comparison_bif::<I, F>(
                &self.site_events_[site_event_iterator_],
                // we checked with !is_empty(), unwrap is safe
                &self.circle_events_.peek().unwrap().0.get(),
            ) {
                self.process_site_event(&mut site_event_iterator_, &mut output)?;
            } else {
                self.process_circle_event(&mut output)?;
            }

            self.circle_events_.pop_inactive_at_top()?;
        }

        self.beach_line_.clear();

        // Finish the diagram construction.
        output.finish();
        Ok(output)
    }

    pub(crate) fn init_sites_queue(&mut self) -> VSE::SiteEventIndexType {
        // Sort site events.
        self.site_events_
            .sort_by(VP::EventComparisonPredicate::event_comparison_ii::<I, F>);

        // Remove duplicates.
        self.site_events_.dedup();

        // Index sites.
        for (cur, s) in self.site_events_.iter_mut().enumerate() {
            s.set_sorted_index(cur);
        }
        #[cfg(feature = "console_debug")]
        {
            tln!("post dedup:");
            self.debug_site_events();
        }
        // Init site iterator.
        let site_event_iterator_: VSE::SiteEventIndexType = 0;
        site_event_iterator_
    }

    pub(crate) fn init_beach_line(
        &mut self,
        site_event_iterator_: &mut VSE::SiteEventIndexType,
        output: &mut VD::Diagram<F>,
    ) -> Result<(), BvError> {
        if self.site_events_.is_empty() {
            return Ok(());
        }
        if self.site_events_.len() == 1 {
            // Handle single site event case.
            let site = &self.site_events_[0];
            output.process_single_site_(site); //site.sorted_index(), site.initial_index(), site.source_category());
            *site_event_iterator_ += 1;
        } else {
            let mut skip = 0;

            while *site_event_iterator_ < self.site_events_.len()
                && VP::Predicates::is_vertical_points::<I, F>(
                    self.site_events_[*site_event_iterator_].point0(),
                    self.site_events_[0].point0(),
                )
                && VP::Predicates::is_vertical_site::<I, F>(
                    &self.site_events_[*site_event_iterator_],
                )
            {
                *site_event_iterator_ += 1;
                skip += 1;
            }

            if skip == 1 {
                // Init beach line with the first two sites.
                self.init_beach_line_default(site_event_iterator_, output)?;
            } else {
                // Init beach line with collinear vertical sites.
                self.init_beach_line_collinear_sites(site_event_iterator_, output)?;
            }
        }
        Ok(())
    }

    /// Init beach line with the two first sites.
    /// The first site is always a point.
    fn init_beach_line_default(
        &mut self,
        site_event_iterator_: &mut VSE::SiteEventIndexType,
        output: &mut VD::Diagram<F>,
    ) -> Result<(), BvError> {
        // Get the first and the second site event.
        let first = *site_event_iterator_ - 1;
        let first = self.site_events_[first];
        let second = *site_event_iterator_;
        let second = self.site_events_[second];
        tln!("insert_new_arc init_beach_line_default");
        let _ = self.insert_new_arc(
            first,
            first,
            second,
            self.beach_line_.last_position()?,
            output,
        );

        // The second site was already processed. Move the iterator.
        *site_event_iterator_ += 1;
        Ok(())
    }

    /// Init beach line with collinear sites.
    fn init_beach_line_collinear_sites(
        &mut self,
        site_event_iterator_: &VSE::SiteEventIndexType,
        output: &mut VD::Diagram<F>,
    ) -> Result<(), BvError> {
        let mut it_first: VSE::SiteEventIndexType = 0;
        let mut it_second: VSE::SiteEventIndexType = 1;
        while it_second != *site_event_iterator_ {
            let first = &self.site_events_[it_first];
            let second = &self.site_events_[it_second];

            // Create a new beach line node.
            let new_node_key = VB::BeachLineNodeKey::<I, F>::new_2(*first, *second);

            // Update the output.
            let edge = output.insert_new_edge_2_(*first, *second).0;

            // Insert a new bisector into the beach line.
            #[cfg(feature = "console_debug")]
            let _ = self.beach_line_.insert(
                self.beach_line_.last_position()?,
                new_node_key,
                VB::BeachLineNodeData::new_1(edge),
                &self.circle_events_,
            );
            #[cfg(not(feature = "console_debug"))]
            let _ = self.beach_line_.insert(
                self.beach_line_.last_position()?,
                new_node_key,
                Some(VB::BeachLineNodeData::new_1(edge)),
            );
            // Update iterators.
            it_first += 1;
            it_second += 1;
        }
        Ok(())
    }

    #[inline(always)]
    fn deactivate_circle_event(
        &mut self,
        beachline_ptr: &PIterator<VB::BeachLineNodeKey<I, F>, VB::BeachLineNodeDataType>,
    ) -> Result<(), BvError> {
        if let Some(mut node_cell) = beachline_ptr.get_v()?.get() {
            self.circle_events_
                .deactivate(node_cell.get_circle_event_id());

            // make sure there are no dangling references to deactivated circle events..
            let _ = node_cell.set_circle_event_id(None);
            beachline_ptr.get_v()?.set(Some(node_cell));
        }
        Ok(())
    }

    pub(crate) fn process_site_event(
        &mut self,
        site_event_iterator_: &mut VSE::SiteEventIndexType,
        output: &mut VD::Diagram<F>,
    ) -> Result<(), BvError> {
        #[cfg(feature = "console_debug")]
        {
            //tln!("->process_site_event");
            if self.debug_site_counter_ >= 109 {
                print!("");
            }
            //self.beach_line_.debug_print_all();
            //}
            self.debug_site_counter_ += 1;
        }
        let (mut right_it, last_index) = {
            // Get next site event to process.
            let site_event = self
                .site_events_
                .get(*site_event_iterator_)
                .ok_or_else(|| {
                    BvError::InternalError(format!(
                        "Could not get a site event from list. {}{}",
                        file!(),
                        line!()
                    ))
                })?;
            tln!("processing site:{}", site_event); //dbg!(&site_event);

            // Move site iterator.
            let mut last_index = *site_event_iterator_ + 1;

            // If a new site is an end point of some segment,
            // remove temporary nodes from the beach line data structure.
            if !site_event.is_segment() {
                while !self.end_points_.is_empty()
                    // we checked with !is_empty(), unwrap is safe
                    && self.end_points_.peek().unwrap().site() == site_event.point0()
                {
                    // we checked with !is_empty(), unwrap is safe
                    let b_it = self.end_points_.pop().unwrap();
                    let mut b_it = cpp_map::PIterator::new_2(
                        Rc::clone(&self.beach_line_.beach_line_),
                        b_it.beachline_index().0,
                    );
                    #[cfg(feature = "console_debug")]
                    {
                        self.beach_line_.dbgpa_compat_(&self.circle_events_)?;
                        t!("erasing beach_line:");
                        self.beach_line_.dbgpa_compat_node_(
                            &b_it.get_k()?,
                            &b_it.get_v()?,
                            &self.circle_events_,
                        )?;
                    }
                    let _ = b_it.remove_current()?;
                    #[cfg(feature = "console_debug")]
                    {
                        self.beach_line_.dbgpa_compat_(&self.circle_events_)?;
                    }
                }
            } else {
                let mut last = self.site_events_.get(last_index);
                while last_index < self.site_events_.len()
                    && last.is_some()
                    // we checked with is_some(), unwrap is safe
                    && last.unwrap().is_segment()
                    && last.unwrap().point0() == site_event.point0()
                {
                    last_index += 1;
                    last = self.site_events_.get(last_index);
                }
            }

            // Find the node in the binary search tree with left arc
            // lying above the new site point.

            let new_key = VB::BeachLineNodeKey::<I, F>::new_1(*site_event);
            tln!("\nbeach_line_.lower_bound key  : {:?} ", site_event);
            let right_it = self.beach_line_.lower_bound(new_key)?;
            #[cfg(feature = "console_debug")]
            {
                if right_it.is_ok()? {
                    tln!("beach_line_.lower_bound found: {:?}: \n", right_it.get_k()?);
                } else {
                    tln!("beach_line_.lower_bound found: Nothing (not an error)\n");
                }
            }

            (right_it, last_index)
        };
        #[cfg(feature = "console_debug")]
        {
            let debug_range = 999999;
            if self.debug_circle_counter_ >= debug_range
                && self.debug_circle_counter_ <= debug_range + 2
            {
                self.beach_line_.dbgpa_compat_(&self.circle_events_)?;
            }
        }
        while *site_event_iterator_ != last_index {
            // site_event is a copy of the the event site_event_iterator_ is indexing
            let mut site_event = self.site_events_[*site_event_iterator_];
            let mut left_it = right_it.clone();

            // Do further processing depending on the above node position.
            // For any two neighboring nodes the second site of the first node
            // is the same as the first site of the second node.
            if !right_it.is_ok()? {
                // The above arc corresponds to the second arc of the last node.
                // Move the iterator to the last node.

                left_it = self.beach_line_.last()?;

                // Get the second site of the last node
                let site_arc = *(left_it.get_k()?.right_site());

                // todo: insert_new_arc should return a new pointer
                // Insert new nodes into the beach line. Update the output.
                let right_it_idx = self.insert_new_arc(
                    site_arc,
                    site_arc,
                    site_event,
                    right_it.current(),
                    output,
                )?;
                right_it = self.beach_line_.get_pointer(right_it_idx)?;

                // Add a candidate circle to the circle event queue.
                // There could be only one new circle event formed by
                // a new bisector and the one on the left.
                {
                    let key = left_it.get_k()?;
                    self.activate_circle_event(
                        *(key.left_site()),
                        *(key.right_site()),
                        site_event,
                        right_it_idx,
                    )?;
                }
            } else if right_it.is_at_head()? {
                // The above arc corresponds to the first site of the first node.
                let site_arc = *right_it.get_k()?.left_site();

                // Insert new nodes into the beach line. Update the output.
                left_it = {
                    let new_key_id = self.insert_new_arc(
                        site_arc,
                        site_arc,
                        site_event,
                        right_it.current(),
                        output,
                    )?;
                    self.beach_line_.get_pointer(new_key_id)?
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
                    *(right_it.get_k()?.left_site()),
                    *(right_it.get_k()?.right_site()),
                    VB::BeachLineIndex(right_it.current()),
                )?;
                right_it = left_it;
            } else {
                let (site_arc2, site3) = {
                    // The above arc corresponds neither to the first,
                    // nor to the last site in the beach line.
                    let key = right_it.get_k()?;
                    (*key.left_site(), *key.right_site())
                };

                // Remove the candidate circle from the event queue.
                self.deactivate_circle_event(&right_it)?;

                // emulate --left_site
                left_it.prev()?;

                let site_arc1 = *(left_it.get_k()?.right_site());
                let site1 = *(left_it.get_k()?.left_site());

                // Insert new nodes into the beach line. Update the output.
                let new_node_it = self.insert_new_arc(
                    site_arc1,
                    site_arc2,
                    site_event,
                    right_it.current(),
                    output,
                )?;

                // Add candidate circles to the circle event queue.
                // There could be up to two circle events formed by
                // a new bisector and the one on the left or right.
                self.activate_circle_event(site1, site_arc1, site_event, new_node_it)?;

                // If the site event is a segment, update its direction.
                if site_event.is_segment() {
                    let _ = site_event.inverse();
                }

                self.activate_circle_event(
                    site_event,
                    site_arc2,
                    site3,
                    VB::BeachLineIndex(right_it.current()),
                )?;
                //right_it = new_node_it;
                right_it = self.beach_line_.get_pointer(new_node_it)?;
            }
            *site_event_iterator_ += 1;
        }
        Ok(())
    }

    /// In general case circle event is made of the three consecutive sites
    /// that form two bisectors in the beach line data structure.
    /// Let circle event sites be A, B, C, two bisectors that define
    /// circle event are (A, B), (B, C). During circle event processing
    /// we remove (A, B), (B, C) and insert (A, C). As beach line comparison
    /// works correctly only if one of the nodes is a new one we remove
    /// (B, C) bisector and change (A, B) bisector to the (A, C). That's
    /// why we use replace_key() there and take all the responsibility that
    /// map data structure keeps correct ordering.
    pub(crate) fn process_circle_event(
        &mut self,
        output: &mut VD::Diagram<F>,
    ) -> Result<(), BvError> {
        #[cfg(feature = "console_debug")]
        {
            self.debug_circle_counter_ += 1;
        }
        #[cfg(feature = "ce_corruption_check")]
        self.circle_events_.ce_corruption_check();

        // Get the topmost circle event.
        let e = self.circle_events_.top()?.ok_or_else(|| {
            BvError::InternalError(format!(
                "No topmost circle event found. {}:{}",
                file!(),
                line!()
            ))
        })?;
        let circle_event = e.0.get();
        tln!("processing:CE{:?}", circle_event);

        if !self
            .circle_events_
            .is_active(circle_event.get_index().unwrap())
        {
            return Err(BvError::InternalError(format!(
                "Internal error, the topmost circle event should be active. {}:{}",
                file!(),
                line!()
            )));
        }
        let mut it_first = self
            .beach_line_
            .get_pointer(e.beach_line_index().ok_or_else(|| {
                BvError::InternalError(format!(
                    "No beachline index found for circle event. {}:{}",
                    file!(),
                    line!()
                ))
            })?)?;
        let mut it_last = it_first.clone();
        #[cfg(feature = "console_debug")]
        {
            t!("it_first:");
            self.beach_line_.dbgpa_compat_node_(
                &it_first.get_k()?,
                &it_first.get_v()?,
                &self.circle_events_,
            )?;
        }
        // Get the C site.
        let site3 = *it_first.get_k()?.right_site();

        // Get the half-edge corresponding to the second bisector - (B, C).
        let bisector2 = it_first
            .get_v()?
            .get()
            .ok_or_else(|| {
                BvError::InternalError(format!("bisector2.is_none() {}:{}", file!(), line!()))
            })?
            .edge_id();

        // Get the half-edge corresponding to the first bisector - (A, B).
        it_first.prev()?;

        let bisector1 = it_first
            .get_v()?
            .get()
            .ok_or_else(|| {
                BvError::InternalError(format!("bisector1.is_none() {}:{}", file!(), line!()))
            })?
            .edge_id();

        // Get the A site.
        let site1 = *it_first.get_k()?.left_site();
        #[allow(clippy::suspicious_operation_groupings)]
        let site3 = if !site1.is_segment() && site3.is_segment() && site3.point1() == site1.point0()
        {
            *site3.clone().inverse()
        } else {
            site3
        };

        // Change the (A, B) bisector node to the (A, C) bisector node.
        {
            let it_first_key_before = it_first.get_k()?;
            let it_first_key_after = {
                let mut tmp = it_first_key_before;
                tmp.set_right_site(&site3);
                tmp
            };
            #[cfg(feature = "console_debug")]
            {
                self.beach_line_.dbgpa_compat_(&self.circle_events_)?;
                //t!("pre: it_first:");
                //self.beach_line_
                //    .dbgpa_compat_node_(&it_first.get_k()?, &it_first.get_v()?, &self.circle_events_)?;
                t!("replace key ");
                self.beach_line_.dbgpa_compat_node_(
                    &it_first_key_before,
                    &it_first.get_v()?,
                    &self.circle_events_,
                )?;
                t!("with:       ");
                self.beach_line_.dbgpa_compat_node_(
                    &it_first_key_after,
                    &it_first.get_v()?,
                    &self.circle_events_,
                )?;
            }

            it_first.replace_key(it_first_key_after)?;

            #[cfg(feature = "console_debug")]
            {
                //t!("post: it_first:");
                //self.beach_line_
                //    .dbgpa_compat_node_(&it_first.get_k()?, &it_first.get_v()?, &self.circle_events_)?;

                self.beach_line_.dbgpa_compat_(&self.circle_events_)?;
                self.beach_line_.dbgp_all_cmp_();
                tln!();
            }
        }

        // Insert the new bisector into the beach line.
        {
            let edge = output
                .insert_new_edge_5_(site1, site3, circle_event, bisector1, bisector2)
                .0;
            let data = if let Some(ref mut node) = it_first.get_v()?.get() {
                let _ = node.set_edge_id(edge);
                //tln!("Updated node data: {:?} new edge:{}", node, edge.0);
                *node
            } else {
                //tln!("Created new node data: new edge:{}", edge.0);
                VB::BeachLineNodeData::new_1(edge)
            };
            let _ = it_first.get_v()?.replace(Some(data));
        }
        #[cfg(feature = "console_debug")]
        {
            self.beach_line_.dbgpa_compat_(&self.circle_events_)?;
            t!("pre: it_first:");
            self.beach_line_.dbgpa_compat_node_(
                &it_first.get_k()?,
                &it_first.get_v()?,
                &self.circle_events_,
            )?;
            t!("pre: it_last:");
            self.beach_line_.dbgpa_compat_node_(
                &it_last.get_k()?,
                &it_last.get_v()?,
                &self.circle_events_,
            )?;
            t!("erasing beach_line:");
            self.beach_line_.dbgpa_compat_node_(
                &it_last.get_k()?,
                &it_last.get_v()?,
                &self.circle_events_,
            )?;
        }
        // Remove the (B, C) bisector node from the beach line.
        if it_first.current() == it_last.current() {
            // todo: is this correct?
            println!("------- it_first.next()?");
            it_first.next()?;
        }
        #[cfg(feature = "console_debug")]
        assert_ne!(it_first.current(), it_last.current());

        let _ = it_last.remove_current()?;

        #[cfg(feature = "console_debug")]
        {
            t!("post: it_first:");
            self.beach_line_.dbgpa_compat_node_(
                &it_first.get_k()?,
                &it_first.get_v()?,
                &self.circle_events_,
            )?;
        }
        //self.beach_line_.erase(it_last.get_k()?.index())?;
        #[cfg(feature = "console_debug")]
        self.beach_line_.dbgpa_compat_(&self.circle_events_)?;

        let mut it_last = it_first.clone();

        // Pop the topmost circle event from the event queue.
        self.circle_events_.pop_and_destroy()?;

        // Check new triplets formed by the neighboring arcs
        // to the left for potential circle events.
        if !self.beach_line_.is_empty() && !it_first.is_at_head()? {
            self.circle_events_.deactivate(
                it_first
                    .get_v()?
                    .get()
                    .and_then(|x| x.get_circle_event_id()),
            );
            //--it_first;
            it_first.prev()?;

            let site_l1 = *it_first.get_k()?.left_site();
            self.activate_circle_event(
                site_l1,
                site1,
                site3,
                VB::BeachLineIndex(it_last.current()),
            )?;
        }

        // Check the new triplet formed by the neighboring arcs
        // to the right for potential circle events.

        it_last.next()?;

        if it_last.is_ok()? {
            let it_last_node = it_last.get_v()?;
            self.circle_events_
                .deactivate(it_last_node.get().and_then(|x| x.get_circle_event_id()));

            let site_r1 = *it_last.get_k()?.right_site();
            self.activate_circle_event(
                site1,
                site3,
                site_r1,
                VB::BeachLineIndex(it_last.current()),
            )?;
        }
        Ok(())
    }

    /// Insert new nodes into the beach line. Update the output.
    fn insert_new_arc(
        &mut self,
        site_arc1: VSE::SiteEvent<I, F>,
        site_arc2: VSE::SiteEvent<I, F>,
        site_event: VSE::SiteEvent<I, F>,
        position: usize,
        output: &mut VD::Diagram<F>,
    ) -> Result<VB::BeachLineIndex, BvError> {
        tln!(
            "->insert_new_arc(\n  site_arc1:{:?}\n  ,site_arc2:{:?}\n  ,site_event:{:?}",
            site_arc1,
            site_arc2,
            site_event
        );
        // Create two new bisectors with opposite directions.
        let new_left_node = VB::BeachLineNodeKey::<I, F>::new_2(site_arc1, site_event);
        let new_right_node =
            // Set correct orientation for the first site of the second node.
            if site_event.is_segment() {
                VB::BeachLineNodeKey::<I, F>::new_2(*site_event.clone().inverse(), site_arc2)
            } else {
                VB::BeachLineNodeKey::<I, F>::new_2(site_event, site_arc2)
            };

        tln!("new bl key:{:?}", new_right_node);
        // Update the output.
        let edges = output.insert_new_edge_2_(site_arc2, site_event);

        #[cfg(not(feature = "console_debug"))]
        let _ = self.beach_line_.insert(
            position,
            new_right_node,
            Some(VB::BeachLineNodeData::new_1(edges.1)),
        );
        #[cfg(feature = "console_debug")]
        let _ = self.beach_line_.insert(
            position,
            new_right_node,
            VB::BeachLineNodeData::new_1(edges.1),
            &self.circle_events_,
        )?;

        #[cfg(feature = "console_debug")]
        {
            self.beach_line_.dbgpa_compat_(&self.circle_events_)?;
            self.beach_line_.dbgp_all_cmp_();
            println!();
        }
        if site_event.is_segment() {
            // Update the beach line with temporary bisector, that will
            // disappear after processing site event corresponding to the
            // second endpoint of the segment site.
            let new_node =
                VB::BeachLineNodeKey::<I, F>::new_2(site_event, *site_event.clone().inverse());

            #[cfg(feature = "console_debug")]
            let (_, index) = self.beach_line_.insert_2(new_node, &self.circle_events_)?;
            #[cfg(not(feature = "console_debug"))]
            let (_, index) = self.beach_line_.insert_2(new_node)?;

            #[cfg(feature = "console_debug")]
            {
                self.beach_line_.dbgpa_compat_(&self.circle_events_)?;
                self.beach_line_.dbgp_all_cmp_();
                println!();
            }
            // Update the data structure that holds temporary bisectors.
            self.end_points_
                .push(VEP::EndPointPair::new(site_event.point1(), index));
        }
        let new_node_data = VB::BeachLineNodeData::new_1(edges.0);

        #[cfg(not(feature = "console_debug"))]
        {
            Ok(self
                .beach_line_
                .insert(position, new_left_node, Some(new_node_data))?
                .1)
        }
        #[cfg(feature = "console_debug")]
        {
            let rv = Ok(self
                .beach_line_
                .insert(position, new_left_node, new_node_data, &self.circle_events_)?
                .1);
            self.beach_line_.dbgpa_compat_(&self.circle_events_)?;
            self.beach_line_.dbgp_all_cmp_();
            println!();
            rv
        }
    }

    /// Add a new circle event to the event queue.
    /// bisector_node corresponds to the (site2, site3) bisector.
    fn activate_circle_event(
        &mut self,
        site1: VSE::SiteEvent<I, F>,
        site2: VSE::SiteEvent<I, F>,
        site3: VSE::SiteEvent<I, F>,
        bisector_node: VB::BeachLineIndex,
    ) -> Result<(), BvError> {
        // Check if the three input sites create a circle event.
        let c_event = Rc::new(VC::CircleEventCell::new(bisector_node));

        if VP::CircleFormationFunctor::circle_formation::<I, F>(&site1, &site2, &site3, &c_event) {
            // Add the new circle event to the circle events queue.
            // Update bisector's circle event iterator to point to the
            // new circle event in the circle event queue.
            tln!("added circle event:{:?}", c_event);

            let e = self.circle_events_.associate_and_push(c_event);
            {
                let b = self.beach_line_.get_node(&bisector_node)?;
                if let Some(mut bd) = b.1.get() {
                    let _ = bd.set_circle_event_id(Some(e.0.get().get_index().unwrap())); // make sure it is_some()
                    b.1.set(Some(bd));
                    #[cfg(feature = "console_debug")]
                    {
                        t!("with bisector_node: ");
                        self.beach_line_
                            .dbgpa_compat_node_(&b.0, &b.1, &self.circle_events_)?;
                    }
                } else {
                    return Err(BvError::InternalError(format!(
                        "activate_circle_event could not find node by key {}",
                        bisector_node.0
                    )));
                }
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    #[cfg(feature = "console_debug")]
    fn debug_site_events(&self) {
        tln!("Site event list:");
        for s in self.site_events_.iter() {
            tln!("{}", s);
        }
        tln!("#site_events={}", self.site_events_.len());
    }
}
