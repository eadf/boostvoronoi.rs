use super::LinkedList;
use super::Pointer;
use crate::BvError;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn linked_list_test1() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.push_front(5, 0)?;
    let _ = ll.push_front(4, 1)?;
    let _ = ll.push_front(3, 2)?;
    let _ = ll.push_front(2, 3)?;
    let _ = ll.push_front(1, 4)?;
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [1_i8, 2, 3, 4, 5].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_test2() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.push_front(5, 0)?; // 0
    let _ = ll.push_front(4, 1)?; // 1
    let _ = ll.push_front(3, 2)?; // 2
    let _ = ll.push_front(2, 3)?; // 3
    let _ = ll.insert_before(3, 6, 4)?; // 4
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [6_i8, 2, 3, 4, 5].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_test3() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.push_front(5, 0)?; // 0
    let _ = ll.push_front(4, 1)?; // 1
    let _ = ll.push_front(3, 2)?; // 2
    let _ = ll.push_front(2, 3)?; // 3
    let _ = ll.insert_before(0, 6, 4)?; // 4
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [2_i8, 3, 4, 6, 5].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_test4() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.push_front(5, 0)?; // 0
    let _ = ll.insert_before(0, 4, 1)?; // 1
    let _ = ll.insert_before(1, 3, 2)?; // 2
    let _ = ll.insert_before(2, 2, 3)?; // 3
    let _ = ll.insert_before(3, 6, 4)?; // 4
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [6_i8, 2, 3, 4, 5].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_test5() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.push_front(5, 0)?; // 0
    let _ = ll.ordered_insert(4, 1)?; // 1
    let _ = ll.ordered_insert(3, 2)?; // 2
    let _ = ll.ordered_insert(2, 3)?; // 3
    let _ = ll.ordered_insert(6, 4)?; // 4
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [2_i8, 3, 4, 5, 6].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_test6() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.ordered_insert(5, 0)?; // 0
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [5_i8].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_test7() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.ordered_insert(5, 0)?; // 0
    let _ = ll.ordered_insert(5, 0)?; // 0
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [5_i8, 5].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_test8() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.ordered_insert(5, 0)?; // 0
    let v = ll.pop_front()?;
    assert_eq!(v, Some(5));
    let _ = ll.ordered_insert(1, 0)?; // 0
    let _ = ll.ordered_insert(2, 1)?; // 1
    let v = ll.pop_front()?;
    assert_eq!(v, Some(1));
    let v = ll.pop_front()?;
    assert_eq!(v, Some(2));
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_test9() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.ordered_insert(5, 0)?; // 0
    let v = ll.pop_front()?;
    assert_eq!(v, Some(5));
    let _ = ll.ordered_insert(1, 0)?; // 0
    let _ = ll.ordered_insert(2, 1)?; // 1
    let v = ll.pop_front()?;
    assert_eq!(v, Some(1));
    let v = ll.pop_front()?;
    assert_eq!(v, Some(2));
    let _ = ll.ordered_insert(5, 0)?;
    let _ = ll.ordered_insert(1, 1)?;
    let _ = ll.ordered_insert(2, 2)?;
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [1_i8, 2, 5].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_test10() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.ordered_insert(1, 0)?; // 0
    let _ = ll.ordered_insert(2, 1)?; // 1
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [1_i8, 2].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    let v = ll.remove(1)?;
    assert_eq!(v, Some(2));
    let v = ll.remove(0)?;
    assert_eq!(v, Some(1));
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_test11() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.ordered_insert(1, 0)?; // 0
    let _ = ll.ordered_insert(2, 1)?; // 1
    let _ = ll.ordered_insert(3, 2)?; // 2
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [1_i8, 2, 3].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    let v = ll.remove(2)?;
    assert_eq!(v, Some(3));
    let v = ll.remove(0)?;
    assert_eq!(v, Some(1));
    let v = ll.remove(1)?;
    assert_eq!(v, Some(2));
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
/// check that old indices are reused.
fn linked_list_test12() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.ordered_insert(1, 0)?; // 0
    let _ = ll.ordered_insert(2, 1)?; // 1
    let _ = ll.ordered_insert(3, 2)?; // 2
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [1_i8, 2, 3].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    let v = ll.remove(2)?;
    assert_eq!(v, Some(3));
    let v = ll.remove(0)?;
    assert_eq!(v, Some(1));
    let v = ll.remove(1)?;
    assert_eq!(v, Some(2));

    let _ = ll.ordered_insert(1, 1)?; // 1
    let _ = ll.ordered_insert(2, 0)?; // 0
    let _ = ll.ordered_insert(3, 2)?; // 2
    println!("ll:{:?}", ll);

    assert_eq!(*ll.get_k(0)?, 2);
    assert_eq!(*ll.get_k(1)?, 1);
    assert_eq!(*ll.get_k(2)?, 3);

    let _ = ll.remove(0)?;
    let _ = ll.remove(1)?;
    let _ = ll.remove(2)?;

    let _ = ll.push_front(1, 0)?; // 2
    let _ = ll.push_front(2, 0)?; // 1
    let _ = ll.push_front(3, 0)?; // 0

    assert_eq!(*ll.get_k(2)?, 1);
    assert_eq!(*ll.get_k(1)?, 2);
    assert_eq!(*ll.get_k(0)?, 3);

    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [3_i8, 2, 1].iter().map(|x| *x).collect::<Vec<i8>>()
    );

    let _ = ll.remove(0)?;
    let _ = ll.remove(1)?;
    let _ = ll.remove(2)?;

    let _ = ll.push_back(1, 2)?; // 2
    let _ = ll.push_back(2, 1)?; // 1
    let _ = ll.push_back(3, 0)?; // 0

    assert_eq!(*ll.get_k(2)?, 1);
    assert_eq!(*ll.get_k(1)?, 2);
    assert_eq!(*ll.get_k(0)?, 3);

    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [1_i8, 2, 3].iter().map(|x| *x).collect::<Vec<i8>>()
    );

    Ok(())
}

