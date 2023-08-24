use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    text: String,
}

async fn chatbot(data: web::Json<Message>) -> impl Responder {
    // Add your chatbot logic here
    let response = format!("Chatbot received: {}", data.text);
    web::Json(Message { text: response })
}

fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .route("/", web::post().to(chatbot))
    });

    // Bind the server to the address
    let bind_result = server.bind("127.0.0.1:8080");
    match bind_result {
        Ok(bound_server) => {
            println!("Server bound successfully.");
            bound_server.run()?;
        }
        Err(err) => {
            eprintln!("Failed to bind server: {}", err);
            return Err(err);
        }
    }

    Ok(()) // Return Ok(()) at the end of main
}