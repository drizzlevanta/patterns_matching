fn main() {
    println!("Hello, world!");
    let coin = Coin::Penny;
    let dime = Coin::Dime;
    let q1 = Coin::Quarter(UsState::DC);
    println!("coin value is {}", value_in_cents(&coin));
    println!("dime value is {}", value_in_cents(&dime));
    println!("q1 value is {}", value_in_cents(&q1));

    //matching with option<T>
    let i = Some(32);
    println!("plus one is {:?}", plus_one(i));
    let none = None;
    println!("plus one for none is {:?}", plus_one(none));
}

// #[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
    Other,
}

#[derive(Debug)]
enum UsState {
    AL,
    NY,
    AK,
    DC,
    VA,
}

fn value_in_cents(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("quarter is of state {:?}", state);
            25
        }
        _ => 100,
    }
}

//matching with option<T>. a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value. If there isn’t a value inside,
//the function should return the None value and not attempt to perform any operations.
fn plus_one(i: Option<i32>) -> Option<i32> {
    match i {
        None => None,
        Some(i) => Some(i + 1),
    }
}
