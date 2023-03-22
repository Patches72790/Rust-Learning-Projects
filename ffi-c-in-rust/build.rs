fn main() {
    cc::Build::new()
        .file("src/stuff.c")
        .include("src")
        .compile("stuff");

    println!("cargo:rerun-if-changed=src/stuff.c");
}
