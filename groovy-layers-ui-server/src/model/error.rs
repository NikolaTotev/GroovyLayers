use crate::{model::store, crypt};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::model::order;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound{entity: &'static str, id: i64},
	Store(store::Error),
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
	Order(order::Error),
	Crypt(crypt::Error)
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl From<store::Error> for Error {
	fn from(value: store::Error) -> Self {
		Self::Store(value)
	}
}

impl From<sqlx::Error> for Error {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}

impl From<order::Error> for Error {
	fn from(value: order::Error) -> Self {
		Self::Order(value)
	}
}

impl From<crypt::Error> for Error {
	fn from(value: crypt::Error) -> Self {
		Self::Crypt(value)
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
