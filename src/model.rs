use std::{error, fmt, str};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TodoState {
    Open,
    Closed,
}

#[derive(Debug)]
pub enum TodoStateError {
    ParsingError(String),
}

impl str::FromStr for TodoState {
    type Err = TodoStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "open" => Ok(TodoState::Open),
            "closed" => Ok(TodoState::Closed),
            _ => Err(TodoStateError::ParsingError(format!("{}", s)))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub content: String,
    pub state: TodoState,
}
