use super::db::connect;
use log::{info, error};
use std::fmt::Debug;

// Trait for logging and doing the .expect()
pub trait LogErrResult<T, E : Debug> {
    fn log_expect (self, msg: &str) -> T;
}

impl<T, E: Debug> LogErrResult<T, E> for Result<T, E> {
    fn log_expect (self, msg: &str) -> T {
        self.map_err(|e| {error!(target: "file", "{}: {:?}", msg, e); e}).expect(msg)
    }
}

pub fn create_purchase_order(cid: i64, bid: i64) -> i64 {
    let db = connect();
    let query = "INSERT INTO PurchaseOrders (customerId, bookId, shipped) VALUES (:cid, :bid, 0)";
    let mut stmt = db.prepare(query).log_expect("expected to prepare statement correctly in prepare");
    stmt.execute(&[(":cid",  &format!("{}",cid)), (":bid",  &format!("{}",bid))])
        .log_expect("expected to be able to insert into PurchaseOrders table in execute");
    info!(target: "file", "Successfully created order of book id: {} from customer id: {}", bid, cid);

    // This return is now used to give the user their order id
    return get_purchase_order_id(cid, bid);
}

pub fn get_purchase_order_id(cid: i64, bid: i64) -> i64 {
    let db = connect();
    let query = "SELECT id FROM PurchaseOrders WHERE customerId = :cid AND bookId = :bid";
    let mut stmt = db.prepare(query).log_expect("expected to be able to select from PurchaseOrders table in prepare");

    let mut rows = stmt
        .query_map(&[(":cid",  &format!("{}",cid)), (":bid",  &format!("{}",bid))], |row| row.get(0))
        .log_expect("expected to be able to get purchase order id from PurchaseOrders table in query_map");

    let id = rows.next().unwrap().unwrap();
    return id;
}

pub fn is_po_shipped(poid: i64) -> i64 {
    let db = connect();
    let query = "SELECT shipped FROM PurchaseOrders WHERE id = :poid";
    let mut stmt = db.prepare(query).log_expect("expected to be able to select from PurchaseOrders table in prepare");

    let mut rows = stmt
        .query_map(&[(":poid",  &format!("{}",poid))], |row| row.get(0))
        .log_expect("expected to be able to get shipped status from PurchaseOrders table in query_map");
    let shipped: i64 = rows.next().unwrap().unwrap();
    info!(target: "file", "Successfully got shipping status of {} for purchase order id {}", shipped, poid);
    return shipped;
}

pub fn ship_po(poid: i64) {
    let db = connect();
    let query = "UPDATE PurchaseOrders SET shipped = 1 WHERE id = :poid";
    let mut stmt = db.prepare(query).log_expect("expected to be able to update PurchaseOrders table in prepare");
    stmt.execute(&[(":poid", &format!("{}",poid))]).log_expect("expected to be able to update PurchaseOrders table in execute");
    info!(target: "file", "Successfully updated shipped status of purchase order id {} to {}", poid, 1);
}
