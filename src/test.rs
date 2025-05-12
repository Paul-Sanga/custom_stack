#[cfg(test)]
use super::*;

#[test]
pub fn test_push() {
    let mut stack: Stack<u32> = Stack::new(5);
    stack.push(5).unwrap();
    assert_eq!(stack.base(), Some(5));
    assert_eq!(stack.top(), Some(5));
    assert_eq!(stack.size(), 1);
    stack.push(6).unwrap();
    assert_eq!(stack.base(), Some(5));
    assert_eq!(stack.top(), Some(6));
    assert_eq!(stack.size(), 2);
    stack.push(7).unwrap();
    assert_eq!(stack.base(), Some(5));
    assert_eq!(stack.top(), Some(7));
    assert_eq!(stack.size(), 3);
    stack.push(8).unwrap();
    assert_eq!(stack.base(), Some(5));
    assert_eq!(stack.top(), Some(8));
    assert_eq!(stack.size(), 4);
    stack.push(9).unwrap();
    assert_eq!(stack.base(), Some(5));
    assert_eq!(stack.top(), Some(9));
    assert_eq!(stack.size(), 5);
    assert_eq!(stack.push(10), Err(format!("Stack is full")));
    // dbg!(stack);
}

#[test]
fn test_pop() {
    let mut stack: Stack<u32> = Stack::new(5);
    stack.push(5).unwrap();
    assert_eq!(stack.base(), Some(5));
    assert_eq!(stack.top(), Some(5));
    assert_eq!(stack.size(), 1);
    stack.push(6).unwrap();
    assert_eq!(stack.base(), Some(5));
    assert_eq!(stack.top(), Some(6));
    assert_eq!(stack.size(), 2);
    stack.push(7).unwrap();
    assert_eq!(stack.base(), Some(5));
    assert_eq!(stack.top(), Some(7));
    assert_eq!(stack.size(), 3);
    stack.push(8).unwrap();
    assert_eq!(stack.base(), Some(5));
    assert_eq!(stack.top(), Some(8));
    assert_eq!(stack.size(), 4);
    stack.push(9).unwrap();
    assert_eq!(stack.base(), Some(5));
    assert_eq!(stack.top(), Some(9));
    assert_eq!(stack.size(), 5);
    assert_eq!(stack.push(10), Err(format!("Stack is full")));
    assert_eq!(stack.pop().unwrap(), Some(9));
    assert_eq!(stack.size(), 4);
    assert_eq!(stack.pop().unwrap(), Some(8));
    assert_eq!(stack.size(), 3);
    assert_eq!(stack.pop().unwrap(), Some(7));
    assert_eq!(stack.size(), 2);
    assert_eq!(stack.pop().unwrap(), Some(6));
    assert_eq!(stack.size(), 1);
    assert_eq!(stack.pop().unwrap(), Some(5));
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.pop(), Err(format!("Stack is empty")));
    dbg!(stack);
}

#[test]
fn test_iter(){
    let mut stack: Stack<u32> = Stack::new(5);
    stack.push(5).unwrap();
    stack.push(6).unwrap();
    stack.push(7).unwrap();
    stack.push(8).unwrap();
    stack.push(9).unwrap();
    let mut result = String::new();
    for val in stack.iter(){
        result.push_str(&val.to_string());
    }
    assert_eq!(format!("98765"), result);
}