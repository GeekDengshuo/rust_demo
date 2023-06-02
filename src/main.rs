use std::io;
use rand::Rng;
use std::cmp::Ordering;




fn main() {
    println!("Guess the number!");

    concept();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is:{}",secret_number);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

    
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{guess}");

        // strong and static type system
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("You win!");
                break;

            }
        }
    }
    let guess:i32 = "42".parse().expect("not a number");
    println!("the guess :{guess}");


    
}

fn concept()
{
    let s_origin = String::from("hello");
    let s_copy = s_origin; // move
    
    println!("{}",s_copy);
}