use actix_web::{
    web,App,HttpRequest,HttpServer,HttpResponse
};  

use actix_web::dev::Server;

use std::net::TcpListener;


async fn app_works()->HttpResponse{
    HttpResponse::Ok().finish()
}

  

pub fn run(listener:TcpListener) -> Result<Server,std::io::Error>{
    let server = HttpServer::new(||{
    App::new()
    .route("/",web::get().to(app_works))
    })
    .listen(listener)?
    .run();
    Ok(server)
}