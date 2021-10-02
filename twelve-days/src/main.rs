mod fibonacci {
    /**  
     * Recursive implementation of fibonacci sequence
     */
    pub fn fib_recursive(n: i32) -> i64 {
        if n <= 1 {
            n.into()
        } else {
            fib_recursive(n-1) + fib_recursive(n-2)
        }
    }
}

mod twelve_days {
    static CHRISTMAS_GIFTS: [&str; 12] = 
                             ["A partridge in a pear tree",
                              "Two turtle doves",
                              "Three French Hens",
                              "Four Calling Birds",
                              "Five Golden Rings",
                              "Six Geese a Laying",
                              "Seven Swans a Swimming",
                              "Eight Maids a Milking",
                              "Nine Ladies Dancing",
                              "Ten Lords a Leaping",
                              "Eleven Pipers Piping",
                              "Twelve Drummers Drumming"];
    
    fn print_repeated_line() {
        println!("On the first day of Christmas my true love sent to me:");
    }
    
    pub fn print_twelve_days_of_christmas() {
        for i in 0..12 {
            print_repeated_line();
            for lyric in (0..i+1).rev() {
                let lyric_ptr: &str = match CHRISTMAS_GIFTS.get(lyric) {
                    Some(x) => x,
                    None => panic!("error!")
                };
                println!("{}", lyric_ptr);
            }
        }
    }
}

fn main() {
    twelve_days::print_twelve_days_of_christmas();
    println!("The {} Fib number = {}", 5, fibonacci::fib_recursive(5));
}
