
use std::mem;

#[allow(dead_code)]

// constants
static LANGUAGE: &str = "Rust";

// scope and shadowing

// type casting

fn main()
{

        // Suffixed literals, their types are known at initialization
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;
    
        // Unsuffixed literals, their types depend on how they are used
        let i = 1;
        let f: f64 = 1.0;
    
        // `size_of_val` returns the size of a variable in bytes
        println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
        println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
        println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
        println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
        println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    let decimal = 65.4312_f32;
    
    println!("decimal = {}",decimal);

    let integer = decimal as u8;
    let character = integer as char;
    //let char2 = decimal as char;
    println!("decimal = {}, -> integer = {},-> character = {}",decimal,integer,character);

    let out_shadowing = 10;
    println!("out_shadowing = {}",out_shadowing);
    {

        let out_shadowing = "abc";
        println!("out_shadowing = {}",out_shadowing);
    }
    let out_shadowing = 20;
    println!("out_shadowing = {}",out_shadowing);



    println!("Hello,Rust !");
    let hello_cargo: &'static str = "Hello,Cargo!";
    println!("{}",hello_cargo);

    
    let arr: [i32;5] = [1,2,3,4,5];
    #[allow(unused_variables)]
    let arr2 :[i32;100] = [0;100];


    for i in arr.iter(){
        println!("{}",i);
    }

    for i in 0..arr.len() + 2{
        let cur = arr.get(i);
        match cur {
            Some(cur) => println!("{}",cur),
            None => println!("Out of range"),
        }
    }

    borrow_arr_as_slice(&arr2);

    borrow_arr_as_slice(&arr[1..3]);

    // array is stack allocated
    println!("array occupies {} bytes",mem::size_of_val(&arr));

    



}
pub fn borrow_arr_as_slice(slice:&[i32])
{
    println!("first element of the slice:{}",slice[0]);
    println!("the slice has {} elements",slice.len());
}

mod tests{
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another_function() {
        println!("just test");
    }
}
