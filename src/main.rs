use std::io;
use std::cmp::Ordering;
// crate
use rand::Rng;




fn main()
{
    //guess_num();
    ownership();
    ownership_reference();

    let a = [1,2,3,4,5];

    let slice = &a[1..3];

    assert_eq!(slice,&[2,3]);
    let scale = 2;
    let rect = Rectangle{
        width: dbg!(scale * 30),               // dbg!() is useful for debug [src/main.rs:22] scale * 30 = 60
        height: 20,
    };

    println!("rect is {:#?}",rect);
    println!{"rect area is {}",rect.area()};

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Option<T> six  {:#?},none  {:#?}",six,none);

}

fn plus_one(x:Option<u32>) -> Option<u32>
{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{

    // method
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self,other:&Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    fn width(&self) -> bool{
        self.width > 0
    }
    //associated functions
    fn square(size: u32) -> Self{
        Self { width: size, height: size }
    }
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
    //change(&str1);

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
fn dangle() -> String
{
    let s = String::from("hello");
    
    s  // move
}

fn first_word(s: &String) -> usize
{
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
