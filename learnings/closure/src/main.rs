// intend to learn closure in rust

fn main() {
    println!("Hello, closure!");

    // a closure is anonymous function or lamdba function that can capture the environment

    // closures are defined using the pipe operator | | and can take parameters

    // Example 1: closure with no parameters
    println!();
    println!("Example 1");
    let no_param = || {
        println!("Hello, closure with no parameters!");
    };
    no_param();


    // Example 2: closure with one parameter
    // The parameter type can be inferred from the context on first call. NOT DYNAMIC!
    println!();
    println!("Example 2");
    let one_param_implict = |x| {
        println!("Hello, closure with one implicit typed parameter: {} typed {}", x, std::any::type_name::<i32>());
    };
    one_param_implict(10); // parameter type is inferred to be i32 at compile time on first call
    // one_param_implict("string"); // This will not compile due to type mismatch


    // Example 3: closure with one parameter with explicit type
    // The parameter type can be explicitly defined
    println!();
    println!("Example 3");
    let one_param_explicit = |x: &str| {
        println!("Hello, closure with one explicit typed parameter: {} typed {}", x, std::any::type_name::<&str>());
    };
    one_param_explicit("string"); // parameter type is explicitly defined as &str
    // one_param_explicit(10); // This will not compile due to type mismatch

    // Example 4: closure with multiple parameters
    println!();
    println!("Example 4");
    let multi_param = |x: i32, y: i32| {
        println!("Hello, closure with multiple parameters: {} + {} = {}", x, y, x + y);
    };
    multi_param(10, 20); // parameter type is explicitly defined as i32
    
    // Example 5: closure with return value
    println!();
    println!("Example 5");
    let return_value = |x: i32| -> i32 {
        println!("Hello, closure with return value: {} typed {}", x, std::any::type_name::<i32>());
        x + 1
    };
    let result = return_value(10);
    println!("Result of closure with return value: {}", result);
    

    // Example 6: environment capture (immutable)
    println!();
    println!("Example 6");
    let a = 10;
    let capture_immut = |x| x + a;
    let x = 5;
    println!("Capture immut x -> x + a: captures a={}, called by x={}, returns {}", a, x, capture_immut(x));
    println!("a is still: {}", a); // a is still 10

    // Example 7: environment capture (mutable)
    // The closure can capture mutable variables, borrowing them and disallowing access until the closure is out of scope
    println!();
    println!("Example 7");
    let mut b = 10;
    println!("b is initialized as: {}", b);
    let mut capture_mut = |x| {
        b += x;
        b
    };
    println!("Capture mut x -> b += x: called by x=1, returns {}", capture_mut(1)); // b is now 11 inside the closure
    // println!{"b is now: {}", b}; // this will not compile because we are still between the closure calls and b is borrowed
    println!("Capture mut x -> b += x: called by x=2, returns {}", capture_mut(2)); // b is now 13 inside the closure
    // b is borrowed so we cannot use it when the closure is still in scope
    // println!("Capture mut x -> b += x: captures b={}, called by x=3, returns {}", b, capture_mut(3)); // This will not compile either
    println!("b is now: {}", b); // the last call is done, the closure is out of scope the main function regains ownership of b

    // Example 8: return and reassign
    println!();
    println!("Example 8");
    let mut c = 10;
    println!("c is initialized as: {}", c);
    let return_reassign = |x, mut borrowed | {
        borrowed += x;
        borrowed
    };

    c = return_reassign(1, c); // c is now 11, and owned by main
    println!("c is now: {}", c); // we can print c from main
    c = return_reassign(2, c); // c is now 13
    println!("c is now: {}", c); // we can print c from main

}
