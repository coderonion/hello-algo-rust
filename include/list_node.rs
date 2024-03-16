/*
 * File: list_node.rs
 * Created Time: 2023-03-05
 * Author: codingonion (coderonion@gmail.com)
 */

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Rc<RefCell<ListNode<T>>> {
        Rc::new(RefCell::new(ListNode {
            val,
            next: None,
        }))
    }
    /* Generate a linked list with an array */
    pub fn arr_to_linked_list(array: &[T]) -> Option<Rc<RefCell<ListNode<T>>>>
    where
        T: Copy + Clone,
    {
        let mut head = None;
        let mut prev = None;
        for item in array.iter().rev() {
            let node = Rc::new(RefCell::new(ListNode {
                val: *item,
                next: prev.take(),
            }));
            prev = Some(node.clone());
            head = Some(node);
        }
        head
    }

    /* Generate a hashmap with a linked_list */
    pub fn linked_list_to_hashmap(
        linked_list: Option<Rc<RefCell<ListNode<T>>>>,
    ) -> HashMap<T, Rc<RefCell<ListNode<T>>>>
    where
        T: std::hash::Hash + Eq + Copy + Clone
    {
        let mut hashmap = HashMap::new();
        let mut current = linked_list;
        while let Some(cur) = current {
            hashmap.insert(cur.borrow().val, cur.clone());
            current = cur.borrow().next.clone();
        }
        hashmap
    }
}
