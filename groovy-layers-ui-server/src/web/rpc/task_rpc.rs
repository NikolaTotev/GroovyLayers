
use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::order::order::{Order, OrderBmc, OrderForCreate};
use crate::web::Result;
use crate::model::order::*;


use crate::web::rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded};


pub async fn create_order(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<OrderForCreate>,
) -> Result<Order> {
    let ParamsForCreate {data} = params;
    let id = OrderBmc::create(&ctx, &mm, data).await?;
    let task = OrderBmc::get(&ctx, &mm, id).await?;

	Ok(task)
}

pub async fn get_orders(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Vec<Order>> {
	let ParamsIded { id } = params;

	let orders = OrderBmc::list(&ctx, &mm, id).await?;
	

	Ok(orders)
}