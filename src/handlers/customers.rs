use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::customers::{self, get_customer_id};
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

#[put("/updateAddress", data = "<customer>")]
pub fn update_address(customer: Json<Customer>) -> Result<(), String> {
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

    match validate_alphanumeric_input(name.clone(), "name".to_string(), "update_address".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    match validate_alphanumeric_input(address.clone(), "shipping_address".to_string(), "update_address".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    let cid = get_customer_id(name, address.clone());
    customers::update_customer_address(cid, address);
    Ok(())
}

#[get("/balance", format = "json", data = "<customer>")]
pub fn get_balance(customer: Json<Customer>) -> Result<String, String> {
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

    let cid = customers::get_customer_id(name.clone(), address);
    let balance = customers::get_customer_balance(cid);

    let result_string = format!("Customer {} has balance: ${:.2}", name, balance);
    Ok(result_string)
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