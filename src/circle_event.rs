// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code..

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

use super::beach_line as VB;
use super::extended_exp_fpt as EX;

use super::OutputType;
use crate::BvError;
use ordered_float::OrderedFloat;
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;
use std::ops::Neg;
use std::rc::Rc;

/// Type-checked placeholder for usize
/// Hopefully rust zero cost abstractions will flatten this out.
#[derive(Copy, Clone)]
pub struct CircleEventIndex(pub usize);

impl CircleEventIndex {
    fn increment(&mut self) -> &Self {
        self.0 += 1;
        self
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
#[derive(Copy, Clone)]
pub struct CircleEvent {
    index_: Option<CircleEventIndex>, // the list index inside CircleEventQueue
    center_x_: OrderedFloat<f64>,
    center_y_: OrderedFloat<f64>,
    lower_x_: OrderedFloat<f64>,
    beach_line_index_: Option<VB::BeachLineIndex>, //beach_line_iterator in C++
    is_site_point_: bool,
}

impl fmt::Debug for CircleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(x:{:.12},y:{:.12},lx:{:.12})",
            self.center_x_, self.center_y_, self.lower_x_,
        )
    }
}

impl Default for CircleEvent {
    fn default() -> Self {
        Self {
            index_: None, // do i really have to put this in an option?
            center_x_: OrderedFloat(0_f64),
            center_y_: OrderedFloat(0_f64),
            lower_x_: OrderedFloat(0_f64),
            beach_line_index_: None,
            is_site_point_: false,
        }
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CircleEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.lower_x() != other.lower_x() {
            if self.lower_x() < other.lower_x() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else if self.y() < other.y() {
            Ordering::Less
        } else if self.y() == other.y() {
            //tln!("cmp self.lx():{:.12}, other.lx:{:.12} si:{:?}", self.lower_x(), other.lower_x(), self._index);
            //tln!("cmp self.y():{:.12}, other.y:{:.12} oi:{:?}", self.y(), other.y(), self._index);
            //tln!("cmp self.idx:{:?}, other.idx:{:?}", self.index_, other.index_);
            if let Some(self_index) = self.index_ {
                if let Some(other_index) = other.index_ {
                    self_index.0.cmp(&other_index.0)
                } else {
                    //todo: fix
                    Ordering::Greater
                }
            } else {
                //todo: fix
                Ordering::Greater
            }
        } else {
            //todo: fix
            Ordering::Greater
        }
    }
}

/// Wrapper object that lets me implement Ord on a Cell<CircleEvent<O>>
#[derive(Clone)]
pub struct CircleEventC(pub Cell<CircleEvent>, pub(crate) Option<VB::BeachLineIndex>);

impl fmt::Debug for CircleEventC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CE{:?}", self.0.get())
    }
}

impl CircleEventC {
    pub(crate) fn new_1(c: CircleEvent) -> Rc<Self> {
        let cc = Self(Cell::new(c), Some(c.beach_line_index_.unwrap())); // todo
        Rc::<Self>::new(cc)
    }

    /// sets the coordinates inside the Cell.
    pub(crate) fn set_3_ext(
        &self,
        x: EX::ExtendedExponentFpt<f64>,
        y: EX::ExtendedExponentFpt<f64>,
        lower_x: EX::ExtendedExponentFpt<f64>,
    ) {
        let mut selfc = self.0.get();
        selfc.set_3_raw(x.d(), y.d(), lower_x.d());
        self.0.set(selfc);
    }

    /// sets the coordinates inside the Cell.
    pub(crate) fn set_3_raw(&self, x: f64, y: f64, lower_x: f64) {
        let mut selfc = self.0.get();
        selfc.set_3_raw(x, y, lower_x);
        self.0.set(selfc);
    }

    /// sets the x coordinates inside the Cell.
    pub(crate) fn set_x_xf(&self, x: EX::ExtendedExponentFpt<f64>) {
        let mut selfc = self.0.get();
        let _ = selfc.set_x_raw(x.d());
        self.0.set(selfc);
    }

    /// sets the y coordinate inside the Cell.
    pub(crate) fn set_y_xf(&self, y: EX::ExtendedExponentFpt<f64>) {
        let mut selfc = self.0.get();
        let _ = selfc.set_raw_y(y.d());
        self.0.set(selfc);
    }

    /// sets the y coordinate inside the Cell.
    pub(crate) fn set_lower_x_xf(&self, x: EX::ExtendedExponentFpt<f64>) {
        let mut selfc: CircleEvent = self.0.get();
        let _ = selfc.set_raw_lower_x(x.d());
        self.0.set(selfc);
    }

    pub(crate) fn set_is_site_point(&self) {
        let mut selfc = self.0.get();
        selfc.is_site_point_ = true;
        self.0.set(selfc)
    }
    #[allow(dead_code)]
    pub(crate) fn is_site_point(&self) -> bool {
        self.0.get().is_site_point_
    }
}

