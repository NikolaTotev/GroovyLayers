#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	//hc.do_get("/index.html").await?.print().await?;

	println!(">>>> Login");
	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);

	req_login.await?.print().await?;

	// let req_create_task = hc.do_post(
	// 	"/api/rpc",
	// 	json!({
	// 		"id": 1,
	// 		"method": "create_order",
	// 		"params": {
	// 			"data": {
	// 				"user_id": 1000,
	// 				"file_location": "file_location",
	// 				"print_job_file": "print_job_file",					
	// 			}
	// 		}
	// 	}),
	// );
	// req_create_task.await?.print().await?;

	let req_get_materials = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "get_materials",			
		}),
	);
	req_get_materials.await?.print().await?;

	Ok(())
}