#[test]
fn linked_list_lower_bound_01() -> Result<(), BvError> {
    let mut ll = LinkedList::<i8, i8>::default();
    let _ = ll.ordered_insert(0, 0)?; // 0
    let _ = ll.ordered_insert(1, 1)?; // 1
    let _ = ll.ordered_insert(2, 2)?; // 2
    let _ = ll.ordered_insert(5, 3)?; // 3
    assert_eq!(
        ll.iter().map(|x| *x).collect::<Vec<_>>(),
        [0_i8, 1, 2, 5].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    // Returns the first element in the container whose key is not considered to go
    // before position (i.e., either it is equivalent or goes after).
    // Returns None if no data is found
    let v = ll.lower_bound(0)?;
    assert_eq!(v, Some(0));
    let v = ll.lower_bound(1)?;
    assert_eq!(v, Some(1));
    let v = ll.lower_bound(2)?;
    assert_eq!(v, Some(2));
    let v = ll.lower_bound(5)?;
    assert_eq!(v, Some(3));
    let v = ll.lower_bound(15)?;
    assert_eq!(v, None);
    Ok(())
}

#[test]
fn linked_list_lower_bound_02() -> Result<(), BvError> {
    let ll = LinkedList::<i8, i8>::default();
    let v = ll.lower_bound(0)?;
    assert_eq!(v, None);
    Ok(())
}

#[test]
fn linked_list_pointer_test01() -> Result<(), BvError> {
    let ll = Rc::from(RefCell::from(LinkedList::<i8, i8>::default()));
    let _ = ll.borrow_mut().ordered_insert(1, 0)?; // 0
    let _ = ll.borrow_mut().ordered_insert(2, 1)?; // 1
    let _ = ll.borrow_mut().ordered_insert(3, 2)?; // 2
    assert_eq!(
        ll.borrow().iter().map(|x| *x).collect::<Vec<_>>(),
        [1_i8, 2, 3].iter().map(|x| *x).collect::<Vec<i8>>()
    );

    let mut p = Pointer::new(Rc::clone(&ll));
    p.next()?;
    p.next()?;
    let v = p.get_k()?;
    assert_eq!(v, 3);
    let v = p.remove_current(false)?;
    assert_eq!(v, 3);
    let v = p.remove_current(false)?;
    assert_eq!(v, 2);
    let v = p.remove_current(false)?;
    assert_eq!(v, 1);
    assert_eq!(
        ll.borrow().iter().map(|x| *x).collect::<Vec<_>>(),
        [].iter().map(|x| *x).collect::<Vec<i8>>()
    );
    Ok(())
}

#[test]
fn linked_list_pointer_test02() -> Result<(), BvError> {
    let ll = Rc::from(RefCell::from(LinkedList::<i8, i8>::default()));
    let p = Pointer::new(Rc::clone(&ll));
    let v = p.lower_bound(1)?;
    assert!(!v.is_ok());
    Ok(())
}

#[test]
fn linked_list_pointer_test03() -> Result<(), BvError> {
    let ll = Rc::from(RefCell::from(LinkedList::<i8, i8>::default()));
    let _ = ll.borrow_mut().ordered_insert(1, 0)?; // 0
    let _ = ll.borrow_mut().ordered_insert(2, 1)?; // 1
    let _ = ll.borrow_mut().ordered_insert(3, 2)?; // 2
    let _ = ll.borrow_mut().ordered_insert(4, 3)?; // 3
    let p = Pointer::new(Rc::clone(&ll));
    let lb = p.lower_bound(3)?;
    assert!(lb.is_ok());
    assert_eq!(lb.get_k()?, 3);

    let lb = p.lower_bound(1)?;
    assert!(lb.is_ok());
    assert_eq!(lb.get_k()?, 1);

    let v = p.lower_bound(5)?;
    assert!(!v.is_ok());
    Ok(())
}
