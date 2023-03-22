#[no_mangle]
pub extern "C" fn call_in_c(t: i32) {
    println!("{}", t);
}
