// Boost.Polygon library voronoi_diagram.hpp header file

//          Copyright Andrii Sydorchuk 2010-2012.
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

// See http://www.boost.org for updates, documentation, and revision history.

// Ported from C++ boost 1.74.0 to Rust in 2020 by Eadf (github.com/eadf)

mod tests;

use super::circleevent as VC;
use super::diagram as VD;
use super::predicate as VP;
use super::siteevent as VSE;

use super::{BigFloatType, BigIntType, InputType, OutputType};
use crate::BvError;
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::Neg;
use std::rc::Rc;
use vec_map::VecMap;

/// debug utility function
#[allow(dead_code)]
#[cfg(feature = "console_debug")]
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

impl BeachLineIndex {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn increment(&mut self) -> &Self {
        self.0 += 1;
        self
    }
}

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
pub struct Beachline<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
    pub(crate) beach_line_: BTreeMap<BeachLineNodeKey<I, O, BI, BF>, BeachLineIndex>,
    pub(crate) next_free_: BeachLineIndex,
    pub(crate) beach_line_vec: VecMap<(BeachLineNodeKey<I, O, BI, BF>, BeachLineNodeDataType)>,
}

impl<I, O, BI, BF> Beachline<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
    pub fn default() -> Self {
        Self {
            beach_line_: BTreeMap::default(),
            next_free_: BeachLineIndex::new(0),
            beach_line_vec: VecMap::default(),
        }
    }

    pub(crate) fn len(&self) -> (usize, usize) {
        (self.beach_line_.len(), self.beach_line_vec.len())
    }

    /// updates the node_index of the key, inserts it into the list and
    /// returns a copy of it
    #[cfg(not(feature = "console_debug"))]
    pub(crate) fn insert(
        &mut self,
        mut key: BeachLineNodeKey<I, O, BI, BF>,
        data: Option<BeachLineNodeData>,
    ) -> BeachLineNodeKey<I, O, BI, BF> {
        key.node_index_ = self.next_free_;

        let node = Rc::new(Cell::new(data));
        let _ = self.beach_line_vec.insert(self.next_free_.0, (key, node));
        let _ = self.beach_line_.insert(key, key.node_index_);
        let _ = self.next_free_.increment();
        key
    }

    /// updates the node_index of the key, inserts it into the list and
    /// returns a copy of it
    #[cfg(feature = "console_debug")]
    pub(crate) fn insert(
        &mut self,
        mut key: BeachLineNodeKey<I, O, BI, BF>,
        data: Option<BeachLineNodeData>,
        _ce: &VC::CircleEventQueue<BF>,
    ) -> BeachLineNodeKey<I, O, BI, BF> {
        key.node_index_ = self.next_free_;

        let node = Rc::new(Cell::new(data));
        let _ = self.beach_line_vec.insert(self.next_free_.0, (key, node));
        let _prev_value = self.beach_line_.insert(key, key.node_index_);
        if _prev_value.is_some() {
            println!("+++++++++++++++++++++++++++++++++++++++++");
            println!(
                "inserted beachline but it collided id:{:?}",
                _prev_value.unwrap()
            );
            println!("with {:?}", _prev_value.unwrap());
        }
        let _ = self.next_free_.increment();
        #[cfg(feature = "console_debug")]
        print!("inserted beachline:");
        #[cfg(feature = "console_debug")]
        self.debug_print_all_compat_node(&key, _ce);
        key
    }

    /// This is a quick & dirty fix. Re-creating the entire beachline BTreeMap
    #[allow(dead_code)]
    fn rebuild_beachline_do_not_use(&mut self) {
        #[cfg(feature = "console_debug")]
        println!("remap_beachline()");
        let mut beachline_tmp = BTreeMap::<BeachLineNodeKey<I, O, BI, BF>, BeachLineIndex>::new();
        // append does not solve the problem
        //beachline_tmp.append(&mut self.beach_line_);
        for i in self.beach_line_.iter() {
            let _ = beachline_tmp.insert(*i.0, *i.1);
        }
        std::mem::swap(&mut self.beach_line_, &mut beachline_tmp);
        //beachline_tmp.clear();
    }

    /// removes a beach-line item from the beach-line priority queue
    pub(crate) fn erase(&mut self, beachline_index: BeachLineIndex) -> Result<(), BvError> {
        if let Some(node) = self.beach_line_vec.get(beachline_index.0) {
            let node = node.0;
            #[cfg(feature = "console_debug")]
            println!("erasing beachline:{:?}", node);

            if self.beach_line_.remove(&node).is_none() {
                #[cfg(feature = "console_debug")]
                self.debug_print_all_dump_and_cmp(&node);
                // We know the item should be in self.beach_line_ if it is in self.beach_line_vec
                // as a work-around we recreate the entire self.beach_line_ map
                //self.rebuild_beachline();

                if self.beach_line_.remove(&node).is_none() {
                    println!("Tried to remove a non-existent beachline, this error can occur if the input data is self-intersecting");
                    println!("{:?}", node);
                    self.debug_print_all_dump_and_cmp(&node);
                    return Err(BvError::SelfIntersecting {txt:"Tried to remove a non-existent beachline, this error can occur if the input data is self-intersecting".to_string()});
                }
            }
            //if self.beach_line_.contains_key(&node) {
            //    return Err(BvError::SomeError {
            //        txt: "Beachline: internal error there are more identical keys".to_string(),
            //   });
            //}
            let _ = self.beach_line_vec.remove(beachline_index.0);
        } else {
            return Err(BvError::SelfIntersecting {txt:"Tried to remove a non-existent beachline, this error can occur if the input data is self-intersecting".to_string()});
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.beach_line_.clear();
        self.next_free_ = BeachLineIndex::new(0);
        self.beach_line_vec.clear();
    }

    /// same as right_it == beach_line_.begin() in c++
    pub(crate) fn is_at_beginning(
        &self,
        right_it: &Option<BeachLineNodeKey<I, O, BI, BF>>,
    ) -> bool {
        // when right_it is None the 'iterator' has passed end
        if right_it.is_none() {
            return false;
        }
        let peek = self.peek_first();
        if peek.is_none() && right_it.is_none() {
            panic!("wtf?");
            //return false;
        }
        if peek.is_none() {
            return false;
        }

        peek.unwrap().0 == right_it.unwrap()
    }

    /// mapping: BeachLineNodeIndexType->(BeachLineNodeKey,BeachLineNodeDataType)
    pub(crate) fn get_node(
        &self,
        beachline_index: &BeachLineIndex,
    ) -> (BeachLineNodeKey<I, O, BI, BF>, BeachLineNodeDataType) {
        if !self.beach_line_vec.contains_key(beachline_index.0) {
            panic!("tried to retrieve a beach line node that doesn't exist");
        }
        let bn = &self.beach_line_vec[beachline_index.0];
        (bn.0, bn.1.clone())
    }

    /// same as get_node() but only returns the key
    pub(crate) fn get_node_key(
        &self,
        beachline_index: BeachLineIndex,
    ) -> BeachLineNodeKey<I, O, BI, BF> {
        self.beach_line_vec[beachline_index.0].0
    }

    /// swaps the 'before' key for the 'after' key
    /// It does this by removing key/value from the map and re-inserting the new values
    pub fn replace_key(
        &mut self,
        before: BeachLineNodeKey<I, O, BI, BF>,
        after: BeachLineNodeKey<I, O, BI, BF>,
    ) -> (BeachLineNodeKey<I, O, BI, BF>, BeachLineNodeDataType) {
        let idx = *self.beach_line_.get(&before).unwrap();
        let _ = self.beach_line_.remove(&before);
        let _ = self.beach_line_.insert(after, idx);

        let item = self.beach_line_vec.remove(idx.0).unwrap().1;
        let _rv = self.beach_line_vec.insert(idx.0, (after, item));
        // todo! why doesn't _rv contain the return value we need?
        self.get_node(&idx)
    }

    /// Returns the left neighbour beach line element
    /// Returns None if no association data is found
    pub(crate) fn get_left_neighbour(
        &self,
        position: BeachLineNodeKey<I, O, BI, BF>,
    ) -> Option<(BeachLineNodeKey<I, O, BI, BF>, BeachLineIndex)> {
        let rv = self
            .beach_line_
            .range((Unbounded, Excluded(&position)))
            .next_back();
        if let Some(rv) = rv {
            Some((*rv.0, *rv.1))
        } else {
            None
        }
    }

    /// Returns the left neighbour beach line element
    /// Returns None if no association data is found
    #[allow(dead_code)]
    pub(crate) fn get_left_neighbour_by_id(
        &self,
        position: BeachLineIndex,
    ) -> Option<(BeachLineNodeKey<I, O, BI, BF>, BeachLineIndex)> {
        self.beach_line_vec
            .get(position.0)
            .and_then(|x| self.get_left_neighbour(x.0))
    }

    /// Returns the right neighbour beach line element
    /// Returns None if no association data is found
    pub fn get_right_neighbour(
        &self,
        position: BeachLineNodeKey<I, O, BI, BF>,
    ) -> Option<BeachLineNodeKey<I, O, BI, BF>> {
        let rv = self
            .beach_line_
            .range((Excluded(&position), Unbounded))
            .next();
        if let Some(rv) = rv {
            Some(*rv.0)
        } else {
            None
        }
    }

    /// Returns the right neighbour beach line element
    /// Returns None if no association data is found
    pub(crate) fn get_right_neighbour_by_id(
        &self,
        position: BeachLineIndex,
    ) -> Option<BeachLineNodeKey<I, O, BI, BF>> {
        self.beach_line_vec
            .get(position.0)
            .and_then(|x| self.get_right_neighbour(x.0))
    }

    /// Returns the first beach line element in the container whose key is not considered to go
    /// before position (i.e., either it is equivalent or goes after).
    /// Returns None if no  data is found
    pub fn lower_bound(
        &self,
        key: BeachLineNodeKey<I, O, BI, BF>,
    ) -> Option<BeachLineNodeKey<I, O, BI, BF>> {
        let rv = self.beach_line_.range((Included(&key), Unbounded)).next();
        if let Some(rv) = rv {
            Some(*rv.0)
        } else {
            None
        }
    }

    /// returns a copy of the last element (key,value)
    pub(crate) fn peek_last(&self) -> Option<(BeachLineNodeKey<I, O, BI, BF>, BeachLineIndex)> {
        self.beach_line_
            .range((Unbounded::<BeachLineNodeKey<I, O, BI, BF>>, Unbounded))
            .next_back()
            .map(|x| (*x.0, *x.1))
    }

    /// returns a copy of the first element (key,value)
    pub(crate) fn peek_first(&self) -> Option<(BeachLineNodeKey<I, O, BI, BF>, BeachLineIndex)> {
        self.beach_line_
            .range((Unbounded::<BeachLineNodeKey<I, O, BI, BF>>, Unbounded))
            .next()
            .map(|x| (*x.0, *x.1))
    }

    #[allow(dead_code)]
    #[cfg(feature = "console_debug")]
    pub(crate) fn debug_cmp_all(&self, key: BeachLineNodeKey<I, O, BI, BF>) {
        for (i, v) in self.beach_line_.iter().enumerate() {
            print!("#{}:", i);
            let _rv =
                VP::NodeComparisonPredicate::<I, O, BI, BF>::node_comparison_predicate(v.0, &key);
        }
    }

    #[cfg(feature = "console_debug")]
    #[allow(dead_code)]
    pub(crate) fn debug_print_all(&self) {
        println!();
        println!("beachline.len()={}", self.beach_line_.len());
        for (i, (node, id)) in self.beach_line_.iter().enumerate() {
            print!(
                "beachline{} L:{:?},R:{:?}, id={:?}",
                i,
                &node.left_site(),
                &node.right_site(),
                id
            );
            if let Some(data) = self.get_node(id).1.get() {
                if let Some(circle_event) = data.circle_event_ {
                    print!(" -> CircleEvent:{}", circle_event);
                } else {
                    print!(" -> CircleEvent:-");
                }
                print!(", edge:{:?}", data.edge_);
            } else {
                print!(" temporary bisector");
            }
            println!();
        }
        println!();
    }

    #[cfg(feature = "console_debug")]
    pub(crate) fn debug_print_all_compat(&self, ce: &VC::CircleEventQueue<BF>) {
        println!("-----beachline----{}", self.beach_line_.len());
        for (i, (node, _id)) in self.beach_line_.iter().enumerate() {
            print!("#{}:", i);
            self.debug_print_all_compat_node(&node, ce);
        }
        println!();
    }

    #[warn(dead_code)]
    pub(crate) fn debug_print_all_dump_and_cmp(&self, key: &BeachLineNodeKey<I, O, BI, BF>) {
        println!("-----beachline----{}", self.beach_line_.len());
        println!("Looking for {:?} in the beachline", key);
        let found = self.beach_line_.get(key);
        println!(
            "Found {:?} cmp1=node.partial_cmp(key).unwrap() cmp2=key.partial_cmp(node).unwrap()",
            found
        );
        for (i, (node, _id)) in self.beach_line_.iter().enumerate() {
            let cmp1 = node.partial_cmp(key);
            let cmp2 = key.partial_cmp(node);

            print!(
                "#{}: key:{:?}, cmp1:{:?}, cmp2:{:?}",
                i,
                node,
                cmp1.unwrap(),
                cmp2.unwrap()
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
    }

    #[cfg(feature = "console_debug")]
    pub(crate) fn debug_print_all_compat_node(
        &self,
        node: &BeachLineNodeKey<I, O, BI, BF>,
        ce: &VC::CircleEventQueue<BF>,
    ) {
        let id = &node.get_index();
        print!("L:{:?},R:{:?}", &node.left_site(), &node.right_site(),);
        if let Some(data) = self.get_node(id).1.get() {
            if let Some(_circle_event) = data.circle_event_ {
                if ce.is_active(_circle_event) {
                    print!(" -> CircleEvent(..)");
                } else {
                    print!(" -> CircleEvent=--");
                }
            } else {
                print!(" -> CircleEvent=-");
            }
        } else {
            print!(" Temporary bisector");
        }
        println!(" id={}", id);
    }
}

impl<I, O, BI, BF> fmt::Debug for Beachline<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rv = String::new();

        for (index, node) in self.beach_line_.iter().enumerate() {
            rv.push_str(format!("{}: {:?}", index, node).as_str());
            rv.push('\n');
        }
        for i in self.beach_line_vec.iter() {
            rv.push_str(format!("{:?}", i).as_str());
            rv.push('\n');
        }
        write!(f, "\n{}\n", rv)
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
pub struct BeachLineNodeKey<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
    left_site_: VSE::SiteEvent<I, O, BI, BF>,
    right_site_: VSE::SiteEvent<I, O, BI, BF>,
    node_index_: BeachLineIndex,
}

impl<I, O, BI, BF> fmt::Debug for BeachLineNodeKey<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rv = String::new();
        rv.push_str(
            format!(
                "L:{:?},R:{:?}, id={:?}",
                &self.left_site(),
                &self.right_site(),
                self.node_index_.0
            )
            .as_str(),
        );
        write!(f, "{}", rv)
    }
}

