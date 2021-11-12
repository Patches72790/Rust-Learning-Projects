use semaphore::Semaphore;

fn main() {
    let s = Semaphore::new(1);

    s.acquire();
    s.release();
}
