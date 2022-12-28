mod config;

use std::{vec, fs::File};
use actix_files as fs;
use serde_json::json;
use actix_web::{ get, post, web, App, HttpResponse, HttpMessage, HttpServer, Responder, Error,  }; 
use tera::Context;
use config::template_engine::{ render_view };
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Users {
    id: i32,
    name: String,
    phone: String,
}

#[get("/")]
async fn home() -> HttpResponse {
    let mut context = Context::new();
    let mut user_list =  vec![];
    user_list.push(Users {
        id:1,
        name: String::from("Helo world"),
        phone: String::from("091232")
    });
    context.insert("nama", &json!({"hello":"world", "data": user_list}));
    let render = render_view("index.html", &context);
    HttpResponse::Ok().body(render)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("Program run on port {} ", 8080);
        App::new()
        .service(home)
        .service(fs::Files::new("/public","src/public").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}