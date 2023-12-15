use actix_web::{
    web, App, HttpRequest, HttpServer, HttpResponse
};  

use tokio_postgres::{NoTls, Error as PgError};

use actix_web::dev::Server;

use std::net::TcpListener;

async fn handle_connection() -> Result<HttpResponse, PgError> {
    let database_url = "";
    let (client,connection) = tokio_postgres::connect(database_url,NoTls).await?;
    
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });


    format!("Connected to db");


    Ok(HttpResponse::Ok().finish())
}


async fn app_works()->HttpResponse{
    HttpResponse::Ok().finish()
}

  

pub async fn run(listener:TcpListener) -> Result<Server,std::io::Error>{
    let result = handle_connection().await;


    //println!("{:?}",result.unwrap());
    let server = HttpServer::new(||{
    
    App::new()
 
    .route("/",web::get().to(app_works))
    })
    .listen(listener)?
    .run();
    Ok(server)
}