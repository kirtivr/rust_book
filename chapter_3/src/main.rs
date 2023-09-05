fn main() {
    test(10);
    strict_if();
    loops();
}

fn test(x: u32) -> u32 {
    // Below is an expression.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {x} and the value of y is {y}");
    y
}

fn strict_if() {
    let x = 2;
    let number = if x > 0 { 5 } else if x < 0 { 6 } else { 7 };

    // Does not work.
    // let number = if x > 0 { 5 } else {"six"};
    // if number { // do something }

    if number != 1000 {
        println!("number was {number}");
    }
}

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // Works with or w/o semicolon.
            break counter * 2
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'loop_label : loop {
        println!("in the outer loop");
        loop {
            println!("in the inner loop");
            if count == 2 {
                break 'loop_label;
            }
            count += 1;
        }
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}