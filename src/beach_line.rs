// Boost.Polygon library detail/robust_fpt.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history of C++ code.

// Ported from C++ boost 1.76.0 to Rust in 2020/2021 by Eadf (github.com/eadf)

//! The data structures needed for the beachline.
#[cfg(test)]
mod test2;
#[cfg(test)]
mod test3;
#[cfg(test)]
mod tests1;

use super::circle_event as VC;
use super::diagram as VD;
use super::predicate as VP;
use super::site_event as VSE;

use super::{InputType, OutputType};
#[allow(unused_imports)]
use crate::predicate::NodeComparisonPredicate;
use crate::BvError;
#[allow(unused_imports)]
use crate::{t, tln};
#[allow(unused_imports)]
use itertools::Itertools;
use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::Neg;
use std::rc::Rc;

/// debug utility function, prints beach line index
#[allow(dead_code)]
#[cfg(feature = "console_debug")]
#[inline(always)]
pub(crate) fn debug_print_bli_id(value: Option<BeachLineIndex>) -> String {
    if let Some(value) = value {
        value.to_string()
    } else {
        String::from("-")
    }
}

/// Type-checked placeholder for usize
/// Hopefully rust zero cost abstractions will flatten this out.
#[derive(Copy, Clone)]
pub(crate) struct BeachLineIndex(pub(crate) usize);

/*
impl BeachLineIndex {
    fn increment(&mut self) -> &Self {
        self.0 += 1;
        self
    }
}*/

impl fmt::Display for BeachLineIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for BeachLineIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BeachLineIndex({})", self.0)
    }
}

pub type BeachLineNodeDataType = Rc<Cell<Option<BeachLineNodeData>>>;

/// Container for BeachLineNodeKey and BeachLineNodeDataType.
/// Has a priority queue and indexed list for BeachLineNodeKey.
///
/// The ordering of beach_line_vec_ is intentionally reversed, pop() will return the last element.
/// Before this change the beach-line ordering was unpredictable.
/// TODO: C++ map does not overwrite already existing (key,value) pairs.
/// TODO: Rust does the opposite
pub struct BeachLine<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    pub(crate) beach_line_:
        Rc<RefCell<cpp_map::LinkedList<BeachLineNodeKey<I, F>, BeachLineNodeDataType>>>,
}

impl<I, F> Default for BeachLine<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    fn default() -> Self {
        Self {
            beach_line_: Rc::from(RefCell::from(cpp_map::LinkedList::default())),
            // Index 0 is reserved for loose keys (lower bound tests, and unit tests)
            //beach_line_vec_: VecMap::default(),
        }
    }
}

