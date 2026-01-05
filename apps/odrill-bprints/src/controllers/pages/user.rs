//! SSR controller for user pages

use axum::response::Html;
use axum_extra::extract::cookie::CookieJar;
use loco_rs::prelude::*;

use crate::models::_entities::users;

async fn get_user(ctx: &AppContext, jar: &CookieJar) -> Option<users::Model> {
    if let Some(token) = jar.get("odrill_token") {
        if let Some(auth_config) = ctx.config.auth.as_ref() {
            let secret = &auth_config.jwt.as_ref().unwrap().secret;
            let validation = jsonwebtoken::Validation::default();

            if let Ok(data) = jsonwebtoken::decode::<serde_json::Value>(
                token.value(),
                &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
                &validation,
            ) {
                let pid = data.claims["pid"].as_str().unwrap_or_default();
                return users::Model::find_by_pid(&ctx.db, pid).await.ok();
            }
        }
    }
    None
}

pub async fn profile(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    jar: CookieJar,
) -> Result<impl IntoResponse> {
    let user = get_user(&ctx, &jar).await;
    let html = v.render("user/profile.html", data!({ "user": user }))?;
    Ok(Html(html.to_string()))
}

pub async fn settings(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    jar: CookieJar,
) -> Result<impl IntoResponse> {
    let user = get_user(&ctx, &jar).await;
    let html = v.render("user/settings.html", data!({ "user": user }))?;
    Ok(Html(html.to_string()))
}
