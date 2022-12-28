use actix_web::{ get, post, web, App, HttpResponse, HttpMessage, HttpServer, Responder, Error }; 
use tera::Tera;
use tera::Context;


#[get("/")]
async fn home() -> HttpResponse {
    let tera = match Tera::new("src/views/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    context.insert("vat_rate", &0.20);
    let render = match tera.render("index.html", &context) {
        Ok(res) => res,
        Err(err) => panic!("{}" , err)
    };
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