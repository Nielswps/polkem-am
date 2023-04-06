use std::fs;
use std::path::Path as KeyLocation;
use sha256::digest;

use axum::{
    http::StatusCode,
};
use axum::extract::Path;

pub async fn key(Path(input): Path<String>) -> (StatusCode, String) {
    let key_path = format!("keys/{}", &input);

    println!("Responding to request with input '{}'...", &input);

    if KeyLocation::new(&key_path).exists() {
        match fs::read_to_string(&key_path) {
            Ok(c) => (StatusCode::OK, c),
            Err(e) => (StatusCode::BAD_REQUEST, format!("Unable to load key for '{}' due to: {}", &input, e.to_string()))
        }
    } else {
        let key = digest(key_path.clone());

        // Ensure 'keys' directory exists
        if !KeyLocation::exists(KeyLocation::new("keys/")) {
            match fs::create_dir("keys") {
                Ok(_) => {}
                Err(e) => panic!("Unable to create dir 'keys' due to: {}", e.to_string())
            }
        }

        match fs::write(key_path, &key) {
            Ok(_) => (StatusCode::OK, key),
            Err(e) => (StatusCode::BAD_REQUEST, format!("Unable to write generated key for '{}' due to: {}", input, e.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        routing::get,
    };
    use super::*;
    use axum_test_helper::TestClient;

    #[tokio::test]
    async fn boot_node_request_returns_correct_id() {
        let app = Router::new()
            .route("/:index", get(key));

        let client = TestClient::new(app);
        let response = client.get("/0").send().await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.text().await, "856b7471e95fcb05ca1685e0a9400535391463a79b0305f22a1a80a6c647ee5d");
    }
}