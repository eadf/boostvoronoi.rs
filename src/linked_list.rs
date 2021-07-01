//           Copyright Eadf (github.com/eadf)
// Distributed under the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE_1_0.txt or copy at
//          http://www.boost.org/LICENSE_1_0.txt)

//! Data structure that emulates a C++ map and it's pointer based iterators.
//! This implementation does not (yet) support anything other than linear search.

use super::BvError;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::rc::Rc;

#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
struct Node<T, U>
where
    T: Clone + Debug,
    U: Clone + Debug,
{
    prev: usize,
    next: usize,
    key: T,
    value: U,
}

#[derive(Clone, Debug)]
pub struct LinkedList<T, U>
where
    T: Clone + Debug,
    U: Clone + Debug,
{
    head_: usize,
    tail_: usize,
    nodes_: Vec<Option<Node<T, U>>>,
    id_pool_: Vec<usize>,
}

impl<T, U> Default for LinkedList<T, U>
where
    T: Clone + Debug,
    U: Clone + Debug,
{
    fn default() -> Self {
        Self {
            head_: usize::MAX,
            tail_: usize::MAX,
            nodes_: Vec::new(),
            id_pool_: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
/// borrow checker work-around
struct EraseOperation {
    // (node index, next index)
    change_prev: Option<(usize, usize)>,
    // the node to erase
    erase: usize,
    // (node index, pre index)
    change_next: Option<(usize, usize)>,
}

#[allow(dead_code)]
impl<'a, T: 'a, U: 'a> LinkedList<T, U>
where
    T: Clone + Debug + Ord + PartialOrd,
    U: Clone + Debug,
{
    pub fn iter(&self) -> ListIterator<'_, T, U> {
        //println!("create iterator with my_next:{}", self.head);
        ListIterator {
            list: self,
            my_next: self.head_,
        }
    }

    #[inline(always)]
    /// Returns the number of active elements
    pub fn len(&self) -> usize {
        self.nodes_.len() - self.id_pool_.len()
    }

    /// Returns the capacity or the vectors
    pub fn capacity(&self) -> (usize, usize) {
        (self.nodes_.capacity(), self.id_pool_.capacity())
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the list.
    /// Warning: any Pointer object referring to this list will be corrupted.
    pub fn clear(&mut self) {
        for (i, node) in self.nodes_.iter_mut().enumerate() {
            let _ = node.take(); // throw away content
            self.id_pool_.push(i);
        }
    }

    /// Returns the next free index
    pub fn next_free_index(&self) -> usize {
        if self.id_pool_.is_empty() {
            self.nodes_.len()
        } else {
            // unwrap is safe after !is_empty() check
            *self.id_pool_.last().unwrap()
        }
    }

    #[inline(always)]
    /// Returns the item key at index
    pub fn get_k(&self, index: usize) -> Result<&T, BvError> {
        let rv = self
            .nodes_
            .get(index)
            .ok_or_else(|| BvError::LinkedList("error, item not found".to_string()))?
            .as_ref()
            .ok_or_else(|| BvError::LinkedList("error, item was not active".to_string()))?;
        Ok(&rv.key)
    }

    #[inline(always)]
    /// Returns the item value at index
    pub fn get_v(&self, index: usize) -> Result<&U, BvError> {
        let rv = self
            .nodes_
            .get(index)
            .ok_or_else(|| BvError::LinkedList("error, item not found".to_string()))?
            .as_ref()
            .ok_or_else(|| BvError::LinkedList("error, item was not active".to_string()))?;
        Ok(&rv.value)
    }

    #[inline(always)]
    /// Returns the item value at index
    pub fn get_kv(&self, index: usize) -> Result<(&T, &U), BvError> {
        let rv = self
            .nodes_
            .get(index)
            .ok_or_else(|| BvError::LinkedList("error, item not found".to_string()))?
            .as_ref()
            .ok_or_else(|| BvError::LinkedList("error, item was not active".to_string()))?;
        Ok((&rv.key, &rv.value))
    }

    #[inline(always)]
    /// Returns the previous item of item at index
    pub fn get_prev(&self, index: usize) -> Result<&T, BvError> {
        let prev = self
            .nodes_
            .get(index)
            .as_ref()
            .ok_or_else(|| BvError::LinkedList("error, item not found".to_string()))?
            .as_ref()
            .ok_or_else(|| BvError::LinkedList("error, item was None".to_string()))?
            .prev;

        let node = self
            .nodes_
            .get(prev)
            .ok_or_else(|| BvError::LinkedList("error, prev item not found".to_string()))?
            .as_ref();
        Ok(&node
            .ok_or_else(|| BvError::LinkedList("error, item was not active".to_string()))?
            .key)
    }

    /// Add an item at the front of the list
    pub fn push_front(&mut self, key: T, value: U) -> Result<usize, BvError> {
        let insertion_index = if !self.id_pool_.is_empty() {
            self.id_pool_.pop().unwrap()
        } else {
            self.nodes_.len()
        };
        let new_node = if let Some(ref mut prev_head) = self.nodes_.get_mut(self.head_) {
            if let Some(prev_head) = prev_head {
                //println!("prev_head:{:?}", prev_head);
                // there were a previous head
                let new_node = Node {
                    next: self.head_,
                    prev: usize::MAX,
                    key,
                    value,
                };
                self.head_ = insertion_index;
                prev_head.prev = insertion_index;
                new_node
            } else {
                return Err(BvError::LinkedList(format!(
                    "Should not happen error™ at {}:{}",
                    file!(),
                    line!()
                )));
            }
        } else {
            // This will be the first element in the list
            //println!("no prev_head:{}", self.head);
            self.head_ = insertion_index;
            self.tail_ = insertion_index;
            Node {
                next: usize::MAX,
                prev: usize::MAX,
                key,
                value,
            }
        };
        //println!("push_front Pushed {:?} at index:{}", new_node, curr_len);
        Ok(self.replace_or_push_(insertion_index, new_node))
    }

    #[inline(always)]
    /// insert at position or append at back of the list
    fn replace_or_push_(&mut self, insertion_index: usize, new_node: Node<T, U>) -> usize {
        if insertion_index == self.nodes_.len() {
            self.nodes_.push(Some(new_node));
        } else {
            // get_mut will never fail
            let _ = self
                .nodes_
                .get_mut(insertion_index)
                .unwrap()
                .replace(new_node);
        }
        insertion_index
    }

    /// insert a new value before the element at index
    pub fn insert_before(&mut self, index: usize, key: T, value: U) -> Result<usize, BvError> {
        if index == usize::MAX {
            return self.push_front(key, value);
        }

        let insertion_index = if !self.id_pool_.is_empty() {
            self.id_pool_.pop().unwrap()
        } else {
            self.nodes_.len()
        };

        let new_node = if let Some(ref mut next_node) = self.nodes_.get_mut(index) {
            if let Some(ref mut next_node) = next_node {
                //println!("next_node:{:?}", next_node);
                // there were a previous head
                let new_node = Node {
                    next: index,
                    prev: next_node.prev,
                    key,
                    value,
                };
                next_node.prev = insertion_index;
                new_node
            } else {
                return Err(BvError::LinkedList(format!(
                    "Should not happen error™ at {}:{}",
                    file!(),
                    line!()
                )));
            }
        } else {
            // This will be the first element in the list
            //println!("no prev_head:{}", self.head);
            self.head_ = insertion_index;
            self.tail_ = insertion_index;
            Node {
                next: usize::MAX,
                prev: usize::MAX,
                key,
                value,
            }
        };
        let prev_node = new_node.prev;

        //println!("insert_before Pushed {:?} at index:{}", new_node, curr_len);
        {
            let _i = self.replace_or_push_(insertion_index, new_node);
            #[cfg(feature = "console_debug")]
            assert_eq!(insertion_index, _i);
        };

        if prev_node != usize::MAX {
            if let Some(prev_node) = self.nodes_.get_mut(prev_node) {
                if let Some(prev_node) = prev_node {
                    prev_node.next = insertion_index;
                } else {
                    return Err(BvError::LinkedList(format!(
                        "Should not happen error™ at {}:{}",
                        file!(),
                        line!()
                    )));
                }
            } else {
                // this case should have been handled by the initial push_front()
                return Err(BvError::LinkedList(format!(
                    "Should not happen error™ at {}:{}",
                    file!(),
                    line!()
                )));
            }
        } else {
            // We just pushed at the first position
            self.head_ = insertion_index;
        }
        //println!("insert_before inserted at {}", insertion_index);
        Ok(insertion_index)
    }

    /// Add an item at the back of the list
    pub fn push_back(&mut self, key: T, value: U) -> Result<usize, BvError> {
        let insertion_index = if !self.id_pool_.is_empty() {
            self.id_pool_.pop().unwrap()
        } else {
            self.nodes_.len()
        };
        let new_node = if let Some(prev_tail) = self.nodes_.get_mut(self.tail_) {
            if let Some(prev_tail) = prev_tail {
                //println!("prev_tail:{:?}", prev_tail);
                // there were a previous tail
                let new_node = Node {
                    next: usize::MAX,
                    prev: self.tail_,
                    key,
                    value,
                };
                self.tail_ = insertion_index;
                prev_tail.next = insertion_index;
                new_node
            } else {
                return Err(BvError::LinkedList(format!(
                    "Should not happen error™ at {}:{}",
                    file!(),
                    line!()
                )));
            }
        } else {
            // This will be the first element in the list
            //println!("no prev_tail:{}", self.tail);
            self.head_ = insertion_index;
            self.tail_ = insertion_index;
            Node {
                next: usize::MAX,
                prev: usize::MAX,
                key,
                value,
            }
        };
        //println!("push_back Pushed {:?} at index:{}", new_node, insertion_index);
        {
            let _insert_index = self.replace_or_push_(insertion_index, new_node);
            #[cfg(feature = "console_debug")]
            assert_eq!(_insert_index, insertion_index);
        }
        Ok(insertion_index)
    }

    /// Insert item at position defined by Order (lesser first)
    pub fn ordered_insert(&mut self, key: T, value: U) -> Result<usize, BvError> {
        let mut curr_index = self.head_;
        let mut insert_before: Option<usize> = None;
        while let Some(Some(sample)) = self.nodes_.get(curr_index) {
            if key.cmp(&sample.key) != Ordering::Greater {
                insert_before = Some(curr_index);
                break;
            } else {
                curr_index = sample.next;
            }
        }
        if let Some(insert_before) = insert_before {
            self.insert_before(insert_before, key, value)
        } else {
            self.push_back(key, value)
        }
    }

    /// Returns the first element in the container whose key is not considered to go
    /// before position (i.e., either it is equivalent or goes after).
    /// Returns None if no data is found
    pub fn lower_bound(&self, key: T) -> Result<Option<usize>, BvError> {
        if self.head_ == usize::MAX {
            return Ok(None);
        }
        let mut curr_index = self.head_;
        while let Some(Some(sample)) = self.nodes_.get(curr_index) {
            if key.cmp(&sample.key) != Ordering::Greater {
                return Ok(Some(curr_index));
            } else {
                curr_index = sample.next;
            }
        }
        Ok(None)
    }

    #[inline(always)]
    /// Pop the head item
    pub fn pop_front(&mut self) -> Result<Option<T>, BvError> {
        self.remove(self.head_)
    }

    #[inline(always)]
    /// Pop the tail item
    pub fn pop_back(&mut self) -> Result<Option<T>, BvError> {
        self.remove(self.tail_)
    }

    #[inline(always)]
    /// Pop the head item
    pub fn peek_front(&self) -> Option<&T> {
        match self.nodes_.get(self.head_) {
            Some(Some(node)) => Some(&node.key),
            _ => None,
        }
    }

    #[inline(always)]
    /// Pop the tail item
    pub fn peek_back(&self) -> Option<&T> {
        match self.nodes_.get(self.tail_) {
            Some(Some(node)) => Some(&node.key),
            _ => None,
        }
    }

    #[inline(always)]
    /// Return the tail index
    pub fn tail(&self) -> usize {
        self.tail_
    }

    #[inline(always)]
    /// Return the head index
    pub fn head(&self) -> usize {
        self.head_
    }

    #[inline(always)]
    /// Remove the item at index, return item value if found
    fn remove(&mut self, index: usize) -> Result<Option<T>, BvError> {
        let rv = self.remove_(index, false)?;
        Ok(Some(rv.1))
    }

    /// Disconnect and remove the item at index, return item value if found
    fn remove_(
        &mut self,
        index: usize,
        only_disconnect: bool,
    ) -> Result<(usize, T, usize), BvError> {
        if self.head_ == usize::MAX {
            return Err(BvError::LinkedList(format!(
                "Could not find element to remove {}:{}",
                file!(),
                line!()
            )));
        }
        //println!("remove {} before:{:?}", index, self);
        let rv = if self.head_ != usize::MAX {
            // list was not empty
            let operation = if let Some(node) = self.nodes_.get(index) {
                let mut operation = EraseOperation {
                    change_prev: None,
                    erase: index,
                    change_next: None,
                };
                if let Some(node) = node {
                    // Check node next
                    if let Some(next) = self.nodes_.get(node.next) {
                        if next.is_some() {
                            // node had a next
                            operation.change_next = Some((node.next, node.prev));
                        } else {
                            return Err(BvError::LinkedList(format!(
                                "Should not happen error™ at {}:{}",
                                file!(),
                                line!()
                            )));
                        }
                    } else {
                        // node had no next
                    }

                    // Check prev node
                    if let Some(prev) = self.nodes_.get(node.prev) {
                        if prev.is_some() {
                            // node had a prev
                            operation.change_prev = Some((node.prev, node.next));
                        } else {
                            return Err(BvError::LinkedList(format!(
                                "Should not happen error™ at {}:{}",
                                file!(),
                                line!()
                            )));
                        }
                    } else {
                        // node had no prev
                    }
                    Some(operation)
                } else {
                    return Err(BvError::LinkedList(format!(
                        "Should not happen error™ at {}:{}",
                        file!(),
                        line!()
                    )));
                }
            } else {
                // index was not found, todo: report error?
                None
            };
            if let Some(operation) = operation {
                Some(self.erase_node_(operation, only_disconnect)?)
            } else {
                None
            }
        } else {
            // list was empty
            None
        };
        rv.ok_or_else(|| {
            BvError::LinkedList(format!(
                "Could not find element to remove {}:{}",
                file!(),
                line!()
            ))
        })
    }

    /// do the actual erase now that we know how to do it (work around for the borrow checker).
    fn erase_node_(
        &mut self,
        operation: EraseOperation,
        only_disconnect: bool,
    ) -> Result<(usize, T, usize), BvError> {
        //println!("erase_operation {:?}", operation);
        match (operation.change_prev, operation.change_next) {
            (Some((prev_i, new_next)), Some((next_i, new_prev))) => {
                #[cfg(feature = "console_debug")]
                {
                    assert_eq!(new_next, next_i);
                    assert_eq!(prev_i, new_prev);
                }
                match self.nodes_.get_mut(prev_i) {
                    Some(Some(node)) => {
                        node.next = new_next;
                    }
                    _ => {
                        return Err(BvError::LinkedList(format!(
                            "Should not happen error™ at {}:{}",
                            file!(),
                            line!()
                        )))
                    }
                };
                match self.nodes_.get_mut(next_i) {
                    Some(Some(node)) => {
                        node.prev = new_prev;
                    }
                    _ => {
                        return Err(BvError::LinkedList(format!(
                            "Should not happen error™ at {}:{}",
                            file!(),
                            line!()
                        )))
                    }
                };
            }
            (None, Some((new_head, new_head_prev))) => match self.nodes_.get_mut(new_head) {
                Some(Some(node)) => {
                    node.prev = new_head_prev;
                    self.head_ = new_head;
                }
                _ => {
                    return Err(BvError::LinkedList(format!(
                        "Should not happen error™ at {}:{}",
                        file!(),
                        line!()
                    )))
                }
            },
            (Some((new_tail, new_tail_next)), None) => match self.nodes_.get_mut(new_tail) {
                Some(Some(node)) => {
                    node.next = new_tail_next;
                    self.tail_ = new_tail;
                }
                _ => {
                    return Err(BvError::LinkedList(format!(
                        "Should not happen error™ at {}:{}",
                        file!(),
                        line!()
                    )))
                }
            },
            (None, None) => {
                self.head_ = usize::MAX;
                self.tail_ = usize::MAX
            }
        }
        match self.nodes_.get_mut(operation.erase) {
            Some(old_head) => {
                if only_disconnect {
                    // only disconnect the node, i.e. leave it in place - disconnected.
                    if let Some(old_head) = old_head {
                        return Ok((old_head.prev, old_head.key.clone(), old_head.next));
                    }
                } else {
                    // Replace the node with None
                    if let Some(old_head) = old_head.take() {
                        self.id_pool_.push(operation.erase);
                        return Ok((old_head.prev, old_head.key, old_head.next));
                    }
                }
                return Err(BvError::LinkedList(format!(
                    "Should not happen error™ at {}:{}",
                    file!(),
                    line!()
                )));
            }
            _ => {
                return Err(BvError::LinkedList(format!(
                    "Should not happen error™, element to erase not found {} at {}:{}",
                    operation.erase,
                    file!(),
                    line!()
                )))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ListIterator<'a, T: 'a, U: 'a>
where
    T: Clone + Debug,
    U: Clone + Debug,
{
    list: &'a LinkedList<T, U>,
    my_next: usize,
}

impl<'a, T: 'a, U: 'a> Iterator for ListIterator<'a, T, U>
where
    T: Clone + Debug,
    U: Clone + Debug,
{
    type Item = &'a T;

    #[inline]
    /// Step the iterator forward one step
    fn next(&mut self) -> Option<&'a T> {
        if self.my_next == usize::MAX {
            return None;
        }
        //println!("Returning value at index:{}", self.my_next);
        if let Some(node) = self.list.nodes_.get(self.my_next)? {
            if self.my_next == self.list.tail_ {
                self.my_next = usize::MAX;
            } else {
                self.my_next = node.next
            }
            Some(&node.key)
        } else {
            self.my_next = usize::MAX;
            None
        }
    }
}

impl<'a, T: 'a, U: 'a> DoubleEndedIterator for ListIterator<'a, T, U>
where
    T: Clone + Debug,
    U: Clone + Debug,
{
    #[inline]
    /// Step the iterator backward one step
    fn next_back(&mut self) -> Option<&'a T> {
        if let Some(node) = self.list.nodes_.get(self.my_next)? {
            if self.my_next == self.list.tail_ {
                self.my_next = usize::MAX;
            } else {
                self.my_next = node.prev
            }
            Some(&node.key)
        } else {
            self.my_next = usize::MAX;
            None
        }
    }
}

/// An effort to emulate a C++ map iterator in Rust
/// It will have functionality like:
/// prev(), next(), get(), erase(), lower_bound(), replace(), insert_before(), insert_after()
pub struct Pointer<T, U>
where
    T: Clone + Debug,
    U: Clone + Debug,
{
    current: usize,
    list: Rc<RefCell<LinkedList<T, U>>>,
}

#[allow(dead_code)]
impl<T, U> Pointer<T, U>
where
    T: Clone + Debug + Unpin + Ord + PartialOrd,
    U: Clone + Debug + Unpin,
{
    /// Initiates the pointer with a list, set current to the head of the list.
    pub fn new(list: Rc<RefCell<LinkedList<T, U>>>) -> Self {
        let head = list.borrow().head_;
        Self {
            current: head,
            list,
        }
    }

    /// Initiates the pointer with a list, set index.
    pub fn new_2(list: Rc<RefCell<LinkedList<T, U>>>, current: usize) -> Self {
        Self { current, list }
    }

    #[inline(always)]
    /// Returns a clone of the data at current position
    pub fn get_k(&self) -> Result<T, BvError> {
        if self.current == usize::MAX {
            //panic!();
            return Err(BvError::LinkedList(format!(
                "Invalid pointer (moved past start/end). {}:{}",
                file!(),
                line!()
            )));
        }
        let node = self
            .list
            .borrow()
            .nodes_
            .get(self.current)
            .ok_or_else(|| {
                BvError::LinkedList(format!(
                    "Node {} not found. {}:{}",
                    self.current,
                    file!(),
                    line!()
                ))
            })?
            .as_ref()
            .ok_or_else(|| {
                BvError::LinkedList(format!(
                    "Node {} was None. {}:{}",
                    self.current,
                    file!(),
                    line!()
                ))
            })?
            .clone();
        Ok(node.key)
    }

    #[inline(always)]
    /// Returns a clone of the data at current position
    pub fn get_v(&self) -> Result<U, BvError> {
        let node = self
            .list
            .borrow()
            .nodes_
            .get(self.current)
            .ok_or_else(|| {
                BvError::LinkedList(format!(
                    "Node {} not found. {}:{}",
                    self.current,
                    file!(),
                    line!()
                ))
            })?
            .as_ref()
            .ok_or_else(|| {
                BvError::LinkedList(format!(
                    "Node {} was None. {}:{}",
                    self.current,
                    file!(),
                    line!()
                ))
            })?
            .clone();
        Ok(node.value)
    }

    /*
    #[inline(always)]
    /// Returns a mutable reference of the data at current position
    pub fn get_mut(&mut self) -> Result<(&mut T, &mut U), BvError> {
        unimplemented!();/*
        let mut node = self
            .list
            .borrow_mut()
            .nodes_
            .get_mut(self.current)
            .ok_or_else(|| {
                BvError::LinkedList(format!(
                    "Node {} not found. {}:{}",
                    self.current,
                    file!(),
                    line!()
                ))
            })?.as_ref()
            .ok_or_else(|| {
                BvError::LinkedList(format!(
                    "Node {} was None. {}:{}",
                    self.current,
                    file!(),
                    line!()
                ))
            })?;
        Ok((node.key, node.data))*/
    }*/

    #[inline(always)]
    /// Move to the next element
    pub fn next(&mut self) -> Result<(), BvError> {
        let list_borrow = self.list.borrow();
        match list_borrow.nodes_.get(self.current) {
            Some(Some(node)) => self.current = node.next,
            // Some(None) nodes should be inaccessible
            Some(None) => {
                eprintln!("next() failed at index:{}", self.current);
                for (i, n) in list_borrow.nodes_.iter().enumerate() {
                    eprintln!(" #{}, {:?}", i, n);
                }
                panic!();
                /*
                return Err(BvError::LinkedList(format!(
                    "next() failed at index:{}. {}:{}",
                    self.current,
                    file!(),
                    line!()
                )))*/
            }
            None => self.current = usize::MAX,
        }
        Ok(())
    }

    #[inline(always)]
    /// Move to the previous element
    pub fn prev(&mut self) -> Result<(), BvError> {
        let list_borrow = self.list.borrow();
        match list_borrow.nodes_.get(self.current) {
            Some(Some(node)) => self.current = node.prev,
            // Some(None) nodes should be inaccessible
            Some(None) => {
                eprintln!("prev() failed at index:{}", self.current);
                for (i, n) in list_borrow.nodes_.iter().enumerate() {
                    eprintln!(" #{}, {:?}", i, n);
                }
                return Err(BvError::LinkedList(format!(
                    "prev() failed at index:{}. {}:{}",
                    self.current,
                    file!(),
                    line!()
                )));
            }
            None => self.current = usize::MAX,
        }
        Ok(())
    }

    #[inline(always)]
    /// Move to the first element
    pub fn move_to_head(&mut self) {
        self.current = self.list.borrow().head_
    }

    #[inline(always)]
    /// Move to the last element
    pub fn move_to_tail(&mut self) {
        self.current = self.list.borrow().tail_
    }

    #[inline(always)]
    /// Return true if pointer has moved past beginning or end of the list
    pub fn is_ok(&self) -> bool {
        self.current != usize::MAX
            && matches!(self.list.borrow().nodes_.get(self.current), Some(Some(_)))
    }

    #[inline(always)]
    /// Return true if pointer is at head position
    pub fn is_at_head(&self) -> bool {
        self.current == self.list.borrow().head_
    }

    #[inline(always)]
    /// Return true if pointer is at tail position
    pub fn is_at_tail(&self) -> bool {
        self.current == self.list.borrow().tail_
    }
    /*
        #[inline(always)]
        /// same as iterator == Container.begin() in c++
        pub fn is_at_beginning(&self) -> bool {
            self.current == self.list.borrow().head_
        }
    */
    #[inline(always)]
    /// Replace current key. This will destroy the internal order of element if you
    /// replace an element with something out of order.
    pub fn replace_key(&mut self, key: T) {
        {
            let mut list = std::pin::Pin::new(self.list.borrow_mut());
            if let Some(Some(ref mut node)) = list.nodes_.get_mut(self.current) {
                node.key = key;
            }
        }
    }

    #[inline(always)]
    /// Inserts a new element before this with *value*
    /// This will destroy the internal ordering of elements if you
    /// insert an 'out of order' element.
    pub fn insert_before(&mut self, key: T, value: U) -> Result<usize, BvError> {
        self.list
            .borrow_mut()
            .insert_before(self.current, key, value)
    }

    #[inline(always)]
    /// Inserts a new element after this with *value*
    /// This will destroy the internal ordering of elements if you
    /// insert an 'out of order' element.
    // todo: implement
    pub fn insert_after(&mut self, _value: T) -> Result<(), BvError> {
        unimplemented!();
        // self.list.borrow_mut().insert_after(self.current,value);
    }

    #[inline(always)]
    /// returns current index
    pub fn get_index(&self) -> usize {
        self.current
    }

    #[inline(always)]
    /// Remove the current element and return it. Move current to the old prev value if exist.
    /// Else pick old next index.
    /// Note: make sure that there are no other Pointer objects at this position.
    pub fn remove_current(&mut self, only_disconnect: bool) -> Result<T, BvError> {
        let rv = self
            .list
            .borrow_mut()
            .remove_(self.current, only_disconnect)?;
        if rv.0 != usize::MAX {
            self.current = rv.0;
        } else {
            self.current = rv.2;
        }
        Ok(rv.1)
    }

    #[inline(always)]
    /// Returns a new Pointer positioned at the lower bound item.
    /// Lower bound item is the first element in the container whose key is not considered to go
    /// before position (i.e., either it is equivalent or goes after).
    /// Returns a Pointer where is_ok() returns false if no data is found
    pub fn lower_bound(&self, key: T) -> Result<Self, BvError> {
        if let Some(position) = self.list.borrow().lower_bound(key)? {
            Ok(Self {
                list: Rc::clone(&self.list),
                current: position,
            })
        } else {
            Ok(Self {
                list: Rc::clone(&self.list),
                current: usize::MAX,
            })
        }
    }
}

impl<T, U> Debug for Pointer<T, U>
where
    T: Clone + Debug + Unpin + Ord + PartialOrd,
    U: Clone + Debug + Unpin,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Pointer({})", self.current)
    }
}

impl<T, U> Clone for Pointer<T, U>
where
    T: Clone + Debug + Unpin + Ord + PartialOrd,
    U: Clone + Debug + Unpin,
{
    fn clone(&self) -> Self {
        Self {
            current: self.current,
            list: Rc::clone(&self.list),
        }
    }
}
