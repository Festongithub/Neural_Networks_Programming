#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}




fn main() {

    let user = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //user.active = false;

    println!("{}-->{}-->{}-->{}", user.email, user.username, user.active, user.sign_in_count);

    let user1 = User {
        active: false,
        username: String::from("Mike"),
        email: String::from("Mike@example.com"),
        sign_in_count: 2,
    };

    let  user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        ..user1
        sign_in_count: user1.sign_in_count,
    }

    println!("{}-->{}-->{}-->{}", user2.active, user2.username, user2.email, user2.sign_in_count);

    //println!("{:?}", get_user_name(user1.username));
}
