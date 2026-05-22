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

fn welcome(message: &String) -> &String{
    message
}

fn main() {
    let i = String::from("Hikes are good ideas");

    println!("{:?}", welcome(&i));

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


}
