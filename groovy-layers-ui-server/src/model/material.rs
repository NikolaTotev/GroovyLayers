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

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Material {
	pub id: i64,
	pub name: String,
	pub description: String,
	pub image_url: String,
	pub material_type: String,
	pub amount_kg: f64,
	pub price_kg: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialForCreate {
	pub name: String,
	pub description: String,
	pub image_url: String,
	pub material_type: String,
	pub amount_kg: f64,
	pub price_kg: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialForUpdate {
	pub name: String,
	pub description: String,
	pub image_url: String,
	pub material_type: String,
	pub amount_kg: f64,
	pub price_kg: f64,
}

pub struct MaterialBmc;

impl MaterialBmc {
	pub async fn create(
		_ctx: &Ctx,
		mm: &ModelManager,
		material_c: Material,
	) -> Result<i64> {
		info!("->> create_material {:?}", material_c);

		let db = mm.db();
		let (id,) = sqlx::query_as::<_, (i64,)>(
			"INSERT INTO groovy_layers.material_inventory 
			(name, description,image_url, amount_kg, material_type, price_kg) 
			values 
			($1, $2, $3, $4, $5, $6) 
			returning id",
		)
		.bind(material_c.name)
		.bind(material_c.description)
		.bind(material_c.image_url)
		.bind(material_c.amount_kg)
		.bind(material_c.material_type)
		.bind(material_c.price_kg)
		.fetch_one(db)
		.await?;

		Ok(id)
	}

	pub async fn update(
		_ctx: &Ctx,
		mm: &ModelManager,
		material_u: MaterialForUpdate,
	) -> Result<i64> {
		todo!()
	}

	pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Material> {
		let db = mm.db();
		info!("->> get_order {:?}", id);

		let material: Material = sqlx::query_as(
			"SELECT * FROM groovy_layers.material_inventory WHERE id = $1",
		)
		.bind(id)
		.fetch_optional(db)
		.await?
		.ok_or(model::Error::EntityNotFound {
			entity: "material_inventory",
			id,
		})?;

		Ok(material)
	}

	pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		let db = mm.db();
		info!("->> delete_material {:?}", id);
		let count = sqlx::query(
			"DELETE * FROM groovy_layers.material_inventory WHERE id = $1",
		)
		.bind(id)
		.execute(db)
		.await?
		.rows_affected();

		if count == 0 {
			return Err(model::Error::EntityNotFound {
				entity: "material_inventory",
				id: id,
			});
		}
		Ok(())
	}

	pub async fn list(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
	) -> Result<Vec<Material>> {
		let db = mm.db();
		info!("->> list_materials");
		let materials: Vec<Material> =
			sqlx::query_as("SELECT * FROM groovy_layers.material_inventory")
				.bind(user_id)
				.fetch_all(db)
				.await?;

		Ok(materials)
	}
}
