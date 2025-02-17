use axum::{
    body::{to_bytes, Body},
    extract::connect_info::MockConnectInfo,
    http::{Request, StatusCode},
    routing::get,
    Router,
};
use hyper::header::{ACCEPT_LANGUAGE, USER_AGENT};
use serde_json;
use std::net::SocketAddr;
use tower::ServiceExt;

use whoami_service::{handlers::whoami::handle_whoami, models::whoami::WhoAmIResponse};

// Constant for body size limit
const BODY_LIMIT: usize = 1024 * 16; // 16kb limit

// Helper function to create test app
fn test_app() -> Router {
    let socket_addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    Router::new()
        .route("/api/whoami", get(handle_whoami))
        .layer(MockConnectInfo(socket_addr))
}

#[tokio::test]
async fn test_basic_request() {
    let app = test_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/whoami")
                .header(USER_AGENT, "test-browser")
                .header(ACCEPT_LANGUAGE, "en-US")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();
    let response: WhoAmIResponse = serde_json::from_slice(&body).unwrap();

    assert!(response.ipaddress.len() > 0);
    assert_eq!(response.software, Some("test-browser".to_string()));
    assert_eq!(response.language, Some("en-US".to_string()));
}

#[tokio::test]
async fn test_missing_headers() {
    let app = test_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/whoami")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        error_response["error"]["message"],
        "Missing required header: User-Agent"
    );
}

#[tokio::test]
async fn test_browser_detection() {
    let app = test_app();
    let test_cases = vec![
        (
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/91.0.4472.124",
            ("Chrome", Some("91.0.4472.124".to_string())),
        ),
        (
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15) Firefox/89.0",
            ("Firefox", Some("89.0".to_string())),
        ),
    ];

    for (user_agent, (expected_browser, expected_version)) in test_cases {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/whoami")
                    .header(USER_AGENT, user_agent)
                    .header(ACCEPT_LANGUAGE, "en-US")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status(); /

        let body = to_bytes(response.into_body(), BODY_LIMIT).await.unwrap();
        let response_body: WhoAmIResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(status, StatusCode::OK);

        assert_eq!(
            response_body.browser_name,
            Some(expected_browser.to_string())
        );
        assert_eq!(response_body.browser_version, expected_version);
    }
}
