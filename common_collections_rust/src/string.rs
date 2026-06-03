fn main() {

    let mut s = String::new();
    s.push_str("Hello world!");

    println!("{}", s);
    // create a vector of strings
    let mut s1: Vec<String> = Vec::new();
    s1.push("Hello".to_string());
    s1.push("World!".to_string());
    s1.push("Rustaceans".to_string());

    for i in &s1 {
        println!("Message is: {:?}", i);
    }

    let s2 = String::from("Rusty") + &s;
    println!("{:?}", s2);
    println!("{}", s);

    //println!("new string is :{}" , add(&s2, &s2));
    

    let m = String::from("Rust is ");
    let n = String::from("Good");
    
    let nm = m + &n;
    println!("{:?}", nm);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toc");

    let su = format!("{s4}-{s5}-{s6}");
    println!("{su}");

    let hello  = "Здравствуйте";
    let g = &hello[0..4];
    println!("{g}");
    for i in hello[0..4].bytes() {
        println!("{}", i);
    }

    for j in 60..61{
        println!("{}", j.str());
    }
}



//fn add(self, s: &str) -> String {
    //self.s + self.s
//}

