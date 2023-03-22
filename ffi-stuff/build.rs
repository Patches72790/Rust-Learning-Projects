fn main() {
    cc::Build::new()
        .file("./c-stuff/main.c")
        .include("./c-stuff/")
        .compile("c_stuff");

    println!("cargo:rerun-if-changed=c-stuff/main.c");
}