impl<I, F> BeachLine<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        self.beach_line_.borrow().len()
    }

    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        self.beach_line_.borrow().is_empty()
    }

    #[inline(always)]
    pub(crate) fn get_pointer(
        &self,
        new_key_id: BeachLineIndex,
    ) -> Result<cpp_map::PIterator<BeachLineNodeKey<I, F>, BeachLineNodeDataType>, BvError> {
        Ok(cpp_map::PIterator::new_2(Rc::clone(&self.beach_line_), new_key_id.0))
    }

    #[inline(always)]
    /// Returns a pointer to the last beach-line item or None
    pub(crate) fn last(
        &self,
    ) -> Result<cpp_map::PIterator<BeachLineNodeKey<I, F>, BeachLineNodeDataType>, BvError> {
        let tail = self.beach_line_.borrow().tail();
        Ok(cpp_map::PIterator::new_2(Rc::clone(&self.beach_line_), tail))
    }

    #[inline(always)]
    /// Returns the last position of the beach-line list
    pub(crate) fn last_position(&self) -> Result<usize, BvError> {
        Ok(self.beach_line_.borrow().tail())
    }

    /// updates the node_index of the key, inserts it into the list and
    /// returns a copy of it
    #[cfg(not(feature = "console_debug"))]
    pub(crate) fn insert(
        &mut self,
        position: usize,
        mut key: BeachLineNodeKey<I, F>,
        data: Option<BeachLineNodeData>,
    ) -> Result<BeachLineNodeKey<I, F>, BvError> {
        key.node_index_ = BeachLineIndex(self.beach_line_.borrow().next_free_index());

        let node = Rc::new(Cell::new(data));
        //let _ = self.beach_line_vec_.insert(key.node_index_.0, (key, node));
        let _ = self
            .beach_line_
            .borrow_mut()
            .ordered_insert_pos(key, node, position, false)?;
        Ok(key)
    }

    /// updates the node_index of the key, inserts it into the list and
    /// returns a copy of it
    #[cfg(feature = "console_debug")]
    pub(crate) fn insert(
        &mut self,
        position: usize,
        mut key: BeachLineNodeKey<I, F>,
        data: BeachLineNodeData,
        _ce: &VC::CircleEventQueue,
    ) -> Result<BeachLineNodeKey<I, F>, BvError> {
        key.node_index_ = BeachLineIndex(self.beach_line_.borrow().next_free_index());
        let data_node = Rc::from(Cell::from(Some(data)));
        {
            let i = self
                .beach_line_
                .borrow_mut()
                .ordered_insert_pos(key, data_node, position, false)?;
            assert_eq!(i, key.node_index_.0);
        }
        //tln!("inserted beach_line with key:{}", key.node_index_.0);
        t!("inserted beach_line:");
        self.dbgpa_compat_node_(&key, _ce)?;
        Ok(key)
    }

    /// updates the node_index of the key, inserts it into the list and
    /// returns a copy of it
    #[cfg(feature = "console_debug")]
    pub(crate) fn insert_2(
        &mut self,
        mut key: BeachLineNodeKey<I, F>,
        _ce: &VC::CircleEventQueue,
    ) -> Result<BeachLineNodeKey<I, F>, BvError> {
        key.node_index_ = BeachLineIndex(self.beach_line_.borrow().next_free_index());
        let data_node = Rc::from(Cell::from(None));
        {
            let insert_index = self
                .beach_line_
                .borrow_mut()
                .ordered_insert(key, data_node, false)?;
            assert_eq!(insert_index, key.node_index_.0);
        }
        t!("inserted beach_line:");
        self.dbgpa_compat_node_(&key, _ce)?;
        Ok(key)
    }

    /// updates the node_index of the key, inserts it into the list and
    /// returns a copy of it
    #[cfg(not(feature = "console_debug"))]
    pub(crate) fn insert_2(
        &mut self,
        mut key: BeachLineNodeKey<I, F>,
    ) -> Result<BeachLineNodeKey<I, F>, BvError> {
        key.node_index_ = BeachLineIndex(self.beach_line_.borrow().next_free_index());

        let node = Rc::new(Cell::new(None));
        let _ = self
            .beach_line_
            .borrow_mut()
            .ordered_insert(key, node, false)?;
        Ok(key)
    }

    /// Clear the beach line list
    pub fn clear(&mut self) {
        #[cfg(feature = "console_debug")]
        tln!(
            "The capacity of the beachline was {:?}",
            self.beach_line_.borrow().capacity()
        );

        self.beach_line_.borrow_mut().clear();
    }

    /// mapping: BeachLineNodeIndexType->(BeachLineNodeKey,BeachLineNodeDataType)
    pub(crate) fn get_node(
        &self,
        beachline_index: &BeachLineIndex,
    ) -> Result<(BeachLineNodeKey<I, F>, BeachLineNodeDataType), BvError> {
        let bl_borrow = self.beach_line_.borrow();
        let node = bl_borrow.get_kv(beachline_index.0);

        if node.is_err() {
            println!("Failed to retrieve beach line key : {}", beachline_index.0);
            //panic!();
            return Err(BvError::InternalError(format!(
                "Tried to retrieve a beach line node that doesn't exist. Id:{}. {}:{}",
                beachline_index.0,
                file!(),
                line!()
            )));
        }
        let node = node?;

        Ok((*node.0, Rc::clone(node.1)))
    }

    #[inline(always)]
    /// Returns the first beach line element in the container whose key is not considered to go
    /// before position (i.e., either it is equivalent or goes after).
    /// Returns None if no data is found
    pub(crate) fn lower_bound(
        &self,
        key: BeachLineNodeKey<I, F>,
    ) -> Result<cpp_map::PIterator<BeachLineNodeKey<I, F>, BeachLineNodeDataType>, BvError> {
        Ok(cpp_map::PIterator::lower_bound(Rc::clone(&self.beach_line_), key, false)?)
    }

    #[allow(dead_code)]
    #[cfg(feature = "console_debug")]
    pub(crate) fn debug_cmp_all(&self, key: BeachLineNodeKey<I, F>) {
        for (i, v) in self.beach_line_.borrow().iter().rev().enumerate() {
            print!("#{}:", i);
            let _rv = VP::NodeComparisonPredicate::<I, F>::node_comparison_predicate(v, &key);
        }
    }

    #[cfg(feature = "beachline_corruption_check")]
    /// check if each item in the btree is actually retrievable
    pub(crate) fn corruption_check(&self) -> Result<(), BvError> {
        for (blk, _) in self.beach_line_.iter().rev() {
            if self.beach_line_.get(blk).is_none() {
                eprintln!("Could not re-find the beach-line key {:?}", blk);
                return Err(BvError::InternalError(format!(
                    "The beach-line is corrupted, could not re-find beach-line key: {:?} {}:{}",
                    blk,
                    file!(),
                    line!()
                )));
            }
        }
        Ok(())
    }

    #[cfg(feature = "console_debug")]
    #[allow(dead_code)]
    pub(crate) fn debug_print_all(&self) -> Result<(), BvError> {
        tln!();
        tln!("beach_line.len()={}", self.beach_line_.borrow().len());
        for (i, node) in self.beach_line_.borrow().iter().rev().enumerate() {
            let id = node.node_index_;
            t!(
                "beach_line{} L:{:?},R:{:?}",
                i,
                &node.left_site(),
                &node.right_site()
            );

            #[cfg(not(feature = "cpp_compat_debug"))]
            t!(", id={:?}", id);
            if let Some(data) = self.get_node(&id)?.1.get() {
                if let Some(circle_event) = data.circle_event_ {
                    t!(" -> CircleEvent:{}", circle_event);
                } else {
                    t!(" -> CircleEvent:-");
                }
                t!(", edge:{:?}", data.edge_);
            } else {
                t!(" temporary bisector");
            }
            tln!();
        }
        tln!();
        Ok(())
    }

    #[cfg(feature = "console_debug")]
    pub(crate) fn dbgpa_compat_(&self, ce: &VC::CircleEventQueue) -> Result<(), BvError> {
        tln!("-----beach_line----{}", self.beach_line_.borrow().len());
        for (i, node) in self.beach_line_.borrow().iter().enumerate() {
            t!("#{}:", i);
            self.dbgpa_compat_node_(&node, ce)?;
        }
        tln!();
        Ok(())
    }

    /*
    pub(crate) fn dgbpa_dump_and_cmp_(&self, key: &BeachLineNodeKey<I, F>) {

        println!("-----beach_line----{}", self.beach_line_.len());
        println!("Looking for {:?} in the beach_line", key);
        let found = self.beach_line_.get(key);
        println!(
            "Found {:?} cmp1=node.partial_cmp(key).unwrap() cmp2=key.partial_cmp(node).unwrap()",
            found
        );
        for (i, (node, _id)) in self.beach_line_.iter().enumerate() {
            let cmp1 = node.partial_cmp(key);
            let ncmp1 = NodeComparisonPredicate::node_comparison_predicate(key, node);
            let ncmp2 = NodeComparisonPredicate::node_comparison_predicate(node, key);

            print!(
                "#{}: key:{:?}, cmp1:{:?}, ncmp1:{:?}, ncmp2:{:?} ccmp:{}",
                i,
                node,
                cmp1.unwrap(),
                ncmp1,
                ncmp2,
                match (ncmp1, ncmp2) {
                    (false, false) => "Equal",
                    (false, true) => "Less",
                    (true, _) => "Greater",
                }
            );
            if cmp1.unwrap() == Ordering::Equal {
                println!("  <----- THIS IS THE PROBLEM, 'get()' could not find it, but it's here!!")
            } else {
                println!()
            };
        }
        println!();
        let mut it1 = self.beach_line_.iter().enumerate();
        for it2_v in self.beach_line_.iter().enumerate().skip(1) {
            let it1_v = it1.next().unwrap();
            print!(
                "key(#{}).partial_cmp(key(#{})) == {:?}",
                it1_v.0,
                it2_v.0,
                it1_v.1 .0.partial_cmp(it2_v.1 .0).unwrap()
            );
            println!(
                "\tkey(#{}).partial_cmp(key(#{})) == {:?}",
                it2_v.0,
                it1_v.0,
                it2_v.1 .0.partial_cmp(it1_v.1 .0).unwrap()
            );
        }
    }*/

    #[cfg(feature = "console_debug")]
    pub(crate) fn dbgp_all_cmp_(&self) {
        let _iter1 = self.beach_line_.borrow();
        let mut it1 = _iter1.iter().enumerate();
        for it2_v in self.beach_line_.borrow().iter().enumerate().skip(1) {
            let it1_v = it1.next().unwrap();
            t!(
                "key(#{}).partial_cmp(key(#{})) == {:?}",
                it1_v.0,
                it2_v.0,
                it1_v.1.partial_cmp(it2_v.1).unwrap()
            );
            tln!(
                "\tkey(#{}).partial_cmp(key(#{})) == {:?}",
                it2_v.0,
                it1_v.0,
                it2_v.1.partial_cmp(it1_v.1).unwrap()
            );
        }
    }

    #[cfg(feature = "console_debug")]
    pub(crate) fn dbgpa_compat_node_(
        &self,
        node: &BeachLineNodeKey<I, F>,
        ce: &VC::CircleEventQueue,
    ) -> Result<(), BvError> {
        let id = &node.index();
        t!("L:{:?},R:{:?}", &node.left_site(), &node.right_site(),);
        if let Some(data) = self.get_node(id)?.1.get() {
            if let Some(_circle_event) = data.circle_event_ {
                if ce.is_active(_circle_event) {
                    t!(" -> CircleEvent: ");
                    ce.dbg_ce(_circle_event);
                } else {
                    t!(" -> CircleEvent=-"); // this is what the c++ code does, should print "inactive"
                }
            } else {
                t!(" -> CircleEvent=-");
            }
            t!(",e={}", data.edge_.0);
        } else {
            t!(" Temporary bisector");
        }
        //#[cfg(not(feature = "cpp_compat_debug"))]
        // print!(" id={}", id);
        tln!();
        Ok(())
    }
}

