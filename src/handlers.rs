use actix_web::{HttpRequest, HttpResponse, Json, Path, Query, Result};

use model::{Todo, TodoState};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct ListParams {
    pub state: String,
}

pub fn list_tasks(filter: Query<ListParams>) -> Result<Json<Vec<Todo>>> {
    info!("list tasks query {:?}", filter.state);

    let state = TodoState::from_str(&filter.state).unwrap();
    info!("deserialized state into: {:?}", state);

    Ok(Json(vec![
        Todo {
            id: 0,
            content: "This is your first task".to_owned(),
            state: TodoState::Open,
        },
        Todo {
            id: 0,
            content: "This is your first task".to_owned(),
            state: TodoState::Open,
        },
    ]))
}

pub fn create_task(task: Json<Todo>) -> Result<Json<Todo>> {
    Ok(task)
}

pub fn get_task(id: Path<u32>) -> Result<Json<Todo>> {
    Ok(Json(Todo {
        id: id.into_inner(),
        content: "Buy milk".to_string(),
        state: TodoState::Open,
    }))
}

pub fn update_task(req: HttpRequest) -> Result<Json<Todo>> {
    unimplemented!()
}

pub fn delete_task(req: HttpRequest) -> Result<Json<Todo>> {
    unimplemented!()
}

pub fn health_check(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().into()
}
