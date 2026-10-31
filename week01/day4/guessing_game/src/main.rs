use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the numnber!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut counter = 0 ;

    println!("The secret number is :{secret_number}");

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed : {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>{ println!("Please type a number");continue},
            
        };

       if !(1..=100).contains(&guess){
        println!("Please type a number between 1 to 100") ;
       } ;


counter += 1;



        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                println!("you win in a  {counter}");
                break;
            }
        }
    }
}
