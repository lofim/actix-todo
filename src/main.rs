extern crate actix_web;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate futures;

#[macro_use]
extern crate log;
extern crate env_logger;

mod handlers;
mod model;

use handlers::{create_task, delete_task, get_task, health_check, list_tasks, update_task};

use actix_web::middleware::Logger;
use actix_web::{server, App};

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();

    let url = "127.0.0.1:8088";
    info!("Running server on {}", url);

    server::new(|| {
        App::new()
            .middleware(Logger::default())
            .middleware(Logger::new("%a %{User-Agent}i"))
            .prefix("/api/v1")
            .scope("tasks", |task_scope| {
                task_scope
                    .resource("", |r| {
                        r.get().with(list_tasks);
                        r.post().with(create_task);
                    })
                    .resource("/{task_id}", |r| {
                        r.get().with(get_task);
                        r.put().f(update_task);
                        r.delete().f(delete_task);
                    })
                    .nested("/{task_id}/relations", |relations_scope| {
                        relations_scope.resource("", |r| r.get().f(health_check))
                    })
            })
            .resource("/health_check", |r| r.get().f(health_check))
    }).bind(url)
        .unwrap()
        .run();
}
