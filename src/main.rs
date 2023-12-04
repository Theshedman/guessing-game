use std::cmp::Ordering;
use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    // Randomly generate a secret number to guess.
    let secret_number: u32 = generate_secret_number();

    let mut guess_count: u32 = 0;

    // Request for user's guess in a continuous loop
    // until they guess the number right or quit the game
    loop{
        println!("Please input your guess: ");

        let mut guess: String = String::new();

        // Take user's guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse and convert their guess, which was taken as a string, to an unsigned 32-bit number
        let guess: u32 = match guess.trim() {
            str => {

                // Quit the guessing game once the user types in "quit'
                if str.to_lowercase().eq("quit") {
                    break;
                }

                match str.parse() {
                    Ok(num) => num,

                    // Ignore invalid input from the user
                    Err(_) => {
                        println!("Invalid input. Try again!");

                        continue;
                    },
                }
            }
        };


        println!("Your guess is {guess}");

        guess_count += 1;
        // compare the user's guess against the secret number and give them a hint
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");

                println!("Number of trial: {guess_count}");

                break;
            },
        }
    }
}

fn generate_secret_number() -> u32 {
    return rand::thread_rng().gen_range(1..=100);
}
