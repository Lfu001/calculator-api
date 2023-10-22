use calculator_api::{build_json_response, extract_queries, MathResponse, Operator};
use vercel_runtime::{run, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let (a, b) = extract_queries(&req);
    let result = match (a, b) {
        (Some(a_val), Some(b_val)) => Some(a_val - b_val),
        _ => None,
    };

    let response = MathResponse::new(result, Operator::SUBTRACT, a, b);
    build_json_response(&response)
}
