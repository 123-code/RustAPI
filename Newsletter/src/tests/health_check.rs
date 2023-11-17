async fn spawn_app()->std::io::Result<()>{
    zero2prod::run().await
}


async fn health_check_works(){
spawn_app().await.expect("fallo al correr la app!")

let client = reqwest.Client::new();
let response = client

.get("http://127.0.0.0:8000/health_check")
.send()
await 

.expect("Failed to execute test!");
assert!(response.status().is_success());
assert_eq!(Some(0),response.content_length());

}


async fn spawn_app()->std::ioResult<()>{
    todo!()
}