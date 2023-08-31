use crate::ctx::Ctx;
use crate::model;
use crate::model::order::Error;
use crate::model::ModelManager;
use crate::model::Result;
use crate::utils::format_time;
use crate::utils::now_utc;
use hmac::digest::typenum::Or;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
	Pending,
	Printing,
	Failed,
	QA,
	RePrinting,
	Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatus {
	status: Status,
	details: String,
	error: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Order {
	pub id: i64,
	pub user_id: i64,
	pub material_id: i64,
	pub quality_setting: i64,
	pub file_location: String,
	pub print_job_file: String,
	pub status: OrderStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderForCreate {
	pub user_id: i64,
	pub material_id: i64,
	pub quality_setting: i64,
	pub file_location: String,
	pub print_job_file: String,
	pub status: OrderStatus,
}

#[derive(Deserialize, FromRow)]
pub struct OrderForGet {
	pub id: i64,
	pub user_id: i64,
	pub material_id: i64,
	pub quality_setting: i64,
	pub file_location: String,
	pub print_job_file: String,
	pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderForUpdate {
	pub id: i64,
	pub status: Status,
}

//order Bmc
pub struct OrderBmc;

impl OrderBmc {
	fn transform_get_order(order: OrderForGet) -> Order {
		let order = Order {
			id: order.id,
			user_id: order.user_id,
			material_id: order.material_id,
			quality_setting: order.quality_setting,
			file_location: order.file_location,
			print_job_file: order.print_job_file,
			status: serde_json::from_str(&order.status).unwrap(),
		};

		order
	}

	pub async fn create(
		_ctx: &Ctx,
		mm: &ModelManager,
		order_c: OrderForCreate,
	) -> Result<i64> {
		info!("->> create_order {:?}", order_c);

		let db = mm.db();
		let (id,) = sqlx::query_as::<_, (i64,)>(
			"INSERT INTO groovy_layers.orders 
			(user_id, material_id, quality_setting file_location, print_job_file, status, last_update) 
			values 
			($1, $2, $3, $4, $5) 
			returning id",
		)
		.bind(order_c.user_id)
		.bind(order_c.material_id)
		.bind(order_c.quality_setting)
		.bind(order_c.file_location)
		.bind(order_c.print_job_file)
		.bind(serde_json::to_string(&order_c.status).unwrap())
		.bind(format_time(now_utc()))
		.fetch_one(db)
		.await?;

		Ok(id)
	}

	pub async fn update_status(
		_ctx: &Ctx,
		mm: &ModelManager,
		order_u: OrderForUpdate,
	) -> Result<i64> {
		info!("->> create_order {:?}", order_u);

		let db = mm.db();
		let serialized_status = serde_json::to_string(&order_u.status)
			.map_err(|err| Error::SerdeJson(err.to_string()))?;
		let (id,) = sqlx::query_as::<_, (i64,)>(
			"UPDATE groovy_layers.orders 
			SET status = $2, 
			last_update = $3 
			WHERE id = $1		
			",
		)
		.bind(order_u.id)
		.bind(serialized_status)
		.bind(format_time(now_utc()))
		.fetch_one(db)
		.await?;

		Ok(id)
	}

	pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Order> {
		let db = mm.db();
		info!("->> get_order {:?}", id);

		let order: OrderForGet =
			sqlx::query_as("SELECT * FROM groovy_layers.orders WHERE id = $1")
				.bind(id)
				.fetch_optional(db)
				.await?
				.ok_or(model::Error::EntityNotFound {
					entity: "orders",
					id,
				})?;

		let order = Self::transform_get_order(order);

		Ok(order)
	}

	pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		let db = mm.db();
		info!("->> delete_order {:?}", id);
		let count = sqlx::query("DELETE * FROM groovy_layers.orders WHERE id = $1")
			.bind(id)
			.execute(db)
			.await?
			.rows_affected();

		if count == 0 {
			return Err(model::Error::EntityNotFound {
				entity: "order",
				id: id,
			});
		}
		Ok(())
	}

	pub async fn list(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
	) -> Result<Vec<Order>> {
		let db = mm.db();
		info!("->> list_orders {:?}", user_id);
		let orders: Vec<OrderForGet> =
			sqlx::query_as("SELECT * FROM groovy_layers.orders WHERE user_id = $1")
				.bind(user_id)
				.fetch_all(db)
				.await?;

		let orders = orders.into_iter().map(Self::transform_get_order).collect();
		Ok(orders)
	}
}

#[cfg(test)]
mod tests {
	#![allow(unused)]
	use crate::{_dev_utils, model};

	use super::*;
	use anyhow::Result;
	use serial_test::serial;
	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();

		let fx_status = OrderStatus {
			status: Status::Printing,
			details: "details".to_string(),
			error: "error".to_string(),
		};
		let fx_user_id = 999;
		let fx_material_id = 999;
		let fx_quality_setting = 999;
		let fx_file_location = "test_create_ok title".to_string();
		let fx_print_location = "test_create_ok title".to_string();

		let order_c = OrderForCreate {
			user_id: fx_user_id,
			material_id:fx_material_id,
			quality_setting: fx_quality_setting,
			file_location: fx_file_location,
			print_job_file: fx_print_location,
			status: fx_status,
		};

		let id = OrderBmc::create(&ctx, &mm, order_c).await?;

		let (user_id,): (i64,) =
			sqlx::query_as("SELECT user_id from groovy_layers.orders where id = $1")
				.bind(id)
				.fetch_one(mm.db())
				.await?;

		assert_eq!(user_id, fx_user_id);

		//Cleanup
		let count = sqlx::query("DELETE FROM groovy_layers.orders WHERE id = $1")
			.bind(id)
			.execute(mm.db())
			.await?
			.rows_affected();
		assert_eq!(count, 1, "Did not delete 1 row?");

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_err_not_found() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;
		let res = OrderBmc::get(&ctx, &mm, fx_id).await;
		assert!(
			matches!(
				res,
				Err(model::Error::EntityNotFound {
					entity: "orders",
					id: 100
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_found() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_status = OrderStatus {
			status: Status::Printing,
			details: "details".to_string(),
			error: "error".to_string(),
		};
		let fx_user_id = 999;
		let fx_material_id = 999;
		let fx_quality_setting = 999;
		let fx_file_location = "test_create_ok title".to_string();
		let fx_print_location = "test_create_ok title".to_string();

		let order_c = OrderForCreate {
			user_id: fx_user_id,
			material_id:fx_material_id,
			quality_setting: fx_quality_setting,
			file_location: fx_file_location,
			print_job_file: fx_print_location,
			status: fx_status,
		};

		let id = OrderBmc::create(&ctx, &mm, order_c).await?;
		let res = OrderBmc::get(&ctx, &mm, id).await?;

		assert_eq!(res.user_id, fx_user_id);
		Ok(())
	}

	//ToDo make delete test.
}
