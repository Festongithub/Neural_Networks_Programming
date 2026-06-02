pub mod r {
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
    }
}
