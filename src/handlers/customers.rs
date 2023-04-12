use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::customers;
use log::error;

// TODO Add a way to check customer id function (Or print it)
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
    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No shipping_address provided".to_string()),
    };
    // Remove leading and trailing whitespace
    name = name.trim().to_string(); 
    address = address.trim().to_string();

    match validate_alphanumeric_input(name.clone(), "name".to_string(), "create_customer".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    match validate_alphanumeric_input(address.clone(), "shipping_address".to_string(), "create_customer".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
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
    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".to_string()),
    };
    // Remove leading and trailing whitespace
    address = address.trim().to_string(); 

    match cid >= 0 {
        true => 0, // Correct return code
        false => return Err("Not a valid customer id. Must be >= 0".to_string()),
    };
    match validate_alphanumeric_input(address.clone(), "address".to_string(), "update_address".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
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
    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".to_string()),
    };
    // Remove leading and trailing whitespace:
    name = name.trim().to_string(); 
    address = address.trim().to_string();
    
    match validate_alphanumeric_input(name.clone(), "name".to_string(), "get_balance".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    match validate_alphanumeric_input(address.clone(), "address".to_string(), "get_balance".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };

    let cid = customers::get_customer_id(name, address);
    let balance = customers::get_customer_balance(cid);
    Ok(Json(Customer {
        id: None,
        name: None,
        shipping_address: None,
        account_balance: Some(balance),
    }))
}

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