impl<I, F> fmt::Debug for BeachLine<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,)?;
        for (index, node) in self.beach_line_.borrow().iter().enumerate() {
            writeln!(f, "{}: {:?}", index, node)?;
        }
        writeln!(f,)
    }
}

/// Represents a bisector node made by two arcs that correspond to the left
/// and right sites. Arc is defined as a curve with points equidistant from
/// the site and from the sweepline. If the site is a point then arc is
/// a parabola, otherwise it's a line segment. A segment site event will
/// produce different bisectors based on its direction.
/// In general case two sites will create two opposite bisectors. That's
/// why the order of the sites is important to define the unique bisector.
/// The one site is considered to be newer than the other one if it was
/// processed by the algorithm later (has greater index).
#[derive(Copy, Clone)]
pub struct BeachLineNodeKey<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    left_site_: VSE::SiteEvent<I, F>,
    right_site_: VSE::SiteEvent<I, F>,
    node_index_: BeachLineIndex,
}

impl<I, F> fmt::Debug for BeachLineNodeKey<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "cpp_compat_debug")]
        {
            write!(f, "L:{:?},R:{:?}", &self.left_site(), &self.right_site())
        }
        #[cfg(not(feature = "cpp_compat_debug"))]
        {
            write!(f, "L:{:?},R:{:?}", &self.left_site(), &self.right_site())?;
            write!(f, ", id={:?}", self.node_index_.0)
        }
    }
}

