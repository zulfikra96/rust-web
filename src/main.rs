mod config;

use actix_web::{ get, post, web, App, HttpResponse, HttpMessage, HttpServer, Responder, Error }; 
use tera::Context;
use config::template_engine::{ render_view };


#[get("/")]
async fn home() -> HttpResponse {
    let mut context = Context::new();
    context.insert("vat_rate", &0.20);
    let render =render_view("index.html", &context);
    HttpResponse::Ok().body(render)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("Program run on port {} ", 8080);
        App::new()
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}