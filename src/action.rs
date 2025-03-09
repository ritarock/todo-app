use crate::util::parse_id;

#[derive(Debug, PartialEq)]
pub enum Action {
    Add { title: String },
    List,
    Completed { id: u32 },
    Delete { id: u32 },
}

pub fn get_action(args: &[String]) -> Result<Action, String> {
    match args.get(1) {
        Some(cmd) => {
            let cmd = cmd.to_uppercase();
            match cmd.as_str() {
                "ADD" => match args.get(2) {
                    Some(title) => Ok(Action::Add {
                        title: title.to_string(),
                    }),
                    None => Err(String::from("enter a todo title")),
                },
                "LIST" => Ok(Action::List),
                "COMPLETED" => match parse_id(args.get(2)) {
                    Ok(id) => Ok(Action::Completed { id: id }),
                    Err(err) => Err(err),
                },
                "DELETE" => match parse_id(args.get(2)) {
                    Ok(id) => Ok(Action::Delete { id: id }),
                    Err(err) => Err(err),
                },
                _ => Err(String::from("unknown command")),
            }
        }
        None => Err(String::from("no command provided")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_action_add() {
        let args = vec!["program".to_string(), "add".to_string(), "task".to_string()];
        let action = get_action(&args);
        assert_eq!(
            action,
            Ok(Action::Add {
                title: "task".to_string()
            })
        );
    }

    #[test]
    fn test_get_action_add_without_title() {
        let args = vec!["program".to_string(), "add".to_string()];
        let action = get_action(&args);
        assert_eq!(action, Err("enter a todo title".to_string()));
    }

    #[test]
    fn test_get_action_list() {
        let args = vec!["program".to_string(), "list".to_string()];
        let action = get_action(&args);
        assert_eq!(action, Ok(Action::List));
    }

    #[test]
    fn test_get_action_completed() {
        let args = vec![
            "program".to_string(),
            "completed".to_string(),
            "1".to_string(),
        ];
        let action = get_action(&args);
        assert_eq!(action, Ok(Action::Completed { id: 1 }));
    }

    #[test]
    fn test_get_action_completed_invalid_id() {
        let args = vec![
            "program".to_string(),
            "completed".to_string(),
            "task".to_string(),
        ];
        let action = get_action(&args);
        assert!(action.is_err());
    }

    #[test]
    fn test_get_action_delete() {
        let args = vec!["program".to_string(), "delete".to_string(), "1".to_string()];
        let action = get_action(&args);
        assert_eq!(action, Ok(Action::Delete { id: 1 }));
    }

    #[test]
    fn test_get_action_delete_invalid_id() {
        let args = vec![
            "program".to_string(),
            "delete".to_string(),
            "task".to_string(),
        ];
        let action = get_action(&args);
        assert!(action.is_err());
    }

    #[test]
    fn test_get_action_unknown_command() {
        let args = vec!["program".to_string(), "unknown".to_string()];
        let action = get_action(&args);
        assert_eq!(action, Err("unknown command".to_string()));
    }

    #[test]
    fn test_get_action_no_command() {
        let args = vec!["program".to_string()];
        let action = get_action(&args);
        assert_eq!(action, Err("no command provided".to_string()))
    }
}
