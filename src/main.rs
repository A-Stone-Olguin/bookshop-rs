#[macro_use]
extern crate rocket;
extern crate serde;

mod db;
mod handlers;
use log::info;

#[launch]
fn rocket() -> _ {
    log4rs::init_file("log4rs.yml", Default::default()).expect("Should initialize");
    info!(target: "file", "Rocket is initialized");
    rocket::build()
        .mount("/books", routes![handlers::books::create_book])
        .mount("/books", routes![handlers::books::get_price])
        .mount("/customers", routes![handlers::customers::create_customer])
        .mount("/customers", routes![handlers::customers::get_balance])
        .mount("/customers", routes![handlers::customers::update_address])
        .mount("/customers", routes![handlers::customers::update_balance])
        .mount("/orders", routes![handlers::orders::create_order])
        .mount("/orders", routes![handlers::orders::get_shipped])
        .mount("/orders", routes![handlers::orders::ship_order])
        .mount("/orders", routes![handlers::orders::get_status])
}
