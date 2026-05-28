#[derive(Debug)]
struct Rectangle {
    w: u32,
    l: u32,
}

fn main() {

    let scale = 2;

    let s1 = Rectangle{
        w: dbg!(20 * scale),
        l: 89,
    };

    let w = Rectangle {
        w: 78,
        l: 90,
    };

    println!("area of the rectangle is {} square pixels", w.area());
    println!("perimeter of the rectangle is {} square pixels", w.perimeter());
    println!("w is {:#?}", w);

    dbg!(&s1);

    if w.w() {
        println!(
            "The rectangle has a nonzero width, it is {}", w.w
            );
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
    self.w * self.l
    }
    fn perimeter(&self) -> u32 {
        2 * (self.w + self.l)
    }

    fn w(&self) -> bool {
        self.w > 0
    }
}
