use rand::Rng; // random number generator
use std::cmp::Ordering;
use std::io; // import the io library w/c is part of the std library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
      println!("Please input your guess.");
      // rust vars are immutable by default, so mut specifies this one is mutable
      // :: indicates that new is an associated function of the String type
      // So `guess` is a mutable variable that is currently bound to a new, empty instance of a String
      let mut guess = String::new();

      io::stdin()
          .read_line(&mut guess) // '&' indicates that this is a reference 
          // This is how the err case is handled - called a Result type, w/c returns either
          // err or Ok. expect being the err
          .expect("Failed to read line"); 

          let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ being a wildcard for anything
        };

      println!("You guessed: {}", guess);

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          break;
        },
    }
}
}