pub mod adder;
pub mod divider;
pub mod multiplier;
pub mod subtracter;

use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
enum Operator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MathResponse {
    result: Option<f64>,
    operator: Operator,
    a: f64,
    b: f64,
}
