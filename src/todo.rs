use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: String, completed: bool) -> Self {
        Todo {
            id,
            title,
            completed,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_completed(&self) -> bool {
        self.completed
    }

    pub fn completed_todo(&mut self) {
        self.completed = true
    }

    pub fn update_id(&mut self, id: u32) {
        self.id = id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_get_id() {
        let todo = Todo::new(1, "task".to_string(), false);
        assert_eq!(todo.get_id(), 1);
    }

    #[test]
    fn test_todo_get_title() {
        let todo = Todo::new(1, "task".to_string(), false);
        assert_eq!(todo.get_title(), "task".to_string());
    }

    #[test]
    fn test_todo_get_completed() {
        let todo = Todo::new(1, "task".to_string(), false);
        assert_eq!(todo.get_completed(), false);
    }

    #[test]
    fn test_todo_get_completed_todo() {
        let mut todo = Todo::new(1, "task".to_string(), false);
        todo.completed_todo();
        assert_eq!(todo.get_completed(), true);
    }

    #[test]
    fn test_todo_get_update_id() {
        let mut todo = Todo::new(2, "task".to_string(), false);
        todo.update_id(1);
        assert_eq!(todo.get_id(), 1);
    }
}
