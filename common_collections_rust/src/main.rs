fn main() {
    let mut  numbers: Vec<u32> = Vec::new();
    numbers.push(1);
    numbers.push(12);
    numbers.push(67);
    println!("{:?}", numbers);

    // Traverse
    for i in &numbers {
        println!("{}", i);
    }


    for j in 1..10 {
        let mut n : Vec<u32> = Vec::new();
        n.push(j);
        println!("{:?}\n", n);


        let third: Option<&u32> = n.get(3);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("no third element"),
        }
    }


    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Hendry"));
    names.push(String::from("John"));
    names.push(String::from("Jane"));
    names.push(String::from("Part"));

    for name in &mut names {
        //names.push(String::from("Holdon"));
        println!("{name}");
    }

    #[derive(Debug)]
    enum List {
        Number(i32),
        Age(i32),
        Email(String),
    }


    let l1 = vec![
        List::Number(23),
        List::Age(12),
        List::Email(String::from("age@number.com")),
    ];

    for i in &l1 {
        println!("{:?}",i);
    }

    println!("{:?}", &l1);

    let u = 78;

    let ui = &u;
    println!("{}", ui);
}
