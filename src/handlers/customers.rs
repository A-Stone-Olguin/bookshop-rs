use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::customers;

// TODO Change validate_name process into a single function call
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    id: Option<i64>,
    name: Option<String>,
    shipping_address: Option<String>,
    account_balance: Option<f64>,
}

#[post("/new", data = "<customer>")]
pub fn create_customer(customer: Json<Customer>) -> Result<(), String> {
    let mut name = match customer.name.clone() {
        Some(n) => n,
        None => return Err("No name provided".to_string()),
    };
    name = name.trim().to_string(); // Remove leading and trailing whitespace:
    let valid_name = validate_name(name.clone(), String::from("name"));
    if !valid_name {
        error!("Invalid name in create_customer: {}", name);
    }
    match valid_name {
        true => 0,  // Correct return code
        false => return Err("Please input a valid name:\n\t Please use only alphabet and numeric values.\n\t Please do not input empty space.".to_string()),
    };

    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".to_string()),
    };
    address = address.trim().to_string(); // Remove leading and trailing whitespace:
    let valid_address = validate_name(address.clone(), String::from("address"));
    if !valid_address {
        error!("Invalid address in create_customer: {}", address);
    }
    match valid_address {
        true => 0,  // Correct return code
        false => return Err("Please input a valid address:\n\t Please use only alphabet and numeric values.\n\t Please do not input empty space.".to_string()),
    };

    customers::create_customer(name, address);
    Ok(())
}

// TODO change to require customer name not cid.
#[put("/updateAddress", data = "<customer>")]
pub fn update_address(customer: Json<Customer>) -> Result<(), String> {
    let cid = match customer.id.clone() {
        Some(i) => i,
        None => return Err("No id provided".to_string()),
    };
    match cid >= 0 {
        true => 0, // Correct return code
        false => return Err("Not a valid customer id. Must be >= 0".to_string()),
    };

    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".to_string()),
    };
    address = address.trim().to_string(); // Remove leading and trailing whitespace:
    let valid_address = validate_name(address.clone(), String::from("address"));
    if !valid_address {
        error!("Invalid address in create_customer: {}", address);
    }
    match valid_address {
        true => 0,  // Correct return code
        false => return Err("Please input a valid address:\n\t Please use only alphabet and numeric values.\n\t Please do not input empty space.".to_string()),
    };

    customers::update_customer_address(cid, address);
    Ok(())
}

// TODO change to only output balance
#[get("/balance", format = "json", data = "<customer>")]
pub fn get_balance(customer: Json<Customer>) -> Result<Json<Customer>, String> {
    let mut name = match customer.name.clone() {
        Some(n) => n,
        None => return Err("No name provided".to_string()),
    };
    name = name.trim().to_string(); // Remove leading and trailing whitespace:
    let valid_name = validate_name(name.clone(), String::from("name"));
    if !valid_name {
        error!("Invalid name in create_customer: {}", name);
    }
    match valid_name {
        true => 0,  // Correct return code
        false => return Err("Please input a valid name:\n\t Please use only alphabet and numeric values.\n\t Please do not input empty space.".to_string()),
    };

    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".to_string()),
    };
    address = address.trim().to_string(); // Remove leading and trailing whitespace:
    let valid_address = validate_name(address.clone(), String::from("address"));
    if !valid_address {
        error!("Invalid address in create_customer: {}", address);
    }
    match valid_address {
        true => 0,  // Correct return code
        false => return Err("Please input a valid address:\n\t Please use only alphabet and numeric values.\n\t Please do not input empty space.".to_string()),
    };

    let cid = customers::get_customer_id(name, address);
    let balance = customers::customer_balance(cid);
    Ok(Json(Customer {
        id: None,
        name: None,
        shipping_address: None,
        account_balance: Some(balance),
    }))
}

fn validate_name(name : String, field : String) -> bool{
    if name.is_empty() || name.chars().all(char::is_whitespace) {
        error!("Empty input given in create_book, field: {}", field);
        return false;
    }
    let valid = name.chars().all(|x| (x.is_alphanumeric() || x.is_whitespace()));
    valid 
}