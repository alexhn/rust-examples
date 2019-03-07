

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {
   println!("hi"); 

   let some_u8_value = 6;

   match some_u8_value {
       1 => println!("one"),
       3 => println!("three"),
       5 => println!("five"),
       7 => println!("seven"),
       _ => println!("else"),
   }

   let some_u8_value2 = Some(0u8);

   match some_u8_value2 {
       Some(3) => println!("three"),
        _ => (),
   }

   if let Some (3) = some_u8_value2 {
       println!("three");
   }

}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}