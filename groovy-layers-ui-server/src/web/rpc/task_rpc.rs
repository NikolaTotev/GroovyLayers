use std::sync::Arc;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::order::order::{Order, OrderBmc, OrderForCreate};
use crate::web::Result;
use crate::model::order::*;


use super::ParamsForCreate;

pub async fn create_task(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<OrderForCreate>,
) -> Result<Order> {
    let ParamsForCreate {data} = params;
    let id = OrderBmc::create(&ctx, &mm, data).await?;
    let task = OrderBmc::get(&ctx, &mm, id).await?;

	Ok(task)
}
