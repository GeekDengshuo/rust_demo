use std::io;
use std::cmp::Ordering;
// crate
use rand::Rng;




fn main()
{
    //guess_num();
    ownership();

}
fn guess_num(){

    println!("Guess the number!");

    let random_num = rand::thread_rng().gen_range(1..=100); // low..=high
  
    // comparing loop
    loop {

        println!("please intput the number:");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("you guessed:{guess}");

        //shadowing variable
        let guess: u32 = guess.trim().parse().expect("please input a number ...");

        match guess.cmp(&random_num){
            Ordering::Less => println!("too small !"),
            Ordering::Greater => println!("to big !"),
            Ordering::Equal => {
                println!("you win ~"); 
                break;
            }
    
        }
    }
    
    println!("The random secret number:{random_num}");
}

fn ownership(){
    let str1 = String::from("hello world");
    // let str2 = str1;               // shadow copy will free str1
    let str2 = str1.clone();  // deep copy

    println!("The string is {str1}");

    
}

