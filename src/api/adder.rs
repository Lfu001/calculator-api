use super::{MathResponse, Operator};
use rocket::serde::json::Json;

#[get("/add?<a>&<b>")]
pub fn add(a: f64, b: f64) -> Json<MathResponse> {
    let result = Some(a + b);
    let response = MathResponse {
        result,
        operator: Operator::ADD,
        a,
        b,
    };
    Json(response)
}
