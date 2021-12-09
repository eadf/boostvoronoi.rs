// Boost.Polygon library detail/voronoi_structures.hpp header file

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

use crate::circle_event as VC;
use crate::diagram as VD;
use crate::site_event as VSE;
#[cfg(feature = "console_debug")]
use crate::predicate as VP;

#[allow(unused_imports)]
use crate::predicate::node_comparison_predicate;
use crate::BvError;
#[allow(unused_imports)]
use crate::{t, tln};
use crate::{InputType, OutputType};
#[allow(unused_imports)]
use itertools::Itertools;
use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
#[allow(unused_imports)]
use std::ops::Bound::{Excluded, Included, Unbounded};
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
pub struct BeachLine<I: InputType, F: OutputType> {
    pub(crate) beach_line_:
        Rc<RefCell<cpp_map::LinkedList<BeachLineNodeKey<I, F>, BeachLineNodeDataType>>>,
}

impl<I: InputType, F: OutputType> Default for BeachLine<I, F> {
    fn default() -> Self {
        Self {
            beach_line_: Rc::from(RefCell::from(cpp_map::LinkedList::default())),
        }
    }
}

impl<I: InputType, F: OutputType> BeachLine<I, F> {
    #[cfg(feature = "console_debug")]
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
        Ok(cpp_map::PIterator::new_2(
            Rc::clone(&self.beach_line_),
            new_key_id.0,
        ))
    }

    #[inline(always)]
    /// Returns a pointer to the last beach-line item or None
    pub(crate) fn last(
        &self,
    ) -> Result<cpp_map::PIterator<BeachLineNodeKey<I, F>, BeachLineNodeDataType>, BvError> {
        Ok(cpp_map::PIterator::new_2(
            Rc::clone(&self.beach_line_),
            self.beach_line_.borrow().tail(),
        ))
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
        key: BeachLineNodeKey<I, F>,
        data: Option<BeachLineNodeData>,
    ) -> Result<(BeachLineNodeKey<I, F>, BeachLineIndex), BvError> {
        let node = Rc::new(Cell::new(data));
        let node_index = BeachLineIndex(
            self.beach_line_
                .borrow_mut()
                .ordered_insert_pos(key, node, position)?,
        );
        Ok((key, node_index))
    }

    /// updates the node_index of the key, inserts it into the list and
    /// returns a copy of it
    #[cfg(feature = "console_debug")]
    pub(crate) fn insert(
        &mut self,
        position: usize,
        key: BeachLineNodeKey<I, F>,
        data: BeachLineNodeData,
        _ce: &VC::CircleEventQueue,
    ) -> Result<(BeachLineNodeKey<I, F>, BeachLineIndex), BvError> {
        let node_data = Rc::from(Cell::from(Some(data)));
        let node_index = BeachLineIndex(self.beach_line_.borrow_mut().ordered_insert_pos(
            key,
            Rc::clone(&node_data),
            position,
        )?);

        //tln!("inserted beach_line with key:{}", key.node_index_.0);
        t!("inserted beach_line:");
        self.dbgpa_compat_node_(&key, &node_data, _ce)?;
        Ok((key, node_index))
    }

    /// inserts a new node key into the list and
    /// returns a copy of it
    #[cfg(feature = "console_debug")]
    pub(crate) fn insert_2(
        &mut self,
        key: BeachLineNodeKey<I, F>,
        _ce: &VC::CircleEventQueue,
    ) -> Result<(BeachLineNodeKey<I, F>, BeachLineIndex), BvError> {
        let data_node = Rc::from(Cell::from(None));
        let node_index = BeachLineIndex(
            self.beach_line_
                .borrow_mut()
                .ordered_insert(key, data_node)?,
        );
        t!("inserted beach_line:");
        self.dbgpa_compat_node_(&key, self.beach_line_.borrow().get_v(node_index.0)?, _ce)?;
        Ok((key, node_index))
    }

    /// updates the node_index of the key, inserts it into the list and
    /// returns a copy of it
    #[cfg(not(feature = "console_debug"))]
    pub(crate) fn insert_2(
        &mut self,
        key: BeachLineNodeKey<I, F>,
    ) -> Result<(BeachLineNodeKey<I, F>, BeachLineIndex), BvError> {
        let node = Rc::new(Cell::new(None));
        let node_index = BeachLineIndex(self.beach_line_.borrow_mut().ordered_insert(key, node)?);
        Ok((key, node_index))
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
        let node = bl_borrow.get(beachline_index.0);

        if node.is_err() {
            eprintln!("Failed to retrieve beach line key : {}", beachline_index.0);
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
        Ok(cpp_map::PIterator::lower_bound(
            Rc::clone(&self.beach_line_),
            key,
        )?)
    }

    #[allow(dead_code)]
    #[cfg(feature = "console_debug")]
    pub(crate) fn debug_cmp_all(&self, key: BeachLineNodeKey<I, F>) {
        for (i, (v, _)) in self.beach_line_.borrow().iter().rev().enumerate() {
            t!("#{}:", i);
            let _rv = VP::node_comparison_predicate::node_comparison::<I, F>(v, &key);
        }
    }

    #[cfg(feature = "console_debug")]
    #[allow(dead_code)]
    pub(crate) fn debug_print_all(&self) -> Result<(), BvError> {
        tln!();
        tln!("beach_line.len()={}", self.beach_line_.borrow().len());
        for (i, (node_key, node_data)) in self.beach_line_.borrow().iter().rev().enumerate() {
            t!(
                "beach_line{} L:{:?},R:{:?}",
                i,
                &node_key.left_site(),
                &node_key.right_site()
            );

            if let Some(data) = node_data.get() {
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
        for (i, (node_key, node_data)) in self.beach_line_.borrow().iter().enumerate() {
            t!("#{}:", i);
            self.dbgpa_compat_node_(node_key, node_data, ce)?;
        }
        tln!();
        Ok(())
    }

    #[cfg(feature = "console_debug")]
    pub(crate) fn dbgp_all_cmp_(&self) {
        let _iter1 = self.beach_line_.borrow();
        let mut it1 = _iter1.iter().map(|x| x.0).enumerate();
        for it2_v in self
            .beach_line_
            .borrow()
            .iter()
            .map(|x| x.0)
            .enumerate()
            .skip(1)
        {
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
        node_key: &BeachLineNodeKey<I, F>,
        node_data: &BeachLineNodeDataType,
        ce: &VC::CircleEventQueue,
    ) -> Result<(), BvError> {
        t!(
            "L:{:?},R:{:?}",
            &node_key.left_site(),
            &node_key.right_site(),
        );
        if let Some(data) = node_data.get() {
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

        // t!(" id={}", id);
        tln!();
        Ok(())
    }
}

impl<I: InputType, F: OutputType> fmt::Debug for BeachLine<I, F> {
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
pub struct BeachLineNodeKey<I: InputType, F: OutputType> {
    left_site_: VSE::SiteEvent<I, F>,
    right_site_: VSE::SiteEvent<I, F>,
}

impl<I: InputType, F: OutputType> fmt::Debug for BeachLineNodeKey<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L:{:?},R:{:?}", &self.left_site(), &self.right_site())
    }
}

impl<I: InputType, F: OutputType> BeachLineNodeKey<I, F> {
    /// Constructs degenerate bisector, used to search an arc that is above
    /// the given site. The input to the constructor is the new site point.
    pub fn new_1(new_site: VSE::SiteEvent<I, F>) -> Self {
        Self {
            left_site_: new_site,
            right_site_: new_site,
        }
    }

    /// Constructs a new bisector. The input to the constructor is the two
    /// sites that create the bisector. The order of sites is important.
    pub fn new_2(left_site: VSE::SiteEvent<I, F>, right_site: VSE::SiteEvent<I, F>) -> Self {
        Self {
            left_site_: left_site,
            right_site_: right_site,
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
}

impl<I: InputType, F: OutputType> PartialOrd for BeachLineNodeKey<I, F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I: InputType, F: OutputType> Ord for BeachLineNodeKey<I, F> {
    // todo: move the content of node_comparison_predicate to here
    fn cmp(&self, other: &Self) -> Ordering {
        if node_comparison_predicate::node_comparison::<I, F>(self, other) {
            Ordering::Less
        } else if node_comparison_predicate::node_comparison::<I, F>(other, self) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl<I: InputType, F: OutputType> PartialEq for BeachLineNodeKey<I, F> {
    // todo: node1.cmp(node2)==Ordering.Equal is not the same as node1 == node2, should it be?
    fn eq(&self, other: &Self) -> bool {
        self.left_site_ == other.left_site_ && self.right_site_ == other.right_site_
    }
}

impl<I: InputType, F: OutputType> Eq for BeachLineNodeKey<I, F> {}

impl<I: InputType, F: OutputType> Hash for BeachLineNodeKey<I, F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.left_site_.hash(state);
        self.right_site_.hash(state);
    }
}

/// Represents edge data structure from the Voronoi output, that is
/// associated as a value with beach line bisector in the beach
/// line. Contains pointer to the circle event in the circle event
/// queue if the edge corresponds to the right bisector of the circle event.
// Todo! this should be rust:ified and made into an Enum
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

    pub fn get_circle_event_id(&self) -> Option<VC::CircleEventIndex> {
        self.circle_event_
    }

    pub(crate) fn set_circle_event_id(&mut self, circle_event: Option<VC::CircleEventIndex>) {
        self.circle_event_ = circle_event;
    }

    pub(crate) fn edge_id(&self) -> VD::EdgeIndex {
        self.edge_
    }

    pub(crate) fn set_edge_id(&mut self, new_edge: VD::EdgeIndex) {
        self.edge_ = new_edge;
    }
}
