use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub enum TodoState {
    Open,
    Closed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub content: String,
    pub state: TodoState,
}
