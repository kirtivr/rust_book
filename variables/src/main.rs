fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours in secs = {}", THREE_HOURS_IN_SECONDS);

    let mut x = 5;

    x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let guess: u32 = "42".parse().expect("Not a number!");

    let truncated = -5 / 3; // Results in -1
    println!("The value of -5/3 is: {truncated}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("weird unicode characters (4bytes per char): {z} {heart_eyed_cat}");

    // Access tuple with a period.
    let x= (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let mut res = 0;
    // Errors: let tup = [];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    /*if tup {
        res = 1;
    }*/
    println!("empty tuple or array does not evaluate to false")
}
