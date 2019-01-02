fn main() {
    let x = build_user(
        String::from("Someone@somewhere.com"),
        String::from("someone")
    );
    println!("{}", x.username);

    let user2 = User {
            email: String::from("someoneelse@nowhere.com"),
            ..x
    };
    println!("user2 email is {} and username is {}", user2.email, user2.username);
}

struct User {
        email: String,
        username: String,
        active: bool,
        sign_in_count: i32,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
