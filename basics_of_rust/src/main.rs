fn takes_ownership(s: String) {
    println!("{s}");
}


fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn give_ownership() -> String{
    let some_string = String::from("Hello world Rust!");
    some_string
}

fn meth(a: String) -> String {
    a
}

fn variadic_function(b: String, c: u32) -> (String, u32) {
    (b, c)
}

fn welcome(message: &mut String) -> &String{
    message
}


fn array_traversal(a: &[u32]) -> u32 {

    for i in a {
        println!("{} --> {}", i, a[i]);
    }
}



fn main() {
    let numbers[] = {3, 4, 5, 2,1};
    array_traversal(&numbers);

    let mut i = String::from("Hikes are good ideas");

    println!("{:?}", welcome(&mut i));

    let a = String::from("hello");

    println!("string value : {}", meth(a));

    let s1 = give_ownership();
    println!("{}", s1);

    println!("{}", add(3, 2));
    let u = String::from("Hello");
    takes_ownership(u);
    println!("Hello, world!");

    let k = String::from("Hello");
    let j: u32 = 89;

    println!("{:?}", variadic_function(k, j));

    // mutability
    let r = String::from("hello vida");

    println!("{:?}", r);

    let mut s3 = &r;

    println!("{:?}", s3);


}
