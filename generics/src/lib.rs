pub use generic_stuff::{MyOption, MyResult, Point};

pub mod largest_thing_list {
    pub struct LargestThingList<T> {
        t: Vec<T>,
    }

    impl<T: PartialOrd + Copy> LargestThingList<T> {
        pub fn largest_copy(&self) -> T {
            let mut largest = self.t[0];

            for item in &self.t {
                if item > &largest {
                    largest = *item;
                }
            }
            largest
        }
    }
    impl<T: PartialOrd + Clone> LargestThingList<T> {
        pub fn largest_clone(&self) -> T {
            let mut largest = &self.t[0];

            for item in &self.t {
                if item > &largest {
                    largest = item;
                }
            }
            largest.clone()
        }
    }
    impl<T: PartialOrd> LargestThingList<T> {
        pub fn largest_no_clone_no_copy(&self) -> &T {
            let mut largest = &self.t[0];

            for item in &self.t {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
    }
}

pub mod largest_thing {
    pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    pub fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
        let mut largest = &list[0];

        for item in list {
            if item > &largest {
                largest = item;
            }
        }
        largest.clone()
    }
    pub fn largest_no_clone_no_copy<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    pub fn largest_lifetimes<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    #[derive(Debug)]
    pub struct ImportantExcerpt<'a> {
        pub part: &'a str,
    }
}

pub mod generic_stuff {

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
}
