use std::env;

use action::{Action, get_action};
use io::{read, write};
use todo::Todo;
use util::generate_id;
mod action;
mod io;
mod todo;
mod util;

const FILE_PATH: &str = "store/data.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    let action = get_action(&args);
    match action {
        Ok(Action::Add { title }) => match read(FILE_PATH) {
            Ok(mut todos) => {
                let todo = Todo::new(generate_id(&todos), title, false);
                todos.push(todo);
                write(&todos, FILE_PATH);
            }
            Err(err) => println!("{}", err),
        },
        Ok(Action::List) => match read(FILE_PATH) {
            Ok(todos) => {
                if todos.is_empty() {
                    println!("No TODOs found");
                    return;
                }
                for todo in &todos {
                    let status = if todo.get_completed() { "✓" } else { " " };
                    println!("[{}] {}: {}", status, todo.get_id(), todo.get_title());
                }
            }
            Err(err) => println!("{}", err),
        },
        Ok(Action::Completed { id }) => match read(FILE_PATH) {
            Ok(mut todos) => {
                let id = id as usize - 1;
                let todo = todos.get_mut(id);
                match todo {
                    Some(todo) => {
                        todo.completed_todo();
                        println!("{} completed", todo.get_title());
                        write(&todos, FILE_PATH);
                    }
                    None => println!("none-existent ID"),
                }
            }
            Err(err) => println!("{}", err),
        },
        Ok(Action::Delete { id }) => match read(FILE_PATH) {
            Ok(mut todos) => {
                let id = id as usize - 1;
                if todos.len() > id {
                    todos.remove(id);
                } else {
                    println!("none-existent ID")
                }
                for (index, todo) in todos.iter_mut().enumerate() {
                    todo.update_id((index + 1) as u32);
                }
                write(&todos, FILE_PATH);
            }
            Err(err) => println!("{}", err),
        },
        Err(err) => println!("{}", err),
    }
}
