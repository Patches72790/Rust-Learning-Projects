use std::fs;

fn main() -> std::io::Result<()> {
    let result = fs::read_to_string("stuff.txt")?;
    println!("{:?}", result);
    Ok(())
}
