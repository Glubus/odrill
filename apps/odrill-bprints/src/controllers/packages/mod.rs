//! Packages controller - handles package registry operations

#![allow(clippy::unused_async)]

mod get;
mod publish;

use axum::routing::{get, post};
use loco_rs::prelude::*;

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/packages")
        .add("/publish", post(publish::handler))
        .add("/{name}", get(get::get_package))
        .add("/{name}/{version}", get(get::get_version))
}
