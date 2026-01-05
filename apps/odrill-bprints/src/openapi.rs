//! OpenAPI documentation

use axum::{Json, Router, routing::get};
use loco_rs::prelude::*;
use serde_json::json;

pub fn router() -> Router {
    Router::new()
        .route("/", get(docs_page))
        .route("/openapi.json", get(openapi_spec))
}

async fn docs_page() -> impl IntoResponse {
    axum::response::Html(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Odrill API</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
</head>
<body>
    <script id="api-reference" data-url="/api/openapi.json"></script>
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
</body>
</html>"#,
    )
}

async fn openapi_spec() -> Json<serde_json::Value> {
    Json(json!({
        "openapi": "3.0.3",
        "info": {
            "title": "Odrill Package Registry API",
            "description": "REST API for the Odrill package registry",
            "version": "1.0.0"
        },
        "servers": [{"url": "/api", "description": "Current"}],
        "paths": {
            "/packages/{name}": {
                "get": {
                    "summary": "Get package info",
                    "tags": ["Packages"],
                    "parameters": [{"name": "name", "in": "path", "required": true, "schema": {"type": "string"}}],
                    "responses": {"200": {"description": "Package info"}}
                }
            },
            "/packages/{name}/{version}": {
                "get": {
                    "summary": "Get version info",
                    "tags": ["Packages"],
                    "parameters": [
                        {"name": "name", "in": "path", "required": true, "schema": {"type": "string"}},
                        {"name": "version", "in": "path", "required": true, "schema": {"type": "string"}}
                    ],
                    "responses": {"200": {"description": "Version detail"}}
                }
            },
            "/packages/publish": {
                "post": {
                    "summary": "Publish package",
                    "tags": ["Packages"],
                    "security": [{"bearerAuth": []}],
                    "responses": {"200": {"description": "Published"}}
                }
            }
        },
        "components": {
            "securitySchemes": {
                "bearerAuth": {"type": "http", "scheme": "bearer", "bearerFormat": "JWT"}
            }
        }
    }))
}
