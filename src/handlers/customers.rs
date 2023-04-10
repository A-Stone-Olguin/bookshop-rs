use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::customers;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    id: Option<i64>,
    name: Option<String>,
    shipping_address: Option<String>,
    account_balance: Option<f64>,
}

#[post("/new", data = "<customer>")]
pub fn create_customer(customer: Json<Customer>) -> Result<(), String> {
    let name = match customer.name.clone() {
        Some(n) => n,
        None => return Err("No name provided".to_string()),
    };
    let address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".to_string()),
    };

    customers::create_customer(name, address);
    Ok(())
}

#[put("/updateAddress", data = "<customer>")]
pub fn update_address(customer: Json<Customer>) -> Result<(), String> {
    let cid = match customer.id.clone() {
        Some(i) => i,
        None => return Err("No id provided".to_string()),
    };
    let address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".to_string()),
    };

    customers::update_customer_address(cid, address);
    Ok(())
}

#[get("/balance", format = "json", data = "<customer>")]
pub fn get_balance(customer: Json<Customer>) -> Result<Json<Customer>, String> {
    let name = match customer.name.clone() {
        Some(n) => n,
        None => return Err("No name provided".to_string()),
    };
    let address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".to_string()),
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
