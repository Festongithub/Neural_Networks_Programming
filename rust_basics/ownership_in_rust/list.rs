struct List {
    name: String,
    number: u32,
}

fn add_numbers(new_number: u32) -> List {
    List{
        name: String::from("Default"),
        number: new_number,
    }
}

fn update_number(list: &mut List, new_number: u32) {
    list.number = new_number;
}
fn main() {
    let mut L1 = List {
        name: String::from("Mike Munene"),
        number: 78,
    };

    println!("{}-->{}", L1.name, L1.number);

    let l2 = add_numbers(100);
    println!("{} --> {}", l2.name, l2.number);

    update_number(&mut L1, 95);
    println!("{}-->{}", L1.name, L1.number);
}
