#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	println!(">>>> Login ==========================================");
	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);

	req_login.await?.print().await?;

	println!(">>>> Create Order #1 ==========================================");
	let req_create_order_1 = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_order",
			"params": {
				"data": {
					"user_id": 1000,
					"material_id": 1,				
					"quality_setting": 1,
					"file_location": "file_location",
					"print_job_file": "print_job_file",					
				}
			}
		}),
	);
	req_create_order_1.await?.print().await?;

	println!(">>>> Create Order #2 ==========================================");
	let req_create_order_2 = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_order",
			"params": {
				"data": {
					"user_id": 1000,
					"material_id": 2,				
					"quality_setting": 2,
					"file_location": "file_location",
					"print_job_file": "print_job_file",					
				}
			}
		}),
	);
	req_create_order_2.await?.print().await?;

	println!(">>>> Get materials #1 ==========================================");
	let req_get_materials = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "get_materials",			
		}),
	);
	req_get_materials.await?.print().await?;

	println!(">>>> Get User ==========================================");
	let req_get_user = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "get_user",
			"params" : {
				"id":1000
			}			
		}),
	);
	req_get_user.await?.print().await?;

	Ok(())
}