impl<I, O, BI, BF> BeachLineNodeKey<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
    // Constructs degenerate bisector, used to search an arc that is above
    // the given site. The input to the constructor is the new site point.
    pub fn new_1(new_site: VSE::SiteEvent<I, O, BI, BF>) -> Self {
        Self {
            left_site_: new_site,
            right_site_: new_site,
            node_index_: BeachLineIndex::new(0), // will be populated by Beachline::insert
        }
    }

    // Constructs a new bisector. The input to the constructor is the two
    // sites that create the bisector. The order of sites is important.
    pub fn new_2(
        left_site: VSE::SiteEvent<I, O, BI, BF>,
        right_site: VSE::SiteEvent<I, O, BI, BF>,
    ) -> Self {
        Self {
            left_site_: left_site,
            right_site_: right_site,
            node_index_: BeachLineIndex::new(0), // will be populated by Beachline::insert
        }
    }

    pub(crate) fn left_site_m(&mut self) -> &mut VSE::SiteEvent<I, O, BI, BF> {
        &mut self.left_site_
    }

    pub fn left_site(&self) -> &VSE::SiteEvent<I, O, BI, BF> {
        &self.left_site_
    }

    #[allow(dead_code)]
    pub(crate) fn set_left_site(&mut self, site: &VSE::SiteEvent<I, O, BI, BF>) {
        self.left_site_ = *site;
    }

    pub(crate) fn right_site_m(&mut self) -> &mut VSE::SiteEvent<I, O, BI, BF> {
        &mut self.right_site_
    }

    pub fn right_site(&self) -> &VSE::SiteEvent<I, O, BI, BF> {
        &self.right_site_
    }

    pub(crate) fn set_right_site(&mut self, site: &VSE::SiteEvent<I, O, BI, BF>) {
        self.right_site_ = *site; // Copy
    }

    pub(crate) fn get_index(&self) -> BeachLineIndex {
        self.node_index_
    }
}

