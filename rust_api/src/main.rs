
use RustAPI::run; 
use std::net::TcpListener;

#[actix_web::main]


async fn main()-> std::io::Result<()>{
let listener: TcpListener = TcpListener::bind("127.0.0.1:8000")?;
run(listener).await?.await 
}