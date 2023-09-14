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
        let hello = String::from("Dobrý den"); ; println!("{}", hello);
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
}
