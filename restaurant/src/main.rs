mod sum;
use sum::calculator::{add, sub, wall, Square};
use sum::{Student, Rectangle};

fn main() {
    let s = String::from("Mike Rusty");
    let i = 89;

    let s1 = Student {name: &s, age: &i };

    println!("{}--> {}", s1.name, s1.age);

    let u = String::from("hello");
    wall(&u);
    println!("sum is: {}", add(9,2));
    println!("diff is : {}", sub(&9, &2));


    let r = Rectangle {
        w:  89,
        l:  89,
    };

    println!("Area is: {}", r.area());
    println!("perimeter is {}", r.perimeter());
    println!("width is : {}", r.w());

    let u1 = Square{side: &i, name1:&s };

    println!("Total side : {} square pixels", u1.n());
    u1.ume();
}
