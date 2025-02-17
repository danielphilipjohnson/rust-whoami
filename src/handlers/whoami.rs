use axum::{extract::ConnectInfo, http::StatusCode, response::IntoResponse, response::Json};
use hyper::header::{HeaderMap, ACCEPT_LANGUAGE, USER_AGENT};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::error::ApiError;
use crate::models::whoami::WhoAmIResponse;
use crate::utils::{parse_browser, parse_language, parse_os};

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(0);

pub async fn handle_whoami(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let direct_ip = addr.ip().to_string();

    let ip = headers
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .or_else(|| {
            headers
                .get("Forwarded")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| {
                    s.split(';')
                        .find(|p| p.trim().to_lowercase().starts_with("for="))
                        .and_then(|f| f.split('=').nth(1))
                })
        })
        .unwrap_or(&direct_ip);

    let user_agent = match headers.get(USER_AGENT).and_then(|h| h.to_str().ok()) {
        Some(ua) => Some(ua.to_string()),
        None => {
            return ApiError::new(
                StatusCode::BAD_REQUEST,
                "Missing required header: User-Agent",
            )
            .into_response()
        }
    };

    let language = match headers.get(ACCEPT_LANGUAGE).and_then(|h| h.to_str().ok()) {
        Some(lang) => Some(lang.to_string()),
        None => {
            return ApiError::new(
                StatusCode::BAD_REQUEST,
                "Missing required header: Accept-Language",
            )
            .into_response()
        }
    };

    let parsed_language = language.as_deref().and_then(parse_language);
    let os = user_agent.as_deref().and_then(parse_os);
    let (browser_name, browser_version) = user_agent
        .as_deref()
        .map(parse_browser)
        .unwrap_or((None, None));

    let total_requests = REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;

    (
        StatusCode::OK,
        Json(WhoAmIResponse {
            total_requests,
            ipaddress: ip.to_string(),
            language,
            software: user_agent,
            os,
            browser_name,
            browser_version,
            parsed_language,
        }),
    )
        .into_response()
}
