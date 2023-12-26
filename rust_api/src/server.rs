use actix_web::{
    web, App, HttpRequest, HttpServer, HttpResponse
}; 
use serde::{Deserialize, Serialize}; 
use tokio_postgres::{NoTls, Error as PgError};
use actix_web::dev::Server;
use std::net::TcpListener;
use dotenv::dotenv;
use std::env;


[derive(Serialize, Deserialize)]

struct ProposalData{
    header:String,
    description:String
}

async fn save_data(form:web::Form<ProposalData>)->HttpResponse{
    let data = form.into_inner();
    HttpResponse::Ok().json(data)
    
}

async fn handle_connection() -> Result<HttpResponse, PgError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    let (client,connection) = tokio_postgres::connect(&database_url,NoTls).await?;
    
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
        else {
            println!("Connected to db");
        }
     
    });


    


    Ok(HttpResponse::Ok().finish())
}


async fn app_works()->HttpResponse{
    HttpResponse::Ok().finish()
}

  

pub async fn run(listener:TcpListener) -> Result<Server,std::io::Error>{
    handle_connection().await;


    //println!("{:?}",result.unwrap());
    let server = HttpServer::new(||{
    
    App::new()
 
    .route("/",web::get().to(app_works))
    })
    .listen(listener)?
    .run();
    Ok(server)
}