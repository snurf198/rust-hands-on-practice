// prelude - Rust has a set of ltems defined in the standard library that it brings into the scope of every program
// if a type you want to use isn't in the prelude, you have to bring that type into scope explicitly with a use statement
use std::io; // bring the io input/output library into scope
use std::cmp::Ordering;
use rand::Rng; // Rng trait deines methods that random number generators implement

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
    
        // create a variable to store the user input
        // we use the let statement to create the variable
        // in Rust, variables are immutable by default, so to make variable mutable, we add mut before the variable name
        // :: syntax in the ::new line indicates that new is an associated function of the String type
        // association function -> a function that's implemented on a type
        let mut guess = String::new();
    
        io::stdin() // returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal
            .read_line(&mut guess) // get input from the user(append into a string), returns a Result value
            .expect("Failed to read line"); // an instance of Result has an expect method. if this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect
    
        let guess: u32 = match guess.trim().parse() {
              Ok(num) => num,
              Err(_) => continue,
        };
        
        println!("You guessed: {guess}"); // when printing the value of a variable, the variable name can go inside the curly brackets.
        // when printing the result of evaluating an expression, place empty curly brackets in the format string, then follow the format string with a comma-seperated list of expressions to print in each empty curly bracket placeholder in the same order
        // ex. println!("x = {x} and y + 2 = {}", y + 2);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}