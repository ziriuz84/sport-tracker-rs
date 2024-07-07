extern crate rocket;

#[rocket::get("/world")]
fn world() -> &'static str {
    "Hello World maddafakka!"
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", rocket::routes![world])
}
