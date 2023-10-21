use super::{MathResponse, Operator};
use rocket::serde::json::Json;

#[get("/divide?<a>&<b>")]
pub fn divide(a: f64, b: f64) -> Json<MathResponse> {
    let result = if b != 0.0 { Some(a / b) } else { None };
    let response = MathResponse {
        result,
        operator: Operator::DIVIDE,
        a,
        b,
    };
    Json(response)
}
