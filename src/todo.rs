use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
