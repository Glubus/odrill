//! Templates controller - handles template registry operations

#![allow(clippy::unused_async)]

mod delete;
mod download;
mod list;
mod publish;
mod security;
mod show;
mod types;
mod update;

use axum::routing::{delete, get, post, put};
use loco_rs::prelude::*;

pub use types::{ListParams, TemplateResponse, UpdateTemplateParams};

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/templates")
        .add("/", get(list::list))
        .add("/publish", post(publish::publish))
        .add("/{guid}", get(show::show))
        .add("/{guid}", put(update::update))
        .add("/{guid}", delete(delete::delete_template))
        .add("/{guid}/download", get(download::download))
}
