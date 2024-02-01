
fn square(x:u32)->u32{
    x * x
}

fn factorial(mut n: u32) -> u32{
    let mut acc = 1u32;
    while n > 0{
        acc *= n;
        n = n -1;
    }
    acc
}
fn main()
{
    let s = square(4);
    let f = factorial(s);

}
