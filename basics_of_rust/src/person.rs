#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn main() {
    let p: Person = Person {
        name: String::from("Hendry Jimmy"),
        age: 24,
        email : String::from("HendryJim@gmail.com"),
    };


    println!("{} --> {}--> {}", p.name, p.age, p.email);
}
