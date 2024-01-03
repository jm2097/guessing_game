use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

const RANGE_LOW: u8 = 1;
const RANGE_HIGH: u8 = 10;

const MSG_TOO_SMALL: &str = "Too small!";
const MSG_TOO_BIG: &str = "Too big!";
const MSG_WIN: &str = "You win!";

fn main() {
    println!(";------------------ ;");
    println!("   GUESS THE GAME    ");
    println!(";------------------ ;");

    let secret_number = rand::thread_rng().gen_range(RANGE_LOW..=RANGE_HIGH);

    loop {
        println!("\nInput your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Result<u8, _> = guess.trim().parse();

        let guess = match guess {
            Ok(input) => input,
            Err(_) => {
                print_out_of_range();
                continue;
            }
        };

        if guess < RANGE_LOW || guess > RANGE_HIGH {
            print_out_of_range();
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", MSG_TOO_SMALL.yellow()),
            Ordering::Greater => println!("{}", MSG_TOO_BIG.yellow()),
            Ordering::Equal => {
                println!("{}", MSG_WIN.green());
                break;
            }
        }
    }
}

fn print_out_of_range() {
    println!(
        "{}",
        format!(
            "Please type a number between {} and {}",
            RANGE_LOW, RANGE_HIGH
        )
        .red()
    );
}
