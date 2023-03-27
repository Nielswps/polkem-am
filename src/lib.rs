use std::fs;
use axum::{
    http::StatusCode,
};
use axum::extract::Path;

pub async fn key(Path(index): Path<u32>) -> (StatusCode, String) {
    match fs::read_to_string(format!("resources/node-{}-key", index)) {
        Ok(c) => (StatusCode::OK, c),
        Err(_) => (StatusCode::BAD_REQUEST, format!("Unable to load key for node '{}'. Must be integer between 0 and 99", index))
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
    async fn index_0_returns_correct_id() {
        let app = Router::new()
            .route("/:index", get(key));

        let client = TestClient::new(app);
        let response = client.get("/0").send().await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.text().await, "5eb92d7eefb3a1e0e58397df26a542ae4dc83da03cd95923defbbec749ab6a54");
    }

    #[tokio::test]
    async fn char_index_returns_bad_request_and_parse_err_msg() {
        let app = Router::new()
            .route("/:index", get(key));

        let client = TestClient::new(app);
        let response = client.get("/i").send().await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(response.text().await, "Invalid URL: Cannot parse `\"i\"` to a `u32`");
    }

    #[tokio::test]
    async fn index_999_returns_bad_request_and_custom_msg() {
        let app = Router::new()
            .route("/:index", get(key));

        let client = TestClient::new(app);
        let response = client.get("/999").send().await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(response.text().await, format!("Unable to load key for node '{}'. Must be integer between 0 and 99", 999));
    }
}