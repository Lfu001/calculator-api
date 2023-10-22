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
    result: Option<f64>,
    operator: Operator,
    a: Option<f64>,
    b: Option<f64>,
}

impl MathResponse {
    pub fn new(
        result: Option<f64>,
        operator: Operator,
        a: Option<f64>,
        b: Option<f64>,
    ) -> MathResponse {
        MathResponse {
            result,
            operator,
            a,
            b,
        }
    }
}

fn parse_string_to_f64(s: Option<&String>) -> Option<f64> {
    match s {
        Some(str_ref) => str_ref.parse::<f64>().ok(),
        _ => None,
    }
}

pub fn extract_queries(req: &Request) -> (Option<f64>, Option<f64>) {
    let parsed_url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    let a = parse_string_to_f64(hash_query.get("a"));
    let b = parse_string_to_f64(hash_query.get("b"));
    (a, b)
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
