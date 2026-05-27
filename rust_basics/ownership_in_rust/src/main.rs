fn main() {

    let mut s = String::from("Hello"); // s is valid from here henceforth
    s.push_str(", Mike!");
    println!("Hello, world!: {}", s);

    let u = String::from("Hello");
    println!("{:?}",u);

    take_ownership(u);

    let x1 = 5;
    make_copy(x1);

    let s2 = String::from("Hello");
    let s3 = takes_ownership(s2);

    println!("new value is : {s3}");

    gives_ownership();

    let i = String::from("hendry");
    let j = 78;

    println!("{:?}", multi_var(i, j));

}


fn take_ownership(some_string: String) {
    println!("{some_string}");
}


fn make_copy(x: i32){
    println!("{x}");
}


fn gives_ownership() -> String {
    let su = String::from("yours");
    su
}

fn multi_var(a: String, b: u32) -> (String, u32) {
    (a, b)
}
fn takes_ownership(a_string: String) -> String {
    a_string
}
