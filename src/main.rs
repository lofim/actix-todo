extern crate actix_web;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate json;

mod handlers;
mod model;

use handlers::{
    create_task,
    list_tasks,
    get_task,
    update_task,
    delete_task,
};

use actix_web::{server, App};

fn main() {
    let url = "127.0.0.1:8088";
    println!("Running server on {}", url);
    
    server::new(||
        App::new()
            .prefix("/api")
            .scope("v1", |scope| {
                scope
                    .resource("/tasks", |r| {
                        r.get().f(list_tasks);
                        r.post().f(create_task);
                    })
                    .resource("/tasks/{task_id}", |r| {
                        r.get().f(get_task);
                        r.put().f(update_task);
                        r.delete().f(delete_task);
                    })
            }))
        .bind(url)
        .unwrap()
        .run();
}
