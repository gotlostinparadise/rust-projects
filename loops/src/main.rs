// loop, while & for
fn main() {
    // loop
    loop_fn();

    // while
    while_fn();

    // for
    for_fn();
}

fn for_fn() {
    // this is FAST
    {
        println!("Iteration through array: for");

        let array = [1, 2, 3, 4, 5, 6, 7, 8];

        for i in array.iter() {
            println!("{}", i);
        }
    }

    // Rocket launch counter: for
    {
        println!("Rocket launch counter");

        for i in (1..4).rev() {
            println!("{}", i);
        }

        println!("LIFTOFF!!!");
    }
}

fn while_fn() {
    {
        println!("Rocket launch counter");
        let mut counter = 3;

        while counter != 0 {
            println!("{}", counter);
            counter -= 1;
        }
        println!("LIFTOFF!!!");
    }

    // this is SLOW
    {
        println!("Iteration through array: while");
        let array = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut index = 0;

        while index <= array.len() - 1 {
            println!("{}", array[index]);
            index += 1;
        }
    }
}

fn loop_fn() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is : {}", result);
    println!("Counter is at : {}", counter);
}