impl<I, F> BeachLineNodeKey<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    /// Constructs degenerate bisector, used to search an arc that is above
    /// the given site. The input to the constructor is the new site point.
    pub fn new_1(new_site: VSE::SiteEvent<I, F>) -> Self {
        Self {
            left_site_: new_site,
            right_site_: new_site,
            node_index_: BeachLineIndex(0), // will be populated by Beachline::insert
        }
    }

    /// Constructs a new bisector. The input to the constructor is the two
    /// sites that create the bisector. The order of sites is important.
    pub fn new_2(left_site: VSE::SiteEvent<I, F>, right_site: VSE::SiteEvent<I, F>) -> Self {
        Self {
            left_site_: left_site,
            right_site_: right_site,
            node_index_: BeachLineIndex(0), // will be populated by Beachline::insert
        }
    }

    pub(crate) fn left_site(&self) -> &VSE::SiteEvent<I, F> {
        &self.left_site_
    }

    pub(crate) fn right_site(&self) -> &VSE::SiteEvent<I, F> {
        &self.right_site_
    }

    pub(crate) fn set_right_site(&mut self, site: &VSE::SiteEvent<I, F>) {
        self.right_site_ = *site; // Copy
    }

    /// returns the index
    pub(crate) fn index(&self) -> BeachLineIndex {
        self.node_index_
    }

    #[cfg(test)]
    /// Sets the key index (only needed for tests)
    pub(crate) fn set_index(mut self, new_index: BeachLineIndex) -> Self {
        self.node_index_ = new_index;
        self
    }
}

