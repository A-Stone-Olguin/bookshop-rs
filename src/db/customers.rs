use super::db::connect;
use log::{info, error};
use std::fmt::Debug;
pub trait LogErrResult<T, E : Debug> {
    fn log_expect (self, msg: &str) -> T;
}

impl<T, E: Debug> LogErrResult<T, E> for Result<T, E> {
    fn log_expect (self, msg: &str) -> T {
        self.map_err(|e| {error!(target: "file", "{}: {:?}", msg, e); e}).expect(msg)
    }
}

pub fn create_customer(name: String, address: String) {
    let db = connect();
    // Default balance of 5 dollars is added
    let query = "INSERT INTO customers (name, shippingAddress, accountBalance) VALUES (:name, :address, 5.00)";
    let mut stmt = db.prepare(query).log_expect("expected to prepare statement correctly in prepare");
    stmt.execute(&[(":name", &name), (":address", &address)])
        .log_expect("expected to be able to insert into Customers table in execute");
    info!(target: "file", "Successfully created customer: {}, Address: {}", name, address);
}

pub fn get_customer_id(name: String, address: String) -> i64 {
    let db = connect();
    let query = "SELECT id FROM customers WHERE name = :name AND shippingAddress = :address";
    let mut stmt = db.prepare(query).log_expect("expected to be able to select from Customers table in prepare");

    let mut rows = stmt
        .query_map(&[(":name", &name), (":address", &address)], |row| row.get(0))
        .log_expect("expected to be able to get id from Customers table in query_map");
    let id = rows.next().unwrap().unwrap();
    return id;
}

pub fn get_customer_address(cid: i64) -> String {
    let db = connect();
    let query = "SELECT shippingAddress FROM customers WHERE id = :cid";
    let mut stmt = db.prepare(query).log_expect("expected to be able to select from Customers table in prepare");

    let mut rows = stmt
        .query_map(&[(":cid", &format!("{}",cid))], |row| row.get(0))
        .log_expect("expected to be able to get shippingAddress from Customers table in query_map");
    let address = rows.next().unwrap().unwrap();
    info!(target: "file", "Successfully retrieved cid {}'s address: {}", cid, address);
    return address;
}

pub fn update_customer_address(cid: i64, address: String) {
    let db = connect();
    let query = "UPDATE customers SET shippingAddress = :address WHERE id = :cid";
    let mut stmt = db.prepare(query).log_expect("expected to be able to update Customers table in prepare");
    stmt.execute(&[(":address", &address), (":cid", &format!("{}",cid))]).log_expect("expected to be able to update Customers table in execute");
    info!(target: "file", "Successfully updated address of cid {} to {}", cid, address);
}

pub fn get_customer_balance(cid: i64) -> f64 {
    let db = connect();
    let query = "SELECT accountBalance FROM customers WHERE id = :cid";
    let mut stmt = db.prepare(query).log_expect("expected to be able to select from Customers table in prepare");

    let mut rows = stmt
        .query_map(&[(":cid", &format!("{}",cid))], |row| row.get(0))
        .log_expect("expected to be able to get accountBalance from Customers table in query_map");
    let balance = rows.next().unwrap().unwrap();
    info!(target: "file", "Successfully retrieved cid {}'s balance: {:.2}", cid, balance);
    return balance;
}

pub fn update_customer_balance(cid : i64, balance : f64) {
    let db = connect();
    let query = "UPDATE customers SET accountBalance = :balance WHERE id = :cid";
    // let mut stmt = db.prepare(query).expect("expected to be able to update Customers table");
    let mut stmt = db.prepare(query).log_expect("expected to be able to update Customers table in prepare");
    stmt.execute(&[(":balance", &balance), (":cid", &(cid as f64))]).log_expect("expected to be able to update Customers table in execute");
    info!(target: "file", "Successfully updated balance of cid {} to {}", cid, balance);
}
