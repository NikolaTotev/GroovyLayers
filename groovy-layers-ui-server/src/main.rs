#![allow(unused)] // For early development.


mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod service_connections;
mod utils;
mod web;

//#[cfg(test)]
pub mod _dev_utils;
pub use self::error::{Error, Result};
use axum::http::header::{CONTENT_TYPE, COOKIE};
use axum::http::{HeaderValue, Method};
use axum::response::Html;
use axum::routing::get;
pub use config::config;

use crate::model::ModelManager;
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static, rpc};
use axum::{middleware, Router};
use tower_http::cors::{self, Any, CorsLayer};

use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::debug;
use tracing_subscriber::EnvFilter;


#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	//FOR DEV ONLY
	_dev_utils::init_dev().await;

	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	//Define Routes
	let routes_rpc =
		rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

	let cors = CorsLayer::new()
		.allow_methods([Method::GET, Method::POST])
		.allow_headers([CONTENT_TYPE, COOKIE])
		.allow_credentials(true)
		.allow_origin("http://127.0.0.1:5173".parse::<HeaderValue>().unwrap());
	
	let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
		.nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_reponse_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir())
		.layer(cors);

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	debug!("->> {:<12} - {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();

	Ok(())
}
