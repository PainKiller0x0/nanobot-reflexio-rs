use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber;

#[derive(Debug, Deserialize)]
struct InteractionData {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct PublishInteractionRequest {
    user_id: String,
    interaction_data_list: Vec<InteractionData>,
    #[serde(default)]
    session_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct GenericResponse {
    success: bool,
    msg: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/publish_interaction", post(publish_interaction));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("🚀 Nano-Reflexio-RS listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn publish_interaction(
    Json(payload): Json<PublishInteractionRequest>,
) -> Json<GenericResponse> {
    println!("Received interaction from user: {}", payload.user_id);
    
    // TODO: 实现核心反思推理逻辑
    // 我们会在这里调用 OpenAI/Minimax 接口进行异步反思
    
    Json(GenericResponse {
        success: true,
        msg: "Interaction queued for reflection (Rust backend)".to_string(),
    })
}
