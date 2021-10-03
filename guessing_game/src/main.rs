use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);    
    let debug_mode: bool = false;

    if debug_mode {
        println!("The secret nuber is {}", secret_number);
    }


    // loop until correct number guessed
    loop {
       println!("Please input your guess:");
       let mut guess = String::new();

       // get the number from stdin
       io::stdin()
           .read_line(&mut guess)
           .expect("failed to read line");
     
       // parse the string into a u32
       let guess: u32 = match guess.trim().parse() { 
            // return number if ok
            Ok(num) => num,
            // report error and continue 
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
       };

       println!("You guessed: {}", guess);
       // compare guess to secret number
       match guess.cmp(&secret_number) {
           Ordering::Less => println!("Too small!"),
           Ordering::Greater => println!("Too big!"),
           Ordering::Equal =>  {
                println!("You win!");
                break;
           }    
       }
    }
}
