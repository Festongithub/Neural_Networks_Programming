pub struct Student<'a, 'b> {
    pub name: &'a String,
    pub age: &'b u32,
}


pub mod calculator {

    pub struct Square<'a, 'b> {
        pub side: &'a u32,
        pub name1: &'b String
    }


    pub fn add(a: u32, b: u32) -> u32 {
    a + b
    }

    pub fn sub(a: &u32, b: &u32) -> u32 {
        a - b
    }

    pub fn wall(s: &String) -> &String {
        println!("Welcome to the {} calculator", s);
        &s
    }

    impl Square <'a, 'b>{
        pub fn n(&'a self) -> u32 {
            self.side * self.side
    }

    pub fn ume(&'a self) -> &'a String {
        println!("The square's name is: {}", self.name1);
        &self.name1
    }
    }
}


pub struct Rectangle {
    pub w: u32,
    pub l: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.w * self.l
    }

    pub fn perimeter(&self) -> u32 {
        2 * (self.w + self.l)
    }

    pub fn w(&self) -> u32 {
        self.w * 2
    }
}
