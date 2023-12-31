use std::sync::Arc;

use axum::{
	extract::State,
	response::{IntoResponse, Response},
	routing::post,
	Json, Router,
};
use serde::Deserialize;
use serde_json::{from_str, from_value, json, to_value, Value};
use tracing::debug;

use crate::{
	ctx::Ctx,
	model::{order::order::OrderForCreate, ModelManager},
	web::{
		rpc::order_rpc::{create_order, update_order},
		rpc::{
			material_rpc::list_materials, order_rpc::get_orders, user_rpc::get_user,
		},
		Error, Result,
	},
};

mod material_rpc;
mod order_rpc;
mod user_rpc;

//RPC Types

#[derive(Deserialize)]
struct RpcRequest {
	id: Option<Value>,
	method: String,
	params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
	id: i64,
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
	id: i64,
}

/// RPC basic information holding the id and method for further logging.
#[derive(Debug)]
pub struct RpcInfo {
	pub id: Option<Value>,
	pub method: String,
}

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/rpc", post(rpc_handler))
		.with_state(mm)
}

async fn rpc_handler(
	State(mm): State<ModelManager>,
	ctx: Ctx,
	Json(rpc_req): Json<RpcRequest>,
) -> Response {
	//   Create the RPC Info to be set to the response.extensions.
	let rpc_info = RpcInfo {
		id: rpc_req.id.clone(),
		method: rpc_req.method.clone(),
	};

	//   Exec & Store RpcInfo in response.
	let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();
	res.extensions_mut().insert(rpc_info);

	res
}

macro_rules! exec_rpc_fn {
	//With params
	($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params:expr) => {{
		let rpc_fn_name = stringify!($rpc_fn);
		let params = $rpc_params.ok_or(Error::RpcMissingParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;
		let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;
		$rpc_fn($ctx, $mm, params).await.map(to_value)??
	}};

	//Without params
	($rpc_fn:expr, $ctx:expr, $mm:expr) => {
		$rpc_fn($ctx, $mm).await.map(to_value)??
	};
}

async fn _rpc_handler(
	ctx: Ctx,
	mm: ModelManager,
	rpc_req: RpcRequest,
) -> Result<Json<Value>> {
	let RpcRequest {
		id: rpc_id,
		method: rpc_method,
		params: rpc_params,
	} = rpc_req;

	debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");
	let result_json: Value = match rpc_method.as_str() {
		"create_order" => exec_rpc_fn!(create_order, ctx, mm, rpc_params),
		"get_orders" => exec_rpc_fn!(get_orders, ctx, mm, rpc_params),
		"update_order" => exec_rpc_fn!(update_order, ctx, mm, rpc_params),

		"get_materials" => exec_rpc_fn!(list_materials, ctx, mm),
		"get_user" => exec_rpc_fn!(get_user, ctx, mm, rpc_params),

		_ => return Err(Error::RpcMethodUnknown(rpc_method)),
	};

	let body_response = json!({
		"id" : rpc_id,
		"result" : result_json
	});

	Ok(Json(body_response))
}
