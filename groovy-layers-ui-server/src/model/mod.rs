// region:    --- Modules

mod error;
mod store;
pub mod task; //Will be order for the groovy layers

pub use self::error::{Error, Result};
use self::store::new_db_pool;
use crate::model::store::Db;

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

//Model manager has states
impl ModelManager {
	///Constructor
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		// FIXME - TBC
		Ok(ModelManager { db })
	}

    ///Returns the sqlx db pool reference.
    ///(Only for the model layer)
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
