
mod struct_stuff {
    // tuple structs are like tuples with no field names, but have struct names
    #[derive(Debug)]
    pub struct Color(pub i32, pub i32, pub i32);

    pub struct Point(i32, i32, i32);
}

mod user_stuff {
    #[derive(Debug)]
    pub struct User {
        pub username: String,
        pub email: String,
        pub sign_in_count: u64,
        pub active: bool,
    }

    impl User {
        pub fn say_name(&self) {
            println!("My name is {}!", self.username);
        }
    }

    pub fn create_user(username: &str, email: &str, sign_in_count: u64, active: bool) -> User {
        User {
            username: username.to_string(),
            email: email.to_string(),
            sign_in_count,
            active,
        }
    }
}

fn main() {
    let p1 = user_stuff::create_user("Petros", "petros@gmail.com", 1024, true);
    let p2 = user_stuff::User {
        username: String::from("Mary"),
        email: String::from("mary@example.de"),
        ..p1
    };

    println!("{:?}\n\t{:?}", p1, p2);
    p1.say_name();
    p2.say_name();

    let c1 = struct_stuff::Color (1, 2, 3 );
    println!("Here is a color: {:?}", c1);
    let g = Box::new(2);
    let ref g_unboxed = &g;
    println!("G is on the heap: {}", g_unboxed);
}
