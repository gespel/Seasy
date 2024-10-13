use axum::{
    Router,
};
use std::fs::File;
use std::io::Write;
use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::routing::{get, post};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/upload", post(upload));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn root() -> Result<String, String> {
    println!("Hello was hit!");
    Ok("Hello, World!".to_string())
}

async fn upload(mut multipart: Multipart) -> Result<(StatusCode, String), (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unknown").to_string();
        let file_name = field.file_name().unwrap_or("file").to_string();
        let data = field.bytes().await.unwrap();
        let data_length = data.len();
        println!("Name: {}, File: {}, Data: {}", name, file_name, data_length);
        // Save the file to disk
        /*let mut file = File::create(format!("./uploads/{}", file_name))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();
        file.write_all(&data)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();*/
    }
    Ok((StatusCode::OK, "ok".to_string()))
    //Ok(Html("<h1>File Uploaded Successfully</h1>"))
}