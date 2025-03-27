use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use scraper::Html;
use log::{info, error};
use dotenvy::dotenv;

#[derive(Deserialize, Debug)]
struct McpRequest {
    command: String,
    args: Option<Args>,
}

#[derive(Deserialize, Debug)]
struct Args {
    #[serde(rename = "crateName")]
    crate_name: Option<String>,
}

#[derive(Serialize)]
struct McpResponse {
    content: Vec<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_error: Option<bool>,
}

#[derive(Serialize)]
struct Content {
    #[serde(rename = "type")]
    type_: String,
    text: String,
}

async fn mcp_handler(req: web::Json<McpRequest>, client: web::Data<Client>) -> impl Responder {
    info!("Received request: {:?}", req);

    if req.command != "lookup_crate_docs" {
        return HttpResponse::BadRequest().json(McpResponse {
            content: vec![Content {
                type_: "text".to_string(),
                text: "Unknown command".to_string(),
            }],
            is_error: Some(true),
        });
    }

    // 获取 crate_name，默认为 "tokio"
    let crate_name = req.args.as_ref()
        .and_then(|args| args.crate_name.clone())
        .unwrap_or_else(|| "tokio".to_string());
    info!("Fetching documentation for crate: {}", crate_name);

    // 构造 URL
    let url = format!("https://docs.rs/{}/latest/{}/index.html", crate_name, crate_name);
    info!("Making request to: {}", url);

    // 发送请求
    match client.get(&url).send().await {
        Ok(response) => {
            info!("Received response with status: {}", response.status());
            if response.status().is_success() {
                let html = response.text().await.unwrap_or_default();
                let document = Html::parse_document(&html);

                // 提取所有文本内容
                let text = document.root_element()
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");

                // 截断文本
                const MAX_LENGTH: usize = 8000;
                let truncated_text = if text.len() > MAX_LENGTH {
                    format!("{}\n\n[Content truncated. Full documentation available at {}]", 
                            &text[..MAX_LENGTH], url)
                } else {
                    text
                };

                info!("Successfully processed docs for {}", crate_name);
                HttpResponse::Ok().json(McpResponse {
                    content: vec![Content {
                        type_: "text".to_string(),
                        text: truncated_text,
                    }],
                    is_error: None,
                })
            } else {
                let error_msg = format!("Failed to fetch documentation: HTTP {}", response.status());
                error!("{}", error_msg);
                HttpResponse::InternalServerError().json(McpResponse {
                    content: vec![Content {
                        type_: "text".to_string(),
                        text: error_msg,
                    }],
                    is_error: Some(true),
                })
            }
        }
        Err(e) => {
            let error_msg = format!("Error contacting docs.rs: {}", e);
            error!("{}", error_msg);
            HttpResponse::InternalServerError().json(McpResponse {
                content: vec![Content {
                    type_: "text".to_string(),
                    text: error_msg,
                }],
                is_error: Some(true),
            })
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();  
    // env_logger::init();
    tracing_subscriber::fmt::init();    

    let client = Client::new();
    let port = std::env::var("PORT").unwrap_or("6666".to_string());
    info!("Starting MCP Crate Docs Server on http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/mcp", web::post().to(mcp_handler))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}