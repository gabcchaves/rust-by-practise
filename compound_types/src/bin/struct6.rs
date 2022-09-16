struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_account: u64,
}

fn main() {
    let user1 = User {
        email: String::from("test@test.com"),
        username: String::from("UserName"),
        active: true,
        sign_in_account: 1,
    };

    let user2 = set_email(user1);

    println!("Success!");
}

// Create another user, with only one different field
fn set_email(user: User) -> User {
    User {
        email: String::from("email@email.com"),
        ..user
    }
}
