use chrono::DateTime;
use test_case::test_case;

use crate::order_id_service::OrderId;

use super::*;

#[test_case(Region::NONE, Environment::DEV, "1", String::from("U0aZcaQks3V-NONE-DEV-kRXT8SBpIBg"))]
#[test_case(Region::NONE, Environment::DEV, "101", String::from("QO+bvfIlrSN-NONE-DEV-ahIAExJcKMQ"))]
#[test_case(Region::NONE, Environment::DEV, "10101", String::from("D9RBdmWvYel-NONE-DEV-Nq+/HvRG0EA"))]
#[test_case(Region::EU, Environment::TEST, "1", String::from("Mx6GlWsQBKe-EU-TEST-Yct/ziOmPUA"))]
#[test_case(Region::EU, Environment::TEST, "101", String::from("C2bVEMyuCQj-EU-TEST-Gjlu2rLLYwg"))]
#[test_case(Region::EU, Environment::TEST, "10101", String::from("NnE6ZjWFjnS-EU-TEST-w1gWJ/G6DlA"))]
#[test_case(Region::EU, Environment::PROD, "1", String::from("NSB2pdtsuG5-PROD-Hzk6lzUcyEQ"))]
#[test_case(Region::EU, Environment::PROD, "101", String::from("xiC4CYvcrGS-PROD-rcNU8O4J7+A"))]
#[test_case(Region::EU, Environment::PROD, "10101", String::from("s6VtRTc6LUi-PROD-l/eUfrPS3ew"))]
fn test_generate_order_id(region: Region, environment: Environment, salt: &str, expected: OrderId) {
    // GIVEN
    let timestamp = DateTime::parse_from_rfc3339("1980-01-01T00:00:00+00:00").unwrap();

    // WHEN
    let order_id = order_id_service::generate_order_id(region, environment, timestamp, salt);

    // THEN
    assert_eq!(order_id, expected);
}


