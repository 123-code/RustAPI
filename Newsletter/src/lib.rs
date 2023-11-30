use actix_web::{
    web,App,HttpRequest,HttpServer,HttpResponse
};
use actix_web::dev::Server;

use std::net::TcpListener;

async fn health_check(_req:HttpRequest)->HttpResponse{
 HttpResponse::Ok().finish()
}


pub fn run(listener:TcpListener) -> Result<Server,std::io::Error>{
    let server = HttpServer::new(||{
    App::new()
    .route("/health_check",web::get().to(health_check))

})

    .bind("127.0.0.1:8000")?
    .run();
Ok(server)
}

/*
pub async fn run()->std::io::Result<()>{
    HttpServer::new(
        ||
        {
            App::new()
            .route("/health_check",web::get().to(health_check))
        }
    )
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
*/


