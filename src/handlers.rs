use futures::{Future};

use actix_web::{
    HttpRequest,
    HttpResponse,
    Error,
    Json,
    Responder,
};

use serde_json;

use model::{Todo, TodoState};

impl Responder for Todo {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

pub fn list_tasks(req: HttpRequest) -> impl Responder {
   Todo{
       id: 0,
       content: "This is your first task".to_owned(),
       state: TodoState::Open,
   }
}

pub fn create_task(req: HttpRequest) -> impl Responder {
    Todo {
        id: 0,
        content: "Brand new task".to_string(),
        state: TodoState::Open,
    }
}

pub fn get_task(req: HttpRequest) -> impl Responder {
    Todo {
        id: 0,
        content: "Buy milk".to_string(),
        state: TodoState::Open,
    }
}

pub fn update_task(req: HttpRequest) -> impl Responder {
    Todo {
        id: 1,
        content: "Updated Buy milk".to_string(),
        state: TodoState::Open,
    }
}

pub fn delete_task(req: HttpRequest) -> impl Responder {
    Todo {
        id: 0,
        content: "Deleted task".to_string(),
        state: TodoState::Open,
    }
}
