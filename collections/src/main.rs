fn main() {
    let mut _v = Vec::new();
    _v.push(4);
    _v.push(5);
    _v.push(6);
    let _w = vec![1, 2, 3];

    let a = vec![1, 2, 3, 4, 5];

    let index = 2;
    let third: &i32 = &a[index];
    println!("The third element is {third}");

    let third: Option<&i32> = a.get(index);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    // Using a vector to store objects of different types.
    {
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String)
        }

        let mut row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        for i in &mut row {
            println!("{:#?}", *i);
        }
    }

    {
        let hello = String::from("السلام عليكم"); println!("{}",hello);
        let hello = String::from("Dobrý den"); println!("{}", hello);
        let hello = String::from("Hello"); println!("{}", hello);
        let hello = String::from("שָׁלוֹם"); println!("{}", hello);
        let hello = String::from("नमस्ते"); println!("{}", hello);
        let hello = String::from("こんにちは"); println!("{}", hello);
        let hello = String::from("안녕하세요"); println!("{}", hello);
        let hello = String::from("你好"); println!("{}", hello);
        let hello = String::from("Olá"); println!("{}", hello);
        let hello = String::from("Здравствуйте"); println!("{}", hello);
        let hello = String::from("Hola"); println!("{}", hello);    
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // The left operand needs to be moved.
        let mut s3 = s1 + &s2[..];
        println!("s2 = {} s3 = {}", s2, s3);
    }

    // For more complicated combining we can use the format macro.
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
    }

    // Strings do not support indexing.
    {
        let s1 = String::from("tic");
        // let h = s1[0];
        // A string is a wrapper over a Vec<u8>.

        let hello = String::from("Hello");
        println!("The length of {} is {}", hello, hello.len());

        let hello = String::from("Здравствуйте");
        println!("The length of {} is {}", hello, hello.len());

        let hello = String::from("你好");
        println!("The length of {} is {}", hello, hello.len());

        // To figure out why, read up on UTF-8 encoding.
        /*
            When encoded in UTF-8, the first byte of З is 208
            and the second is 151, so it would seem that answer
            should in fact be 208, but 208 is not a valid character
            on its own. Returning 208 is likely not what a user would
            want if they asked for the first letter of this string;
            however, that’s the only data that Rust has at byte index 0.
        */
        // The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

        // This is fine.
        let hello = String::from("你好Здравствуйте");
        println!("The length of {} is {}", hello, hello.len());
        
        // Weirdness here.
        let hello = "Здравствуйте";
        // Get the first 4 bytes of hello, this is OK.
        let s = &hello[0..4];
        println!("first four bytes are {}", s);

        // Get the first byte of hello, we have a runtime panic.
        // let s = &hello[0..1];
        // Also panics.
        // let s = &hello[0..3];
        // println!("first byte is {}", s);        
    }

    // How to iterate over strings.
    {
/*        for c in "Зд".chars() {
            println!("{c}");
        }
*/
        for b in "Здравствуйте".bytes() {
            println!("{:02X}", b);
        }
    }

    // HashMaps.
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
    
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    
        let team_name = String::from("Blue");
        // convert Option<&i32> -> Option<i32> and if it is None,
        // set score to 0.
        let score = scores.get(&team_name).copied().unwrap_or(0);

        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        // For types that implement the Copy trait, like i32
        // the values are copied into the hash map.
        // For owned values like String, the values will be moved
        // and the hash map will be the owner of those values.

        // Adding a Key and Value Only If a Key Isn’t Present.
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        let mut map = HashMap::new();
        let text = "hello world wonderful world";

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
    
        println!("{:?}", map);
    }
}
