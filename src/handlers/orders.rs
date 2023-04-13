use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::{customers, purchaseOrders};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    order_id: Option<i64>,
    customer_id: Option<i64>,
    book_id: Option<i64>,
    shipped: Option<i64>,
}

#[post("/new", data = "<order>")]
pub fn create_order(order: Json<Order>) -> Result<String, String> {
    let cid = match order.customer_id.clone() {
        Some(c) => c,
        None => return Err("No customer id provided".to_string()),
    };
    let bid = match order.book_id.clone() {
        Some(b) => b,
        None => return Err("No book id provided".to_string()),
    };

    let oid = purchaseOrders::create_purchase_order(cid, bid);
    let success_msg = format!("Successfully created order for TODO!\n\t Your orderId is {}", oid);
    Ok(success_msg)
}

#[get("/shipped", format = "json", data = "<order>")]
pub fn get_shipped(order: Json<Order>) -> Result<String, String> {
    let cid = match order.customer_id.clone() {
        Some(c) => c,
        None => return Err("No customer id provided".to_string()),
    };
    let bid = match order.book_id.clone() {
        Some(b) => b,
        None => return Err("No book id provided".to_string()),
    };

    let oid = purchaseOrders::get_purchase_order_id(cid, bid);
    let shipped = purchaseOrders::is_po_shipped(oid);
    let shipped_status = match shipped {
        0 => "Not Shipped".to_string(),
        1 => "Shipped".to_string(),
        _ => return Err("Invalid shipped status somehow".to_string()),
    };

    let success_message = format!("The shipped of Order ID {} is: {}", oid, shipped_status);
    Ok(success_message)
}

#[put("/ship", data = "<order>")]
pub fn ship_order(order: Json<Order>) -> Result<(), String> {
    let oid = match order.order_id.clone() {
        Some(o) => o,
        None => return Err("No order id provided".to_string()),
    };

    purchaseOrders::ship_po(oid);
    Ok(())
}

#[get("/status", format = "json", data = "<order>")]
pub fn get_status(order: Json<Order>) -> Result<String, String> {
    // Removed clone for ints, not necessary
    let oid = match order.order_id{
        Some(o) => o,
        None => return Err("No order id provided".to_string()),
    };

    let cid = match order.customer_id.clone() {
        Some(c) => c,
        None => return Err("No customer id provided".to_string()),
    };

    let bid = match order.book_id.clone() {
        Some(b) => b,
        None => return Err("No book id provided".to_string()),
    };

    let addr = customers::get_customer_address(cid);
    let shipped = purchaseOrders::is_po_shipped(oid);
    let shipped_status = match shipped {
        0 => "Not Shipped".to_string(),
        1 => "Shipped".to_string(),
        _ => return Err("Invalid shipped status somehow".to_string()),
    };

    // Changed html output to just string since we don't use html anywhere else
    // Don't need to check address since it is already from the database, so it has been validated
    let success_msg = format!("Order Status of Order ID: {} is {}\n\t Book ID: {}\n\t Customer ID: {}\n\t Shipping Address: {}",
                                     oid, shipped_status, bid, cid, addr);

    return Ok(success_msg)
}

