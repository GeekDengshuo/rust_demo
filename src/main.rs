use std::io;
use std::cmp::Ordering;
// crate
use rand::Rng;




fn main()
{
    //guess_num();
    ownership();
    ownership_reference();

}
fn guess_num()
{

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

fn ownership()
{
    let str1 = String::from("hello world");
    // let str2 = str1;               // shadow copy will free str1
    let str2 = str1.clone();  // deep copy

    println!("The string is {str1}");


}

fn ownership_reference()
{
    let str1 = String::from("hello");
    let len = calculate_length(&str1);
    change(&str1);

    println!("the length of {} ,is {}",str1,len);


}
fn calculate_length(s:&String) -> usize
{
    s.len()
}

/*
// cannot borrow `*str` as mutable, as it is behind a `&` reference
fn change(str:&String)
{
    str.push_str(",world");
}
*/
