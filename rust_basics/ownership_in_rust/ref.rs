fn length(s: &String) -> usize {
    s.len()
}

fn username(name: &String) -> &String{
    println!("welcome: {}", name);
    name
}

fn user_details(s: &String, number: u32) -> (&String, u32) {
    (s, number)
}

#[warn(dead_code)]
fn lin(line: &mut String) {
    line.push_str(", world");
}

fn main() {
    let s1 = String::from("Hello");
    let len = length(&s1);

    println!("The length {len} is {s1}");
    username(&s1);
    let k = 78;

    println!("the details include {:?}", user_details(&s1, k));

    let mut m = String::from("hello");
    let m1 = &mut m;
    println!("{}", m1);
}
