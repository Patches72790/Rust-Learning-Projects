pub mod average {
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }
}

pub mod gui_stuff {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {}
    }

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {}
    }

    pub struct Screen {
        // Trait object <dyn Draw> is any type
        // inside a Box that implements the Draw trait
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
}
