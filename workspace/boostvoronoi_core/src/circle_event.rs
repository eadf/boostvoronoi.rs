// Boost.Polygon library detail/voronoi_structures.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code..

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

use crate::beach_line as VB;
use crate::extended_exp_fpt as EX;

#[cfg(feature = "console_debug")]
use crate::tln;
use crate::{BvError, GrowingVob, VobU32};
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;

/// Type-checked placeholder for usize
/// Hopefully rust zero cost abstractions will flatten this out.
#[derive(Copy, Clone)]
pub struct CircleEventIndex(pub usize);

impl CircleEventIndex {
    #[inline]
    fn increment(&mut self) {
        self.0 += 1;
    }
}

impl fmt::Display for CircleEventIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for CircleEventIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CircleEventIndex({})", self.0)
    }
}

/// Circle event type.
/// Occurs when the sweepline sweeps over the rightmost point of the Voronoi
/// circle (with the center at the intersection point of the bisectors).
/// Circle event is made of the two consecutive nodes in the beach line data
/// structure. In case another node was inserted during algorithm execution
/// between the given two nodes circle event becomes inactive.
/// Variables:
///   center_x_ - center x-coordinate;
///   center_y_ - center y-coordinate;
///   lower_x_ - leftmost x-coordinate;
/// NOTE: lower_y coordinate is always equal to center_y.
///
#[derive(Clone)]
pub struct CircleEvent {
    index_: Option<CircleEventIndex>, // the list index inside CircleEventQueue
    center_x_: f64,
    center_y_: f64,
    lower_x_: f64,
    beach_line_index_: Option<VB::BeachLineIndex>, //beach_line_iterator in C++
    is_site_point_: bool,
}

impl fmt::Debug for CircleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CE(x:{:.12},y:{:.12},lx:{:.12})",
            self.center_x_, self.center_y_, self.lower_x_,
        )
    }
}

impl PartialEq for CircleEvent {
    fn eq(&self, other: &Self) -> bool {
        //tln!("eq self.idx:{:?}, other.idx:{:?}", self.index_, other.index_);
        self.center_x_ == other.center_x_
            && self.center_y_ == other.center_y_
            && self.lower_x_ == other.lower_x_
        // todo! Should self.index_ and beach_line_index be in here too?
        // todo! ulp comparison, just like vertex_equality_predicate()?
        //&& self.index_  == other.index_
        //&& self.index_  == other.index_
    }
}

impl Eq for CircleEvent {}

impl PartialOrd for CircleEvent {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CircleEvent {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        match OrderedFloat(self.lower_x_).cmp(&OrderedFloat(other.lower_x_)) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            _ => match OrderedFloat(self.center_y_).cmp(&OrderedFloat(other.center_y_)) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => {
                    // self_lower_x_ == other_lower_x_ and self_center_y_ == other_center_y_
                    // Sort by reverse order of circle event id (highest value==youngest first)
                    // This implementation differ from C++, because i can only keep one
                    // unique circle-event in the circle-event BTreeSet
                    if let (Some(self_index), Some(other_index)) = (self.index_, other.index_) {
                        #[cfg(feature = "console_debug")]
                        println!(
                            "ce_id: {}.cmp({}) {:?}",
                            self_index.0,
                            other_index.0,
                            self_index.0.cmp(&other_index.0).reverse()
                        );
                        self_index.0.cmp(&other_index.0).reverse()
                    } else {
                        //todo: this should never happen, but still..
                        debug_assert!(false);
                        Ordering::Greater
                    }
                }
            },
        }
    }
}

impl CircleEvent {
    pub(crate) fn new(bech_line_index: VB::BeachLineIndex) -> CircleEvent {
        Self {
            center_x_: 0_f64,
            center_y_: 0_f64,
            lower_x_: 0_f64,
            beach_line_index_: Some(bech_line_index),
            index_: None,
            is_site_point_: false,
        }
    }

    #[inline]
    pub(crate) fn set_3_ext(
        &mut self,
        x: EX::ExtendedExponentFpt<f64>,
        y: EX::ExtendedExponentFpt<f64>,
        lower_x: EX::ExtendedExponentFpt<f64>,
    ) {
        self.set_3(x.d(), y.d(), lower_x.d());
    }

    pub(crate) fn get_index(&self) -> Option<CircleEventIndex> {
        self.index_
    }

    pub(crate) fn set_index(&mut self, index: CircleEventIndex) {
        self.index_ = Some(index);
    }

    #[allow(dead_code)]
    pub(crate) fn x(&self) -> f64 {
        self.center_x_
    }

    pub(crate) fn x_as_xf(&self) -> EX::ExtendedExponentFpt<f64> {
        EX::ExtendedExponentFpt::<f64>::from(self.center_x_)
    }

    pub(crate) fn set_x(&mut self, x: f64) -> &mut Self {
        self.center_x_ = x;
        self
    }

    pub(crate) fn y(&self) -> f64 {
        self.center_y_
    }

