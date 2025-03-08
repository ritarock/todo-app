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

    let todos = match read(FILE_PATH) {
        Ok(todos) => todos,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    match action {
        Ok(Action::Add { title }) => service::add(todos, title),
        Ok(Action::List) => service::list(todos),
        Ok(Action::Completed { id }) => service::completed(todos, id),
        Ok(Action::Delete { id }) => service::delete(todos, id),
        Err(err) => println!("{}", err),
    }
}
