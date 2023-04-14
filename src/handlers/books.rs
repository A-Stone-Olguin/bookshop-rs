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
    let mut author = match book.author.clone() {
        Some(a) => a,
        None => return Err("No author provided".to_string()),
    };
    title = fix_whitespace(title.clone());
    author = fix_whitespace(author.clone());

    match validate_title_and_author(title.clone(), author.clone(), "create_book".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    
    let price = match book.price {
        Some(p) => p,
        None => return Err("No price provided".to_string()),
    };
    match validate_price(price, "create_book".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };

    books::create_book(title, author, price);
    Ok(())
}

// yes this throws a warning, it's how we're going it
// get methods can consume data in my world
// because putting and posting to get the price makes less
// sense in my mind
#[get("/price", format = "json", data = "<book>")]
pub fn get_price(book: Json<Book>) -> Result<String, String> {
    let mut title = match book.title.clone() {
        Some(t) => t,
        None => return Err("No title provided".to_string()),
    };
    let mut author = match book.author.clone() {
        Some(a) => a,
        None => return Err("No author provided".to_string()),
    };
    title = fix_whitespace(title.clone());
    author = fix_whitespace(author.clone());

    match validate_title_and_author(title.clone(), author.clone(), "get_price".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };

    let bid = books::get_book_id(title.clone(), author);
    let price = books::get_book_price(bid);
    let result_string = format!("{}, with bookId {}, has price: ${:.2}", title, bid, price);
    Ok(result_string)
}

fn validate_price(price : f64, function : String) -> Result<(), String> {
    if price <= 0.00 {
        error!(target: "file", "Price of 0.00 given in {}", function);
        let error_msg = format!("Please give a positive value (>0) for price");
        return Err(error_msg);
    }
    // Adding .'s to integer prices for regex
    let mut price_string = price.to_string();
    match price_string.contains(".") {
        true => (),
        false => price_string.push('.'),
    };


    // Unwraps the regex error to see if it's a valid regex, decimals no greater than 10000
    let re = Regex::new(r"^\d{1,4}\.\d{0,2}$").unwrap();
    let valid = re.is_match(&price_string);
    if !valid {
        error!(target: "file", "Invalid price in {}: {}", function, price);
        let error_msg = "Please input a valid price of form X.YY: 0 <= X <= 9999, 0 <= Y <= 9".to_string();
        return Err(error_msg);
    }
    else {
        return Ok(());
    }
}

// Allows only alphabetic and numeric input for these fields, no weird ones like 💜 or < or /
fn validate_alphanumeric_input(input : String, field : String, function: String) -> Result<(), String>{
    if input.is_empty() || input.chars().all(char::is_whitespace) {
        error!(target: "file", "Empty input given in {}, field: {}", function, field);
        let error_msg = format!("Please input a valid {}:\n\t Please do not input only empty space.", field);
        return Err(error_msg);
    }
    let valid = input.chars().all(|x| (x.is_alphanumeric() || x.is_whitespace())); // Gets only 'word' characters and spaces

    if !valid {
        error!(target: "file", "Invalid {} in {}: {}", field, function, input);
        let error_msg = format!("Please input a valid {}:\n\t Please use only alphabet and numeric values.", field);
        return Err(error_msg);
    }
    else {
        return Ok(());
    }   
}

fn fix_whitespace(input : String) -> String {
    // Remove spaces at beginning and end of string
    let temp_string = input.trim().to_string();
    // Remove extra spaces within
    let ex_sp_re = Regex::new(r"\s+").unwrap();
    return ex_sp_re.replace_all(temp_string.as_str(), " ").to_string();
}

// Validates both the names and addresses, returning errors if they fail
fn validate_title_and_author(title : String, author : String, function: String) -> Result<(), String> {
    match validate_alphanumeric_input(title.clone(), "title".to_string(), function.clone()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    match validate_alphanumeric_input(author.clone(), "author".to_string(), function) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    return Ok(());
}