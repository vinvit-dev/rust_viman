use std::io::stdin;
use viman::{create_user, establish_connection};

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

    let mut pass = String::new();
    println!("Password:");
    stdin().read_line(&mut pass).unwrap();
    let pass = pass.trim_end();


    let user = create_user(connection, username, pass, email);
    println!("User {} with id {} created!!", user.username, user.id);
}
