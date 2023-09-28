use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;

fn main() -> Result<(), Box<dyn Error>> {
    let v = vec![1, 2, 3];

    // v[99]; // Panics.

    {
        // enum Result<T, E> {
        //   Ok(T),
        //   Err(E),
        // }

        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
//            Err(error) => panic!("Problem opening the file: {:?}", error),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            }
        };
    }

    {
        let greeting_file = File::open("hello.txt").unwrap_or_else(
            |error| {
                if error.kind() == ErrorKind::NotFound {
                    File::create("hello.txt").unwrap_or_else(|error| {
                        panic!("Problem creating the file: {:?}", error);
                    })
                } else {
                    panic!("Problem opening the file: {:?}", error);
                }
            });
    }

    {
        // Better formatting of the error message printed on panic.
        let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
    }
    let username = read_username_from_file().unwrap();
    println!("Read username {} from hello.txt", username);

    {
        let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");
        println!("Got the IP Address {:?} from string", home);
    }

    // Using the type system to avoid having invariants violated.
    {
        // One way to define a Guess type that will only create an instance of Guess if the new function receives
        // a value between 1 and 100.

        pub struct Guess {
            // Note that the value field here is private.
            // Cannot be set or got directly, users have to use value().
            value: i32,
        }

        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("Guess value must be between 1 and 100, got {}.", value);
                }

                Guess { value }
            }

            pub fn value(&self) -> i32 {
                self.value
            }
        }
    }
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}