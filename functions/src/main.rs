fn main() {
    //
    println!("Hello, world!");
    print_hello();
    //
    function_with_args(42);
    //
    print_labeled_measurement(5, 'h');
    //
    expression_test();
    //
    let five = return_five();
    function_with_args(five);
    //
    let x = plus_one(42);
    function_with_args(x);
}

fn print_hello() {
    println!("Hello, Rust!");
}

// In function signatures, we must declare the type of each parameter.
fn function_with_args(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn expression_test() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

// implicit return
fn return_five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
