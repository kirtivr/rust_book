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

    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of {} is {}.", s1, len);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }

        //let s2 = dangling_reference(s1);
       // println!("The s2 is {}.", s1);

        // Throws an error.
        // fn dangling_reference(s: String) -> &String {
        //     &s
        // }
    }

    // References are by default immutable.
    {
        let s = String::from("Hello");

//        change(&s);

//        fn change(sref: &String) {
//            sref.push_str(", World!");
//        }
//        println!("modified s is {}.", s);
    }

    // Mutable reference.
    {
        let mut s = String::from("Hello");

        change(&mut s);

        fn change(sref: &mut String) {
            sref.push_str(", World!");
        }
        println!("modified s is {}.", s);
    }

    // You can only have ONE mutable reference.
    {
        let mut s = String::from("Hello");
        let mut s1 = &mut s;
        let mut s2 = &mut s1;
//        s1.push_str(", World!");
//        println!("{} {}", s1, s2);
    }
    // Transfer ownership from method to other code.
    {
        let s = transfer_ownership();

        fn transfer_ownership() -> String {
            let mut s = String::from("Hello");
            let s1 = &mut s;
            s1.push_str(", World!");
            println!("s is {} s1 is {}.", "Something", s1);
            s
        }
        println!("s is {}.", s);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        // variables r1 and r2 will not be used after this point.

        let r3 = &mut s;
        println!("{}", r3);

        let r4 = r1;
        println!("{}", r4);
    }
}
