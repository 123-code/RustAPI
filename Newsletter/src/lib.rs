use actix_web::{
    web,App,HttpRequest,HttpServer,HttpResponse
};

async fn health_check(_req:HttpRequest)->HttpResponse{
 HttpResponse::Ok().finish()
}


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



