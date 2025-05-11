use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::{Rc, Weak},
};

mod test;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Copy + Display + PartialEq + PartialOrd + Debug> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}
#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    capacity: usize,
    top: Option<Rc<RefCell<Node<T>>>>,
    base: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy + Display + PartialEq + PartialOrd + Debug> Stack<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            size: 0,
            capacity,
            top: None,
            base: None,
        }
    }

    pub fn top(&self) -> Option<T> {
        self.top.as_ref().map(|top| top.borrow().value)
    }

    pub fn base(&self) -> Option<T> {
        self.base.as_ref().map(|base| base.borrow().value)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size >= self.capacity
    }

    pub fn push(&mut self, value: T) -> Result<(), String> {
        if self.is_full() {
            return Err(format!("Stack is full"));
        }

        let node = Rc::new(RefCell::new(Node::new(value)));

        if self.is_empty() {
            self.top = Some(Rc::clone(&node));
            self.base = Some(Rc::clone(&node));
        } else {
            if let Some(top) = self.top.as_ref() {
                top.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::downgrade(&top));
            }
            self.top = Some(node);
        }

        self.size += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Option<T>, String> {
        if self.is_empty() {
            return Err(format!("Stack is empty"));
        }

        let old_top_node = self.top.take();

        if let Some(top) = old_top_node.as_ref() {
            if let Some(new_top) = top.borrow().prev.as_ref() {
                if let Some(new_top) = new_top.upgrade() {
                    new_top.borrow_mut().next = None;
                    self.top = Some(new_top);
                }
            } else {
                self.base = None
            }
        }

        self.size -= 1;
        Ok(old_top_node.as_ref().map(|old_top| old_top.borrow().value))
    }
}
