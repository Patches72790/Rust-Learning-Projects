struct FibGenerator {
    previous: usize,
    next: usize,
}

impl Default for FibGenerator {
    fn default() -> Self {
        FibGenerator {
            previous: 0,
            next: 1,
        }
    }
}

impl Iterator for FibGenerator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let fib_num = self.previous;
        self.previous = self.next;
        self.next += fib_num;

        Some(fib_num)
    }
}

fn main() {
    println!("Fib numbers:");

    let fib_gen = FibGenerator::default();

    //    for (i, fib_num) in fib_gen.enumerate().take_while(|(i, _)| i < &20) {
    //        println!("fib({}) = {}", i, fib_num)
    //    }

    let fib_nums: Vec<usize> = fib_gen
        .take(64)
        .collect::<Vec<usize>>()
        .iter()
        .map(|fib_n| fib_n % 8)
        .collect();

    println!("Fib nums: {:?}", fib_nums);
}
