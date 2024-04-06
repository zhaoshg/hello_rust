use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_num);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type a number ,not a string!");
                continue;
            }
        };

        println!("your guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small !\n\n"),
            Ordering::Greater => println!("too big !\n\n"),
            Ordering::Equal => {
                println!("you win !\n\n");
                break;
            }
        }
    }
}