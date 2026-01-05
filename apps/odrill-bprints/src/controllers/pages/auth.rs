//! SSR controller for auth pages (login, register)

use axum::{
    extract::{Form, State},
    response::{Html, Redirect},
};
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::models::_entities::users;

/// GET /login - Show login page
pub async fn login_page(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    Ok(Html(v.render("auth/login.html", data!({}))?.to_string()))
}

/// GET /register - Show register page
pub async fn register_page(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    Ok(Html(v.render("auth/register.html", data!({}))?.to_string()))
}

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

/// POST /login - Handle login form
pub async fn login_submit(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Form(form): Form<LoginForm>,
) -> Result<impl IntoResponse> {
    let user = users::Model::find_by_email(&ctx.db, &form.email).await;

    match user {
        Ok(u) if u.verify_password(&form.password) => {
            let auth_config = ctx
                .config
                .auth
                .as_ref()
                .ok_or_else(|| Error::string("Auth not configured"))?;
            let secret = &auth_config
                .jwt
                .as_ref()
                .ok_or_else(|| Error::string("JWT not configured"))?
                .secret;

            let expiration = chrono::Utc::now()
                .checked_add_signed(chrono::Duration::days(7))
                .unwrap()
                .timestamp() as usize;

            let claims = serde_json::json!({
                "pid": u.pid.to_string(),
                "exp": expiration,
            });

            let token = jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
            )
            .map_err(Error::wrap)?;

            let cookie = axum_extra::extract::cookie::Cookie::build(("odrill_token", token))
                .path("/")
                .http_only(true)
                .build();

            let mut jar = axum_extra::extract::cookie::CookieJar::new();
            jar = jar.add(cookie);

            Ok((jar, Redirect::to("/")).into_response())
        }
        _ => {
            // Failed
            let html = v.render(
                "auth/login.html",
                data!({
                    "error": "Invalid email or password"
                }),
            )?;
            Ok(Html(html.to_string()).into_response())
        }
    }
}

#[derive(Deserialize)]
pub struct RegisterForm {
    name: String,
    email: String,
    password: String,
    password_confirm: String,
}

/// POST /register - Handle registration form
pub async fn register_submit(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Form(form): Form<RegisterForm>,
) -> Result<impl IntoResponse> {
    // Validate passwords match
    if form.password != form.password_confirm {
        let html = v.render(
            "auth/register.html",
            data!({
                "error": "Passwords do not match"
            }),
        )?;
        return Ok(Html(html.to_string()).into_response());
    }

    // Try to create user
    let params = crate::models::users::RegisterParams {
        email: form.email.clone(),
        password: form.password,
        name: form.name,
    };

    match users::Model::create_with_password(&ctx.db, &params).await {
        Ok(_) => {
            let html = v.render(
                "auth/register.html",
                data!({
                    "success": "Account created! Check your email to verify."
                }),
            )?;
            Ok(Html(html.to_string()).into_response())
        }
        Err(e) => {
            let html = v.render(
                "auth/register.html",
                data!({
                    "error": format!("Failed to create account: {}", e)
                }),
            )?;
            Ok(Html(html.to_string()).into_response())
        }
    }
}
