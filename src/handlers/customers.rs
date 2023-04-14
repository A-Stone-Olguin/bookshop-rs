use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use regex::Regex;

use crate::db::customers::{self, get_customer_id};
use log::error;

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
    name = fix_whitespace(name.clone());
    address = fix_whitespace(address.clone());

    match validate_name_and_address(name.clone(), address.clone(), "create_customer".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };

    customers::create_customer(name, address);
    Ok(())
}

#[put("/updateAddress", data = "<customer>")]
pub fn update_address(customer: Json<Customer>) -> Result<String, String> {
    let mut name = match customer.name.clone() {
        Some(n) => n,
        None => return Err("No name provided".to_string()),
    };
    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No shipping_address provided".to_string()),
    };
    name = fix_whitespace(name.clone());
    address = fix_whitespace(address.clone());

    match validate_name_and_address(name.clone(), address.clone(), "update_address".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    let cid = get_customer_id(name.clone(), address.clone());
    customers::update_customer_address(cid, address.clone());
    let success_msg = format!("Successfully updated address for customer: {} to {}", name, address);
    Ok(success_msg)
}

#[get("/balance", format = "json", data = "<customer>")]
pub fn get_balance(customer: Json<Customer>) -> Result<String, String> {
    let mut name = match customer.name.clone() {
        Some(n) => n,
        None => return Err("No name provided".to_string()),
    };
    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No shipping_address provided".to_string()),
    };
    name = fix_whitespace(name.clone());
    address = fix_whitespace(address.clone());
    
    match validate_name_and_address(name.clone(), address.clone(), "get_balance".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };

    let cid = customers::get_customer_id(name.clone(), address);
    let balance = customers::get_customer_balance(cid);

    let result_string = format!("Customer {}, with customerID {}, has balance: ${:.2}", name, cid, balance);
    Ok(result_string)
}


#[put("/updateBalance", data = "<customer>")]
pub fn update_balance(customer: Json<Customer>) -> Result<String, String> {
    let mut name = match customer.name.clone() {
        Some(n) => n,
        None => return Err("No name provided".to_string()),
    };
    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No shipping_address provided".to_string()),
    };
    name = fix_whitespace(name.clone());
    address = fix_whitespace(address.clone());
    
    match validate_name_and_address(name.clone(), address.clone(), "update_balance".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };

    let balance = match customer.account_balance {
        Some(p) => p,
        None => return Err("No account_balance provided".to_string()),
    };
    match validate_balance(balance, "update_balance".to_string()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    let cid = get_customer_id(name.clone(), address);
    customers::update_customer_balance(cid, balance);

    let success_msg = format!("Successfully updated balance for customer: {} to ${:.2}", name, balance);
    Ok(success_msg)
}

fn validate_balance(balance : f64, function : String) -> Result<(), String> {
    if balance <= 0.00 {
        error!(target: "file", "Balance of 0.00 given in {}", function);
        let error_msg = format!("Please give a positive value (>0) for balance");
        return Err(error_msg);
    }
    
    // Unwraps the regex error to see if it's a valid regex, decimals no greater than 10000
    let re = Regex::new(r"\d{1,4}\.\d{2}$").unwrap();
    let valid = re.is_match(&balance.to_string());
    if !valid {
        error!(target: "file", "Invalid balance in {}: {}", function, balance);
        let error_msg = "Please input a valid balance of form X.YY: 0 <= X <= 9999, 0 <= Y <= 9".to_string();
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
fn validate_name_and_address(name : String, address : String, function : String) -> Result<(), String> {
    match validate_alphanumeric_input(name.clone(), "name".to_string(), function.clone()) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    match validate_alphanumeric_input(address.clone(), "address".to_string(), function) {
        Ok(()) => 0,
        Err(err_msg) => return Err(err_msg),
    };
    return Ok(());
}