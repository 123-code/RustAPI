
mod db;
extern crate actix_web;
extern crate diesel; 
extern crate dotenv;
extern crate openssl;
extern crate r2d2;

/*
async fn status() -> impl Responder{
    HttpResponse::Ok()
    .json((Status{status:"UP".to_string()}))
   
}
*/
use actix_web::{web, App, HttpResponse, HttpServer,Responder,HttpRequest,put,delete,post,get};
use openssl::ssl::{SslAcceptor,SslFiletype,SslMethod};



#[get("/hola")]
async fn index(_req:HttpRequest) -> impl Responder {
    println!("Server / route ");
    HttpResponse::Ok().body("Hola Mundo")
}


#[post("/holapost")]
async fn index_post(_req:HttpRequest) -> impl Responder {
    println!("Server / route ");
    HttpResponse::Ok().body("Hola Mundo POST")
}


#[delete("/holadelete")]
async fn index_delete(_req:HttpRequest) -> impl Responder {
    println!("Server / route ");
    HttpResponse::Ok().body("Hola Mundo DELETE")
}


#[put("/holaput")]
async fn index_put(_req:HttpRequest) -> impl Responder {
    println!("Server / route ");
    HttpResponse::Ok().body("Hola Mundo PUT")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection();
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}