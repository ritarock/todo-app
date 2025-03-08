use std::env;

use action::{Action, get_action};
use io::read;
mod action;
mod io;
mod service;
mod todo;
mod util;

const FILE_PATH: &str = "store/data.json";

fn main() {
    let args: Vec<String> = env::args().collect();
    let action = get_action(&args);
    match action {
        Ok(Action::Add { title }) => match read(FILE_PATH) {
            Ok(todos) => service::add(todos, title),
            Err(err) => println!("{}", err),
        },
        Ok(Action::List) => match read(FILE_PATH) {
            Ok(todos) => service::list(todos),
            Err(err) => println!("{}", err),
        },
        Ok(Action::Completed { id }) => match read(FILE_PATH) {
            Ok(todos) => service::completed(todos, id),
            Err(err) => println!("{}", err),
        },
        Ok(Action::Delete { id }) => match read(FILE_PATH) {
            Ok(todos) => service::delete(todos, id),
            Err(err) => println!("{}", err),
        },
        Err(err) => println!("{}", err),
    }
}
