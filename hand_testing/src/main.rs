use std::io;

use crate::add_user::add_user;

mod add_user;

#[tokio::main]
async fn main() {
    let mut choice = String::new();

    loop {
        print_all_choices(&mut choice);

        match choice.trim().parse::<i32>().unwrap() {
            0 => break,
            1 => {
                if let Err(err) = add_user().await {
                    println!("Error: {}", err);
                }
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}

fn print_all_choices(choice: &mut String) {
    println!("Make a choice:");
    println!("0. Quit");
    println!("1. Add user");
    println!("2. Delete user");

    choice.clear(); // Clear previous input
    io::stdin()
        .read_line(choice)
        .expect("Failed to read line, try again");
}
