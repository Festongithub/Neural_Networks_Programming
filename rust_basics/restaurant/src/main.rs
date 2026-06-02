mod rect;
use rect::r::Rectangle;
//use rect::R::Rectangle::{area, perimeter};

fn main() {
    let r1 = Rectangle {
        w: 78,
        l: 90,
    };

    println!("Area is : {}", r1.area());
    println!("perimeter is: {}", r1.perimeter());
}
