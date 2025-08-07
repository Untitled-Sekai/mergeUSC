use warp::{Filter, Rejection, Reply};
use mergeusc_core::{UscFile, UscMerger};
use std::convert::Infallible;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
struct ClientUscFile {
    filename: String,
    content: String,
}

#[tokio::main]
async fn main() {
    let merge = warp::post()
        .and(warp::path("merge"))
        .and(warp::body::content_length_limit(1024 * 1024 * 10))
        .and(warp::body::json())
        .and_then(|client_files: Vec<ClientUscFile>| async move {
            println!("Received {} files", client_files.len());
            
            let mut usc_files = Vec::new();
            for client_file in client_files {
                println!("Processing file: {}", client_file.filename);
                
                match serde_json::from_str::<UscFile>(&client_file.content) {
                    Ok(usc_file) => {
                        usc_files.push(usc_file);
                    },
                    Err(e) => {
                        println!("Error parsing content as UscFile: {}", e);
                        return Err(warp::reject::custom(JsonError(format!(
                            "file {} failed: {}", client_file.filename, e
                        ))));
                    }
                }
            }
            
            handle_merge(usc_files).await
        });

    let options = warp::options()
        .and(warp::path("merge"))
        .map(|| {
            println!("OPTIONS request received for /merge");
            warp::reply::with_status(warp::reply(), warp::http::StatusCode::OK)
        });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec!["Content-Type", "Authorization", "Accept"])
        .build();

    let static_files = warp::path::end()
        .and(warp::fs::file("static/index.html"))
        .or(warp::path("static")
            .and(warp::fs::dir("static")));

    let routes = merge
        .or(options)
        .or(static_files)
        .with(cors)
        .recover(handle_rejection)
        .with(warp::log("api"));
    
    println!("Server started at http://localhost:3030");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

async fn handle_merge(files: Vec<UscFile>) -> Result<impl Reply, Rejection> {
    println!("Processing merge request with {} files", files.len());
    match UscMerger::merge(files) {
        Ok(merged) => {
            let merged_json_str = serde_json::to_string_pretty(&merged)
                .unwrap_or_else(|_| "{}".to_string());
            
            Ok(warp::reply::json(&serde_json::json!({
                "content": merged_json_str,
                "filename": "merged.usc"
            })))
        },
        Err(e) => {
            println!("Merge error: {:?}", e);
            Err(warp::reject::custom(MergeError))
        }
    }
}

#[derive(Debug)]
struct MergeError;
impl warp::reject::Reject for MergeError {}

#[derive(Debug)]
struct JsonError(String);
impl warp::reject::Reject for JsonError {}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if let Some(JsonError(detail)) = err.find() {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = format!("Invalid JSON: {}", detail);
    } else if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "Not Found".into();
    } else if let Some(_) = err.find::<MergeError>() {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = "Failed to merge USC files".into();
    } else {
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error".into();
    }

    println!("Error response: {} - {}", code, message);
    
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({ "error": message })),
        code,
    ))
}