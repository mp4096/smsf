use smsflib::prelude::*;

fn main() {
    let mut stack = ClassicStack::<u32>::new(1, 2, 3, 4);
    println!("{}", stack);
    let _ = stack.rotate_up().unwrap();
    println!("After rotation (up):\n{}", stack);
    let _ = stack.rotate_down().unwrap();
    println!("After rotation (down):\n{}", stack);
    let _ = stack.swap().unwrap();
    println!("After swap:\n{}", stack);
    let _ = stack.swap().unwrap();
    println!("After swap:\n{}", stack);
    let _ = stack.drop().unwrap();
    println!("After drop:\n{}", stack);
    dbg! {stack.pop().unwrap()};
    println!("After pop:\n{}", stack);
    let _ = stack.push(7).unwrap();
    println!("After pushing 7:\n{}", stack);
    let _ = stack.clear().unwrap();
    println!("After clearing:\n{}", stack);
    let _ = stack.push(1).unwrap();
    println!("After pushing 1:\n{}", stack);
    let _ = stack.push(2).unwrap();
    println!("After pushing 2:\n{}", stack);
    let _ = stack.push(3).unwrap();
    println!("After pushing 3:\n{}", stack);
    let _ = stack.push(4).unwrap();
    println!("After pushing 4:\n{}", stack);

    let mut stack = DynamicSizedStack::<f64>::new();
    let _ = stack.push(10.0).unwrap();
    let _ = stack.push(7.0).unwrap();
    let _ = stack.push(1.2).unwrap();
    println!("{}", stack);
    let _ = dbg! {stack.ln()};
    println!("{}", stack);
    let _ = dbg! {stack.push(123.0)};
    println!("{}", stack);
    let _ = dbg! {stack.divide()};
    println!("{}", stack);
    let _ = dbg! {stack.multiply()};
    println!("{}", stack);
    let _ = dbg! {stack.divide()};
    println!("{}", stack);
    let _ = dbg! { stack.divide() };
    println!("{}", stack);
}
