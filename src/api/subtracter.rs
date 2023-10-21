use super::{MathResponse, Operator};
use rocket::serde::json::Json;

#[get("/subtract?<a>&<b>")]
pub fn subtract(a: f64, b: f64) -> Json<MathResponse> {
    let result = Some(a - b);
    let response = MathResponse {
        result,
        operator: Operator::SUBTRACT,
        a,
        b,
    };
    Json(response)
}
