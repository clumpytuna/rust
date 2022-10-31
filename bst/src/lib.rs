#![forbid(unsafe_code)]

use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

#[derive(PartialEq)]
struct Node {
    key: i64,
    left_ptr: Option<Box<Node>>,
    right_ptr: Option<Box<Node>>,
}

#[derive(Default)]
pub struct BstSet {
    root: Option<Box<Node>>,
    size: usize,
}

impl BstSet {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0
        }
    }

    pub fn len(&self) -> usize {
        return self.size;
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    pub fn contains(&self, key: i64) -> bool {
        let mut current: &Option<Box<Node>> = &self.root;

        while let Some(node) = current {
            match node.key.cmp(&key) {
                Ordering::Equal => return true,
                Ordering::Less => {
                    current = &node.right_ptr;
                }
                Ordering::Greater => {
                    current = &node.left_ptr;
                }
            }
        }
        false
    }

    fn find_node_mut(&mut self, key: i64) -> &mut Option<Box<Node>> {
        let mut current = &mut self.root;

        while let Some(node) = current {
            match node.key.cmp(&key) {
                Ordering::Equal => { return current;}
                Ordering::Less => {
                    current = &mut current.as_mut().unwrap().right_ptr;
                }
                Ordering::Greater => {
                    current = &mut current.as_mut().unwrap().left_ptr;
                }
            }
        }
        current
    }

    pub fn insert(&mut self, key: i64) -> bool {
        // If the key is already contained in set, return false.
        // Otherwise insert the key and return true.
        let mut place_to_insert = self.find_node_mut(key);
        if place_to_insert.is_some() {
            return false;
        }

        *place_to_insert = Some(Box::new(Node {
                key,
                left_ptr: None,
                right_ptr: None,
            }));
        self.size += 1;
        true
    }

    fn find_min<'a>(&'a mut self, root: &'a mut Option<Box<Node>>) -> &mut Option<Box<Node>> {
        let mut current = root;
        while (current != &None) & (current.as_mut().unwrap().left_ptr != None) {
            current = &mut current.as_mut().unwrap().left_ptr;
        }

        return current;
    }

    pub fn remove(&mut self, key: i64) -> bool {
        let mut node_ptr = self.find_node_mut(key);
        let mut node = match node_ptr {
            Some(node) => node,
            None => return false,
        };

        while node.right_ptr.is_some() {
            node_ptr = Self::rotate_left(node_ptr);
            node = node_ptr.as_mut().unwrap();
        }

        *node_ptr = node.left_ptr.take();
        self.size -= 1;
        true
    }

    fn rotate_left(root_ptr: &mut Option<Box<Node>>) -> &mut Option<Box<Node>> {
        let mut root = root_ptr.take().unwrap();
        let mut right = root.right_ptr.unwrap();
        root.right_ptr = right.left_ptr;
        right.left_ptr = Some(root);
        *root_ptr = Some(right);
        &mut root_ptr.as_mut().unwrap().left_ptr
    }
}
