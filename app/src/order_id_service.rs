use base64::{CharacterSet, Config};
use chrono::{DateTime, FixedOffset};
use md5;
use strum_macros::Display;

pub(crate) type OrderId = String;

#[derive(Display, Clone, Copy)]
pub(crate) enum Region {
    NONE,
    EU,
}

#[derive(Display, Clone, Copy)]
pub(crate) enum Environment {
    DEV,
    TEST,
    PROD,
}

pub(crate) fn generate_order_id(region: Region, environment: Environment, timestamp: DateTime<FixedOffset>, salt: &str) -> OrderId {
    let value_to_hash = region.to_string() + &environment.to_string() + &timestamp.to_rfc3339() + &salt.to_string();
    let md5_sum = md5::compute(value_to_hash);

    let base64_string = base64::encode_config(md5_sum.0, Config::new(CharacterSet::Standard, false));
    let base64_without_underscore = replace_hyphen_underscore(base64_string);

    let region_identifier = build_region_identifier(region, environment);

    let base64_string_half_length = str::len(&base64_without_underscore) / 2;

    base64_without_underscore[..base64_string_half_length].to_string() + &region_identifier[..].to_string() + &base64_without_underscore[base64_string_half_length..].to_string()
}

fn replace_hyphen_underscore(input: String) -> String {
    str::replace(&input, "-", "!")
}

fn build_region_identifier(region: Region, environment: Environment) -> String {
    match environment {
        Environment::PROD => return format!("-{}-", environment.to_string()),
        _ => format!("-{}-{}-", region.to_string(), environment.to_string()),
    }
}

