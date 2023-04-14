use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::{customers, purchaseOrders, books};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    order_id: Option<i64>,
    customer_id: Option<i64>,
    book_id: Option<i64>,
    shipped: Option<i64>,
}

// TODO Do some validation on id numbers
// TODO logging for insufficient funds
// TODO logging example for orders
#[post("/new", data = "<order>")]
pub fn create_order(order: Json<Order>) -> Result<String, String> {
    let cid = match order.customer_id {
        Some(c) => c,
        None => return Err("No customer_id provided".to_string()),
    };
    let bid = match order.book_id {
        Some(b) => b,
        None => return Err("No book_id provided".to_string()),
    };
    let balance = customers::get_customer_balance(cid);
    let price = books::get_book_price(bid);

    match balance - price >= 0.0 {
        true => 0,
        false => return Err(format!("Insufficient funds. You have ${:.2}, the price of the book is ${:.2}", balance, price)),
    };
    customers::update_customer_balance(cid, balance-price);

    let oid = purchaseOrders::create_purchase_order(cid, bid);
    let success_msg = format!("Successfully created order for Customer id: {}\n\t Your orderId is {}", cid, oid);
    Ok(success_msg)
}

#[get("/shipped", format = "json", data = "<order>")]
pub fn get_shipped(order: Json<Order>) -> Result<String, String> {
    let cid = match order.customer_id {
        Some(c) => c,
        None => return Err("No customer_id provided".to_string()),
    };
    let bid = match order.book_id {
        Some(b) => b,
        None => return Err("No book_id provided".to_string()),
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
pub fn ship_order(order: Json<Order>) -> Result<String, String> {
    let oid = match order.order_id.clone() {
        Some(o) => o,
        None => return Err("No order_id provided".to_string()),
    };

    purchaseOrders::ship_po(oid);
    let success_msg = format!("Successfully shipped your Order ID: {}!", oid);
    Ok(success_msg)
}

#[get("/status", format = "json", data = "<order>")]
pub fn get_status(order: Json<Order>) -> Result<String, String> {
    // Removed clone for ints, not necessary
    let oid = match order.order_id{
        Some(o) => o,
        None => return Err("No order_id provided".to_string()),
    };

    let cid = match order.customer_id.clone() {
        Some(c) => c,
        None => return Err("No customer_id provided".to_string()),
    };

    let bid = match order.book_id.clone() {
        Some(b) => b,
        None => return Err("No book_id provided".to_string()),
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

