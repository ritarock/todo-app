use crate::todo::Todo;

pub fn parse_id(id: Option<&String>) -> Result<u32, String> {
    match id {
        Some(id) => {
            let id = id.parse::<u32>();
            match id {
                Ok(id) => Ok(id),
                Err(_) => Err(String::from("invalid ID")),
            }
        }
        None => Err(String::from("missing ID")),
    }
}

pub fn generate_id(todo: &Vec<Todo>) -> u32 {
    todo.len() as u32 + 1
}
