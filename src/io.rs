use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use crate::todo::Todo;

pub fn read(file_path: &str) -> Result<Vec<Todo>, String> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()),
    };

    let buf_reader = BufReader::new(file);
    let json =
        serde_json::from_reader(buf_reader).map_err(|_| String::from("deserialize failed"))?;
    Ok(json)
}

pub fn write(data: &Vec<Todo>, file_path: &str) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data).map_err(|_| String::from("serialize faild"))?;
    let mut file = File::create(file_path).map_err(|_| String::from("failed to open file"))?;
    write!(file, "{}", json).map_err(|_| String::from("failed to write file"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_read() {
        let todos = vec![
            Todo::new(1, "task1".to_string(), true),
            Todo::new(2, "task3".to_string(), false),
        ];

        let file_path = "test_read.json";
        let json = serde_json::to_string(&todos).unwrap();
        let mut file = File::create(file_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();

        assert_eq!(read(file_path), Ok(todos));
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_read_file_not_found() {
        let file_path = "test_read_file_not_found.json";
        assert_eq!(read(file_path), Ok(Vec::new()));
    }

    #[test]
    fn test_read_failed_deserialize() {
        let file_path = "test_read_failed_deserialize.json";
        let invalid_data = r#"
            [
                { "id": 1, "title": "task1", "completed": true },
                { "id": "id2", "title": "task2", "completed": false }
            ]
        "#;

        let mut file = File::create(file_path).unwrap();
        file.write_all(invalid_data.as_bytes()).unwrap();

        assert_eq!(read(file_path), Err("deserialize failed".to_string()));
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_write() {
        let file_path = "test_write.json";
        let todos = vec![
            Todo::new(1, "task1".to_string(), true),
            Todo::new(2, "task3".to_string(), false),
        ];

        let result = write(&todos, file_path);
        assert!(result.is_ok());
        fs::remove_file(file_path).unwrap();
    }
}
