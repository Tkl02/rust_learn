use std::io;
use std::cmp::Ordering;
use rand::Rng;
/// Function to match the number
pub fn guess_number(){
    println!("Guess The Number!!");

    let secret_number  = rand::rng().random_range(1..=100);
  
    println!("secret_number: {}", secret_number);

    loop {
    println!("Plesa input your guess");

    let mut  guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("Please, insert a number");


    println!("You guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less =>  println!("Too small"),
            Ordering::Greater =>  println!("Too big"),
            Ordering::Equal =>  {println!("You win");
            break;
        }
    }}
}