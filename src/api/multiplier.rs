use super::{MathResponse, Operator};
use rocket::serde::json::Json;

#[get("/multiply?<a>&<b>")]
pub fn multiply(a: f64, b: f64) -> Json<MathResponse> {
    let result = Some(a * b);
    let response = MathResponse {
        result,
        operator: Operator::MULTIPLY,
        a,
        b,
    };
    Json(response)
}
