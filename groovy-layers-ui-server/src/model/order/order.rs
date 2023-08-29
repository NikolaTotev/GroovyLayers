use crate::ctx::Ctx;
use crate::model::Error;
use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::info;


#[derive(Debug, Clone, Serialize)]
pub enum Status {	
	Pending, 
	Printing,
	Failed,
	QA,
	RePrinting,
	Completed,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderStatus {
	status: Status,
	details: String,
	error: String
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Order {
	pub id: i64,
	pub user_id: i64,
	pub file_location: String,
	pub print_job_file: String,
	pub status: String

}

#[derive(Deserialize)]
pub struct OrderForCreate {
	pub title: String,
}

#[derive(Deserialize)]
pub struct OrderForUpdate {
	pub title: String,
}

//order Bmc
pub struct OrderBmc;

impl OrderBmc {
	pub async fn create(
		_ctx: &Ctx,
		mm: &ModelManager,
		order_c: OrderForCreate,
	) -> Result<i64> {
		let db = mm.db();
		let (result,) = sqlx::query_as::<_, (i64,)>(
			"INSERT INTO groovy_layers.orders (title) values ($1) returning id",
		)
		.bind(order_c.title)
		.fetch_one(db)
		.await?;
		info!("s");
		Ok(result)
	}

	pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Order> {
		let db = mm.db();

		let order: Order =
			sqlx::query_as("SELECT * FROM groovy_layers.orders WHERE id = $1")
				.bind(id)
				.fetch_optional(db)
				.await?
				.ok_or(Error::EntityNotFound { entity: "orders", id })?;

		Ok(order)
	}

	pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		let db = mm.db();

		let count = sqlx::query("DELETE * FROM groovy_layers.orders WHERE id = $1")
			.bind(id)
			.execute(db)
			.await?
			.rows_affected();

		if count == 0 {
			return Err(Error::EntityNotFound {
				entity: "order",
				id: id,
			});
		}
		Ok(())
	}

	pub async fn list(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Vec<Order>> {
		let db = mm.db();

		let orders: Vec<Order> =
			sqlx::query_as("SELECT * FROM groovy_layers.orders order BY id")
				.fetch_all(db)
				.await?;

		Ok(orders)
	}
}

#[cfg(test)]
mod tests {
	#![allow(unused)]
	use crate::_dev_utils;

	use super::*;
	use anyhow::Result;
	use serial_test::serial;
	#[serial]
	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_title = "test_create_ok title";

		let order_c = OrderForCreate {
			title: fx_title.to_string(),
		};

		let id = OrderBmc::create(&ctx, &mm, order_c).await?;

		let (title,): (String,) =
			sqlx::query_as("SELECT title from groovy_layers.orders where id = $1")
				.bind(id)
				.fetch_one(mm.db())
				.await?;
		assert_eq!(title, fx_title);

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
				Err(Error::EntityNotFound {
					entity: "orders",
					id: 100
				})
			),
			"EntityNotFound not matching"
		);

		Ok(())
	}

	//ToDo make delete test.
}
