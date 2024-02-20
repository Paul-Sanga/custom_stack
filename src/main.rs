use stack_impl::stack::Stack;

fn main() {
    let mut stack: Stack<char> = Stack::new();
    stack.push('P');
    stack.push('A');
    if let Some(node) = stack.pop(){
        println!("Removed {}", node.borrow());
    }
    if let Some(node) = stack.pop(){
        println!("Removed {}", node.borrow());
    }
    stack.print_stack();
}
