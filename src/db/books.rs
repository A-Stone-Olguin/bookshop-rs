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

pub fn create_book(title: String, author: String, price: f64) {
    let db = connect();
    let query = "INSERT INTO books (title, author, price) VALUES (:title, :author, :price)";
    let mut stmt = db.prepare(query).log_expect("expected to prepare statement correctly in prepare");
    stmt.execute(&[(":title", &title), (":author", &author), (":price", &format!("{}",price))])
        .log_expect("expected to be able to insert into Books table in execute");
    info!(target: "file", "Successfully created book: Author: {}, Title: {}, Price: {:.2}", author, title, price);
}

pub fn get_book_id(title: String, author: String) -> i64 {
    let db = connect();
    let query = "SELECT id FROM books WHERE title = :title AND author = :author";
    let mut stmt = db.prepare(query).log_expect("expected to prepare statement correctly in prepare");

    let mut rows = stmt
        .query_map(&[(":title", &title), (":author", &author)], |row| row.get(0))
        .log_expect("expected to be able to get id from Books table in query_map");
    let id = rows.next().unwrap().unwrap();
    return id;
}

pub fn get_book_price(bid: i64) -> f64 {
    let db = connect();
    let query = "SELECT price FROM books WHERE id = :bid";
    let mut stmt = db.prepare(query).log_expect("expected to prepare statement correctly in prepare");
    
    let mut rows = stmt
        .query_map(&[(":bid", &format!("{}", bid))], |row| row.get(0))
        .log_expect("expected to be able to get price from Books table in query_map");
    let price = rows.next().unwrap().unwrap();

    info!(target: "file", "Successfully got book id: {}'s price of {:.2}", bid, price);
    return price;
}