use aggregator::{Summary, Tweet};

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 'Type' as a parameter.
// We restrict the types valid for T to only those that implement PartialOrd
// and this example will compile, because the standard library implements PartialOrd on both i32 and char.
fn largest<T: std::cmp::PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// The purpose of this example is to demonstrate a situation in
// which some generic parameters are declared with impl and some
// are declared with the method definition. Here, the generic
// parameters X1 and Y1 are declared after impl because they go
// with the struct definition. The generic parameters X2 and Y2
// are declared after fn mixup, because they’re only relevant to
// the method.
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x : self.x,
            y : other.y,
        }
    }
}

// This method accepts something that has the summarize trait implemented on it.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

use core::fmt::Display;
// Specifying multiple trait bounds with the + syntax.
pub fn notify2(item: &(impl Summary + Display)) {

}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + std::fmt::Debug,
{ return 0; }

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }

    {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };
    
        let p3 = p1.mixup(p2);
    
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };
    
        println!("1 new tweet: {}", tweet.summarize());
    }

    {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
    }
}