    /// convert self.y() to ExtendedExponentFpt
    #[allow(dead_code)]
    pub(crate) fn y_as_ext(&self) -> EX::ExtendedExponentFpt<f64> {
        EX::ExtendedExponentFpt::<f64>::from(self.center_y_)
    }

    pub(crate) fn set_y(&mut self, y: f64) -> &mut Self {
        self.center_y_ = y;
        self
    }

    #[inline(always)]
    pub(crate) fn lower_x(&self) -> f64 {
        self.lower_x_
    }

    #[inline(always)]
    pub(crate) fn set_lower_x(&mut self, x: f64) -> &mut Self {
        self.lower_x_ = x;
        self
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn lower_y(&self) -> f64 {
        self.center_y_
    }

    #[inline(always)]
    pub(crate) fn set_3(&mut self, x: f64, y: f64, lower_x: f64) {
        let _ = self.set_x(x);
        let _ = self.set_y(y);
        let _ = self.set_lower_x(lower_x);
    }

    #[inline(always)]
    pub(crate) fn is_site_point(&self) -> bool {
        self.is_site_point_
    }

    #[inline(always)]
    pub(crate) fn set_is_site_point(&mut self) {
        self.is_site_point_ = true
    }

    #[cfg(any(feature = "ce_corruption_check", feature = "console_debug"))]
    #[allow(dead_code)]
    pub fn dbg(&self) {
        println!("[{},{}]", self.x(), self.y());
    }

    #[inline]
    /// sets the x coordinates inside the Cell.
    pub(crate) fn set_x_xf(&mut self, x: EX::ExtendedExponentFpt<f64>) {
        let _ = self.set_x(x.d());
    }

    #[inline]
    /// sets the y coordinate inside the Cell.
    pub(crate) fn set_y_xf(&mut self, y: EX::ExtendedExponentFpt<f64>) {
        let _ = self.set_y(y.d());
    }

    #[inline]
    /// sets the y coordinate inside the Cell.
    pub(crate) fn set_lower_x_xf(&mut self, x: EX::ExtendedExponentFpt<f64>) {
        let _ = self.set_lower_x(x.d());
    }

    #[inline]
    /// Returns the beach line index
    pub(crate) fn beach_line_index(&self) -> Option<VB::BeachLineIndex> {
        self.beach_line_index_
    }
}

/// Event queue data structure, holds circle events.
/// During algorithm run, some of the circle events disappear (become
/// inactive). Priority queue data structure doesn't support
/// iterators (there is no direct ability to modify its elements).
/// Instead list is used to store all the circle events and priority queue
/// of the iterators to the list elements is used to keep the correct circle
/// events ordering.
// todo: this comment text is from c++, convert to rust
pub(crate) struct CircleEventQueue {
    /// circle events sorted by order
    /// Note that the `CircleEvent`s must be identical clones of the ones in `ce_by_id_`
    ce_by_order_: BTreeSet<CircleEvent>,
    /// circle events sorted by id
    /// Note that the `CircleEvent`s must be identical clones of the ones in `ce_by_order_`
    ce_by_id_: ahash::AHashMap<usize, CircleEvent>,
    c_list_next_free_index_: CircleEventIndex,
    inactive_circle_ids_: VobU32, // Circle events turned inactive
}

impl Default for CircleEventQueue {
    fn default() -> CircleEventQueue {
        Self {
            ce_by_order_: BTreeSet::new(),
            ce_by_id_: ahash::AHashMap::new(),
            c_list_next_free_index_: CircleEventIndex(0),
            inactive_circle_ids_: VobU32::fill(128),
        }
    }
}

impl CircleEventQueue {
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.ce_by_order_.is_empty()
    }

    /// the real first() for +nightly builds
    #[inline(always)]
    #[cfg(feature = "map_first_last")]
    pub(crate) fn peek(&self) -> Option<&CircleEvent> {
        self.ce_by_order_.first()
    }

    /// simulated first() for +stable builds
    #[cfg(not(feature = "map_first_last"))]
    #[inline(always)]
    pub(crate) fn peek(&self) -> Option<&CircleEvent> {
        // super inefficient implementation of 'first()' for +stable
        self.ce_by_order_.iter().next()
    }

    #[cfg(feature = "ce_corruption_check")]
    pub(crate) fn ce_corruption_check(&self) {
        if self.ce_by_order_.len() >= 2 {
            let mut iter = self.ce_by_order_.iter();
            let first_ce = iter.next().unwrap();
            let second_ce = iter.next().unwrap();

            if first_ce.cmp(second_ce) != Ordering::Less {
                println!("*************************************************");
                println!("topmost CE could just as well been the second CE.");
                println!("topmost CE :{:?}", first_ce);
                println!("second CE :{:?}", second_ce);
            }
        }
    }

    /// the real pop_firs() for +nightly builds
    #[inline(always)]
    #[cfg(feature = "map_first_last")]
    fn pop_first(&mut self) -> Option<CircleEvent> {
        self.ce_by_order_.pop_first()
    }

