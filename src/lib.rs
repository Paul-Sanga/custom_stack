#![allow(dead_code)]
pub mod stack {

    use std::rc::Rc;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Node<T> {
        value: T,
        next: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T: std::fmt::Display> std::fmt::Display for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Node: {}", self.value)
        }
    }

    impl<T: std::fmt::Display> Node<T> {
        fn new(value: T) -> Self {
            Node { value, next: None }
        }
    }

    #[derive(Debug)]
    pub struct Stack<T> {
        count: usize,
        peek: Option<Rc<RefCell<Node<T>>>>,
        base: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T: std::fmt::Display> std::fmt::Display for Stack<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut current = self.base.clone();

            while let Some(node) = current {
                write!(f, "{}, ", node.borrow())?;
                current = node.borrow().next.clone();
            }

            Ok(())
        }
    }

    impl<T: std::fmt::Display> Stack<T> {
        pub fn new() -> Self {
            Stack {
                count: 0,
                peek: None,
                base: None,
            }
        }

        pub fn push(&mut self, value: T) {
            let new_node = Rc::new(RefCell::new(Node::new(value)));
            match self.peek.take() {
                Some(node) => {
                    node.borrow_mut().next = Some(Rc::clone(&new_node));
                    self.peek = Some(new_node);
                    self.count += 1;
                }
                None => {
                    self.peek = Some(Rc::clone(&new_node));
                    self.base = Some(new_node);
                    self.count += 1;
                }
            }
        }

        pub fn pop(&mut self) {
            let mut current = self.base.clone();
            let mut prev: Option<Rc<RefCell<Node<T>>>> = None;

            if self.count == 0{
                println!("stack is empty")
            }else if self.count == 1 {
                self.base = None;
                self.peek = None;
                self.count -= 1;
            }else {
                while let Some(node) = current.clone() {
                    if node.borrow().next.is_none(){
                        break;
                    }else {
                        prev = Some(Rc::clone(&node));
                        current = node.borrow().next.clone();
                    }
                }
                if let Some(node) = prev.clone(){
                    node.borrow_mut().next = None;
                    self.peek = Some(Rc::clone(&node));
                    self.count -= 1;
                }   
            }
        }

        pub fn is_empty(&self)->bool{
            if self.count == 0{
                true
            }else {
                false
            }
        }

        pub fn peek(&self)->Option<Rc<RefCell<Node<T>>>> {
            self.peek.clone()
        }

        pub fn print_stack(&self) {
            println!("{}", self);
        }
    }
}
