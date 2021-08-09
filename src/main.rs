use std::io; // For standard input/output functions.
use std::cmp::Ordering; // For comparing numbers.
use rand::Rng; // For generating a random number to guess.

fn main() {
    println!("Let's play a guessing game.");

    // Generate random number on 'main' thread
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Print random secret number
    println!("The secret number is {}.",secret_number);

    println!("Please enter a number to guess");

    // Create a mutable String type variable called 'guess' and assign empty String
    let mut  guess = String::new();
    io::stdin()// Get input from user
    .read_line(&mut guess)// Append input to empty 'guess' string
    .expect("Failed to read line");// Print this out if fails

    // Use shadow of the mutable guess variable change its type to 32 bit Unsigned Number.
    // trim() function removes blank spaces at the start and end of 'guess' string.
    // The purpose of trim() here is also to remove carraige return '/r/n' that gets added 
    // at the end of string when Enter key is pressed to confirm input.
    let guess:u32 = guess.trim()
    .parse()// Convert number string to number of type u32.
    .expect("Please enter a number!");// If input string is not a number then print this message.

    // Print user input
    println!("You guessed {}",guess);

    // Compare user's guess to random secret number.
    // The cmp() function returns one of three enumerations to the 'match' expression
    // Ordering::{Equal | Greater | Less}
    // The match expression will compare the received arm(enum) to the arms(enums) specified in the curly braces.
    // and will execute the code adjacent(on the right of the arrow) to that arm.
    match guess.cmp(&secret_number){
        Ordering::Equal=>println!("You guessed it right"),
        Ordering::Greater=>println!("Too big"),
        Ordering::Less=>println!("Too small"),
    }
}
