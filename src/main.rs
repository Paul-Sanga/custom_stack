use stack_impl::stack::Stack;

fn main() {
    let mut stack: Stack<u32> = Stack::new();
    stack.push(32);
    stack.push(42);
    stack.pop();
    stack.print_stack();
}
