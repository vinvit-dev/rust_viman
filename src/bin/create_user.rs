use std::io::stdin;
use viman::{create_user, establish_connection};
use viman::models::NewUser;

fn main() {
    let connection = &mut establish_connection();

    let mut username = String::new();
    println!("Username:");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end();

    let mut email = String::new();
    println!("Email:");
    stdin().read_line(&mut email).unwrap();
    let email = email.trim_end();

    let mut password = String::new();
    println!("Password:");
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end();

    let new_user = NewUser{username, email, password};


    let user = create_user(connection, new_user);
    println!("User {} with id {} created!!", user.username, user.id);
}
