#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}


#[derive(Debug)]
struct Borrowed<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

struct Pair(i32, f32);
struct Point<'a, 'b>{
    x: &'a f32,
    y: &'b f32,
}

#[derive(Debug)]
enum Student<'a> {
    Num(i32),
    Ref(&'a i32),
}


fn main() {
    let a = 34;
    let b = 89;

    let num = Student::Ref(&b);
    let ref = Student::Num(a);

    //let s1 = Student {name: String::from("Mike"), email: String::from("mike@example.com") };
    println!("{:?} --> {:?}", ref, num);
    let p1 = Pair(1, 0.2);

    println!("{} --> {}", p1.0, p1.1);
    let x = 18;
    let y = 15;

    let single = Borrowed{x: &x, y: &y};

    println!("{} --> {}", single.x, single.y);

    let name = String::from("Peter");
    let age = 27;

    let peter = Person {name, age};
    println!("{:?}", peter);

    let point: Point = Point {x: &5.2, y: &77.8};
    let another_point: Point = Point {x: &78.2, y: &7.2 };

    println!("{}--> {}", point.x, point.y);

    let b_right = Point {x: &10.3, ..another_point};
    println!("second point: ({}, {})", b_right.x, b_right.y);
}
