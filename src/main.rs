use std::thread;
use std::ops::Deref;

mod foo;
fn main()
{
    let store = Inventory{
        shirts: vec![ShirtColor::Red,ShirtColor::Blue,ShirtColor::Blue],
    };
    let user_preferences = vec![Some(ShirtColor::Red),None,Some(ShirtColor::Blue)];
    
    for user_pref in user_preferences{
        let giveaway = store.giveaway(user_pref);
        println!("The user with preference {:?} get {:?}",
        user_pref,giveaway);
    }

    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    //let n = example_closure(5);

    let mut mutable_list = vec![1,2,3];
    println!("Before defining closure: {:?}",mutable_list);

    let mut mut_borrows = || mutable_list.push(4);

    //println!("Before calling closure:{:?}",mutable_list);

    mut_borrows();

    println!("After calling closure:{:?}",mutable_list);


    let list = vec![1,2,3,4,5];
    println!("Before defining closure:{:?}",list);
    // move can force get ownership
    thread::spawn(move || println!("From thread : {:?}",list))
        .join()
        .unwrap();


    let v1 = vec![1,2,3,4];
    let v2: Vec<_> = v1.iter().map(|x| x +1 ).collect();

    assert_eq!(v2,vec![2,3,4,5]);

    foo::hello();

    let x = 5;
    let y = Box::new(5);
    let z = MyBox::new(5);

    assert_eq!(5,x);
    assert_eq!(5,*y);

    assert_eq!(5,*z);


}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}


#[derive(Debug,PartialEq,Copy,Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory{
    shirts: Vec<ShirtColor>,
}

impl Inventory{
    fn giveaway(&self,user_preference: Option<ShirtColor>) -> ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts{
            match color{
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue{
            ShirtColor::Red
        }else{
            ShirtColor::Blue
        }
    }
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