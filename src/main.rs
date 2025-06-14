use std::io;
use std::io::Write;

mod TodoListItem;
mod TodoList;


fn main() {
    let finished = false;
    println!("Welcome to your To Do list:)");
    println!("Its your time to manage - what do you want to do and manage it here");
    while !finished {
        print_menu();
        let mut user_operation = String::new();
        if (std::io::stdin().read_line(&mut user_operation).is_err()) {
            println!("Please enter a valid command");
            return;
        }
        match user_operation.trim() {
            "add" => handle_add(),
            "list" => handle_list(),
            "remove" => handle_remove(),
            "quit" => handle_quit(),
            _ => println!("Please enter a valid command"),
        }
    }
}

fn print_menu() {
    println!("enter the operation you want to do");
    print!("your options are : add, remove, list and quit");
}

fn handle_add() {
    
}