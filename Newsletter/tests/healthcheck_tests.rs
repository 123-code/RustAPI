
use std::net::TcpListener;
use zero2prod::run;


 fn spawn_app()->String{
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)

}

#[actix_rt::test]
async fn health_check_works(){
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client.get(&format!("{}/health_check",&address)).send().await
    .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0),response.content_length());
    /*
spawn_app().await.expect("fallo al correr la app!");

let client = reqwest::Client::new();
let response = client

.get("http://127.0.0.0:8000/health_check")
.send();
assert!(response.status().is_success());
assert_eq!(Some(0),response.content_length());
*/
}
 