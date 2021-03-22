// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.75.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::beach_line as VB;
use super::extended_exp_fpt as EX;

use super::OutputType;
use ordered_float::OrderedFloat;
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;
use std::ops::Neg;
use std::rc::Rc;
use vec_map::VecMap;

// todo! make wrapper
pub type CircleEventIndexType = usize;

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
    index_: Option<CircleEventIndexType>, // the list index inside CircleEventQueue
    center_x_: OrderedFloat<f64>,
    center_y_: OrderedFloat<f64>,
    lower_x_: OrderedFloat<f64>,
    beach_line_index_: Option<VB::BeachLineIndex>, //beach_line_iterator in C++
    is_site_point: bool,
}

impl fmt::Debug for CircleEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rv = String::new();

        rv.push_str(
            format!(
                "(x:{:.12},y:{:.12},lx:{:.12})",
                self.center_x_, self.center_y_, self.lower_x_,
            )
            .as_str(),
        );
        write!(f, "{}", rv)
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
            is_site_point: false,
        }
    }
}

impl PartialEq for CircleEvent {
    fn eq(&self, other: &Self) -> bool {
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
            return if self.lower_x() < other.lower_x() {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        } else if self.y() < other.y() {
            return Ordering::Less;
        }
        Ordering::Greater
    }
}

/// Wrapper object that lets me implement Ord on a Cell<CircleEvent<O>>
#[derive(Clone)]
pub struct CircleEventC(pub Cell<CircleEvent>, pub(crate) Option<VB::BeachLineIndex>);

impl fmt::Debug for CircleEventC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rv = String::new();
        let cell = self.0.get();
        rv.push_str(format!("CE{:?}", cell).as_str());
        write!(f, "{}", rv)
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
        selfc.is_site_point = true;
        self.0.set(selfc)
    }
    #[allow(dead_code)]
    pub(crate) fn is_site_point(&self) -> bool {
        self.0.get().is_site_point
    }
}

impl PartialOrd for CircleEventC {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CircleEventC {
    fn cmp(&self, other: &Self) -> Ordering {
        let cself = self.0.get();
        let cother = other.0.get();
        cself.cmp(&cother)
    }
}

impl PartialEq for CircleEventC {
    fn eq(&self, other: &Self) -> bool {
        let cself = self.0.get();
        let cother = other.0.get();

        cself.center_x_ == cother.center_x_
            && cself.center_y_ == cother.center_y_
            && cself.lower_x_ == cother.lower_x_
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
            is_site_point: false,
        }
    }

    pub(crate) fn get_index(&self) -> Option<CircleEventIndexType> {
        self.index_
    }

    pub(crate) fn set_index(&mut self, index: CircleEventIndexType) -> &mut Self {
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
        self.is_site_point
    }
}

pub type CircleEventType = Rc<CircleEventC>;

/// Event queue data structure, holds circle events.
/// During algorithm run, some of the circle events disappear (become
/// inactive). Priority queue data structure doesn't support
/// iterators (there is no direct ability to modify its elements).
/// Instead list is used to store all the circle events and priority queue
/// of the iterators to the list elements is used to keep the correct circle
/// events ordering.
pub(crate) struct CircleEventQueue {
    c_: BTreeSet<CircleEventType>,
    c_list_: VecMap<CircleEventType>,
    c_list_next_free_index_: CircleEventIndexType,
    inactive_circle_ids_: yabf::Yabf, // Circle events turned inactive
}

impl Default for CircleEventQueue {
    fn default() -> CircleEventQueue {
        Self {
            c_: BTreeSet::new(),
            c_list_: VecMap::new(),
            c_list_next_free_index_: 0,
            inactive_circle_ids_: yabf::Yabf::default(),
        }
    }
}

impl CircleEventQueue {
    pub(crate) fn is_empty(&self) -> bool {
        self.c_.is_empty()
    }

    #[allow(dead_code)]
    pub(crate) fn len(&self) -> usize {
        //todo! assert_eq!(self.c_.len(), self.c_list_.len());
        assert!(self.c_list_.len() <= self.c_list_next_free_index_);
        self.c_.len()
    }

    pub(crate) fn peek(&self) -> Option<&CircleEventType> {
        // Todo: maybe iterate until a non-removed event is found
        self.c_.first()
    }

    pub(crate) fn pop_inactive_at_top(&mut self) {
        let size_b4 = self.c_.len();

        while !self.is_empty() {
            if let Some(peek) = self.c_.first() {
                if let Some(peek) = peek.0.get().get_index() {
                    //dbg!(peek);
                    if !self.is_active(peek) {
                        self.pop_and_destroy();
                        continue;
                    }
                } else {
                    panic!("there should be no id-less circle events in the heap");
                }
            }
            break;
        }
        if size_b4 != self.c_.len() {
            /*dbg!(
                "popped some circle events",
                size_b4,
                self.c_list_.len(),
                self.c_.len(),
                self.inactive_circle_ids_.len()
            );*/
        }
        if self.c_list_.len() != self.c_.len() {
            assert_eq!(self.c_list_.len(), self.c_.len());
        }
    }

    ///
    /// was named pop in C++, but it was never used to actually get the item, only to destroy it
    ///
    pub(crate) fn pop_and_destroy(&mut self) {
        if let Some(circle) = self.c_.pop_first() {
            if let Some(circle_id) = circle.0.get().index_ {
                let _ = self.c_list_.remove(circle_id);
                let _ = self.inactive_circle_ids_.set_bit(circle_id, true);
            } else {
                panic!("This should not have happened")
            }
        }
        if self.c_list_.len() != self.c_.len() {
            assert_eq!(self.c_list_.len(), self.c_.len());
        }
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
        //assert!(!self.c_.contains(&cc)); // todo: is this supposed to happen?
        {
            let mut c = cc.0.get();
            let _ = c.set_index(self.c_list_next_free_index_);
            cc.0.set(c);
        }

        let _ = self
            .c_list_
            .insert(self.c_list_next_free_index_, cc.clone());
        self.c_list_next_free_index_ += 1;
        let _ = self.c_.insert(cc.clone());
        cc
    }

    pub(crate) fn is_active(&self, circle_event_id: CircleEventIndexType) -> bool {
        !self.inactive_circle_ids_.bit(circle_event_id)
    }

    pub(crate) fn deactivate(&mut self, circle_event_id: Option<CircleEventIndexType>) {
        #[cfg(not(feature = "console_debug"))]
        if let Some(circle_event_id) = circle_event_id {
            let _ = self.inactive_circle_ids_.set_bit(circle_event_id, true);
        }
        #[cfg(feature = "console_debug")]
        if let Some(circle_event_id) = circle_event_id {
            if !self.inactive_circle_ids_.bit(circle_event_id) {
                if self.c_list_.contains_key(circle_event_id) {
                    println!("deactivate {:?}", self.c_list_[circle_event_id]);
                } else {
                    println!("circle {} not present", circle_event_id);
                }
                let _ = self.inactive_circle_ids_.set_bit(circle_event_id, true);
            }
        }
    }

    pub(crate) fn top(&self) -> Option<&CircleEventType> {
        let c = self.c_.first();
        if let Some(cc) = c {
            let id = cc.0.get().index_.unwrap();
            if !self.is_active(id) {
                panic!("Tried to use an inactive circle event");
            }
        }
        c
    }

    #[allow(dead_code)]
    pub(crate) fn dbg(&self) {
        for c in self.c_.iter() {
            println!("{:?}", c);
        }
    }
}
