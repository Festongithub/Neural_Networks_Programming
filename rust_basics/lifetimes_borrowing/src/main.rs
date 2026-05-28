#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}




fn main() {

    let x = 18;
    let y = 15;

    let s2 = Borrowed(&x);

    println!("value of x: {:?}", s2);

    let d = NBorrowed {x: &x, y: &y};
    println!("{:?}", d);


    let u = String::from("Mike!");
    println!("{}", message(&u));

    let mut p = String::from("Hilton Hotel");
    p = String::from("Hold On!");

    println!("message1: {}", message1(&mut p));
    let i: u32 = 90;

    println!("{:?}", check_number(&u, i));


    let i = 3;

    {
        let borrow1 = &i;
        println!("value of borrow1 is : {}", borrow1);
    }

    {
        let borrow2 = &i;
        println!("value of borrow2 is: {}", borrow2);
    }

    let (j, k) = (4, 3);
    print_ref(&j, &k);
    println!("sum of two is : {}", sum_two(&j, &k));

    pass_x(&j, &k);
}


fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    println!("The value of x: {}", x);
    x
}
fn message(s: &String) -> &String {
    s
}

fn message1<'a>(s1: &'a mut String) -> &'a mut String {
    s1
}

fn check_number(s: &String, num: u32) -> (&String, u32){
    (s, num)
}

fn print_ref<'a, 'b>(x: &'a i32, y: &'b i32){
    println!("x is {} and y is {}", x, y);
}


fn sum_two<'a, 'b>(x: &'a i32, y: &'b i32)-> i32 {
    x + y
}
