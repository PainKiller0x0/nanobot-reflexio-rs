mod provider;
mod storage;
mod reasoning;

use ax_auth_middleware::AuthLayer; // 占位，未来可扩展鉴权
use axum::{
    routing::post,
    extract::State,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::env;
use dotenvy::dotenv;

use crate::provider::{LlmProvider, ChatMessage};
use crate::storage::DbStore;

struct AppState {
    provider: LlmProvider,
    db: Mutex<DbStore>,
}

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
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let api_key = env::var("LLM_API_KEY").expect("LLM_API_KEY must be set");
    let base_url = env::var("LLM_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let db_path = env::var("DATABASE_URL").unwrap_or_else(|_| "reflexio.db".to_string());

    let state = Arc::new(AppState {
        provider: LlmProvider::new(api_key, base_url),
        db: Mutex::new(DbStore::new(db_path).expect("Failed to init DB")),
    });

    let app = Router::new()
        .route("/api/publish_interaction", post(publish_interaction))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    println!("🚀 Nano-Reflexio-RS (Rust) listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn publish_interaction(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PublishInteractionRequest>,
) -> Json<GenericResponse> {
    // 1. 快速保存到本地数据库
    if let Some(interaction) = payload.interaction_data_list.first() {
        let _ = state.db.lock().unwrap().save_interaction(
            &payload.user_id,
            payload.session_id.as_deref(),
            &interaction.content
        );
    }

    // 2. 异步触发反思逻辑 (Fire and Forget)
    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        if let Some(interaction) = payload.interaction_data_list.first() {
            let messages = vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: crate::reasoning::PROFILE_UPDATE_PROMPT.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: interaction.content.clone(),
                },
            ];
            
            // 调用 LLM 进行事实提取
            if let Ok(reflection) = state_clone.provider.query("minimax-m2.7", messages).await {
                if reflection.len() > 5 && !reflection.contains("PASS") {
                    let _ = state_clone.db.lock().unwrap().save_fact(&payload.user_id, &reflection);
                    println!("✨ New fact extracted for {}: {}", payload.user_id, reflection);
                }
            }
        }
    });

    Json(GenericResponse {
        success: true,
        msg: "Accepted".to_string(),
    })
}
