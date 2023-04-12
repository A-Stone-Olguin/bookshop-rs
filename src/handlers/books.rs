use crate::db::books;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use regex::Regex;
use log::error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    id: Option<i64>,
    title: Option<String>,
    author: Option<String>,
    price: Option<f64>,
}

#[post("/new", data = "<book>")]
pub fn create_book(book: Json<Book>) -> Result<(), String> {
    let mut title = match book.title.clone() {
        Some(t) => t,
        None => return Err("No title provided".to_string()),
    };
    title = title.trim().to_string(); // Remove leading and trailing whitespace:
    let valid_title = validate_name(title.clone(), String::from("title"));
    if !valid_title {
        error!("Invalid title in create_book: {}", title);
    }
    match valid_title {
        true => 0,  // Correct return code
        false => return Err("Please input a valid title:\n\t Please use only alphabet and numeric values.\n\t Please do not input empty space.".to_string()),
    };

    let mut author = match book.author.clone() {
        Some(a) => a,
        None => return Err("No author provided".to_string()),
    };
    author = author.trim().to_string(); // Remove leading and trailing whitespace:
    let valid_author = validate_name(author.clone(), String::from("author"));
    if !valid_author {
        error!("Invalid author in create_book: {}", author);
    }
    match valid_author {
        true => 0,  // Correct return code
        false => return Err("Please input a valid author:\n\t Please use only alphabet and numeric values.\n\t Please do not input empty space.".to_string()),
    };

    
    let price = match book.price.clone() {
        Some(p) => p,
        None => return Err("No price provided".to_string()),
    };
    let valid_price = validate_price(price);
    if !valid_price {
        error!("Invalid price in create_book: {}", price);
    }
    match valid_price {
        true => 0, // Correct return code
        false => return Err("Please input a valid price of form X.YY: 0 <= X <= 9999, Y is 0-9".to_string()),
    };

    books::create_book(title, author, price);
    Ok(())
}

// yes this throws a warning, it's how we're going it
// get methods can consume data in my world
// because putting and posting to get the price makes less
// sense in my mind
#[get("/price", format = "json", data = "<book>")]
pub fn get_price(book: Json<Book>) -> Result<Json<Book>, String> {
    let mut title = match book.title.clone() {
        Some(t) => t,
        None => return Err("No title provided".to_string()),
    };
    title = title.trim().to_string(); // Remove leading and trailing whitespace:
    let valid_title = validate_name(title.clone(), String::from("title"));
    if !valid_title {
        error!("Invalid title in get_price: {}", title);
    }
    match valid_title {
        true => 0,  // Correct return code
        false => return Err("Please input a valid title:\n\t Please use only alphabet and numeric values.\n\t Please do not input empty space.".to_string()),
    };

    let mut author = match book.author.clone() {
        Some(a) => a,
        None => return Err("No author provided".to_string()),
    };
    author = author.trim().to_string(); // Remove leading and trailing whitespace:
    let valid_author = validate_name(author.clone(), String::from("author"));
    if !valid_author {
        error!("Invalid author in get_price: {}", author);
    }
    match valid_author {
        true => 0,  // Correct return code
        false => return Err("Please input a valid author:\n\t Please use only alphabet and numeric values.\n\t Please do not input empty space.".to_string()),
    };

    let bid = books::get_book_id(title, author);
    let price = books::get_book_price(bid);
    Ok(Json(Book {
        id: None,
        title: None,
        author: None,
        price: Some(price),
    }))
}

fn validate_price(price : f64) -> bool {
    if price == 0.0 {
        return false;
    }

    // Unwraps the regex error to see if it's a valid regex, decimals no greater than 10000
    let re = Regex::new(r"\d{1,4}\.\d{2}0*").unwrap();
    let valid = re.is_match(&price.to_string());

    valid
}

fn validate_name(name : String, field : String) -> bool{
    if name.is_empty() || name.chars().all(char::is_whitespace) {
        error!("Empty input given in create_book, field: {}", field);
        return false;
    }
    let valid = name.chars().all(|x| (x.is_alphanumeric() || x.is_whitespace()));
    valid 
}
