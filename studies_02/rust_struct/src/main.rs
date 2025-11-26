
fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!(" {:?} ", user1);
    println!(" email : {} ", user1.email);
    println!(" username : {} ", user1.username);
    println!(" active : {} ", user1.active);
    println!(" sign_in_count: {} ", user1.sign_in_count);


}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}