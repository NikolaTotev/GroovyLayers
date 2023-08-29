use crate::crypt::pwd;
use crate::crypt::EncryptContent;
use crate::ctx::Ctx;
use crate::model::Error;
use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use tracing::info;
use uuid::Uuid;
#[derive(Clone, FromRow, Debug, Serialize)]
pub struct User {
	pub id: i64,
	pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
	pub email: String,
	pub username: String,
	pub pwd_clear: String,
}

struct UserForInsert {
	email: String,
	username: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForLogin {
	pub id: i64,
	pub email: String,
	pub username: String,
	pub pwd: Option<String>,
	pub pwd_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Debug)]
pub struct UserForAuth {
	pub id: i64,
	pub email: String,
	pub username: String,
	pub token_salt: Uuid,
}

pub trait UserBy: for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

pub struct UserBmc;

impl UserBmc {
	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		let db = mm.db();

		let order: E =
			sqlx::query_as("SELECT * FROM groovy_layers.users WHERE id = $1")
				.bind(id)
				.fetch_optional(db)
				.await?
				.ok_or(Error::EntityNotFound {
					entity: "users",
					id,
				})?;

		Ok(order)
	}

	pub async fn first_by_username<E>(
		ctx: &Ctx,
		mm: &ModelManager,
		username: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		let db = mm.db();

		let order =
			sqlx::query_as("SELECT * FROM groovy_layers.users WHERE username = $1")
				.bind(username)
				.fetch_optional(db)
				.await?;

		Ok(order)
	}

	pub async fn update_pwd(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		pwd_clear: &str,
	) -> Result<()> {
		let db = mm.db();

		let user: UserForLogin = Self::get(ctx, mm, id).await?;
		let pwd = pwd::encrypt_pwd(&EncryptContent {
			content: pwd_clear.to_string(),
			salt: user.pwd_salt.to_string(),
		})?;

		sqlx::query(
			"UPDATE groovy_layers.users
		SET pwd = $1
		WHERE id = $2",
		)
		.bind(pwd)
		.bind(id)
		.execute(db)
		.await?;

		Ok(())
	}
}

// region:    --- Tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::_dev_utils;
	use anyhow::{Context, Result};
	use serial_test::serial;

	#[serial]
	#[tokio::test]
	async fn test_first_ok_demo1() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_username = "demo1";

		// -- Exec
		let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
			.await?
			.context("Should have user 'demo1'")?;

		// -- Check
		assert_eq!(user.username, fx_username);

		Ok(())
	}
}
// endregion: --- Tests
