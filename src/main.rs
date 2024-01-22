
use std::mem;
use std::convert::From;
use std::fmt;

#[allow(dead_code)]

// constants
static LANGUAGE: &str = "Rust";

// scope and shadowing

// type casting
#[derive(Debug)]
struct Number{
    value:i32,
}

impl From<i32> for Number{
    fn from(item:i32) -> Self{
        Number{value:item}
    }
}
// loop

// match

#[cfg(target_os = "linux")]
fn are_you_on_linux(){
    println!("You are running linux!");
}

//Generic
struct MyVal{
    val : i32
}

struct MyGenVal<T>{
    val : T
}

impl MyVal{
    fn get_val(&self) -> &i32 {
        &self.val
    }
}

impl <T> MyGenVal<T> {
    fn get_val(&self) -> &T {
        &self.val
    }
}

// ownership
fn destroy_box(c:Box<i32>){
    println!("Destroying a box that contains {}",c);
}

// borrowing
struct Point { x: i32, y: i32, z: i32 }



fn main(){

    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // 数据可以通过引用或原始类型来访问
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);
    
    // 报错！`point` 不能以可变方式借用，因为当前还有不可变借用。
    //let mutable_borrow = &mut point;
    // TODO ^ 试一试去掉此行注释

    // 被借用的值在这里被重新使用
    //println!("Point has coordinates: ({}, {}, {})",
    //            borrowed_point.x, another_borrow.y, point.z);

    // 不可变的引用不再用于其余的代码，因此可以使用可变的引用重新借用。
    let mutable_borrow = &mut point;

    // 通过可变引用来修改数据
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // 报错！不能再以不可变方式来借用 `point`，因为它当前已经被可变借用。
    // let y = &point.y;
    // TODO ^ 试一试去掉此行注释

    // 报错！无法打印，因为 `println!` 用到了一个不可变引用。
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ 试一试去掉此行注释

    // 正常运行！可变引用能够以不可变类型传入 `println!`
    println!("Point has coordinates: ({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // 可变引用不再用于其余的代码，因此可以重新借用
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);

    // stack move(copy)
    let x = 5u32;
    let y = x;
    println!("x = {},y = {}",x,y);

    // heap move
    let a = Box::new(5i32);
    let b = a;

    //println!("a = {}",a);

    destroy_box(b);
    //println!("b = {}",b);




    let my_val = MyVal{val:10};
    let my_gen_val = MyGenVal{val:"string"};
    let my_gen_val2 = MyGenVal{val:10};

    println!("my_val = {},my_gen_val = {},my_gen_val2 = {}",
            my_val.get_val(),
            my_gen_val.get_val(),
            my_gen_val2.get_val());

    //crate_test::public_function();

    //crate_test::private_function();

    //crate_test::indirect_access();

    let triple = (0, -2, 3);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

    let mut count = 0u32;
    loop{

        count+=1;
        if count == 3{
            println!("three");
            continue;
        }
        println!("{}",count);
        
        if count == 5 {
            println!("OK,that's enough");
            break;
        }

    }

    let names = vec!["Bob","Frank","Ferris"];
    for name in names.iter(){
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}",name),
        }
    }
    let num1 = Number::from(50);
    println!("My number is {:?}",num1);

    let int = 25;
    let num:Number = int.into();
    
    println!("My num is {:?}",num);

    let parsed:i32 = "5".parse().unwrap();

    let turbo_parsed = "10".parse::<i32>().unwrap();

    println!("parsed = {}, turbo_parsed = {}",parsed,turbo_parsed);


    let my_str = "hello";
    let my_string = String::from(my_str);

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
