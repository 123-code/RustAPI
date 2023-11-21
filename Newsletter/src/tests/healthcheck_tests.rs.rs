fn spawn_app()->std::io::Result<()>{
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
    zero2prod::run().await
}


async fn health_check_works(){
spawn_app().await.expect("fallo al correr la app!");

let client = reqwest::Client::new();
let response = client

.get("http://127.0.0.0:8000/health_check")
.send();
assert!(response.status().is_success());
assert_eq!(Some(0),response.content_length());

}
