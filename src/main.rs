use warp::{Filter, Rejection, Reply};
use mergeusc_core::{UscFile, UscMerger};

#[tokio::main]
async fn main() {
    let merge = warp::post()
        .and(warp::path("merge"))
        .and(warp::body::json())
        .and_then(handle_merge);

    println!("Server started at http://localhost:3030");
    warp::serve(merge).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_merge(files: Vec<UscFile>) -> Result<impl Reply, Rejection> {
    match UscMerger::merge(files) {
        Ok(merged) => Ok(warp::reply::json(&merged)),
        Err(_) => Err(warp::reject::custom(MergeError)),
    }
}

#[derive(Debug)]
struct MergeError;
impl warp::reject::Reject for MergeError {}