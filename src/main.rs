extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess number!");
    let secret_number = rand::thread_rng().
        gen_range(1..101);
    loop {
    println!("Enter your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess) .ok()
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
        };
    println!("Your try: {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Smaller!"),
        Ordering::Greater => println!("Greater!"),
        Ordering::Equal => {
            println!("Win!");
            break;
        }
    }
    }
}