impl<I, F> PartialOrd for BeachLineNodeKey<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I, F> Ord for BeachLineNodeKey<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        //if self.node_index_.0 == other.node_index_.0 {
        //    return Ordering::Equal;
        //}
        if VP::NodeComparisonPredicate::<I, F>::node_comparison_predicate(self, other) {
            Ordering::Less
        } else if VP::NodeComparisonPredicate::<I, F>::node_comparison_predicate(other, self) {
            Ordering::Greater
        } else {
            //self.node_index_.0.cmp(&other.node_index_.0)
            Ordering::Equal
        }
    }
}

impl<I, F> PartialEq for BeachLineNodeKey<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    fn eq(&self, other: &Self) -> bool {
        self.left_site_ == other.left_site_ && self.right_site_ == other.right_site_
    }
}

impl<I, F> Eq for BeachLineNodeKey<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
}

impl<I, F> Hash for BeachLineNodeKey<I, F>
where
    I: InputType + Neg<Output = I>,
    F: OutputType + Neg<Output = F>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.left_site_.hash(state);
        self.right_site_.hash(state);
    }
}

/// Represents edge data structure from the Voronoi output, that is
/// associated as a value with beach line bisector in the beach
/// line. Contains pointer to the circle event in the circle event
/// queue if the edge corresponds to the right bisector of the circle event.
/// Todo! this should be rust:ified and made into an Enum
#[derive(Copy, Clone, Debug)]
pub struct BeachLineNodeData {
    circle_event_: Option<VC::CircleEventIndex>,
    edge_: VD::EdgeIndex,
}

impl BeachLineNodeData {
    pub fn new_1(new_edge: VD::EdgeIndex) -> Self {
        Self {
            circle_event_: None,
            edge_: new_edge,
        }
    }
    /*
    fn new_2(circle: Option<VC::CircleEventIndex>, new_edge: VD::VoronoiEdgeIndex) -> Self {
        Self {
            circle_event_: circle,
            edge_: new_edge,
        }
    }*/

    pub fn get_circle_event_id(&self) -> Option<VC::CircleEventIndex> {
        self.circle_event_
    }

    pub(crate) fn set_circle_event_id(
        &mut self,
        circle_event: Option<VC::CircleEventIndex>,
    ) -> &mut Self {
        self.circle_event_ = circle_event;
        self
    }

    pub(crate) fn edge_id(&self) -> VD::EdgeIndex {
        self.edge_
    }

    pub(crate) fn set_edge_id(&mut self, new_edge: VD::EdgeIndex) -> &mut Self {
        self.edge_ = new_edge;
        self
    }
}
