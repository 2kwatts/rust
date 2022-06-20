use std::any::type_name; // for type_of

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    /* 'mut' makes the variable mutable, meaning that it's value
        will be (or can be) changed later in the program.
        Variables are immutable by default, so you'll probably
        want to be adding it most of the time.
    */
    let mut x = 5;
    println!("Hello, World!");
    get_value(x);
    println!("Changing 'x' to 500 now.");
    x = 500; // let is not needed for adding new values to mutable vars.
    get_value(x);
}

fn get_value(x: i32) {
    // Get value of variable and print it (+ var type)
    println!("Variable x (type: {}) is equal to: {}", type_of(x), x);
}