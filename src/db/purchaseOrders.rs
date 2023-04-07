use super::db::connect;
use log::info;

pub fn create_purchase_order(cid: i64, bid: i64) -> i64 {
    let db = connect();
    let query = "INSERT INTO PurchaseOrders (customerId, bookId, shipped) VALUES (:cid, :bid, 0)";
    let mut stmt = db.prepare(query).expect("expected to prepare statement correctly");
    stmt.execute(&[(":cid",  &format!("{}",cid)), (":bid",  &format!("{}",bid))])
        .expect("expected to be able to insert into PurchaseOrders table");
    info!(target: "file", "Successfully created order of book id: {} from customer id: {}", bid, cid);

    // TODO another return, need to investigate why
    return get_purchase_order_id(cid, bid);
}

pub fn get_purchase_order_id(cid: i64, bid: i64) -> i64 {
    let db = connect();
    let query = "SELECT id FROM PurchaseOrders WHERE customerId = :cid AND bookId = :bid";
    let mut stmt = db.prepare(query).expect("expected to be able to select from PurchaseOrders table");

    let mut rows = stmt
        .query_map(&[(":cid",  &format!("{}",cid)), (":bid",  &format!("{}",bid))], |row| row.get(0))
        .expect("expected to be able to get purchase order id from PurchaseOrders table");

    let id = rows.next().unwrap().unwrap();
    return id;
}

pub fn is_po_shipped(poid: i64) -> i64 {
    let db = connect();
    let query = "SELECT shipped FROM PurchaseOrders WHERE id = :poid";
    let mut stmt = db.prepare(query).expect("expected to be able to select from PurchaseOrders table");

    let mut rows = stmt
        .query_map(&[(":poid",  &format!("{}",poid))], |row| row.get(0))
        .expect("expected to be able to get shipped status from PurchaseOrders table");
    let shipped: i64 = rows.next().unwrap().unwrap();
    info!(target: "file", "Successfully got shipping status of {} for purchase order id {}", shipped, poid);
    return shipped;
}

pub fn ship_po(poid: i64) {
    let db = connect();
    let query = "UPDATE PurchaseOrders SET shipped = 1 WHERE id = :poid";
    let mut stmt = db.prepare(query).expect("expected to be able to update PurchaseOrders table");
    stmt.execute(&[(":poid", &format!("{}",poid))]).expect("expected to be able to update PurchaseOrders table");
    info!(target: "file", "Successfully updated shipped status of purchase order id {} to {}", poid, 1);
}
