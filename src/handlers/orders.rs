use rocket::{response::content::RawHtml, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::db::{customers, purchaseOrders};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    id: Option<i64>,
    customer_id: Option<i64>,
    book_id: Option<i64>,
    shipped: Option<i64>,
}

#[post("/new", data = "<order>")]
pub fn create_order(order: Json<Order>) -> Result<(), String> {
    let cid = match order.customer_id.clone() {
        Some(c) => c,
        None => return Err("No customer id provided".to_string()),
    };
    let bid = match order.book_id.clone() {
        Some(b) => b,
        None => return Err("No book id provided".to_string()),
    };

    purchaseOrders::create_purchase_order(cid, bid);
    Ok(())
}

#[get("/shipped", format = "json", data = "<order>")]
pub fn get_shipped(order: Json<Order>) -> Result<Json<Order>, String> {
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
    Ok(Json(Order {
        id: None,
        customer_id: None,
        book_id: None,
        shipped: Some(shipped),
    }))
}

#[put("/ship", data = "<order>")]
pub fn ship_order(order: Json<Order>) -> Result<(), String> {
    let oid = match order.id.clone() {
        Some(o) => o,
        None => return Err("No order id provided".to_string()),
    };

    purchaseOrders::ship_po(oid);
    Ok(())
}

#[get("/status", format = "json", data = "<order>")]
pub fn get_status(order: Json<Order>) -> Result<RawHtml<String>, String> {
    let oid = match order.id.clone() {
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

    let response_html = format!(
        "
        <html>
        <title>Order Status</title>
        </head>
        <body>
        <h1>Order Status</h1>
        <p>Order ID: {}</p>
        <p>Book ID: {}</p>
        <p>Customer ID: {}</p>
        <p>Shipping Address: {}</p>
        </body>
        </html>
    ",
        oid,
        bid,
        cid,
        &addr.as_str()
    );

    Ok(RawHtml(response_html.clone()))
}
