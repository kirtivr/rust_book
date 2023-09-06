fn main() {
    {
        let s = String::from("hello");
        println!("value of s is {s}");
    } // s in no longer valid.


    // Rust handles shallow copies differently.
    // When s1 is "copied" to s2, s1 ceases to be valid.

    // copy is basically a unique_ptr move, except the compiler
    // ensures the moved from pointer is never reused.
    {
        let s1 = String::from("hello");
        let s2 = s1;

        // Compilation error!
        // println!("{}, world!", s1);
    }

    // Stack allocations are fine.
    {
        let x = 5;
        let y = x;
    
        println!("x = {}, y = {}", x, y);    
    }

    // Create a deep copy using clone() for types that derive from Clone.
    {
        let mut s1 = String::from("many");
        let s2 = s1.clone();
        s1.remove(0);
        println!("{} {}", s1, s2);    
    }

    // Note how the ordering in which functions are defined, v/s when they are used does not matter :)
    {
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here
        
        // println!("value of s is {s}");

        let x = 5;                      // x comes into scope
    
        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward
        
            let s2 = String::from("hello");     // s2 comes into scope
        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3

        println!("value of s3 is {s3}");
        // This function takes a String and returns one
        fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
            // scope
            a_string  // a_string is returned and moves out to the calling function
        }

        fn takes_ownership(some_string: String) { // some_string comes into scope
            println!("{}", some_string);
        } // Here, some_string goes out of scope and `drop` is called. The backing
          // memory is freed.
        
        fn makes_copy(some_integer: i32) { // some_integer comes into scope
            println!("{}", some_integer);
        } // Here, some_integer goes out of scope. Nothing special happens.
    }
}
