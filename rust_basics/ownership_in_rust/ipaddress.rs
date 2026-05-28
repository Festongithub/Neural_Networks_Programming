enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }

        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Dime a dozen");
            10
        }

        Coin::Quarter => 25
    }
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let null : Option<i32> = None;

    println!("{:?}---> {:?}-->{:?}", some_number, some_char, null);


    let c = Coin {

    }
}
