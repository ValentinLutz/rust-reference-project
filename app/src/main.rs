use chrono::DateTime;

use order_id_service::{Environment, Region};

mod order_id_service;

#[cfg(test)]
mod order_id_service_test;

#[actix_web::main]
async fn main() {
    let timestamp = DateTime::parse_from_rfc3339("1980-01-01T00:00:00+00:00").unwrap();

    let order_id = order_id_service::generate_order_id(Region::NONE, Environment::DEV, timestamp, "1");

    println!("{}", order_id)
}
