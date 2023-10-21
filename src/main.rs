#[macro_use]
extern crate rocket;

mod api;

#[launch]
fn app() -> _ {
    rocket::build().mount(
        "/",
        routes![
            api::adder::add,
            api::subtracter::subtract,
            api::multiplier::multiply,
            api::divider::divide
        ],
    )
}
