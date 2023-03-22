#[repr(C)]
struct Thing {
    stuff: i32,
}

extern "C" {
    fn gimme_a_thing<'a>(size: u32) -> &'a Thing;
}

impl std::fmt::Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stuff)
    }
}

fn main() {
    let x = unsafe { gimme_a_thing(8) };

    println!("Return value from c func: {}", x);
}
