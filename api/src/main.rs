mod models;
mod config;
use crate::models::Status;
use actix_web::{HttpServer,App,web,Responder};
use actix_web::HttpResponse;
use std::fmt::format;
use std::io;
use dotenv::dotenv;


async fn status() -> impl Responder{
    HttpResponse::Ok()
    .json((Status{status:"UP".to_string()}))
   
}

#[actix_rt::main]
async fn main() -> io::Result<()>{
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    println!("Server running at port {}",config.server.port);
    HttpServer::new(||{
        App::new().route("/", web::get().to(status))

    })
    .bind(format!("{}{}",config.server.host,config.server.port))?
    .run()
    .await
}