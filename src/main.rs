use std::io;
use rand::Rng;
use std::cmp::Ordering;



fn main()
{
    println!("please guess the number!");

    println!("please intput the number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        //.expect("Failed to read line");
    println!("you guessed:{guess}");
}

