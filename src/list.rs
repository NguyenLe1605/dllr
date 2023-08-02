use std::fmt::Display;
use std::{cell::RefCell, rc::Rc};

struct Node {
    value: String,
    next: Link,
    prev: Link,
}

impl Node {
    fn new(value: String, next: Link, prev: Link) -> Node {
        return Node { value, next, prev };
    }
}

pub struct List {
    head: Link,
    tail: Link,
    size: usize,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        let mut idx = 0;
        let mut tmp = self.head.clone();
        while let Some(node) = tmp {
            let value = node.borrow().value.clone();
            string.push_str(&value);
            tmp = node.borrow().next.clone();
            idx += 1;
            if idx < self.size {
                string.push_str(" <-> ");
            }
        }

        write!(f, "{}", string)
    }
}

impl List {
    pub fn new() -> List {
        List {
            size: 0,
            head: None,
            tail: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    pub fn get_size(&self) -> usize {
        return self.size;
    }

    // Insert new node at the front of the list.
    pub fn insert_front(&mut self, value: String) {
        let node = Rc::new(RefCell::new(Node::new(value, None, None)));
        self.size += 1;
        match &self.head {
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
            Some(head) => {
                head.borrow_mut().prev = Some(Rc::clone(&node));
                node.borrow_mut().next = self.head.take();
                self.head = Some(node);
            }
        }
    }

    pub fn insert_back(&mut self, value: String) {
        match &self.tail {
            None => self.insert_front(value),
            Some(tail) => {
                self.size += 1;
                let node = Rc::new(RefCell::new(Node::new(value, None, None)));
                tail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = self.tail.take();
                self.tail = Some(node);
            }
        }
    }

    // Get the node at index, return None if index is less than 0 or larger than size or the list is empty
    pub fn get(&self, index: isize) -> Option<String> {
        if index < 0 || index >= self.size as isize {
            return None;
        }

        let mut tmp = self.head.clone();
        let mut idx: isize = 0;
        while let Some(node) = tmp {
            if idx == index {
                return Some(node.borrow().value.clone());
            }

            tmp = node.borrow().next.clone();
            idx += 1;
        }
        return None;
    }

    // Delete the head node, return the value inside the head
    pub fn delete_front(&mut self) -> Option<String> {
        if self.head.is_none() {
            return None;
        }

        self.size -= 1;

        let value = self.head.clone().unwrap().borrow().value.clone();
        let next_head = self.head.clone().unwrap().borrow().next.clone();

        if next_head.is_some() {
            next_head.clone().unwrap().borrow_mut().prev = None;
        }

        self.head = next_head;

        return Some(value);
    }

    // Delete the tail node, return the value inside the tail
    pub fn delete_back(&mut self) -> Option<String> {
        if self.tail.is_none() {
            return None;
        }

        if self.size == 1 {
            self.tail = None;
            return self.delete_front();
        }

        self.size -= 1;

        let value = self.tail.clone().unwrap().borrow().value.clone();
        let next_tail = self.tail.clone().unwrap().borrow().prev.clone();

        if next_tail.is_some() {
            next_tail.clone().unwrap().borrow_mut().next = None;
        }

        self.tail = next_tail;

        return Some(value);
    }

    // Get the front of the list
    pub fn front(&self) -> Option<String> {
        match &self.head {
            None => None,
            Some(node) => Some(node.borrow().value.clone()),
        }
    }

    // Get the back of the list
    pub fn back(&self) -> Option<String> {
        match &self.tail {
            None => None,
            Some(node) => Some(node.borrow().value.clone()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_list() {
        let empty = List::new();
        assert!(empty.is_empty());
        assert_eq!(empty.get_size(), 0);
        assert!(empty.front().is_none());
    }

    #[test]
    fn insert_one_value_at_front() {
        let mut list = List::new();
        list.insert_front("1".to_string());

        assert!(!list.is_empty());
        assert_eq!(list.get_size(), 1);
        assert_eq!("1", list.front().unwrap());
    }

    #[test]
    fn insert_one_value_at_back() {
        let mut list = List::new();
        list.insert_back("1".to_string());

        assert!(!list.is_empty());
        assert_eq!(list.get_size(), 1);
        assert_eq!("1", list.back().unwrap());
        assert_eq!(list.back().unwrap(), list.front().unwrap());
    }

    #[test]
    fn insert_multiple_values_at_front() {
        let mut list = List::new();
        list.insert_front("1".to_string());
        list.insert_front("2".to_string());
        list.insert_front("3".to_string());
        list.insert_front("4".to_string());
        list.insert_front("5".to_string());
        assert!(!list.is_empty());
        assert_eq!(list.get_size(), 5)
    }

    #[test]
    fn insert_multiple_values_at_back() {
        let mut list = List::new();
        list.insert_back("1".to_string());
        list.insert_back("2".to_string());
        list.insert_back("3".to_string());
        list.insert_back("4".to_string());
        list.insert_back("5".to_string());
        assert!(!list.is_empty());
        assert_eq!(list.get_size(), 5);

        assert_eq!(list.get(0).unwrap(), "1");
        assert_eq!(list.get(1).unwrap(), "2");
        assert_eq!(list.get(2).unwrap(), "3");
        assert_eq!(list.get(3).unwrap(), "4");
        assert_eq!(list.get(4).unwrap(), "5");
    }

    #[test]
    fn insert_back_and_delete_front() {
        let mut list = List::new();

        list.insert_back("3".to_string());
        list.insert_back("4".to_string());

        list.insert_front("2".to_string());
        list.insert_front("1".to_string());

        assert_eq!(list.delete_front().unwrap(), "1");

        assert_eq!(list.get_size(), 3);

        assert_eq!(list.front().unwrap(), "2");
        assert_eq!(list.back().unwrap(), "4");

        assert_eq!(list.get(1).unwrap(), "3");
    }

    #[test]
    fn get_one_value() {
        let mut list = List::new();
        list.insert_front("1".to_string());
        assert_eq!(list.get(0).unwrap(), "1");
    }

    #[test]
    fn get_multiple_values() {
        let mut list = List::new();
        list.insert_front("1".to_string());
        list.insert_front("2".to_string());
        list.insert_front("3".to_string());

        assert_eq!(list.get(0).unwrap(), "3");
        assert_eq!(list.get(1).unwrap(), "2");
        assert_eq!(list.get(2).unwrap(), "1");
    }

    #[test]
    fn error_case_with_get() {
        let mut list = List::new();
        assert!(list.get(0).is_none());
        assert!(list.get(1).is_none());
        assert!(list.get(-1).is_none());

        list.insert_front("1".to_string());
        assert_eq!(list.front().unwrap(), "1");
        assert!(list.get(2).is_none());
        assert!(list.get(-1).is_none());

        list.insert_front("2".to_string());
        list.insert_front("3".to_string());
        assert!(list.get(10).is_none());
        assert!(list.get(-999).is_none());
    }

    #[test]
    fn delete_one_value_at_front() {
        let mut list = List::new();
        list.insert_front("1".to_string());
        assert_eq!(list.delete_front().unwrap(), "1");
        assert_eq!(list.get_size(), 0);
        assert_eq!(list.front(), None);
    }

    #[test]
    fn delete_one_value_at_back() {
        let mut list = List::new();
        list.insert_back("1".to_string());
        assert_eq!(list.delete_back().unwrap(), "1");
        assert_eq!(list.get_size(), 0);
        assert_eq!(list.back(), None);
        assert_eq!(list.front(), None);
    }

    #[test]
    fn delete_multiple_values_at_front() {
        let mut list = List::new();
        list.insert_front("1".to_string());
        list.insert_front("2".to_string());
        list.insert_front("3".to_string());

        assert_eq!(list.delete_front().unwrap(), "3");
        assert_eq!(list.front().unwrap(), "2");
        assert_eq!(list.get_size(), 2);

        assert_eq!(list.delete_front().unwrap(), "2");
        assert_eq!(list.front().unwrap(), "1");
        assert_eq!(list.get_size(), 1);

        assert_eq!(list.delete_front().unwrap(), "1");
        assert_eq!(list.front(), None);
        assert_eq!(list.get_size(), 0);

        assert_eq!(list.delete_front(), None);
    }

    #[test]
    fn delete_multiple_values_at_back() {
        let mut list = List::new();
        list.insert_back("1".to_string());
        list.insert_back("2".to_string());
        list.insert_back("3".to_string());

        assert_eq!(list.delete_back().unwrap(), "3");
        assert_eq!(list.back().unwrap(), "2");
        assert_eq!(list.get_size(), 2);

        assert_eq!(list.delete_back().unwrap(), "2");
        assert_eq!(list.back().unwrap(), "1");
        assert_eq!(list.get_size(), 1);

        assert_eq!(list.delete_back().unwrap(), "1");
        assert_eq!(list.back(), None);
        assert_eq!(list.front(), None);
        assert_eq!(list.get_size(), 0);

        assert_eq!(list.delete_back(), None);
    }

    #[test]
    fn delete_front_empty_list_multiples() {
        let mut list = List::new();
        assert_eq!(list.delete_front(), None);
        assert_eq!(list.delete_front(), None);
        assert_eq!(list.delete_front(), None);
        assert_eq!(list.delete_front(), None);
    }

    #[test]
    fn insert_front_and_delete_front() {
        let mut list = List::new();
        list.insert_front("1".to_string());
        list.insert_front("2".to_string());
        assert_eq!(list.get_size(), 2);
        assert_eq!(list.get(1).unwrap(), "1");

        assert_eq!(list.delete_front().unwrap(), "2");
        assert_eq!(list.get_size(), 1);

        list.insert_front("3".to_string());
        assert_eq!(list.get_size(), 2);
        assert_eq!(list.get(0).unwrap(), "3");
        assert_eq!(list.get(1).unwrap(), "1");
    }

    #[test]
    fn multiple_insert_and_delete() {
        let mut list = List::new();

        list.insert_front("2".to_string());
        list.insert_front("1".to_string());

        list.insert_back("3".to_string());
        list.insert_back("4".to_string());

        assert_eq!(list.get_size(), 4);

        assert_eq!(list.delete_back().unwrap(), "4");
        assert_eq!(list.get_size(), 3);

        assert_eq!(list.delete_front().unwrap(), "1");
        assert_eq!(list.get_size(), 2);

        assert_eq!(list.delete_back().unwrap(), "3");
        assert_eq!(list.get_size(), 1);

        assert_eq!(list.delete_back().unwrap(), "2");
        assert_eq!(list.get_size(), 0);

        assert_eq!(list.back(), None);
        assert_eq!(list.front(), list.back());
    }

    #[test]
    fn test_fmt_string() {
        let mut list = List::new();
        assert_eq!(list.to_string(), "");
        list.insert_front("2".to_string());
        assert_eq!(list.to_string(), "2");

        list.insert_back("3".to_string());
        assert_eq!(list.to_string(), "2 <-> 3");

        list.insert_front("1".to_string());
        assert_eq!(list.to_string(), "1 <-> 2 <-> 3");

        list.insert_back("4".to_string());
        assert_eq!(list.to_string(), "1 <-> 2 <-> 3 <-> 4");
    }
}
