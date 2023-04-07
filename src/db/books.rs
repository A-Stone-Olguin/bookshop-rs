// use std::fmt::format;

use super::db::connect;
use log::info;

pub fn create_book(title: String, author: String, price: f64) {
    let db = connect();
    let query = "INSERT INTO books (title, author, price) VALUES (:title, :author, :price)";
    let mut stmt = db.prepare(query).expect("expected to prepare statement correctly");
    stmt.execute(&[(":title", &title), (":author", &author), (":price", &format!("{}",price))])
        .expect("expected to be able to insert into Books table");
    info!(target: "file", "Successfully created book: Author: {}, Title: {}, Price: {}", author, title, price);
}

pub fn get_book_id(title: String, author: String) -> i64 {
    let db = connect();
    let query = "SELECT id FROM books WHERE title = :title AND author = :author";
    let mut stmt = db.prepare(query).expect("expected to prepare statement correctly");

    let mut rows = stmt
        .query_map(&[(":title", &title), (":author", &author)], |row| row.get(0))
        .expect("expected to be able to get id from Books table");
    let id = rows.next().unwrap().unwrap();

    // Might remove in future, kinda messy
    // info!(target: "file", "Successfully got book id: {}", id);
    return id;
}

pub fn get_book_price(bid: i64) -> f64 {
    let db = connect();
    let query = "SELECT price FROM books WHERE id = :bid";
    let mut stmt = db.prepare(query).expect("expected to prepare statement correctly");
    
    let mut rows = stmt
        .query_map(&[(":bid", &format!("{}", bid))], |row| row.get(0))
        .expect("expected to be able to get price from Books table");
    let price = rows.next().unwrap().unwrap();

    info!(target: "file", "Successfully got book id: {}'s price of {}", bid, price);
    return price;
}
