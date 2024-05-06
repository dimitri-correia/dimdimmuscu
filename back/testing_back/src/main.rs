use std::io;

use crate::add_muscle::add_muscle;
use crate::add_user::add_user;
use crate::delete_user::delete_user;

mod add_muscle;
mod add_user;
mod delete_user;

#[tokio::main]
async fn main() {
    let mut users = vec![];
    infinite_loop(&mut users).await;
}

async fn infinite_loop(users: &mut Vec<(String, String)>) {
    let mut choice = String::new();

    loop {
        print_all_choices(&mut choice);

        match choice.trim().parse::<i32>().unwrap() {
            0 => break,
            1 => users.push(add_user().await.unwrap()),
            2 => add_muscle(&users[0].1).await.unwrap(),
            _ => println!("Invalid choice, try again."),
        }
    }

    for user in users {
        delete_user(&user.1).await.expect("Should delete");
    }
}

fn print_all_choices(choice: &mut String) {
    println!("Make a choice:");
    println!("0. Quit");
    println!("1. Add user");
    println!("2. Add muscle");

    choice.clear(); // Clear previous input
    io::stdin()
        .read_line(choice)
        .expect("Failed to read line, try again");
}

pub fn get_address() -> String {
    "http://127.0.0.1:8000".to_string()
}
