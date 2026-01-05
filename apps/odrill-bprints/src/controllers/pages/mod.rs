//! SSR page controllers

pub mod auth;
pub mod packages;
pub mod templates;
pub mod user;

use axum::routing::get;
use loco_rs::prelude::*;

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(packages::index))
        .add("/login", get(auth::login_page).post(auth::login_submit))
        .add(
            "/register",
            get(auth::register_page).post(auth::register_submit),
        )
        .add("/packages/{name}", get(packages::show))
        .add("/templates", get(templates::index))
        .add("/templates/{guid}", get(templates::show))
        .add("/profile", get(user::profile))
        .add("/settings", get(user::settings))
}
