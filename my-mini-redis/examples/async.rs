use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    read_n().await?;
    read_to_end().await?;
    write().await?;
    copy().await?;
    Ok(())
}

async fn read_n() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);

    Ok(())
}

async fn read_to_end() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();

    let n = f.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer[..n]);

    Ok(())
}

async fn write() -> io::Result<()> {
    let mut file = File::create("bar.txt").await?;
    file.write_all(b"some bytes\n").await?;
    Ok(())
}

async fn copy() -> io::Result<()> {
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo2.txt").await?;

    io::copy(&mut reader, &mut file).await?;

    Ok(())
}
