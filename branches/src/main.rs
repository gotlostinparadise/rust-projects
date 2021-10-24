fn main() {
    casuall_if_expression();
    if_in_let_statement();
}

// Using if in a let Statement
fn if_in_let_statement() {
    let condition = true;
    let x = if condition { 5 } else { 3 };

    println!("x is: {}", x);
}

// Using if casually
fn casuall_if_expression() {
    let number = 6;

    // We must be explicit and always provide `if` with a Boolean as its condition.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
