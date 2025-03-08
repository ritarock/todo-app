use crate::util::parse_id;

#[derive(Debug)]
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
                    None => Err(String::from("missing title for ADD command")),
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
