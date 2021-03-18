// Boost.Polygon library voronoi_diagram.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

use super::beach_line as VB;
use super::robust_fpt as RF;

use super::OutputType;
use ordered_float::OrderedFloat;
use rb_tree::RBTree;
use std::cell::Cell;
use std::cmp::Ordering;
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
///   is_active_ - states whether circle event is still active.
/// NOTE: lower_y coordinate is always equal to center_y.
///

#[derive(Copy, Clone)]
pub struct CircleEvent<F2: OutputType + Neg<Output = F2>> {
    index_: Option<CircleEventIndexType>, // the list index inside CircleEventQueue
    center_x_: OrderedFloat<F2>,
    center_y_: OrderedFloat<F2>,
    lower_x_: OrderedFloat<F2>,
    beach_line_index_: Option<VB::BeachLineIndex>, //beach_line_iterator in C++
}

impl<F2> fmt::Debug for CircleEvent<F2>
where
    F2: OutputType + Neg<Output = F2>,
{
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

impl<F2: OutputType + Neg<Output = F2>> Default for CircleEvent<F2> {
    fn default() -> Self {
        Self {
            index_: None, // do i really have to put this in an option?
            center_x_: OrderedFloat(F2::zero()),
            center_y_: OrderedFloat(F2::zero()),
            lower_x_: OrderedFloat(F2::zero()),
            //is_active_: false,
            beach_line_index_: None,
        }
    }
}

impl<F2: OutputType + Neg<Output = F2>> PartialEq for CircleEvent<F2> {
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

impl<F2: OutputType + Neg<Output = F2>> Eq for CircleEvent<F2> {}

impl<F2: OutputType + Neg<Output = F2>> PartialOrd for CircleEvent<F2> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<F2: OutputType + Neg<Output = F2>> Ord for CircleEvent<F2> {
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
pub struct CircleEventC<F2: OutputType + Neg<Output = F2>>(
    pub Cell<CircleEvent<F2>>,
    pub(crate) Option<VB::BeachLineIndex>,
);

impl<F2> fmt::Debug for CircleEventC<F2>
where
    F2: OutputType + Neg<Output = F2>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rv = String::new();
        let cell = self.0.get();
        rv.push_str(format!("CE{:?}", cell).as_str());
        write!(f, "{}", rv)
    }
}

impl<F2: OutputType + Neg<Output = F2>> CircleEventC<F2> {
    pub(crate) fn new_1(c: CircleEvent<F2>) -> Rc<Self> {
        let cc = Self(Cell::new(c), Some(c.beach_line_index_.unwrap())); // todo
        Rc::<Self>::new(cc)
    }

    /// sets the coordinates inside the Cell.
    pub(crate) fn set_3_ext(
        &self,
        x: RF::ExtendedExponentFpt<f64>,
        y: RF::ExtendedExponentFpt<f64>,
        lower_x: RF::ExtendedExponentFpt<f64>,
    ) {
        let mut selfc = self.0.get();
        selfc.set_3_raw(
            num::cast::<f64, F2>(x.d()).unwrap(),
            num::cast::<f64, F2>(y.d()).unwrap(),
            num::cast::<f64, F2>(lower_x.d()).unwrap(),
        );
        self.0.set(selfc);
    }

    /// sets the coordinates inside the Cell.
    pub(crate) fn set_3_raw(&self, x: F2, y: F2, lower_x: F2) {
        let mut selfc = self.0.get();
        selfc.set_3_raw(x, y, lower_x);
        self.0.set(selfc);
    }

    /// sets the x coordinates inside the Cell.
    pub(crate) fn set_x_xf(&self, x: RF::ExtendedExponentFpt<f64>) {
        let mut selfc = self.0.get();
        let _ = selfc.set_x_raw(num::cast::<f64, F2>(x.d()).unwrap());
        self.0.set(selfc);
    }

    /// sets the y coordinate inside the Cell.
    pub(crate) fn set_y_xf(&self, y: RF::ExtendedExponentFpt<f64>) {
        let mut selfc = self.0.get();
        let _ = selfc.set_raw_y(num::cast::<f64, F2>(y.d()).unwrap());
        self.0.set(selfc);
    }

    /// sets the y coordinate inside the Cell.
    pub(crate) fn set_lower_x_xf(&self, x: RF::ExtendedExponentFpt<f64>) {
        let mut selfc: CircleEvent<F2> = self.0.get();
        let _ = selfc.set_raw_lower_x(num::cast::<f64, F2>(x.d()).unwrap());
        self.0.set(selfc);
    }
}

impl<F2: OutputType + Neg<Output = F2>> PartialOrd for CircleEventC<F2> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<F2: OutputType + Neg<Output = F2>> Ord for CircleEventC<F2> {
    fn cmp(&self, other: &Self) -> Ordering {
        let cself = self.0.get();
        let cother = other.0.get();
        cself.cmp(&cother)
    }
}

impl<F2: OutputType + Neg<Output = F2>> PartialEq for CircleEventC<F2> {
    fn eq(&self, other: &Self) -> bool {
        let cself = self.0.get();
        let cother = other.0.get();

        cself.center_x_ == cother.center_x_
            && cself.center_y_ == cother.center_y_
            && cself.lower_x_ == cother.lower_x_
    }
}

impl<F2: OutputType + Neg<Output = F2>> Eq for CircleEventC<F2> {}

impl<F2> CircleEvent<F2>
where
    F2: OutputType + Neg<Output = F2>,
{
    pub(crate) fn new_1(bech_line_index: VB::BeachLineIndex) -> CircleEvent<F2> {
        Self {
            center_x_: OrderedFloat(F2::zero()),
            center_y_: OrderedFloat(F2::zero()),
            lower_x_: OrderedFloat(F2::zero()),
            beach_line_index_: Some(bech_line_index),
            index_: None,
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
    pub(crate) fn x(&self) -> OrderedFloat<F2> {
        self.center_x_
    }

    pub(crate) fn x_as_xf(&self) -> RF::ExtendedExponentFpt<f64> {
        RF::ExtendedExponentFpt::<f64>::from(
            num::cast::<F2, f64>(self.center_x_.into_inner()).unwrap(),
        )
    }

    pub(crate) fn raw_x(&self) -> F2 {
        self.center_x_.into_inner()
    }

    #[allow(dead_code)]
    pub(crate) fn set_x(&mut self, x: OrderedFloat<F2>) -> &mut Self {
        self.center_x_ = x;
        self
    }

    pub(crate) fn set_x_raw(&mut self, x: F2) -> &mut Self {
        self.center_x_ = OrderedFloat(x);
        self
    }

    pub(crate) fn y(&self) -> OrderedFloat<F2> {
        self.center_y_
    }

    #[allow(dead_code)]
    pub(crate) fn y_as_ext(&self) -> RF::ExtendedExponentFpt<f64> {
        RF::ExtendedExponentFpt::<f64>::from(
            num::cast::<F2, f64>(self.center_y_.into_inner()).unwrap(),
        )
    }

    pub(crate) fn raw_y(&self) -> F2 {
        self.center_y_.into_inner()
    }

    #[allow(dead_code)]
    pub(crate) fn set_y(&mut self, y: OrderedFloat<F2>) -> &mut Self {
        self.center_y_ = y;
        self
    }

    pub(crate) fn set_raw_y(&mut self, y: F2) -> &mut Self {
        self.center_y_ = OrderedFloat(y);
        self
    }

    #[inline(always)]
    pub(crate) fn lower_x(&self) -> OrderedFloat<F2> {
        self.lower_x_
    }

    #[allow(dead_code)]
    pub(crate) fn set_lower_x(&mut self, x: OrderedFloat<F2>) -> &mut Self {
        self.lower_x_ = x;
        self
    }

    pub(crate) fn set_raw_lower_x(&mut self, x: F2) -> &mut Self {
        self.lower_x_ = OrderedFloat(x);
        self
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn lower_y(&self) -> OrderedFloat<F2> {
        self.center_y_
    }

    pub(crate) fn set_3_raw(&mut self, x: F2, y: F2, lower_x: F2) {
        let _ = self.set_x_raw(x);
        let _ = self.set_raw_y(y);
        let _ = self.set_raw_lower_x(lower_x);
    }
}

pub type CircleEventType<F2> = Rc<CircleEventC<F2>>;

/// Event queue data structure, holds circle events.
/// During algorithm run, some of the circle events disappear (become
/// inactive). Priority queue data structure doesn't support
/// iterators (there is no direct ability to modify its elements).
/// Instead list is used to store all the circle events and priority queue
/// of the iterators to the list elements is used to keep the correct circle
/// events ordering.
pub(crate) struct CircleEventQueue<F2>
where
    F2: OutputType + Neg<Output = F2>,
{
    c_: RBTree<CircleEventType<F2>>,
    c_list_: VecMap<CircleEventType<F2>>,
    c_list_next_free_index_: CircleEventIndexType,
    inactive_circle_ids_: yabf::Yabf, // Circle events turned inactive
}

impl<F2: OutputType + Neg<Output = F2>> Default for CircleEventQueue<F2> {
    fn default() -> CircleEventQueue<F2> {
        Self {
            c_: RBTree::new(),
            c_list_: VecMap::new(),
            c_list_next_free_index_: 0,
            inactive_circle_ids_: yabf::Yabf::default(),
        }
    }
}

impl<F2> CircleEventQueue<F2>
where
    F2: OutputType + Neg<Output = F2>,
{
    pub(crate) fn is_empty(&self) -> bool {
        self.c_.is_empty()
    }

    #[allow(dead_code)]
    pub(crate) fn len(&self) -> usize {
        //todo! assert_eq!(self.c_.len(), self.c_list_.len());
        assert!(self.c_list_.len() <= self.c_list_next_free_index_);
        self.c_.len()
    }

    pub(crate) fn peek(&self) -> Option<&CircleEventType<F2>> {
        // Todo: maybe iterate until a non-removed event is found
        self.c_.peek()
    }

    pub(crate) fn pop_inactive_at_top(&mut self) {
        let size_b4 = self.c_.len();

        while !self.is_empty() {
            if let Some(peek) = self.c_.peek() {
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
        if let Some(circle) = self.c_.pop() {
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
    pub(crate) fn associate_and_push(&mut self, cc: CircleEventType<F2>) -> Rc<CircleEventC<F2>> {
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

    pub(crate) fn top(&self) -> Option<&CircleEventType<F2>> {
        let c = self.c_.peek();
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
