pub use generic_stuff::{MyOption, MyResult, Point};

pub mod generic_stuff {
    //    pub fn largest<T>(list: &[T]) -> T {
    //        let mut largest = list[0];
    //
    //        for &item in list {
    //            if item > largest {
    //                largest = item;
    //            }
    //        }
    //        largest
    //    }

    #[derive(Debug)]
    pub struct Point<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Point<T> {
        pub fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        pub fn area(&self) -> f32 {
            self.x * self.y
        }
    }

    pub enum MyOption<T> {
        Some(T),
        None,
    }

    pub enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }
}

pub mod news {

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    pub trait Summary {
        fn summarize_author(&self) -> String;

        /**
         * Default implementation for abstract method may be overriden.
         */
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("Written by {}", self.author)
        }

        fn summarize(&self) -> String {
            format!(
                "{}, by {} ({})",
                self.headline,
                self.summarize_author(),
                self.location
            )
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    //    impl std::fmt::Display for Tweet {
    //        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //            ""
    //        }
    //    }
}
