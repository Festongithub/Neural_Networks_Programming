struct List {
    name: String,
    number: u32,
}

impl List {

    fn number(&self) -> u32 {
        self.number * self.number
    }

    fn name(&self) -> &String {
        &self.name
    };
}


fn main() {

    let l = List {
        name: String::from("Mike Biekerg"),
        number: 90,
    };

    println!("number is: {}", l.number());
    println!("name is : {}", l.name());
}
