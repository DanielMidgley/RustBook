
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = user {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String:from("another@example.com");
        ..user1 // gets other fields from user1
    }

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,       // username: username
        email,          // email: email
        sign_in_count: 1,
    }
}

fn _tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}
