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

fn main() {
    let s1 = String::from("Hello");
    let len = length(&s1);

    println!("The length {len} is {s1}");
    username(&s1);
    let k = 78;

    println!("the details include {:?}", user_details(&s1, k));
}
