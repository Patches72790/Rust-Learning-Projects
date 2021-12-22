use oop_rust::gui_stuff::{Button, Screen, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 5,
                height: 5,
                label: "I'm a button!".to_string(),
            }),
            Box::new(SelectBox {
                width: 5,
                height: 5,
                options: vec![String::from("Option 1")],
            }),
        ],
    };
    screen.run()
}
