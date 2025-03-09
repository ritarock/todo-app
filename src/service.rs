use crate::{io::write, todo::Todo, util::generate_id};
const FILE_PATH: &str = "store/data.json";

pub fn add(mut todos: Vec<Todo>, title: String) {
    let todo = Todo::new(generate_id(&todos), title, false);
    println!("add: {}", todo.get_title());
    todos.push(todo);
    write(&todos, FILE_PATH).unwrap_or_else(|err| println!("{}", err));
}

pub fn list(todos: Vec<Todo>) {
    if todos.is_empty() {
        println!("No TODOs found");
        return;
    }
    for todo in &todos {
        let status = if todo.get_completed() { "✓" } else { " " };
        println!("[{}] {}: {}", status, todo.get_id(), todo.get_title());
    }
}

pub fn completed(mut todos: Vec<Todo>, id: u32) {
    let id = id as usize - 1;
    let todo = todos.get_mut(id);
    match todo {
        Some(todo) => {
            todo.completed_todo();
            println!("completed: {}", todo.get_title());
            write(&todos, FILE_PATH).unwrap_or_else(|err| println!("{}", err));
        }
        None => println!("none-existent ID"),
    }
}

pub fn delete(mut todos: Vec<Todo>, id: u32) {
    let id = id as usize - 1;
    let todo = todos.get_mut(id);
    match todo {
        Some(todo) => {
            println!("deleted: {}", todo.get_title());
        }
        None => println!("none-existent ID"),
    }
    todos.remove(id);

    for (index, todo) in todos.iter_mut().enumerate() {
        todo.update_id((index + 1) as u32);
    }
    write(&todos, FILE_PATH).unwrap_or_else(|err| println!("{}", err));
}