    /// simulated pop_firs() for +stable builds
    #[cfg(not(feature = "map_first_last"))]
    #[inline(always)]
    fn pop_first(&mut self) -> Option<CircleEvent> {
        if let Some(item) = self.ce_by_order_.iter().next().cloned() {
            #[cfg(feature = "console_debug")]
            debug_assert!(self.ce_by_order_.remove(&item));
            #[cfg(not(feature = "console_debug"))]
            let _ = self.ce_by_order_.remove(&item);
            Some(item)
        } else {
            None
        }
    }

    pub(crate) fn pop_inactive_at_top(&mut self) -> Result<(), BvError> {
        while !self.is_empty() {
            if let Some(peek) = self.peek() {
                if let Some(peek) = peek.get_index() {
                    //dbg!(peek);
                    if !self.is_active(peek) {
                        self.pop_and_destroy()?;
                        continue;
                    }
                } else {
                    return Err(BvError::InternalError(format!(
                        "Circle event had no id {}:{}",
                        file!(),
                        line!()
                    )));
                }
            }
            break;
        }
        debug_assert!({
            // todo: remove this when stable
            if self.ce_by_id_.len() != self.ce_by_order_.len() {
                return Err(BvError::InternalError(format!(
                    "The two circle event lists should always be of the same size. {}:{}",
                    file!(),
                    line!()
                )));
            };
            true
        });
        Ok(())
    }

    /// Was named pop in C++, but it was never used to actually get the item, only to destroy it
    pub(crate) fn pop_and_destroy(&mut self) -> Result<(), BvError> {
        if let Some(circle) = self.pop_first() {
            if let Some(circle_id) = circle.index_ {
                let _ = self.ce_by_id_.remove(&circle_id.0);
                self.inactive_circle_ids_.set_grow(circle_id.0, true);
            } else {
                return Err(BvError::InternalError(format!(
                    "circle event lists corruption, circle event id {:?} not found {}:{}",
                    circle.index_,
                    file!(),
                    line!()
                )));
            }
        }
        debug_assert!({
            // todo: remove this when stable
            if self.ce_by_id_.len() != self.ce_by_order_.len() {
                return Err(BvError::InternalError(format!(
                    "The two circle event lists should always be of the same size. {}:{}",
                    file!(),
                    line!()
                )));
            };
            true
        });
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn clear(&mut self) {
        self.ce_by_order_.clear();
        self.ce_by_id_.clear();
        self.inactive_circle_ids_ = {
            let mut cids = VobU32::fill(512);
            cids.resize(512, false);
            cids
        }
    }

    /// Take ownership of the circle event,
    /// Update index
    /// return the `CircleEventIndex` of the inserted element
    pub(crate) fn associate_and_push(&mut self, mut cc: CircleEvent) -> CircleEventIndex {
        let circle_event_id = self.c_list_next_free_index_;
        // set the correct index on the circle event
        cc.set_index(circle_event_id);
        let _ = self.ce_by_id_.insert(circle_event_id.0, cc.clone());

        let _ = self.ce_by_order_.insert(cc);
        self.c_list_next_free_index_.increment();
        circle_event_id
    }

    #[inline(always)]
    pub(crate) fn is_active(&self, circle_event_id: CircleEventIndex) -> bool {
        !self.inactive_circle_ids_.get_f(circle_event_id.0)
    }

    pub(crate) fn deactivate(&mut self, circle_event_id: Option<CircleEventIndex>) {
        #[cfg(not(feature = "console_debug"))]
        if let Some(circle_event_id) = circle_event_id {
            self.inactive_circle_ids_.set_grow(circle_event_id.0, true);
        }
        #[cfg(feature = "console_debug")]
        if let Some(circle_event_id) = circle_event_id {
            if !self.inactive_circle_ids_.get_f(circle_event_id.0) {
                if self.ce_by_id_.contains_key(&circle_event_id.0) {
                    tln!("deactivate {:?}", self.ce_by_id_[&circle_event_id.0]);
                } else {
                    tln!("circle {} not present", circle_event_id);
                }
                self.inactive_circle_ids_.set_grow(circle_event_id.0, true);
            }
        }
    }

    pub(crate) fn top(&self) -> Result<Option<&CircleEvent>, BvError> {
        let c = self.peek();
        if let Some(cc) = c {
            let id = cc.index_.unwrap();
            if !self.is_active(id) {
                return Err(BvError::InternalError(format!(
                    "Tried to use an inactive circle event {}, {}:{}",
                    id.0,
                    file!(),
                    line!()
                )));
            }
        }
        Ok(c)
    }

    #[cfg(feature = "console_debug")]
    pub(crate) fn dbg_ce(&self, cei: CircleEventIndex) {
        if let Some(ce) = self.ce_by_id_.get(&cei.0) {
            print!("{:?}", ce);
        } else {
            print!("{}: not found", cei);
        }
    }

    /// Returns the number of circle events (both active and inactive)
    /// Only used by test code.
    #[cfg(any(feature = "test", feature = "console_debug"))]
    pub(crate) fn len(&self) -> usize {
        self.ce_by_order_.len()
    }
}
