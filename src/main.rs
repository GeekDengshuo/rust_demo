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

    
}

fn concept()
{
    //const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;

    // shadowing

    let x = 5;

    let x = x +1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is :{x}");

    }
    println!("The value of x is {x}");

}