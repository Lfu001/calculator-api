use bigdecimal::BigDecimal;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use url::Url;
use vercel_runtime::{http::bad_request, Body, Error, Request, Response, StatusCode};

#[derive(Serialize)]
#[serde(crate = "serde")]
pub enum Operator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

#[derive(Serialize)]
#[serde(crate = "serde")]
pub struct MathResponse {
    result: String,
    operator: Operator,
    a: String,
    b: String,
}

impl MathResponse {
    pub fn new(result: String, operator: Operator, a: String, b: String) -> MathResponse {
        MathResponse {
            result,
            operator,
            a,
            b,
        }
    }
}

fn parse_string_to_big_decimal(s: Option<&String>) -> Option<BigDecimal> {
    match s {
        Some(str_ref) => str_ref.parse::<BigDecimal>().ok(),
        _ => None,
    }
}

pub fn extract_queries(req: &Request) -> (Option<BigDecimal>, Option<BigDecimal>) {
    let parsed_url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    let a = parse_string_to_big_decimal(hash_query.get("a"));
    let b = parse_string_to_big_decimal(hash_query.get("b"));
    (a, b)
}

pub fn option_big_decimal_to_string(val: Option<BigDecimal>) -> String {
    match val {
        Some(number) => number.to_string(),
        None => String::from("null"),
    }
}

pub fn build_json_response(res: &MathResponse) -> Result<Response<Body>, Error> {
    let serialized = json!(res);
    if let Ok(r) = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::Text(serialized.to_string()))
    {
        Ok(r)
    } else {
        bad_request("Response building failed")
    }
}
