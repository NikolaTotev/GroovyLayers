use crate::ctx::Ctx;
use crate::model::material::{Material, MaterialBmc};
use crate::model::order::order::{Order, OrderBmc, OrderForCreate};
use crate::model::order::*;
use crate::model::user::{User, UserBmc, UserBy, UserForLogin};
use crate::model::ModelManager;
use crate::web::Result;

use crate::web::rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded};

// pub async fn create_order(
// 	ctx: Ctx,
// 	mm: ModelManager,
// 	params: ParamsForCreate<OrderForCreate>,
// ) -> Result<Order> {
//     let ParamsForCreate {data} = params;
//     let id = OrderBmc::create(&ctx, &mm, data).await?;
//     let task = OrderBmc::get(&ctx, &mm, id).await?;

// 	Ok(task)
// }

pub async fn get_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<User> {
	let ParamsIded { id } = params;

	let user = UserBmc::get_user(&ctx, &mm, id).await?;
	Ok(user)
}
