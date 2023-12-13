use std::mem;

#[allow(dead_code)]
fn main()
{
    println!("Hello,Rust !");

    #[allow(unused_variables)]
    let arr: [i32;5] = [1,2,3,4,5];
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