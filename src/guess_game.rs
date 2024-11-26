use rand::Rng;
use std::io;
use std::cmp::Ordering;
pub fn guess_number() {
    let mut rng = rand::thread_rng();
    println!("Number guessing game");
    let secret :u32 = rng.gen_range(1..100);
    
    loop {
        let mut user_guess = String::new();
        io::stdin().read_line(&mut user_guess).expect("Failed to read number");
        let user_guess : u32 = user_guess.trim().parse().expect("The input is not a number");

        match user_guess.cmp(&secret) {
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Congratulation, you guess right");
                break;
            }
        }
    }
}