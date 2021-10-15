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

    impl<T, U> Point<T,U> {

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
