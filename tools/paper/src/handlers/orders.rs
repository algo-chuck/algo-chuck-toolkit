use axum::Json;
use schwab_api::prelude::trader::{Order, PreviewOrder};

use crate::{Created, EmptyOK, Result};

pub async fn get_orders_by_path_param() -> Result<Json<Vec<Order>>> {
    println!("->> {:<12} - get_orders_by_path_param", "HANDLER");
    Ok(Json(vec![Order::new()]))
}

pub async fn place_order() -> Result<Created> {
    println!("->> {:<12} - place_order", "HANDLER");
    Ok(Created {})
}

pub async fn get_order() -> Result<Json<Order>> {
    println!("->> {:<12} - get_order", "HANDLER");
    Ok(Json(Order::new()))
}

pub async fn cancel_order() -> Result<EmptyOK> {
    println!("->> {:<12} - cancel_order", "HANDLER");
    Ok(EmptyOK {})
}

pub async fn replace_order() -> Result<Created> {
    println!("->> {:<12} - replace_order", "HANDLER");
    Ok(Created {})
}

pub async fn get_orders_by_query_param() -> Result<Json<Vec<Order>>> {
    println!("->> {:<12} - get_orders_by_query_param", "HANDLER");
    Ok(Json(vec![Order::new()]))
}

pub async fn preview_order() -> Result<Json<PreviewOrder>> {
    println!("->> {:<12} - preview_order", "HANDLER");
    Ok(Json(PreviewOrder::new()))
}
