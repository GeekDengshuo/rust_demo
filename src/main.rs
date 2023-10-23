
fn main()
{
    let store = Inventory{
        shirts: vec![ShirtColor::Red,ShirtColor::Blue,ShirtColor::Blue],
    };
    let user_preferences = vec![Some(ShirtColor::Red),None,Some(ShirtColor::Blue)];
    
    for user_pref in user_preferences{
        let giveaway = store.giveaway(user_pref);
        println!("The user with perference {:?} get {:?}",
        user_pref,giveaway);
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
    fn giveaway(&self,user_perference: Option<ShirtColor>) -> ShirtColor{
        user_perference.unwrap_or_else(|| self.most_stocked())
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