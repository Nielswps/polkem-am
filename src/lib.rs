use std::fs;
use axum::{
    http::StatusCode,
};
use axum::extract::Path;

pub async fn key(Path(str_with_numerics): Path<String>) -> (StatusCode, String) {
    let id: String = <String as Into<String>>::into(str_with_numerics.clone()).chars()
        .filter(|c| c.is_numeric())
        .collect();

    if id.is_empty() {
        return (StatusCode::BAD_REQUEST, format!("'{:}' does not contain any numbers", str_with_numerics));
    }

    match fs::read_to_string(format!("resources/node-{}-key", id)) {
        Ok(c) => (StatusCode::OK, c),
        Err(_) => (StatusCode::BAD_REQUEST, format!("Unable to load key for node '{}'. Must be integer between 0 and 99", id))
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
    async fn index_with_prefix_returns_correct_id() {
        let app = Router::new()
            .route("/:index", get(key));

        let client = TestClient::new(app);
        let response = client.get("/some-random-prefix-string-0").send().await;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.text().await, "5eb92d7eefb3a1e0e58397df26a542ae4dc83da03cd95923defbbec749ab6a54");
    }

    #[tokio::test]
    async fn index_with_pre_and_postfix_returns_correct_id() {
        let app = Router::new()
            .route("/:index", get(key));

        let client = TestClient::new(app);
        let response = client.get("/some-random-prefix-string0and-some-random-postfix-string").send().await;

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
        assert_eq!(response.text().await, "'i' does not contain any numbers");
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

    #[tokio::test]
    async fn no_index_returns_not_found() {
        let app = Router::new()
            .route("/:index", get(key));

        let client = TestClient::new(app);
        let response = client.get("/").send().await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn url_prefix_returns_not_found() {
        let app = Router::new()
            .route("/:index", get(key));

        let client = TestClient::new(app);
        let response = client.get("/keys/0").send().await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
