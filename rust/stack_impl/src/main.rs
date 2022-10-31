mod stack;

use stack::Stack;

fn main() {
    let mut my_stack = Stack::new(Some(5));
    my_stack.push(2);
    my_stack.push(3);
    my_stack.push(5);

    let mut my_stack2 = Stack::new(None);
    my_stack2.push(3);
    my_stack2.push(2);
    my_stack2.push(1);

    println!("------------");
    let mut my_stack3 = Stack::new(None);
    my_stack3.push("ahmed");
    my_stack3.push("rami");
    my_stack3.push("mustafa");
}
