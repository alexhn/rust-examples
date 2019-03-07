

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {

    let mut user1 = User {
        email: String::from("blabla"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1
    };

    let user2 = User {
        email: String::from("Blabla"),
        username: String::from("test"),
        ..user1
    };

    user1.email = String::from("blah");

    let black = Color(0, 0, 0);

    let origin = Point(0, 0, 0);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}