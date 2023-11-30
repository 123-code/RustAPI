fn spawn_app()->std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Error encontrando pueto");
    // traer el puerto asignado po os 
    let port + listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("error binding address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)
    //zero2prod::run().await
}


async fn health_check_works(){
//spawn_app().await.expect("fallo al correr la app!");

let address = spawn_app();
let client = reqwest::Client::new();
let response = client

.get(&format!("{}/health_check",&address))
.send()
.await 
.expect("failed to execute request");

/*
assert!(response.status().is_success());
assert_eq!(Some(0),response.content_length());
*/

}
