use crate::todo::Todo;

pub fn parse_id(id: Option<&String>) -> Result<u32, String> {
    match id {
        Some(id) => {
            match id.parse::<u32>() {
                Ok(id) => return  Ok(id),
                Err(_) => Err(String::from("invalid ID"))
            }
        }
        None => Err(String::from("missing ID")),
    }
}

pub fn generate_id(todo: &Vec<Todo>) -> u32 {
    todo.len() as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_id() {
        let id = Some("10".to_string());
        assert_eq!(parse_id(id.as_ref()), Ok(10));
    }

    #[test]
    fn test_parse_id_invalid() {
        let id = Some("id".to_string());
        assert_eq!(parse_id(id.as_ref()), Err("invalid ID".to_string()));
    }

    #[test]
    fn test_parse_id_missing() {
        let id = None;
        assert_eq!(parse_id(id.as_ref()), Err("missing ID".to_string()));
    }

    #[test]
    fn test_generate_id() {
        let todos = vec![
            Todo::new(1, "task1".to_string(), true),
            Todo::new(2, "task2".to_string(), false),
        ];
        assert_eq!(generate_id(&todos), 3);
    }

    #[test]
    fn test_generate_id_with_empty_list() {
        let todos = vec![];
        assert_eq!(generate_id(&todos), 1);
    }
}
