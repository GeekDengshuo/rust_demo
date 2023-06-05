use std::io;
use rand::Rng;
use std::cmp::Ordering;




fn main() {
    println!("Guess the number!");

    concept();

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_give_back(s2);


    
}

fn concept()
{
    let s_origin = String::from("hello");
    let s_copy = s_origin; // move
    
    println!("{}",s_copy);
}


fn gives_ownership()->String{

    let some_string = String::from("yours");
    some_string
}

fn takes_and_give_back(a_string:String)->String{
    a_string
}