impl<I, O, BI, BF> PartialOrd for BeachLineNodeKey<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<I, O, BI, BF> Ord for BeachLineNodeKey<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let is_less =
            VP::NodeComparisonPredicate::<I, O, BI, BF>::node_comparison_predicate(self, other);
        if is_less {
            Ordering::Less
        } else {
            //let is_less_reverse = VP::NodeComparisonPredicate::<I, O, BI, BF>::node_comparison_predicate(other, self);
            //if !is_less_reverse {
            // is_less=false && is_less_reverse=false -> must be equal

            if self.left_site_ == other.left_site_ && self.right_site_ == other.right_site_ {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
    }
}

impl<I, O, BI, BF> PartialEq for BeachLineNodeKey<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
    fn eq(&self, other: &Self) -> bool {
        self.left_site_ == other.left_site_ && self.right_site_ == other.right_site_
    }
}

impl<I, O, BI, BF> Eq for BeachLineNodeKey<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
{
}

impl<I, O, BI, BF> Hash for BeachLineNodeKey<I, O, BI, BF>
where
    I: InputType + Neg<Output = I>,
    O: OutputType + Neg<Output = O>,
    BI: BigIntType + Neg<Output = BI>,
    BF: BigFloatType + Neg<Output = BF>,
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
    circle_event_: Option<VC::CircleEventIndexType>,
    edge_: VD::VoronoiEdgeIndex,
}

impl BeachLineNodeData {
    pub fn new_1(new_edge: VD::VoronoiEdgeIndex) -> Self {
        Self {
            circle_event_: None,
            edge_: new_edge,
        }
    }
    /*
    fn new_2(circle: Option<VC::CircleEventIndexType>, new_edge: VD::VoronoiEdgeIndex) -> Self {
        Self {
            circle_event_: circle,
            edge_: new_edge,
        }
    }*/

    pub fn get_circle_event_id(&self) -> Option<VC::CircleEventIndexType> {
        self.circle_event_
    }

    pub(crate) fn set_circle_event_id(
        &mut self,
        circle_event: Option<VC::CircleEventIndexType>,
    ) -> &mut Self {
        self.circle_event_ = circle_event;
        self
    }

    pub(crate) fn edge_id(&self) -> VD::VoronoiEdgeIndex {
        self.edge_
    }

    pub(crate) fn set_edge_id(&mut self, new_edge: VD::VoronoiEdgeIndex) -> &mut Self {
        self.edge_ = new_edge;
        self
    }
}