impl PartialOrd for CircleEventC {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CircleEventC {
    fn cmp(&self, other: &Self) -> Ordering {
        let cell_self = self.0.get();
        let cell_other = other.0.get();
        cell_self.cmp(&cell_other)
    }
}

impl PartialEq for CircleEventC {
    fn eq(&self, other: &Self) -> bool {
        let c_self = self.0.get();
        let c_other = other.0.get();

        c_self.center_x_ == c_other.center_x_
            && c_self.center_y_ == c_other.center_y_
            && c_self.lower_x_ == c_other.lower_x_
    }
}

impl Eq for CircleEventC {}

impl CircleEvent
where
    f64: OutputType + Neg<Output = f64>,
{
    pub(crate) fn new_1(bech_line_index: VB::BeachLineIndex) -> CircleEvent {
        Self {
            center_x_: OrderedFloat(0_f64),
            center_y_: OrderedFloat(0_f64),
            lower_x_: OrderedFloat(0_f64),
            beach_line_index_: Some(bech_line_index),
            index_: None,
            is_site_point_: false,
        }
    }

    pub(crate) fn get_index(&self) -> Option<CircleEventIndex> {
        self.index_
    }

    pub(crate) fn set_index(&mut self, index: CircleEventIndex) -> &mut Self {
        self.index_ = Some(index);
        self
    }

    #[allow(dead_code)]
    pub(crate) fn x(&self) -> OrderedFloat<f64> {
        self.center_x_
    }

    pub(crate) fn x_as_xf(&self) -> EX::ExtendedExponentFpt<f64> {
        EX::ExtendedExponentFpt::<f64>::from(self.center_x_.into_inner())
    }

    pub(crate) fn raw_x(&self) -> f64 {
        self.center_x_.into_inner()
    }

    #[allow(dead_code)]
    pub(crate) fn set_x(&mut self, x: OrderedFloat<f64>) -> &mut Self {
        self.center_x_ = x;
        self
    }

    pub(crate) fn set_x_raw(&mut self, x: f64) -> &mut Self {
        self.center_x_ = OrderedFloat(x);
        self
    }

    pub(crate) fn y(&self) -> OrderedFloat<f64> {
        self.center_y_
    }

    /// convert self.y() to ExtendedExponentFpt
    #[allow(dead_code)]
    pub(crate) fn y_as_ext(&self) -> EX::ExtendedExponentFpt<f64> {
        EX::ExtendedExponentFpt::<f64>::from(self.center_y_.into_inner())
    }

    pub(crate) fn raw_y(&self) -> f64 {
        self.center_y_.into_inner()
    }

    #[allow(dead_code)]
    pub(crate) fn set_y(&mut self, y: OrderedFloat<f64>) -> &mut Self {
        self.center_y_ = y;
        self
    }

    pub(crate) fn set_raw_y(&mut self, y: f64) -> &mut Self {
        self.center_y_ = OrderedFloat(y);
        self
    }

    #[inline(always)]
    pub(crate) fn lower_x(&self) -> OrderedFloat<f64> {
        self.lower_x_
    }

    #[allow(dead_code)]
    pub(crate) fn set_lower_x(&mut self, x: OrderedFloat<f64>) -> &mut Self {
        self.lower_x_ = x;
        self
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn raw_lower_x(&self) -> f64 {
        self.lower_x_.into_inner()
    }

    #[inline(always)]
    pub(crate) fn set_raw_lower_x(&mut self, x: f64) -> &mut Self {
        self.lower_x_ = OrderedFloat(x);
        self
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn lower_y(&self) -> OrderedFloat<f64> {
        self.center_y_
    }

    #[inline(always)]
    pub(crate) fn set_3_raw(&mut self, x: f64, y: f64, lower_x: f64) {
        let _ = self.set_x_raw(x);
        let _ = self.set_raw_y(y);
        let _ = self.set_raw_lower_x(lower_x);
    }

    #[inline(always)]
    pub(crate) fn is_site_point(&self) -> bool {
        self.is_site_point_
    }
}

pub type CircleEventType = Rc<CircleEventC>;

/// Event queue data structure, holds circle events.
/// During algorithm run, some of the circle events disappear (become
/// inactive). Priority queue data structure doesn't support
/// iterators (there is no direct ability to modify its elements).
/// Instead list is used to store all the circle events and priority queue
/// of the iterators to the list elements is used to keep the correct circle
/// events ordering. (todo: this comment text is from c++, convert to rust)
pub(crate) struct CircleEventQueue {
    // circle events sorted by order
    c_: BTreeSet<CircleEventType>,
    // circle events sorted by id
    c_list_: ahash::AHashMap<usize, CircleEventType>,
    c_list_next_free_index_: CircleEventIndex,
    inactive_circle_ids_: yabf::Yabf, // Circle events turned inactive
}

impl Default for CircleEventQueue {
    fn default() -> CircleEventQueue {
        Self {
            c_: BTreeSet::new(),
            c_list_: ahash::AHashMap::new(),
            c_list_next_free_index_: CircleEventIndex(0),
            inactive_circle_ids_: yabf::Yabf::default(),
        }
    }
}

impl CircleEventQueue {
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.c_.is_empty()
    }

    /// the real first() for +nightly builds
    #[inline(always)]
    #[cfg(feature = "map_first_last")]
    pub(crate) fn peek(&self) -> Option<&CircleEventType> {
        self.c_.first()
    }

    /// simulated first() for +stable builds
    #[cfg(not(feature = "map_first_last"))]
    #[inline]
    pub(crate) fn peek(&self) -> Option<&CircleEventType> {
        // super inefficient implementation of 'first()' for +stable
        self.c_.iter().next()
    }

    /// the real pop_firs() for +nightly builds
    #[inline(always)]
    #[cfg(feature = "map_first_last")]
    fn pop_first(&mut self) -> Option<CircleEventType> {
        self.c_.pop_first()
    }

    /// simulated pop_firs() for +stable builds
    #[cfg(not(feature = "map_first_last"))]
    #[inline(always)]
    fn pop_first(&mut self) -> Option<CircleEventType> {
        if let Some(item) = self.c_.iter().next().cloned() {
            #[cfg(feature = "console_debug")]
            assert!(self.c_.remove(&item));
            #[cfg(not(feature = "console_debug"))]
            let _ = self.c_.remove(&item);
            Some(item)
        } else {
            None
        }
    }

    pub(crate) fn pop_inactive_at_top(&mut self) -> Result<(), BvError> {
        //let size_b4 = self.c_.len();

        while !self.is_empty() {
            if let Some(peek) = self.peek() {
                if let Some(peek) = peek.0.get().get_index() {
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
        // todo: remove this when stable
        if self.c_list_.len() != self.c_.len() {
            return Err(BvError::InternalError(format!(
                "The two circle event lists should always be of the same size. {}:{}",
                file!(),
                line!()
            )));
        }
        Ok(())
    }

    ///
    /// was named pop in C++, but it was never used to actually get the item, only to destroy it
    ///
    pub(crate) fn pop_and_destroy(&mut self) -> Result<(), BvError> {
        if let Some(circle) = self.pop_first() {
            if let Some(circle_id) = circle.0.get().index_ {
                let _ = self.c_list_.remove(&circle_id.0);
                let _ = self.inactive_circle_ids_.set_bit(circle_id.0, true);
            } else {
                return Err(BvError::InternalError(format!(
                    "circle event lists corruption, circle event id {:?} not found {}:{}",
                    circle.0.get().index_,
                    file!(),
                    line!()
                )));
            }
        }
        // todo: remove this when stable
        if self.c_list_.len() != self.c_.len() {
            return Err(BvError::InternalError(format!(
                "The two circle event lists should always be of the same size. {}:{}",
                file!(),
                line!()
            )));
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn clear(&mut self) {
        self.c_.clear();
        self.c_list_.clear();
        self.inactive_circle_ids_ = yabf::Yabf::default()
    }

    /// Take ownership of the circle event,
    /// Update index
    /// Insert Rc ref into the list
    /// Insert Rc wrapped object in self.c_
    /// return a Rc ref of the inserted element
    pub(crate) fn associate_and_push(&mut self, cc: CircleEventType) -> Rc<CircleEventC> {
        {
            let mut c = cc.0.get();
            let _ = c.set_index(self.c_list_next_free_index_);
            cc.0.set(c);
        }

        let _ = self
            .c_list_
            .insert(self.c_list_next_free_index_.0, cc.clone());
        let _ = self.c_list_next_free_index_.increment();
        let _ = self.c_.insert(cc.clone());
        cc
    }

    #[inline(always)]
    pub(crate) fn is_active(&self, circle_event_id: CircleEventIndex) -> bool {
        !self.inactive_circle_ids_.bit(circle_event_id.0)
    }

    pub(crate) fn deactivate(&mut self, circle_event_id: Option<CircleEventIndex>) {
        #[cfg(not(feature = "console_debug"))]
        if let Some(circle_event_id) = circle_event_id {
            let _ = self.inactive_circle_ids_.set_bit(circle_event_id.0, true);
        }
        #[cfg(feature = "console_debug")]
        if let Some(circle_event_id) = circle_event_id {
            if !self.inactive_circle_ids_.bit(circle_event_id.0) {
                if self.c_list_.contains_key(&circle_event_id.0) {
                    tln!("deactivate {:?}", self.c_list_[&circle_event_id.0]);
                } else {
                    tln!("circle {} not present", circle_event_id);
                }
                let _ = self.inactive_circle_ids_.set_bit(circle_event_id.0, true);
            }
        }
    }

    pub(crate) fn top(&self) -> Result<Option<&CircleEventType>, BvError> {
        let c = self.peek();
        if let Some(cc) = c {
            let id = cc.0.get().index_.unwrap();
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
        if let Some(ce) = self.c_list_.get(&cei.0) {
            print!("{:?}", ce);
        } else {
            print!("{}: not found", cei);
        }
    }

    /// Returns the number of circle events (both active and inactive)
    /// Only used by test code.
    #[cfg(any(feature = "test", feature = "console_debug"))]
    pub(crate) fn len(&self) -> usize {
        self.c_.len()
    }
}
