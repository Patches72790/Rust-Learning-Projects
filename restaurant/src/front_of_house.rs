pub mod hosting {
    pub fn add_to_waitlist() {
        println!("Added to WaitList!");
    }

    pub fn seat_at_table() {
        println!("You are now seated");
    }
}

pub mod serving {
    pub fn take_order() {}

    pub fn serve_order() {}

    pub fn take_payment() {}
}
