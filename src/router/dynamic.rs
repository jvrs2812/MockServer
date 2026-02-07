use axum::{
    Router,
    extract::State,
    http::{StatusCode, HeaderMap, Method},
    response::Json,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;

use crate::config::{ConfigManager, MockConfig};
use crate::middleware::apply_delay;
use crate::response::generate_response_body;

type AppState = Arc<ConfigManager>;

#[allow(dead_code)]
pub fn create_dynamic_router(_config_manager: Arc<ConfigManager>) -> Router<AppState> {
    let router = Router::new();

    // Add a catch-all handler that processes requests dynamically
    router.fallback(handle_dynamic_request)
}

pub async fn handle_dynamic_request(
    State(config_manager): State<AppState>,
    method: Method,
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    headers: HeaderMap,
    body: Option<Json<Value>>,
) -> (StatusCode, HeaderMap, Json<Value>) {
    let path = uri.path();
    let method_str = method.as_str();

    // Handle /_config endpoint for configuration management
    if path == "/_config" {
        return handle_config_endpoint(&config_manager, method_str, body).await;
    }

    tracing::info!("{} {}", method_str, path);

    let config = config_manager.get_config();

    // Find matching endpoint
    let matching_endpoint = config.endpoints.iter().find(|ep| {
        ep.method.eq_ignore_ascii_case(method_str) && path_matches(&ep.path, path)
    });

    match matching_endpoint {
        Some(endpoint) => {
            // Handle timeout simulation
            if endpoint.timeout {
                tracing::info!("Simulating timeout for {}", path);
                // Sleep for a very long time to simulate timeout
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
                return (
                    StatusCode::GATEWAY_TIMEOUT,
                    HeaderMap::new(),
                    Json(json!({"error": "timeout"})),
                );
            }

            // Apply delay if configured
            if let Some(delay) = &endpoint.delay {
                apply_delay(delay).await;
            }

            // Extract path parameters
            let params = extract_path_params(&endpoint.path, path);

            // Check conditions
            for condition in &endpoint.conditions {
                if check_condition(&condition.condition, &params, &headers, body.as_ref()) {
                    let response_body = generate_response_body(
                        &condition.response.body,
                        &params,
                        body.as_ref().map(|b| &b.0),
                    );
                    
                    let mut response_headers = HeaderMap::new();
                    for (key, value) in &condition.response.headers {
                        if let (Ok(name), Ok(val)) = (
                            key.parse::<axum::http::header::HeaderName>(),
                            value.parse::<axum::http::header::HeaderValue>()
                        ) {
                            response_headers.insert(name, val);
                        }
                    }
                    
                    return (
                        StatusCode::from_u16(condition.response.status).unwrap_or(StatusCode::OK),
                        response_headers,
                        Json(response_body),
                    );
                }
            }

            // Generate normal response
            let response_body = generate_response_body(
                &endpoint.response.body,
                &params,
                body.as_ref().map(|b| &b.0),
            );

            let mut response_headers = HeaderMap::new();
            for (key, value) in &endpoint.response.headers {
                if let (Ok(name), Ok(val)) = (
                    key.parse::<axum::http::header::HeaderName>(),
                    value.parse::<axum::http::header::HeaderValue>()
                ) {
                    response_headers.insert(name, val);
                }
            }

            (
                StatusCode::from_u16(endpoint.response.status).unwrap_or(StatusCode::OK),
                response_headers,
                Json(response_body),
            )
        }
        None => {
            tracing::warn!("No matching endpoint found for {} {}", method_str, path);
            (
                StatusCode::NOT_FOUND,
                HeaderMap::new(),
                Json(json!({
                    "error": "Endpoint not found",
                    "path": path,
                    "method": method_str
                })),
            )
        }
    }
}

fn path_matches(pattern: &str, actual_path: &str) -> bool {
    
    let pattern_parts: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
    let actual_parts: Vec<&str> = actual_path.split('/').filter(|s| !s.is_empty()).collect();

    if pattern_parts.len() != actual_parts.len() {
        return false;
    }

    for (pattern_part, actual_part) in pattern_parts.iter().zip(actual_parts.iter()) {
        if pattern_part.starts_with(':') {
            // This is a path parameter, it matches anything
            continue;
        }
        if pattern_part != actual_part {
            return false;
        }
    }

    true
}

fn extract_path_params(pattern: &str, actual_path: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    
    let pattern_parts: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
    let actual_parts: Vec<&str> = actual_path.split('/').filter(|s| !s.is_empty()).collect();

    for (pattern_part, actual_part) in pattern_parts.iter().zip(actual_parts.iter()) {
        if let Some(param_name) = pattern_part.strip_prefix(':') {
            params.insert(param_name.to_string(), actual_part.to_string());
        }
    }

    params
}

fn check_condition(
    condition: &crate::config::ConditionCheck,
    params: &HashMap<String, String>,
    headers: &HeaderMap,
    _body: Option<&Json<Value>>,
) -> bool {
    // Check param condition
    if let Some(param_name) = &condition.param {
        if let Some(param_value) = params.get(param_name) {
            if let Some(equals) = &condition.equals {
                return param_value == equals;
            }
            if let Some(contains) = &condition.contains {
                return param_value.contains(contains);
            }
        }
    }

    // Check header condition
    if let Some(header_name) = &condition.header {
        if let Some(header_value) = headers.get(header_name).and_then(|v| v.to_str().ok()) {
            if let Some(equals) = &condition.equals {
                return header_value == equals;
            }
            if let Some(contains) = &condition.contains {
                return header_value.contains(contains);
            }
        }
    }

    false
}

async fn handle_config_endpoint(
    config_manager: &Arc<ConfigManager>,
    method: &str,
    body: Option<Json<Value>>,
) -> (StatusCode, HeaderMap, Json<Value>) {
    match method {
        "GET" => {
            // Return current configuration
            let config = config_manager.get_config();
            let config_json = serde_json::to_value(&config).unwrap_or(json!({}));
            (
                StatusCode::OK,
                HeaderMap::new(),
                Json(json!({
                    "success": true,
                    "config": config_json
                })),
            )
        }
        "POST" | "PUT" => {
            // Update configuration from request body
            match body {
                Some(Json(config_value)) => {
                    match serde_json::from_value::<MockConfig>(config_value) {
                        Ok(new_config) => {
                            config_manager.update_config(new_config);
                            tracing::info!("Configuration updated via HTTP");
                            (
                                StatusCode::OK,
                                HeaderMap::new(),
                                Json(json!({
                                    "success": true,
                                    "message": "Configuration updated successfully",
                                    "endpoints_count": config_manager.get_config().endpoints.len()
                                })),
                            )
                        }
                        Err(e) => {
                            tracing::error!("Failed to parse config: {}", e);
                            (
                                StatusCode::BAD_REQUEST,
                                HeaderMap::new(),
                                Json(json!({
                                    "success": false,
                                    "error": format!("Invalid configuration: {}", e)
                                })),
                            )
                        }
                    }
                }
                None => (
                    StatusCode::BAD_REQUEST,
                    HeaderMap::new(),
                    Json(json!({
                        "success": false,
                        "error": "Request body is required"
                    })),
                ),
            }
        }
        _ => (
            StatusCode::METHOD_NOT_ALLOWED,
            HeaderMap::new(),
            Json(json!({
                "success": false,
                "error": "Method not allowed. Use GET to view config or POST/PUT to update."
            })),
        ),
    }
}
