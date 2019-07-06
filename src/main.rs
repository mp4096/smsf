use smsflib::prelude::*;

fn main() {
    let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    println!("{}", stack);
    stack.rotate_up();
    println!("After rotation (up):\n{}", stack);
    stack.rotate_down();
    println!("After rotation (down):\n{}", stack);
    stack.swap();
    println!("After swap:\n{}", stack);
    stack.swap();
    println!("After swap:\n{}", stack);
    stack.drop();
    println!("After drop:\n{}", stack);
    dbg! {stack.pop()};
    println!("After pop:\n{}", stack);
    stack.push(7);
    println!("After pushing 7:\n{}", stack);
    stack.clear();
    println!("After clearing:\n{}", stack);
    stack.push(1);
    println!("After pushing 1:\n{}", stack);
    stack.push(2);
    println!("After pushing 2:\n{}", stack);
    stack.push(3);
    println!("After pushing 3:\n{}", stack);
    stack.push(4);
    println!("After pushing 4:\n{}", stack);
}
