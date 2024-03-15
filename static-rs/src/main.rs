use cve_rs::null_mut;

fn main() -> Result<(), std::io::Error> {
    //let a: u8 = unsafe { std::mem::transmute("abc") };
    //
    struct Thing {
        name: String,
        age: u64,
    }
    let t = Thing {
        name: String::from("Greg"),
        age: 42,
    };
    let a: u32 = cve_rs::transmute(t);

    let size = std::mem::size_of::<Thing>();
    println!("{} -> {}", size, a);

    let n: &mut u8 = cve_rs::transmute(0usize);
    *n = 42;

    Ok(())
}
